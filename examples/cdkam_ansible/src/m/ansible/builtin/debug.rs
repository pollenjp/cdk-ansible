use cdk_ansible::{OptU, TaskModule};
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
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub msg: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub var: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub verbosity: OptU<i64>,
}
