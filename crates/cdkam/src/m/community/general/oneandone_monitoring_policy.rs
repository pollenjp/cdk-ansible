#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.oneandone_monitoring_policy")]
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
    pub add_ports: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub add_processes: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub add_servers: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub agent: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub api_url: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub auth_token: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub description: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub email: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub monitoring_policy: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub name: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ports: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub processes: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub remove_ports: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub remove_processes: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub remove_servers: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub state: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub thresholds: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub update_ports: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub update_processes: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_interval: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_timeout: OptU<i64>,
}
