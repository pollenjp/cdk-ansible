#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "ansible.builtin.set_fact")]
    pub module: ::serde_json::Map<String, ::serde_json::Value>,
}
impl TaskModule for Module {}

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::{Map, Number, Value};

    #[test]
    fn it_works() {
        let m = Module {
            module: [
                (
                    "val_string".to_owned(),
                    Value::String("string value".to_owned()),
                ),
                ("val_int".to_owned(), Value::Number(Number::from(1))),
                ("val_bool".to_owned(), Value::Bool(true)),
                (
                    "val_array".to_owned(),
                    vec![Value::String("a".to_owned()), Value::String("b".to_owned())].into(),
                ),
                (
                    "val_dict".to_owned(),
                    [
                        ("key1".to_owned(), Value::String("value1".to_owned())),
                        ("key2".to_owned(), Value::Number(Number::from(1))),
                    ]
                    .into_iter()
                    .collect::<Map<String, Value>>()
                    .into(),
                ),
            ]
            .into_iter()
            .collect::<Map<String, Value>>(),
        };
        let serialized = serde_json::to_string(&m).unwrap();
        assert_eq!(
            serialized,
            r#"{"ansible.builtin.set_fact":{"val_string":"string value","val_int":1,"val_bool":true,"val_array":["a","b"],"val_dict":{"key1":"value1","key2":1}}}"#
        );
    }
}
