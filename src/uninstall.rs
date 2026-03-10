use crate::config::Config;
use crate::service::InitSystem;
use crate::update::{detect_install_method, get_current_exe, InstallMethod};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn print_uninstall_instructions(config: &Config) -> Result<()> {
    let current_exe = get_current_exe()?;
    let install_method = detect_install_method(&current_exe);

    println!("ZeroClaw uninstall guide");
    println!("Detected binary: {}", current_exe.display());
    println!();
    println!("1) Remove background service first (recommended):");
    println!("   zeroclaw service stop || true");
    println!("   zeroclaw service uninstall || true");
    println!();
    println!("2) Remove the current binary:");

    match install_method {
        InstallMethod::Homebrew => {
            println!("   brew uninstall zeroclaw");
        }
        InstallMethod::SelfManaged => {
            if cfg!(windows) {
                println!("   cargo uninstall zeroclaw || true");
                println!("   del \"{}\"", current_exe.display());
            } else {
                println!("   rm -f \"{}\"", current_exe.display());
            }
        }
        InstallMethod::Unknown => {
            println!(
                "   Use the package manager that installed {}",
                current_exe.display()
            );
            println!("   or remove it manually if you installed it yourself.");
        }
    }

    println!();
    println!("3) Optional: remove retained runtime data:");
    for path in retained_data_paths(config) {
        println!("   {}", path.display());
    }

    Ok(())
}

pub fn run(config: &Config, init_system: InitSystem) -> Result<()> {
    println!("🦀 ZeroClaw Uninstall");
    println!();

    let current_exe = get_current_exe()?;
    let install_method = detect_install_method(&current_exe);
    println!("Current binary: {}", current_exe.display());
    println!();

    if let Err(err) =
        crate::service::handle_command(&crate::ServiceCommands::Uninstall, config, init_system)
    {
        eprintln!("⚠️  Warning: could not remove background service cleanly: {err}");
    }

    match install_method {
        InstallMethod::Homebrew => uninstall_homebrew(),
        InstallMethod::SelfManaged => uninstall_self_managed(&current_exe),
        InstallMethod::Unknown => {
            println!("Detected install method: unknown");
            println!("ZeroClaw did not remove the binary automatically.");
            println!("Run `zeroclaw uninstall --instructions` for the safest manual steps.");
            bail!("Unsupported install location: {}", current_exe.display())
        }
    }?;

    println!();
    println!("Retained runtime data:");
    for path in retained_data_paths(config) {
        println!("  {}", path.display());
    }
    println!("Remove those paths manually if you want a full cleanup.");

    Ok(())
}

fn uninstall_homebrew() -> Result<()> {
    println!("Detected install method: Homebrew");
    let status = Command::new("brew")
        .args(["uninstall", "zeroclaw"])
        .status()
        .context(
            "Detected Homebrew-managed install, but failed to run `brew uninstall zeroclaw`",
        )?;

    if !status.success() {
        bail!("`brew uninstall zeroclaw` exited with status {status}");
    }

    println!("✅ Removed Homebrew-managed ZeroClaw");
    Ok(())
}

fn uninstall_self_managed(current_exe: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        let _ = current_exe;
        bail!(
            "Automatic binary removal is not supported on Windows while zeroclaw is running. \
             Run `zeroclaw uninstall --instructions`."
        );
    }

    #[cfg(not(windows))]
    {
        let resolved = fs::canonicalize(current_exe).unwrap_or_else(|_| current_exe.to_path_buf());
        fs::remove_file(&resolved)
            .with_context(|| format!("Failed to remove {}", resolved.display()))?;
        println!("Detected install method: self-managed binary");
        println!("✅ Removed binary: {}", resolved.display());
        Ok(())
    }
}

fn retained_data_paths(config: &Config) -> Vec<PathBuf> {
    let mut paths = vec![config.workspace_dir.clone()];

    let config_root = config
        .config_path
        .parent()
        .map_or_else(|| config.config_path.clone(), PathBuf::from);
    if !paths.contains(&config_root) {
        paths.push(config_root);
    }

    paths.sort();
    paths.dedup();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_data_paths_include_workspace_and_config_root() {
        let mut config = Config::default();
        config.workspace_dir = PathBuf::from("/tmp/zeroclaw-workspace");
        config.config_path = PathBuf::from("/tmp/zeroclaw-config/config.toml");

        let paths = retained_data_paths(&config);

        assert_eq!(
            paths,
            vec![
                PathBuf::from("/tmp/zeroclaw-config"),
                PathBuf::from("/tmp/zeroclaw-workspace")
            ]
        );
    }

    #[test]
    fn retained_data_paths_include_nested_workspace_and_config_root() {
        let mut config = Config::default();
        config.workspace_dir = PathBuf::from("/tmp/.zeroclaw/workspace");
        config.config_path = PathBuf::from("/tmp/.zeroclaw/config.toml");

        let paths = retained_data_paths(&config);

        assert_eq!(
            paths,
            vec![
                PathBuf::from("/tmp/.zeroclaw"),
                PathBuf::from("/tmp/.zeroclaw/workspace")
            ]
        );
    }
}
