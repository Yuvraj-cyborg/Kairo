use crate::{config::SiteConfig, io, parser, render};
use std::path::PathBuf;
use anyhow::Result;

pub fn build_site(cfg: &SiteConfig) -> Result<()> {
    let vault = cfg.vault_path.clone().unwrap_or(PathBuf::from("."));
    let md_files = io::read_markdowns(vault.to_string_lossy().to_string());

    for file in md_files {
        let content = io::read_file(&file)?;
        let html_body = parser::parse_markdown(&content);

        let final_html = render::render_page(&cfg.title, &html_body);

        let mut out_path = cfg.output_dir.clone().unwrap_or(PathBuf::from("public"));
        let filename = file.file_stem().unwrap().to_string_lossy().to_string() + ".html";
        out_path.push(filename);

        io::write_file(&out_path, &final_html)?;
        println!("Built {:?}", out_path);
    }

    Ok(())
}

