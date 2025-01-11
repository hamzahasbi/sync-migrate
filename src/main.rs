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
use std::str;
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
    println!("Config: {:#?}", config_path);
    // Create the backup directory
    fs::create_dir_all(&backup_dir).expect("Failed to create backup directory");
    let  ignore = config.omit_folders;

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
    let _backup_via_package_manager = backup_via_package_manager(config.packages_managers);
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

fn backup_via_package_manager(track: Vec<String>) -> Result<()> {
    // Get the appropriate directory for storing our files
    let packages_dir = get_data_dir()?;
    println!("Storing package lists in: {:?}", packages_dir);

    if track.contains(&"composer".to_string()) {
      // Execute the composer command and write output to composer.txt
      let composer_output = get_composer_packages().expect("Failed to get composer packages");
      let mut composer_file = create_file(&packages_dir, "composer.txt")?;
      for package in composer_output {
        writeln!(composer_file, "{}", package)?;
      }
    }

    if track.contains(&"npm".to_string()) {
      // Execute the npm command and write output to node.txt
      let npm_output = Command::new("npm")
          .arg("list")
          .arg("-g")
          .arg("--depth=0")
          .output()
          .context("Failed to execute npm command")?;

      let mut node_file: fs::File = create_file_mode(&packages_dir, "node.txt", 'w')?;
      write!(node_file, "{}", String::from_utf8_lossy(&npm_output.stdout).into_owned()).context("Failed to write to node.txt")?;

    }

    if track.contains(&"pnpm".to_string()) {
      // Execute the pnpm command and write output to node.txt
      let pnpm_output = Command::new("pnpm")
        .arg("list")
        .arg("-g")
        .arg("--depth=0")
        .output()
        .context("Failed to execute pnpm command")?;

      let mut node_file: fs::File = create_file_mode(&packages_dir, "node.txt", 'a')?;
      write!(node_file, "{}", String::from_utf8_lossy(&pnpm_output.stdout).into_owned()).context("Failed to write to node.txt")?;
    }

    if track.contains(&"yarn".to_string()) {
        // Execute the yarn command and append output to node.txt
      let yarn_output = Command::new("yarn")
        .arg("global")
        .arg("list")
        .arg("--depth=0")
        .output()
        .context("Failed to execute yarn command")?;

      let mut node_file: fs::File = create_file_mode(&packages_dir, "node.txt", 'a')?;
      writeln!(node_file, "{}", String::from_utf8_lossy(&yarn_output.stdout).into_owned()).context("Failed to write to node.txt")?;
    }

    if track.contains(&"bun".to_string()) {
      // Execute the bun command and append output to node.txt
      let bun_output = Command::new("bun")
      .arg("pm")
      .arg("ls")
      .arg("-g")
      .output()
      .context("Failed to execute bun command")?;

      let mut node_file: fs::File = create_file_mode(&packages_dir, "node.txt", 'a')?;
      writeln!(node_file, "{}", String::from_utf8_lossy(&bun_output.stdout).into_owned()).context("Failed to write to node.txt")?;

    }
    let _normalize_node_file = node_normalizer( format!("{}/node.txt", packages_dir.display()).as_str());

    if track.contains(&"brew".to_string()) {
      // Execute the brew command and write output to brew.txt
      let brew_output = Command::new("brew")
          .arg("list")
          .output()
          .context("Failed to execute brew command")?;

      let mut brew_file = create_file(&packages_dir, "brew.txt")?;
      brew_file.write_all(&brew_output.stdout)
          .context("Failed to write to brew.txt")?;
    }

    if track.contains(&"gem".to_string()) {
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
    }

    if track.contains(&"cargo".to_string()) {
      // Execute the cargo command, process output, and write to rust.txt
      let cargo_output = Command::new("cargo")
          .arg("install")
          .arg("--list")
          .output()
          .context("Failed to execute cargo command")?;

      let cargo_stdout: BufReader<&[u8]> = BufReader::new(&cargo_output.stdout[..]);
      let mut rust_file = create_file(&packages_dir, "rust.txt")?;
      for line in cargo_stdout.lines() {
        if let Ok(line) = line {
            if let Some(package_name) = line.split_whitespace().next() {
                writeln!(rust_file, "{}", package_name)
                    .context("Failed to write to rust.txt")?;
            }
        }
      }
    }

    if track.contains(&"pip".to_string()) {
      // Execute the pip command and write output to pip.txt
      let pip_output = Command::new("pip3")
        .arg("freeze")
        .output()
        .context("Failed to execute pip command")?;
      // Create the output file
      let mut pip_file = create_file(&packages_dir, "python.txt")?;

      // Process the output to extract package names
      let output_str = String::from_utf8_lossy(&pip_output.stdout);
      for line in output_str.lines() {
          if line.contains("wheel") {
            continue;
          }
          if let Some(package_name) = line.split("==").next() {
              writeln!(pip_file, "{}", package_name.trim())?; // Write only the package name
          }
      }
    }
    println!("All package lists have been saved successfully accordinng to ~/dotporter/backup_config.toml");
    Ok(())
}

fn create_file_mode(base_dir: &Path, filename: &str, mode: char) -> Result<fs::File> {
  let file_path = base_dir.join(filename);

  let mut options = OpenOptions::new();
  options.write(true);

  if mode == 'a' {
    options.append(true);
  } else {
    options.create(true).truncate(true);
  }
  options.open(&file_path)
  .with_context(|| format!("Failed to create file: {:?}", file_path))
}

fn get_composer_packages() -> Result<Vec<String>, Box<dyn std::error::Error>> {
  // Execute the composer command
  let composer_output = Command::new("composer")
      .arg("global")
      .arg("show")
      .output()?;

  // Check if the command was successful
  if !composer_output.status.success() {
      return Err("Failed to execute composer command".into());
  }

  // Convert the output to a string
  let output_str = str::from_utf8(&composer_output.stdout)?;

  // Filter the output to get package names
  let packages: Vec<String> = output_str
      .lines()
      .filter_map(|line| {
          if let Some(package_name) = line.split_whitespace().next() {
              if package_name.contains('/') {
                  return Some(package_name.to_string());
              }
          }
          None
      })
      .collect();

  Ok(packages)
}

fn node_normalizer(file_path: &str) -> std::io::Result<()> {
  // Read the original contents of the file
  let file = fs::File::open(file_path)?;
  let reader = std::io::BufReader::new(file);
  let mut lines: Vec<String> = Vec::new();

  for line in reader.lines() {
      let line = line?;
      // Skip lines that do not contain "@"
      if !line.contains('@') {
          continue;
      }
      let parts: Vec<&str> = line.split_whitespace().collect();
      // Normalize the line to keep only the package name
      // Assuming the package name is the first part that contains "@"
      for part in parts {
        if part.contains('@') {
            let package_name = part.trim_matches('"'); // Remove quotes
            lines.push(package_name.trim().to_string()); // Collect the package names
            break; // Exit the loop after finding the first package name
        }
    }
  }

  // Write the processed lines back to the same file
  let mut output_file = fs::File::create(file_path)?; // Overwrite the original file
  for package in lines {
      writeln!(output_file, "{}", package)?; // Write each package name to the file
  }

  Ok(())
}
