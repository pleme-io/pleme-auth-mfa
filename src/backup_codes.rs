//! Backup code generation and validation

use rand::{Rng, thread_rng};

/// Backup code generator
pub struct BackupCodeGenerator {
    code_length: usize,
    count: usize,
}

impl BackupCodeGenerator {
    pub fn new(code_length: usize, count: usize) -> Self {
        Self { code_length, count }
    }

    /// Generate backup codes
    pub fn generate(&self) -> Vec<String> {
        let mut codes = Vec::with_capacity(self.count);
        let mut rng = thread_rng();

        for _ in 0..self.count {
            let code: String = (0..self.code_length)
                .map(|_| {
                    let digit = rng.gen_range(0..10);
                    char::from_digit(digit, 10).unwrap()
                })
                .collect();
            codes.push(code);
        }

        codes
    }

    /// Check if a code is valid (exists in the list)
    pub fn is_valid(&self, code: &str, backup_codes: &[String]) -> bool {
        backup_codes.iter().any(|c| c == code)
    }
}
