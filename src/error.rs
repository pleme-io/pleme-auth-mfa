//! MFA error types

#[derive(Debug, thiserror::Error)]
pub enum MfaError {
    #[error("MFA setup failed")]
    SetupFailed,

    #[error("MFA setup expired")]
    SetupExpired,

    #[error("Invalid MFA code")]
    InvalidCode,

    #[error("MFA not enabled")]
    NotEnabled,

    #[error("Redis error: {0}")]
    Redis(String),

    #[error("QR code generation failed")]
    QrCodeFailed,
}
