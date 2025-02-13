//! This application loads compliance rules from a JSON file and parses them into a vector of `ComplianceRule` structs.
//! The JSON file is loaded from the `../../compliance-rules/rules.json` file.
//! The JSON file contains an array of objects, each representing a compliance rule.
//! The application uses the `serde` crate to deserialize the JSON into a vector of `ComplianceRule` structs.

use serde::Deserialize;
use serde_json;

// Load the rules.json file to provide configs
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
    // Deserialize the JSON string into a vector of ComplianceRule structs
    // and return the vector. Note this is not safe and will panic if the JSON
    // is not in the expected format. This is fine for this example, but in a
    // real application we would want to handle this error more gracefully.
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

fn main() {
    // Load the compliance rules from the JSON file
    let rules = load_rules();
    // Print the loaded rules in a pretty format
    println!("{:#?}", rules);
}
