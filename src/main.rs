use std::os::unix::fs::MetadataExt;
use std::env;
use std::fs; // Import env module
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{Write, BufRead, BufReader};
use anyhow::{Context, Result};
use fs_extra::dir::copy as copy_dir_all;
use fs_extra::dir::CopyOptions;
use directories::BaseDirs;
mod mod_backup;
use mod_backup::BackupConfig;
fn check_ownership(path: &Path) -> bool {
  let metadata = match fs::metadata(path) {
      Ok(meta) => meta,
      Err(e) => {
          eprintln!("Unable to read metadata for '{}': {}", path.display(), e);
          return false;
      }
  };
  let owner_id = metadata.uid();
  let current_user_id: u32 = unsafe { libc::getuid() };
  owner_id == current_user_id
}
fn main() {
    // Get the current timestamp
    // let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_dir = format!("./dotfiles"); // Specify your backup path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let backup_path = current_dir.join("dotfiles");

    // Get config directory
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
    // Create the backup directory
    fs::create_dir_all(&backup_dir).expect("Failed to create backup directory");
    let  ignore = config.omit_folders;
    // Define the files and directories to back up
    // let files_to_backup = vec![
    //     (format!("{}/.zshrc", env::var("HOME").unwrap()), ".zshrc"), // Use $HOME
    //     (format!("{}/.config", env::var("HOME").unwrap()), ".config"), // Use $HOME
    // ];

    // // Copy each specified file/directory
    // for (source, name) in files_to_backup {
    //     let destination = Path::new(&backup_dir).join(name);
    //     if Path::new(&source).is_dir() {
    //         copy_dir_all(&source, &destination).expect("Failed to copy directory");
    //     } else {
    //         fs::copy(&source, &destination).expect("Failed to copy file");
    //     }
    // }

    let mut options = CopyOptions::new(); //Initialize default values for CopyOptions
    options.overwrite = true; // To mirror copy the whole structure of the source directory
    options.copy_inside = true; // To mirror copy the whole structure of the source directory
    // Copy all dotfiles and directories from the home directory
    let home_dir = env::var("HOME").unwrap(); // Use $HOME
    for entry in fs::read_dir(&home_dir).expect("Failed to read home directory") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        let pathname = path.as_path();
        let pathstr = path.file_name().unwrap().to_str().unwrap();
        if pathstr.starts_with('.') && pathstr != "." && pathstr != ".." {
          if ignore.contains(&pathstr.to_string()) || !check_ownership(&path) {
            eprintln!("Error: Check permissions or the config file under ommit_folders '{}'.", path.display());
            continue;
          }
          let destination: PathBuf = backup_path.join(path.file_name().unwrap());
          let destination_name = backup_path.as_path();
          println!("Backing up {} to {}", pathname.to_str().unwrap(), destination_name.to_str().unwrap());
          // unsafe { exit(0) };
          if path.is_dir() {
              let _ = copy_dir_all(&pathname, &destination_name, &options);
          } else if path.is_file() {
              fs::copy(&path, &destination).expect("Failed to copy dot file");
          }
        }
    }

    // println!("Backup completed successfully to {}", backup_dir);
    let _backup_via_package_manager = backup_via_package_manager();
    println!("Backup via package manager completed successfully");
}

fn get_data_dir() -> Result<PathBuf> {
  // Get current directory
  let current_dir = env::current_dir()
      .context("Failed to get current directory")?;

  // Create packages directory in current path
  let packages_dir = current_dir.join("packages");

  // Create all necessary directories
  fs::create_dir_all(&packages_dir)
      .with_context(|| format!("Failed to create directory: {:?}", packages_dir))?;

  Ok(packages_dir)
}

fn create_file(base_dir: &Path, filename: &str) -> Result<fs::File> {
  let file_path = base_dir.join(filename);

  OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(&file_path)
      .with_context(|| format!("Failed to create file: {:?}", file_path))
}
// Function to copy a directory recursively
// fn copy_dir_all<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> std::io::Result<()> {
//     fs::create_dir_all(&dst)?;
//     for entry in fs::read_dir(src)? {
//         let entry = entry?;
//         let path = entry.path();
//         let dest_path = dst.as_ref().join(entry.file_name());

//         if path.is_dir() {
//             copy_dir_all(&path, &dest_path)?;
//         } else {
//             fs::copy(&path, &dest_path)?;
//         }
//     }
//     Ok(())
// }

fn backup_via_package_manager() -> Result<()> {
    // Get the appropriate directory for storing our files
    let packages_dir = get_data_dir()?;
    println!("Storing package lists in: {:?}", packages_dir);

    // Execute the composer command and write output to composer.txt
    let composer_output = Command::new("composer")
        .arg("global")
        .arg("show")
        .arg("-i")
        .output()
        .context("Failed to execute composer command")?;

    let mut composer_file = create_file(&packages_dir, "composer.txt")?;
    composer_file.write_all(&composer_output.stdout)
        .context("Failed to write to composer.txt")?;

    // Execute the npm command and write output to node.txt
    let npm_output = Command::new("npm")
        .arg("list")
        .arg("-g")
        .arg("--depth=0")
        .output()
        .context("Failed to execute npm command")?;

    let mut node_file = create_file(&packages_dir, "node.txt")?;
    node_file.write_all(&npm_output.stdout)
        .context("Failed to write to node.txt")?;

    // Execute the yarn command and append output to node.txt
    let yarn_output = Command::new("yarn")
        .arg("global")
        .arg("list")
        .arg("--depth=0")
        .output()
        .context("Failed to execute yarn command")?;

    node_file.write_all(&yarn_output.stdout)
        .context("Failed to append yarn output to node.txt")?;

    // Execute the bun command and append output to node.txt
    let bun_output = Command::new("bun")
        .arg("pm")
        .arg("ls")
        .arg("-g")
        .output()
        .context("Failed to execute bun command")?;

    let flattened_bun = flatten_bun_output(&bun_output.stdout);
    node_file.write_all(flattened_bun.as_bytes())
        .context("Failed to append bun output to node.txt")?;

    // Execute the brew command and write output to brew.txt
    let brew_output = Command::new("brew")
        .arg("list")
        .output()
        .context("Failed to execute brew command")?;

    let mut brew_file = create_file(&packages_dir, "brew.txt")?;
    brew_file.write_all(&brew_output.stdout)
        .context("Failed to write to brew.txt")?;

    // Execute the gem command and write output to gems.txt
    let gem_output = Command::new("ruby")
        .arg("-S")
        .arg("gem")
        .arg("list")
        .arg("--local")
        .output()
        .context("Failed to execute gem command")?;

    let mut gems_file = create_file(&packages_dir, "gems.txt")?;
    gems_file.write_all(&gem_output.stdout)
        .context("Failed to write to gems.txt")?;

    // Execute the cargo command, process output, and write to rust.txt
    let cargo_output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .context("Failed to execute cargo command")?;

    let cargo_stdout = BufReader::new(&cargo_output.stdout[..]);
    let mut rust_file = create_file(&packages_dir, "rust.txt")?;

    for line in cargo_stdout.lines() {
        if let Ok(line) = line {
            if let Some(package_name) = line.split_whitespace().next() {
                writeln!(rust_file, "{}", package_name)
                    .context("Failed to write to rust.txt")?;
            }
        }
    }

    println!("All package lists have been saved successfully!");
    Ok(())
}

fn flatten_bun_output(output: &[u8]) -> String {
  let output_str = String::from_utf8_lossy(output);
  let mut flattened = String::new();

  for line in output_str.lines() {
      let trimmed = line.trim();
      if !trimmed.is_empty() && !trimmed.starts_with("├") && !trimmed.starts_with("└") {
          // Remove any version information (assuming it's in parentheses)
          if let Some(package_name) = trimmed.split_whitespace().next() {
              flattened.push_str(package_name);
              flattened.push('\n');
          }
      }
  }

  flattened
}
