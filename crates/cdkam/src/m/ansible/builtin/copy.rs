#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible::OptU;
use cdk_ansible::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "ansible.builtin.copy")]
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
        rename = "attributes"
    )]
    pub attributes: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "backup"
    )]
    pub backup: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "checksum"
    )]
    pub checksum: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "content"
    )]
    pub content: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "decrypt"
    )]
    pub decrypt: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "dest"
    )]
    pub dest: OptU<std::path::PathBuf>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "directory_mode"
    )]
    pub directory_mode: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "follow"
    )]
    pub follow: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "force"
    )]
    pub force: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "group"
    )]
    pub group: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "local_follow"
    )]
    pub local_follow: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "mode"
    )]
    pub mode: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "owner"
    )]
    pub owner: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "remote_src"
    )]
    pub remote_src: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "selevel"
    )]
    pub selevel: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "serole"
    )]
    pub serole: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "setype"
    )]
    pub setype: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "seuser"
    )]
    pub seuser: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "src"
    )]
    pub src: OptU<std::path::PathBuf>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "unsafe_writes"
    )]
    pub unsafe_writes: OptU<bool>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "validate"
    )]
    pub validate: OptU<String>,
}
