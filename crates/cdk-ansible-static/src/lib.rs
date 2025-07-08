/**
 * Define environment variables.
 *
 * Prefix: "CDK_ANSIBLE_"
 *
 * env!: Required at build time.
 * option_env!: Optional at build time.
 *
 * std::env::var(): Required at runtime.
 *
 * When using IDEs, you need to set these environment variables at
 *   * VSCode: `rust-analyzer.cargo.extraEnv` in `.vscode/settings.json`
 *   * TODO: please write here for other IDEs
 */
use cdk_ansible_macro::{attr_hidden, attribute_env_vars_metadata};
pub struct EnvVars;

#[attribute_env_vars_metadata]
impl EnvVars {
    /// Used at build time via `build.rs`.
    #[attr_hidden]
    pub const CARGO_MANIFEST_DIR: &'static str = "CARGO_MANIFEST_DIR";
    #[attr_hidden]
    pub const CDK_ANSIBLE_COMMIT_HASH: &'static str = "CDK_ANSIBLE_COMMIT_HASH";
    #[attr_hidden]
    pub const CDK_ANSIBLE_COMMIT_SHORT_HASH: &'static str = "CDK_ANSIBLE_COMMIT_SHORT_HASH";
    #[attr_hidden]
    pub const CDK_ANSIBLE_COMMIT_DATE: &'static str = "CDK_ANSIBLE_COMMIT_DATE";
    #[attr_hidden]
    pub const CDK_ANSIBLE_LAST_TAG: &'static str = "CDK_ANSIBLE_LAST_TAG";
    #[attr_hidden]
    pub const CDK_ANSIBLE_LAST_TAG_DISTANCE: &'static str = "CDK_ANSIBLE_LAST_TAG_DISTANCE";
    #[attr_hidden]
    pub const CDK_ANSIBLE_LAST_TAG_DISTANCE_DIRTY: &'static str =
        "CDK_ANSIBLE_LAST_TAG_DISTANCE_DIRTY";
}
