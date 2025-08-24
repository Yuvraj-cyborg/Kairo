use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: Option<String>,
    pub theme: Option<String>,
    pub output_dir: Option<PathBuf>,
    pub vault_path: Option<PathBuf>,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "My Kairo Site".into(),
            base_url: None,
            theme: Some("kairo-classic".into()),
            output_dir: Some(PathBuf::from("public")),
            vault_path: Some(PathBuf::from(".")),
        }
    }
}

pub fn load_config(path: &Path) -> anyhow::Result<SiteConfig> {
    let s = std::fs::read_to_string(path)?;
    let cfg: SiteConfig = toml::from_str(&s)?;
    Ok(cfg)
}

pub fn write_default_config(path: &Path) -> anyhow::Result<()> {
    let cfg = SiteConfig::default();
    let toml = toml::to_string_pretty(&cfg)?;
    std::fs::write(path, toml)?;
    Ok(())
}
