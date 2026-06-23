use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Playbook {
    pub name: String,
    pub plays: Vec<Play>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Play {
    pub name: String,
    pub hosts: String,
    pub become_value: bool,
    pub vars: HashMap<String, String>,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub name: String,
    pub when: Option<String>,
    pub loop_items: Option<Vec<String>>,
    pub module: Module,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "params")]
pub enum Module {
    Apt {
        name: Option<Vec<String>>,
        state: Option<String>,
        update_cache: Option<bool>,
    },
    Yum {
        name: Option<Vec<String>>,
        state: Option<String>,
        update_cache: Option<bool>,
    },
    Copy {
        src: Option<String>,
        dest: String,
        content: Option<String>,
        owner: Option<String>,
        group: Option<String>,
        mode: Option<String>,
    },
    Service {
        name: String,
        state: String,
        enabled: Option<bool>,
    },
    File {
        path: String,
        src: Option<String>,
        owner: Option<String>,
        group: Option<String>,
        mode: Option<String>,
        state: Option<String>,
    },
    User {
        name: String,
        state: Option<String>,
        groups: Option<Vec<String>>,
        shell: Option<String>,
        home: Option<String>,
    },
    Ping,
    Command {
        command: String,
        chdir: Option<String>,
    },
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    // Testing serializing a playbook into YAML through my AST
    #[test]
    fn test_serialize_playbook() {
        let playbook = Playbook {
            name: "Test Playbook".to_string(),
            plays: vec![Play {
                name: "Test Play".to_string(),
                hosts: "all".to_string(),
                become_value: false,
                vars: HashMap::from([("env".to_string(), "production".to_string())]),
                tasks: vec![
                    Task {
                        name: "Install nginx".to_string(),
                        when: None,
                        loop_items: None,
                        module: Module::Apt {
                            name: Some(vec!["nginx".to_string()]),
                            state: Some("present".to_string()),
                            update_cache: None,
                        },
                    },
                    Task {
                        name: "Ping host".to_string(),
                        when: Some("env == 'production'".to_string()),
                        loop_items: None,
                        module: Module::Ping,
                    },
                ],
            }],
        };

        let yaml = serde_yaml::to_string(&playbook).unwrap();
        println!("{}", yaml);

        assert!(yaml.contains("Test Playbook"));
        assert!(yaml.contains("nginx"));
        assert!(yaml.contains("all"));
    }

    //Testing deserializing from a playbook into my AST
    #[test]
    fn test_deserialize_service_playbook() {
        let yaml = r#"
    name: Service Playbook
    plays:
      - name: Start nginx
        hosts: webservers
        become_value: true
        vars:
          env: staging
        tasks:
          - name: Start nginx service
            when: "env == 'staging'"
            loop_items: null
            module:
              type: Service
              params:
                name: nginx
                state: started
                enabled: true
          - name: Copy config
            when: null
            loop_items: null
            module:
              type: Copy
              params:
                src: /tmp/nginx.conf
                dest: /etc/nginx/nginx.conf
                owner: root
                group: root
                mode: "0644"
    "#;

        let playbook: Playbook = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(playbook.name, "Service Playbook");
        assert_eq!(playbook.plays[0].hosts, "webservers");
        assert_eq!(playbook.plays[0].become_value, true);
        assert_eq!(playbook.plays[0].tasks[0].name, "Start nginx service");
        assert_eq!(playbook.plays[0].tasks[1].name, "Copy config");
    }
}
