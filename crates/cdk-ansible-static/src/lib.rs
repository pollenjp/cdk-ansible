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
use cdk_ansible_macro::attribute_env_vars_metadata;

pub const CDK_ANSIBLE_COMMIT_HASH: Option<&str> = option_env!("CDK_ANSIBLE_COMMIT_HASH");
pub const CDK_ANSIBLE_COMMIT_SHORT_HASH: Option<&str> =
    option_env!("CDK_ANSIBLE_COMMIT_SHORT_HASH");
pub const CDK_ANSIBLE_COMMIT_DATE: Option<&str> = option_env!("CDK_ANSIBLE_COMMIT_DATE");
pub const CDK_ANSIBLE_LAST_TAG: Option<&str> = option_env!("CDK_ANSIBLE_LAST_TAG");
pub const CDK_ANSIBLE_LAST_TAG_DISTANCE: Option<&str> =
    option_env!("CDK_ANSIBLE_LAST_TAG_DISTANCE");

pub struct EnvVars;

#[attribute_env_vars_metadata]
impl EnvVars {
    pub const CDK_ANSIBLE_CONFIG_FILE: &'static str = "CDK_ANSIBLE_CONFIG_FILE";
}
