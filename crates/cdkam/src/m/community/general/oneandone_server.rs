#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.oneandone_server")]
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
    pub api_url: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub appliance: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub auth_token: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub auto_increment: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub cores_per_processor: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub count: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub datacenter: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub description: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub firewall_policy: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub fixed_instance_size: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub hdds: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub hostname: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub load_balancer: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub monitoring_policy: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub private_network: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ram: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub server: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub server_type: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ssh_key: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub state: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vcore: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_interval: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_timeout: OptU<i64>,
}
