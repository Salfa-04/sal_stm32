//!
//! # Config
//!

#[::toml_cfg::toml_config]
struct Config {
    #[default("")]
    pub version: &'static str,
}
