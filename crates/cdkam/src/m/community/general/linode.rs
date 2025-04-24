#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.linode")]
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
        rename = "additional_disks"
    )]
    pub additional_disks: OptU<Vec<::serde_json::Value>>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_bwin_enabled"
    )]
    pub alert_bwin_enabled: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_bwin_threshold"
    )]
    pub alert_bwin_threshold: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_bwout_enabled"
    )]
    pub alert_bwout_enabled: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_bwout_threshold"
    )]
    pub alert_bwout_threshold: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_bwquota_enabled"
    )]
    pub alert_bwquota_enabled: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_bwquota_threshold"
    )]
    pub alert_bwquota_threshold: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_cpu_enabled"
    )]
    pub alert_cpu_enabled: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_cpu_threshold"
    )]
    pub alert_cpu_threshold: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_diskio_enabled"
    )]
    pub alert_diskio_enabled: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alert_diskio_threshold"
    )]
    pub alert_diskio_threshold: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "api_key"
    )]
    pub api_key: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "backupweeklyday"
    )]
    pub backupweeklyday: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "backupwindow"
    )]
    pub backupwindow: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "datacenter"
    )]
    pub datacenter: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "displaygroup"
    )]
    pub displaygroup: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "distribution"
    )]
    pub distribution: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "kernel_id"
    )]
    pub kernel_id: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "linode_id"
    )]
    pub linode_id: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "name"
    )]
    pub name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "password"
    )]
    pub password: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "payment_term"
    )]
    pub payment_term: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "plan"
    )]
    pub plan: OptU<i64>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "private_ip"
    )]
    pub private_ip: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ssh_pub_key"
    )]
    pub ssh_pub_key: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "state"
    )]
    pub state: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "swap"
    )]
    pub swap: OptU<i64>,
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
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "watchdog"
    )]
    pub watchdog: OptU<bool>,
}
