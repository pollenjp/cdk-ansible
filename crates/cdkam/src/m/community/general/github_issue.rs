#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.github_issue")]
    pub module: Args,
}
impl TaskModule for Module {}
#[derive(Clone, Debug, Serialize)]
pub struct Args {
    #[serde(flatten)]
    pub options: Opt,
}
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Opt {
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub action: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub issue: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub organization: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub repo: OptU<String>,
}
