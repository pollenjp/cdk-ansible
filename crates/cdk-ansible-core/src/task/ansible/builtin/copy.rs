use crate::core::{OptionUnset, TaskModule};
use serde::Serialize;

/// [ansible.builtin.copy](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/copy_module.html)
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Module {
    #[serde(rename = "ansible.builtin.copy")]
    pub module: Args,
}

impl TaskModule for Module {}

/// [ansible.builtin.copy parameters](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/copy_module.html#parameters)
#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct Args {
    // dest
    // path / required
    // Remote absolute path where the file should be copied to.
    pub dest: String,
    #[serde(flatten)]
    pub options: OptArgs,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct OptArgs {
    // attributes
    // aliases: attr
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub attributes: OptionUnset<String>,

    // backup
    // boolean
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub backup: OptionUnset<bool>,

    // checksum
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub checksum: OptionUnset<String>,

    // content
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub content: OptionUnset<String>,

    // decrypt
    // boolean
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub decrypt: OptionUnset<bool>,

    // directory_mode
    // any
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub directory_mode: OptionUnset<String>,

    // follow
    // boolean
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub follow: OptionUnset<bool>,

    // force
    // boolean
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub force: OptionUnset<bool>,

    // group
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub group: OptionUnset<String>,

    // local_follow
    // boolean
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub local_follow: OptionUnset<bool>,

    // mode
    // any
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub mode: OptionUnset<String>,

    // owner
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub owner: OptionUnset<String>,

    // remote_src
    // boolean
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub remote_src: OptionUnset<bool>,

    // selevel
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub selevel: OptionUnset<String>,

    // serole
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub serole: OptionUnset<String>,

    // setype
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub setype: OptionUnset<String>,

    // seuser
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub seuser: OptionUnset<String>,

    // src
    // path
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub src: OptionUnset<String>,

    // unsafe_writes
    // boolean
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub unsafe_writes: OptionUnset<bool>,

    // validate
    // string
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub validate: OptionUnset<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansible_builtin_copy_task_module() {
        let task_module = Module {
            module: Args {
                dest: "/path/to/dest".to_string(),
                options: OptArgs {
                    src: OptionUnset::Some("/path/to/src".to_string()),
                    ..Default::default()
                },
            },
        };
        let json = serde_json::to_string(&task_module).unwrap();
        assert_eq!(
            json,
            r#"{"ansible.builtin.copy":{"dest":"/path/to/dest","src":"/path/to/src"}}"#
        );
    }
}
