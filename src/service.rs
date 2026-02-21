//! MFA service implementation

use uuid::Uuid;
use totp_rs::{Algorithm, Secret, TOTP};
use rand::RngCore;
use qrcode::QrCode;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use redis::{aio::ConnectionManager, AsyncCommands};

use crate::{config::MfaConfig, error::MfaError, backup_codes::BackupCodeGenerator};

/// MFA service for TOTP and backup codes
pub struct MfaService {
    config: MfaConfig,
}

impl MfaService {
    /// Create new MFA service
    pub fn new(config: MfaConfig) -> Self {
        Self { config }
    }

    /// Get Redis key for temporary MFA secret storage
    fn mfa_setup_key(&self, user_id: Uuid) -> String {
        format!("{}:auth:mfa_setup:{}", self.config.product_id, user_id)
    }

    /// Setup TOTP for a user (returns QR code data URL)
    pub async fn setup_totp(
        &self,
        user_id: Uuid,
        user_email: &str,
        redis: &mut ConnectionManager,
    ) -> Result<String, MfaError> {
        // Generate secret
        let mut secret_bytes = [0u8; 20];
        rand::thread_rng().fill_bytes(&mut secret_bytes);
        let secret = Secret::Raw(secret_bytes.to_vec());
        let secret_string = secret.to_encoded().to_string();

        // Generate QR code URL
        let qr_code_url = format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}",
            &self.config.product_id,
            user_email,
            secret_string,
            &self.config.product_id
        );

        let qr_code = QrCode::new(&qr_code_url)
            .map_err(|_| MfaError::QrCodeFailed)?;

        // Convert QR code to data URL
        let qr_data_url = self.qr_code_to_data_url(qr_code)?;

        // Store the secret temporarily in Redis
        let key = self.mfa_setup_key(user_id);
        let _: () = redis.set_ex(&key, &secret_string, self.config.setup_secret_ttl)
            .await
            .map_err(|e| {
                tracing::error!("Failed to store MFA secret in Redis: {}", e);
                MfaError::SetupFailed
            })?;

        tracing::info!("MFA setup initiated for user {}", user_id);

        Ok(qr_data_url)
    }

    /// Get temporary secret from Redis
    pub async fn get_setup_secret(
        &self,
        user_id: Uuid,
        redis: &mut ConnectionManager,
    ) -> Result<String, MfaError> {
        let key = self.mfa_setup_key(user_id);
        let secret: Option<String> = redis.get(&key)
            .await
            .map_err(|e| {
                tracing::error!("Failed to retrieve MFA secret from Redis: {}", e);
                MfaError::SetupFailed
            })?;

        secret.ok_or_else(|| {
            tracing::warn!("MFA setup secret not found or expired for user {}", user_id);
            MfaError::SetupExpired
        })
    }

    /// Delete temporary secret from Redis
    pub async fn delete_setup_secret(
        &self,
        user_id: Uuid,
        redis: &mut ConnectionManager,
    ) -> Result<(), MfaError> {
        let key = self.mfa_setup_key(user_id);
        let _: () = redis.del(&key)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete MFA setup secret from Redis: {}", e);
                MfaError::Redis(e.to_string())
            })?;
        Ok(())
    }

    /// Verify TOTP code
    pub fn verify_totp_code(&self, secret: &str, totp_code: &str) -> Result<bool, MfaError> {
        let secret_bytes = Secret::Encoded(secret.to_string())
            .to_bytes()
            .map_err(|_| MfaError::InvalidCode)?;

        let totp = TOTP::new(
            Algorithm::SHA1,
            self.config.totp_digits,
            self.config.totp_skew,
            self.config.totp_step,
            secret_bytes,
            None,
            "".to_string(),
        ).map_err(|_| MfaError::SetupFailed)?;

        Ok(totp.check_current(totp_code).map_err(|_| MfaError::InvalidCode)?)
    }

    /// Generate backup codes
    pub fn generate_backup_codes(&self) -> Vec<String> {
        let generator = BackupCodeGenerator::new(
            self.config.backup_code_length,
            self.config.backup_code_count,
        );
        generator.generate()
    }

    /// Generate a TOTP secret (for testing purposes)
    ///
    /// This generates a random 20-byte secret encoded as base32.
    /// Useful for test scenarios where MFA needs to be force-enabled
    /// without going through the normal setup flow.
    pub fn generate_secret(&self) -> String {
        let mut secret_bytes = [0u8; 20];
        rand::thread_rng().fill_bytes(&mut secret_bytes);
        let secret = Secret::Raw(secret_bytes.to_vec());
        secret.to_encoded().to_string()
    }

    /// Verify backup code
    pub fn verify_backup_code(&self, code: &str, backup_codes: &[String]) -> bool {
        let generator = BackupCodeGenerator::new(
            self.config.backup_code_length,
            self.config.backup_code_count,
        );
        generator.is_valid(code, backup_codes)
    }

    /// Convert QR code to base64 data URL (SVG format)
    fn qr_code_to_data_url(&self, qr_code: QrCode) -> Result<String, MfaError> {
        use qrcode::render::svg;

        let svg_string = qr_code
            .render::<svg::Color>()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#FFFFFF"))
            .build();

        // Convert SVG to data URL
        let encoded = STANDARD.encode(svg_string.as_bytes());
        Ok(format!("data:image/svg+xml;base64,{}", encoded))
    }
}
