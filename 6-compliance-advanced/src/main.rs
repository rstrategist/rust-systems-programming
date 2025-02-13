//! This application loads compliance rules from a JSON file and parses them into a vector of `ComplianceRule` structs.
//! The JSON file is loaded from the `rules.json` file in the same directory as the source code.
//! The JSON file contains an array of objects, each representing a compliance rule.
//! The application uses the `serde` crate to deserialize the JSON into a vector of `ComplianceRule` structs.

use glob::glob;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::os::unix::fs::PermissionsExt;

// Load the regex rules.json file to provide configs
const JSON: &str = include_str!("../../compliance-rules/rules.json");

#[derive(Deserialize, Debug)]
struct ComplianceRule {
    path_regex: String,
    file_permissions: u32,
    required_files: Vec<String>,
    non_existent_files: Vec<String>,
}

impl ComplianceRule {
    /// Creates a new `ComplianceRule` instance.
    ///
    /// # Arguments
    ///
    /// * `path_regex` - A string representing the regex pattern for the file path.
    /// * `file_permissions` - An integer representing the required file permissions.
    /// * `required_files` - A vector of strings representing the required files.
    fn new(
        path_regex: String,
        file_permissions: u32,
        required_files: Vec<String>,
        non_existent_files: Vec<String>,
    ) -> Self {
        ComplianceRule {
            path_regex,
            file_permissions,
            required_files,
            non_existent_files,
        }
    }
}

// Load the rules.json file and parse it into a vector of ComplianceRule structs
fn load_rules() -> Vec<ComplianceRule> {
    // Load the compliance rules from the JSON file
    // Deserialize the JSON string into a vector of ComplianceRule structs
    // and return the vector. Note this is not safe and will panic if the JSON
    // is not in the expected format. This is fine for this example, but in a
    // real application we would want to handle this error more gracefully.
    println!("Loading compliance rules from JSON file...");
    let loaded_json: Vec<ComplianceRule> = serde_json::from_str(JSON).unwrap();

    let mut rules: Vec<ComplianceRule> = Vec::new();
    for rule in loaded_json {
        rules.push(ComplianceRule::new(
            rule.path_regex,
            rule.file_permissions,
            rule.required_files,
            rule.non_existent_files,
        ));
    }
    rules
}

fn apply_rules(rules: Vec<ComplianceRule>) {
    // Iterate over the rules and apply them to the file system
    println!("Applying compliance rules...");
    for rule in rules {
        let mut seen_files: Vec<String> = Vec::new();
        for entry in glob(&rule.path_regex).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if path.is_dir() {
                        continue;
                    }
                    seen_files.push(path.to_str().unwrap().to_string());
                    let metadata = fs::metadata(&path).unwrap();
                    if metadata.permissions().mode() != rule.file_permissions {
                        println!(
                            "[FAIL] Incorrect file permissions for path: {:?}",
                            path.display()
                        );
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }

        for file in rule.required_files {
            if !seen_files.contains(&file) {
                println!(
                    "[FAIL] Required file {file} not found in path {}: ",
                    rule.path_regex
                );
            }
        }
    }
}

fn main() {
    // Load the compliance rules from the JSON file
    let rules = load_rules();
    // Print the loaded rules in a pretty format
    println!("{:#?}", rules);
    // Apply the rules to the file system
    apply_rules(rules);
}
