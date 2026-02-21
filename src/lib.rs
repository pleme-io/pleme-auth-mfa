//! Multi-factor authentication library
//!
//! Provides TOTP (Time-based One-Time Password) and backup code functionality:
//! - TOTP secret generation
//! - QR code generation for authenticator apps
//! - TOTP code verification
//! - Backup code generation and validation
//! - Redis-based temporary secret storage
//!
//! # Example
//! ```rust
//! use pleme_auth_mfa::{MfaService, MfaConfig};
//!
//! let config = MfaConfig::default();
//! let service = MfaService::new(config);
//!
//! // Setup TOTP
//! let qr_code = service.setup_totp(user_id, "user@example.com", redis).await?;
//!
//! // Verify code
//! let valid = service.verify_totp_code(&secret, "123456")?;
//! ```

mod service;
mod config;
mod error;
mod backup_codes;

pub use service::MfaService;
pub use config::MfaConfig;
pub use error::MfaError;
pub use backup_codes::BackupCodeGenerator;
