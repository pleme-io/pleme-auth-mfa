//! MFA configuration

/// MFA configuration
#[derive(Debug, Clone)]
pub struct MfaConfig {
    /// Product identifier for QR code issuer
    pub product_id: String,

    /// TOTP algorithm (default: SHA1)
    pub totp_algorithm: String,

    /// TOTP digits (default: 6)
    pub totp_digits: usize,

    /// TOTP step in seconds (default: 30)
    pub totp_step: u64,

    /// TOTP time skew tolerance (default: 1)
    pub totp_skew: u8,

    /// Number of backup codes to generate (default: 10)
    pub backup_code_count: usize,

    /// Backup code length (default: 8)
    pub backup_code_length: usize,

    /// Temporary secret TTL in seconds (default: 600 = 10 minutes)
    pub setup_secret_ttl: u64,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            product_id: "novaskyn".to_string(),
            totp_algorithm: "SHA1".to_string(),
            totp_digits: 6,
            totp_step: 30,
            totp_skew: 1,
            backup_code_count: 10,
            backup_code_length: 8,
            setup_secret_ttl: 600,
        }
    }
}
