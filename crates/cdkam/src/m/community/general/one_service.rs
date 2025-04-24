#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.one_service")]
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
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "api_password"
    )]
    pub api_password: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "api_url"
    )]
    pub api_url: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "api_username"
    )]
    pub api_username: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "cardinality"
    )]
    pub cardinality: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "custom_attrs"
    )]
    pub custom_attrs: OptU<indexmap::IndexMap<String, ::serde_json::Value>>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "force"
    )]
    pub force: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "group_id"
    )]
    pub group_id: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "mode"
    )]
    pub mode: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "owner_id"
    )]
    pub owner_id: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "role"
    )]
    pub role: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "service_id"
    )]
    pub service_id: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "service_name"
    )]
    pub service_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "state"
    )]
    pub state: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "template_id"
    )]
    pub template_id: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "template_name"
    )]
    pub template_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "unique"
    )]
    pub unique: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "wait"
    )]
    pub wait: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "wait_timeout"
    )]
    pub wait_timeout: OptU<i64>,
}
