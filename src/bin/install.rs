// src/install.rs

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;
use std::process::exit;
use directories::BaseDirs;
use std::fs; // Import env module
mod super::mod_backup;
use mod_backup::BackupConfig; // Import BackupConfig


pub fn init() -> BackupConfig {
  let base_dirs = BaseDirs::new().expect("Failed to get base directories");
  let config_dir = base_dirs.home_dir();
  let config_path = config_dir.join("dotporter/backup_config.toml");

  // Create config directory if it doesn't exist
  if !config_dir.exists() {
      fs::create_dir_all(config_dir.join("dotporter")).expect("Failed to create DotPorter directory");
  }

  // Load or create config
  let config = if config_path.exists() {
      BackupConfig::load(config_path.to_str().unwrap()).unwrap()
  } else {
      // Create default config file
      let _ = BackupConfig::create_default_config(config_path.to_str().unwrap());
      BackupConfig::default_config()
  };
  println!("Config: {:#?}", config_path);
  config
}
pub fn install_packages() -> io::Result<()> {
  // Load the BackupConfig
  let backup_config = init();
  let package_managers = backup_config.package_managers;
  let node_manager = backup_config.fnode_manager;
  let shell = backup_config.shell;

  for package_manager in package_managers {

    if package_manager == "gem" {
      let file = File::open("packages/gems.txt")?;
      let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(' ').collect();
            let name = parts[0];
            let versions: Vec<&str> = parts[1..].join(" ").split('(').nth(1).unwrap().trim_end_matches(')').split(',').collect();

            for version in versions {
                let cmd = format!("sudo gem install {} --version={}", name, version.trim());
                println!("{}", cmd);
                Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .status()?;
            }
        }
    } else if package_manager == "brew" {
        let brew_file_path = "packages/brew.txt"; // Path to the file containing brew packages
        let brew_file = File::open(brew_file_path)?;
        let brew_reader = io::BufReader::new(brew_file);

        for line in brew_reader.lines() {
            let package = line?;
            let cmd = format!("brew install {}", package.trim());
            println!("{}", cmd);
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status()?;
        }
    } else if package_manager == "cargo" {
      let file = File::open("packages/rust.txt")?;
      let reader = io::BufReader::new(file);
        // Logic for installing packages using Cargo
        for line in reader.lines() {
            let line = line?;
            let cmd = format!("cargo install {}", line.trim());
            println!("{}", cmd);
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status()?;
        }
    } else if package_manager == "pip" {
      let file = File::open("packages/python.txt")?;
      let reader = io::BufReader::new(file);
        // Logic for installing packages using Pip
        for line in reader.lines() {
            let line = line?;
            let cmd = format!("pip install {}", line.trim());
            println!("{}", cmd);
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status()?;
        }
    } else {
      let mut manager = String::new();
        if package_manager == "npm" {
          manager = "npm -g install".to_string();
        } else if package_manager == "yarn" {
          manager = "yarn global add".to_string();
        } else if package_manager == "pnpm" {
          manager = "pnpm --global add".to_string();
        } else {
          manager = "bun --globalinstall".to_string();
        }
      let file = File::open("packages/node.txt")?;
      let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let cmd = format!("{} {}", manager, line.trim());
            println!("{}", cmd);
            Command::new(shell)
                .arg("-c")
                .arg(&cmd)
                .status()?;
        }
    }
  }
  Ok(())
}
fn main() {
    if let Err(e) = install_packages() {
        eprintln!("Error installing packages: {}", e);
        exit(1);
    }
}
