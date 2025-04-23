#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.utm_dns_host")]
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
    pub address: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub address6: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub comment: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub headers: OptU<indexmap::IndexMap<String, ::serde_json::Value>>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub hostname: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub interface: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub name: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub resolved: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub resolved6: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub state: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub timeout: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub utm_host: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub utm_port: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub utm_protocol: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub utm_token: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub validate_certs: OptU<bool>,
}
