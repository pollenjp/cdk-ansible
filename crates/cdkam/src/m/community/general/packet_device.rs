#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.packet_device")]
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
    pub always_pxe: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub auth_token: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub count: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub count_offset: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub device_ids: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub facility: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub features: OptU<indexmap::IndexMap<String, ::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub hostnames: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ipxe_script_url: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub locked: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub operating_system: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub plan: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub project_id: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub state: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub tags: OptU<Vec<::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub user_data: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_for_public_i_pv: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub wait_timeout: OptU<i64>,
}
