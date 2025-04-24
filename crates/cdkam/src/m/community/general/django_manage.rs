#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.django_manage")]
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
        rename = "ack_venv_creation_deprecation"
    )]
    pub ack_venv_creation_deprecation: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "apps"
    )]
    pub apps: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "cache_table"
    )]
    pub cache_table: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "clear"
    )]
    pub clear: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "command"
    )]
    pub command: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "database"
    )]
    pub database: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "failfast"
    )]
    pub failfast: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "fixtures"
    )]
    pub fixtures: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "link"
    )]
    pub link: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "merge"
    )]
    pub merge: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "project_path"
    )]
    pub project_path: OptU<std::path::PathBuf>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "pythonpath"
    )]
    pub pythonpath: OptU<std::path::PathBuf>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "settings"
    )]
    pub settings: OptU<std::path::PathBuf>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "skip"
    )]
    pub skip: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "testrunner"
    )]
    pub testrunner: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "virtualenv"
    )]
    pub virtualenv: OptU<std::path::PathBuf>,
}
