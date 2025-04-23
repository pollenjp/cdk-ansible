#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.ibm_sa_vol_map")]
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
    pub cluster: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub endpoints: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub host: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub lun: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub override_x_: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub password: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub state: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub username: OptU<String>,
    #[serde(default = "OptU::default", skip_serializing_if = "OptU::is_unset")]
    pub vol: OptU<String>,
}
