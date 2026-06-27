use anyhow::{Context, Result};
use service_manager::{
    ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx, ServiceStopCtx,
    ServiceUninstallCtx,
};
use std::env::current_exe;

fn label() -> Result<ServiceLabel> {
    "dev.agents-mcp"
        .parse()
        .context("Failed to parse service label")
}

fn manager() -> Result<Box<dyn ServiceManager>> {
    service_manager::native::<dyn ServiceManager>().context("Failed to create service manager")
}

pub fn install() -> Result<()> {
    let manager = manager()?;
    let exe = current_exe().context("Failed to get current exe path")?;

    manager
        .install(ServiceInstallCtx {
            label: label()?,
            program: exe,
            args: vec!["--daemon".into()],
            autostart: true,
            contents: None,
        })
        .context("Failed to install service")?;

    manager
        .start(&ServiceStartCtx {
            label: label()?,
        })
        .context("Failed to start service")?;

    Ok(())
}

pub fn start() -> Result<()> {
    let manager = manager()?;
    manager
        .start(&ServiceStartCtx {
            label: label()?,
        })
        .context("Failed to start service")
}

pub fn stop() -> Result<()> {
    let manager = manager()?;
    manager
        .stop(&ServiceStopCtx {
            label: label()?,
        })
        .context("Failed to stop service")
}

pub fn uninstall() -> Result<()> {
    let manager = manager()?;
    let _ = manager.stop(&ServiceStopCtx {
        label: label()?,
    });
    manager
        .uninstall(ServiceUninstallCtx {
            label: label()?,
        })
        .context("Failed to uninstall service")
}

pub fn is_installed() -> bool {
    manager()
        .and_then(|m| m.list_available_services())
        .map(|services| services.iter().any(|s| s.label.to_string() == "dev.agents-mcp"))
        .unwrap_or(false)
}
