use crate::core::{OptionUnset, TaskModule};
use serde::Serialize;

/// [ansible.builtin.debug](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/debug_module.html)
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Module {
    #[serde(rename = "ansible.builtin.debug")]
    pub module: Args,
}

impl TaskModule for Module {}

#[derive(Serialize, Default, Clone, Debug, PartialEq)]
pub struct Args {
    #[serde(
        flatten,
        default = "MsgOrVar::default",
        skip_serializing_if = "MsgOrVar::is_unset"
    )]
    pub msg_or_var: MsgOrVar,
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub verbosity: OptionUnset<i64>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MsgOrVar {
    /// msg
    /// string
    /// The customized message that is printed. If omitted, prints a generic message.
    /// Default: "Hello world!"
    Msg(OptionUnset<String>),
    /// var
    /// string
    /// A variable name to debug.
    /// Mutually exclusive with the msg option.
    /// Be aware that this option already runs in Jinja2 context and has an implicit {{ }} wrapping,
    /// so you should not be using Jinja2 delimiters unless you are looking for double interpolation.
    Var(String),
}

impl MsgOrVar {
    pub fn is_unset(&self) -> bool {
        matches!(self, MsgOrVar::Msg(OptionUnset::Unset))
    }
}

impl Default for MsgOrVar {
    fn default() -> Self {
        MsgOrVar::Msg(OptionUnset::Some("Hello world!".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let debug = Module {
            module: Args {
                msg_or_var: MsgOrVar::default(),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&debug).unwrap();
        assert_eq!(json, r#"{"ansible.builtin.debug":{"msg":"Hello world!"}}"#);
    }

    #[test]
    fn test_msg() {
        let debug = Module {
            module: Args {
                msg_or_var: MsgOrVar::Msg(OptionUnset::Some("msg value".to_string())),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&debug).unwrap();
        assert_eq!(json, r#"{"ansible.builtin.debug":{"msg":"msg value"}}"#);
    }

    #[test]
    fn test_var() {
        let debug = Module {
            module: Args {
                msg_or_var: MsgOrVar::Var("var value".to_string()),
                ..Default::default()
            },
        };

        let json = serde_json::to_string(&debug).unwrap();
        assert_eq!(json, r#"{"ansible.builtin.debug":{"var":"var value"}}"#);
    }
}
