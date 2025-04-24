#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "ansible.builtin.meta")]
    pub module: String,
}
impl TaskModule for Module {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m = Module {
            module: "reset_connection".to_owned(),
        };
        let serialized = serde_json::to_string(&m).unwrap();
        assert_eq!(serialized, r#"{"ansible.builtin.meta":"reset_connection"}"#);
    }
}
