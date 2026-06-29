use std::fs;

//error handling
use anyhow::Result;

use crate::ast::Playbook;

pub fn generate_from_file(path: &str) -> Result<Playbook> {
    let content = fs::read_to_string(path)?;
    let playbook: Playbook = serde_json::from_str(&content)?;
    Ok(playbook)
}

fn turn_to_yaml(playbook: Playbook) -> Result<String> {
    let yaml = serde_yaml::to_string(&playbook)?;
    Ok(yaml)
}

pub fn output_file_yaml(playbook: Playbook, path: &str) -> Result<()> {
    let yaml = turn_to_yaml(playbook)?;
    fs::write(path, yaml)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_generate_from_file() {
        let json = r#"
        {
            "name": "Test Playbook",
            "plays": [
                {
                    "name": "Test Play",
                    "hosts": "all",
                    "become_value": false,
                    "vars": { "env": "production" },
                    "tasks": [
                        {
                            "name": "Install nginx",
                            "when": null,
                            "loop_items": null,
                            "module": {
                                "type": "Apt",
                                "params": {
                                    "name": ["nginx"],
                                    "state": "present",
                                    "update_cache": null
                                }
                            }
                        }
                    ]
                }
            ]
        }"#;

        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(json.as_bytes()).unwrap();
        let path = tmp.path().to_str().unwrap();

        let playbook = generate_from_file(path).unwrap();
        assert_eq!(playbook.name, "Test Playbook");
        assert_eq!(playbook.plays[0].hosts, "all");
        assert_eq!(playbook.plays[0].tasks[0].name, "Install nginx");
    }

    #[test]
    fn test_output_file_yaml() {
        let json = r#"
        {
            "name": "Output Test",
            "plays": [
                {
                    "name": "Play 1",
                    "hosts": "webservers",
                    "become_value": true,
                    "vars": {},
                    "tasks": [
                        {
                            "name": "Ping hosts",
                            "when": null,
                            "loop_items": null,
                            "module": {
                                "type": "Ping",
                                "params": null
                            }
                        }
                    ]
                }
            ]
        }"#;

        let mut tmp_in = NamedTempFile::new().unwrap();
        tmp_in.write_all(json.as_bytes()).unwrap();
        let input_path = tmp_in.path().to_str().unwrap();

        let tmp_out = NamedTempFile::new().unwrap();
        let output_path = tmp_out.path().to_str().unwrap();

        let playbook = generate_from_file(input_path).unwrap();
        output_file_yaml(playbook, output_path).unwrap();

        let written = std::fs::read_to_string(output_path).unwrap();
        assert!(written.contains("Output Test"));
        assert!(written.contains("webservers"));
    }
}
