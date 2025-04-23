#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.haproxy")]
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
    pub agent: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub backend: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub drain: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub fail_on_not_found: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub health: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub host: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub shutdown_sessions: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub socket: OptU<std::path::PathBuf>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub state: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_interval: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_retries: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub weight: OptU<String>,
}
