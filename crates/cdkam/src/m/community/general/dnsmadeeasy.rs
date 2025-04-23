#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.dnsmadeeasy")]
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
    pub account_key: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub account_secret: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub auto_failover: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub contact_list: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub domain: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub failover: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub http_file: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub http_fqdn: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub http_query_string: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ip1: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ip2: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ip3: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ip4: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub ip5: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub max_emails: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub monitor: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub port: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub protocol: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub record_name: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub record_ttl: OptU<i64>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub record_type: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub record_value: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub sandbox: OptU<bool>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub sensitivity: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub state: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub system_description: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub validate_certs: OptU<bool>,
}
