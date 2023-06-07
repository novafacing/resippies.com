use anyhow::Result;
use std::path::Path;
use tera::Tera;
use tokio::fs::read_dir;
use tracing::debug;

const TEMPLATE_DIR: &str = "templates";

pub async fn init_templates() -> Result<Tera> {
    // Get template dir relative to crate root
    let template_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(TEMPLATE_DIR);
    let mut tera = Tera::default();
    let mut dir = read_dir(&template_dir).await?;
    let mut templates = Vec::new();
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("html") {
            let template_path = path.clone();
            // Get template name as the path relative to the template dir, minus the extension
            let template_name = format!(
                "/{}",
                path.strip_prefix(&template_dir)?
                    .with_extension("")
                    .to_string_lossy()
            );

            debug!(
                "Adding template: {} => {}",
                template_name,
                template_path.display()
            );

            templates.push((template_path, Some(template_name)));
        }
    }

    tera.add_template_files(templates)?;

    Ok(tera)
}
