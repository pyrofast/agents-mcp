use std::path::Path;

use anyhow::Result;

use crate::config::read_universal_config;
use crate::detect::all_renderers;

pub fn sync_all(project_root: &Path) -> Result<()> {
    let source = project_root.join(".agents").join("mcp.json");
    let config = read_universal_config(&source)?;

    for renderer in all_renderers() {
        renderer.write(&config, project_root)?;
    }

    Ok(())
}
