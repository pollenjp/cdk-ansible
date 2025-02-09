use cdk_ansible::{OptionUnset, TaskModule};
use serde::Serialize;
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Module {
    #[serde(rename = "ansible.builtin.debug")]
    pub module: Args,
}
impl TaskModule for Module {}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Args {
    #[serde(flatten)]
    pub options: Opt,
}
#[derive(Clone, Debug, PartialEq, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Opt {
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub msg: OptionUnset<String>,
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub var: OptionUnset<String>,
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub verbosity: OptionUnset<i64>,
}
