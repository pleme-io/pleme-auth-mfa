# pleme-auth-mfa

Multi-factor authentication (TOTP + backup codes) for authentication services

## Installation

```toml
[dependencies]
pleme-auth-mfa = "0.1"
```

## Usage

```rust
use pleme_auth_mfa::{TotpManager, BackupCodes};

let totp = TotpManager::new();
let secret = totp.generate_secret(user_id)?;
let qr_code = totp.generate_qr(&secret, "user@example.com")?;
let valid = totp.verify(&secret, "123456")?;
```

## Development

This project uses [Nix](https://nixos.org/) for reproducible builds:

```bash
nix develop            # Dev shell with Rust toolchain
nix run .#check-all    # cargo fmt + clippy + test
nix run .#publish      # Publish to crates.io (--dry-run supported)
nix run .#regenerate   # Regenerate Cargo.nix
```

## License

MIT - see [LICENSE](LICENSE) for details.
