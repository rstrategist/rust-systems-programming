use log::{debug, error};
use serde_json;
use std::process::Command;

/// Runs a shell command and returns its output as a string.
///
/// # Arguments
///
/// * `command` - A string slice that holds the command to run.
///
/// # Returns
///
/// A string containing the command's output.
fn run_command(command: &str) -> String {
    debug!("Raw command: {command}");
    let args: Vec<&str> = command.split(" ").collect();
    debug!("Raw command split: {args:?}");
    let output = Command::new(args[0]).args(&args[1..]).output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.to_string()
        }
        Err(error) => {
            println!("Command failed: {command}");
            error!("error: {}", error);
            "".to_string()
        }
    }
}

/// Json that represents a filesystem entry.
#[derive(Debug, serde::Serialize)]
pub struct Filesystem {
    pub filesystem: String,
    pub size: String,
    pub used: String,
    pub available: String,
    pub use_percent: String,
    pub mounted_on: String,
}

impl Filesystem {
        /// Creates a new `Filesystem` instance.
    ///
    /// # Arguments
    ///
    /// * `filesystem` - The name of the filesystem.
    /// * `size` - The size of the filesystem.
    /// * `used` - The used space of the filesystem.
    /// * `available` - The available space of the filesystem.
    /// * `use_percent` - The percentage of used space.
    /// * `mounted_on` - The mount point of the filesystem.
    ///
    /// # Returns
    ///
    /// A new `Filesystem` instance.
    pub fn new(
        filesystem: String,
        size: String,
        used: String,
        available: String,
        use_percent: String,
        mounted_on: String,
    ) -> Filesystem {
        Filesystem {
            filesystem,
            size,
            used,
            available,
            use_percent,
            mounted_on,
        }
    }
}

/// Parses the output of the `df` command and returns a vector of `Filesystem` instances.
///
/// # Arguments
///
/// * `input` - A string slice that holds the output of the `df` command.
///
/// # Returns
///
/// A vector of `Filesystem` instances.
pub fn parse_df_output(input: &str) -> Vec<Filesystem> {
    // Parse output of df command:
    // Filesystem     1K-blocks     Used Available Use% Mounted on
    // overlay         32847680 14055420  17098164  46% /
    // tmpfs              65536        0     65536   0% /dev
    // shm                65536        0     65536   0% /dev/shm
    // /dev/root       30298176 13645408  16636384  46% /vscode
    // /dev/sdb1       46127956 20409036  23343344  47% /tmp
    // /dev/loop4      32847680 14055420  17098164  46% /workspaces

    let mut devices: Vec<Filesystem> = Vec::new();

    for line in input.lines() {
        if line.starts_with("Filesystem") {
            debug!("Skipping header line: {line}");
            continue;
        }
        if line.len() == 0 {
            debug!("skipping that is empty");
            continue;
        }
        // Only include filesystems that are devices (i.e. that start with '/')
        if !line.starts_with("/") {
            debug!("skipping that is not a device");
            continue;
        }
        let mut parts = line.split_whitespace();
        let filesystem = parts.next().unwrap().to_string();
        let size = parts.next().unwrap().to_string();
        let used = parts.next().unwrap().to_string();
        let available = parts.next().unwrap().to_string();
        let use_percent = parts.next().unwrap().to_string();
        let mounted_on = parts.next().unwrap().to_string();
        let device = Filesystem::new(filesystem, size, used, available, use_percent, mounted_on);
        devices.push(device);
    }
    devices
}

/// Finds the full path of an executable by searching in common system paths.
///
/// # Arguments
///
/// * `command` - The name of the command to find.
///
/// # Returns
///
/// A string containing the full path of the command if found, otherwise the original command.
pub fn which_executable(command: &str) -> String {
    // find in different system paths the executable
    let acceptable_paths = vec!["/bin", "/usr/bin", "/usr/local/bin"];
    for path in acceptable_paths {
        let full_path = format!("{}/{}", path, command);
        // if the path exists then return it
        if std::path::Path::new(&full_path).exists() {
            return full_path;
        }
    }
    return command.to_string();
}

/// Runs the `df` command and returns its output as a JSON value.
///
/// # Arguments
///
/// * `path` - The path to query (optional).
///
/// # Returns
///
/// A JSON value containing the parsed output of the `df` command.
pub fn run_df(path: &str) -> serde_json::Value {
    let command = "df";
    let output = run_command(command);
    if output.is_empty() {
        error!("No output from command: {command}");
        return serde_json::json!({});
    }

    // serialize the result of the parsing and return the JSON array
    let devices = parse_df_output(&output);
    if path.len() == 0 {
        return serde_json::json!(devices);
    } else {
        for device in devices {
            if device.mounted_on == path {
                return serde_json::json!(device);
            }
        }
        .into()
    }
}
