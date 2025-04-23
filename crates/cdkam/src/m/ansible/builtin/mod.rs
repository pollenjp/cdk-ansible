#[cfg(feature = "ansible-builtin-add_host")]
pub mod add_host;

#[cfg(feature = "ansible-builtin-apt")]
pub mod apt;

#[cfg(feature = "ansible-builtin-apt_key")]
pub mod apt_key;

#[cfg(feature = "ansible-builtin-apt_repository")]
pub mod apt_repository;

#[cfg(feature = "ansible-builtin-assemble")]
pub mod assemble;

#[cfg(feature = "ansible-builtin-assert")]
pub mod assert;

#[cfg(feature = "ansible-builtin-async_status")]
pub mod async_status;

#[cfg(feature = "ansible-builtin-blockinfile")]
pub mod blockinfile;

#[cfg(feature = "ansible-builtin-command")]
pub mod command;

#[cfg(feature = "ansible-builtin-copy")]
pub mod copy;

#[cfg(feature = "ansible-builtin-cron")]
pub mod cron;

#[cfg(feature = "ansible-builtin-deb822_repository")]
pub mod deb822_repository;

#[cfg(feature = "ansible-builtin-debconf")]
pub mod debconf;

#[cfg(feature = "ansible-builtin-debug")]
pub mod debug;

#[cfg(feature = "ansible-builtin-dnf")]
pub mod dnf;

#[cfg(feature = "ansible-builtin-dnf5")]
pub mod dnf5;

#[cfg(feature = "ansible-builtin-dpkg_selections")]
pub mod dpkg_selections;

#[cfg(feature = "ansible-builtin-expect")]
pub mod expect;

#[cfg(feature = "ansible-builtin-fail")]
pub mod fail;

#[cfg(feature = "ansible-builtin-fetch")]
pub mod fetch;

#[cfg(feature = "ansible-builtin-file")]
pub mod file;

#[cfg(feature = "ansible-builtin-find")]
pub mod find;

#[cfg(feature = "ansible-builtin-gather_facts")]
pub mod gather_facts;

#[cfg(feature = "ansible-builtin-get_url")]
pub mod get_url;

#[cfg(feature = "ansible-builtin-getent")]
pub mod getent;

#[cfg(feature = "ansible-builtin-git")]
pub mod git;

#[cfg(feature = "ansible-builtin-group")]
pub mod group;

#[cfg(feature = "ansible-builtin-group_by")]
pub mod group_by;

#[cfg(feature = "ansible-builtin-hostname")]
pub mod hostname;

#[cfg(feature = "ansible-builtin-import_playbook")]
pub mod import_playbook;

#[cfg(feature = "ansible-builtin-import_role")]
pub mod import_role;

#[cfg(feature = "ansible-builtin-import_tasks")]
pub mod import_tasks;

#[cfg(feature = "ansible-builtin-include_role")]
pub mod include_role;

#[cfg(feature = "ansible-builtin-include_tasks")]
pub mod include_tasks;

#[cfg(feature = "ansible-builtin-include_vars")]
pub mod include_vars;

#[cfg(feature = "ansible-builtin-iptables")]
pub mod iptables;

#[cfg(feature = "ansible-builtin-known_hosts")]
pub mod known_hosts;

#[cfg(feature = "ansible-builtin-lineinfile")]
pub mod lineinfile;

#[cfg(feature = "ansible-builtin-meta")]
pub mod meta;

#[cfg(feature = "ansible-builtin-mount_facts")]
pub mod mount_facts;

#[cfg(feature = "ansible-builtin-package")]
pub mod package;

#[cfg(feature = "ansible-builtin-package_facts")]
pub mod package_facts;

#[cfg(feature = "ansible-builtin-pause")]
pub mod pause;

#[cfg(feature = "ansible-builtin-ping")]
pub mod ping;

#[cfg(feature = "ansible-builtin-pip")]
pub mod pip;

#[cfg(feature = "ansible-builtin-raw")]
pub mod raw;

#[cfg(feature = "ansible-builtin-reboot")]
pub mod reboot;

#[cfg(feature = "ansible-builtin-replace")]
pub mod replace;

#[cfg(feature = "ansible-builtin-rpm_key")]
pub mod rpm_key;

#[cfg(feature = "ansible-builtin-script")]
pub mod script;

#[cfg(feature = "ansible-builtin-service")]
pub mod service;

#[cfg(feature = "ansible-builtin-service_facts")]
pub mod service_facts;

#[cfg(feature = "ansible-builtin-set_fact")]
pub mod set_fact;

#[cfg(feature = "ansible-builtin-set_stats")]
pub mod set_stats;

#[cfg(feature = "ansible-builtin-setup")]
pub mod setup;

#[cfg(feature = "ansible-builtin-shell")]
pub mod shell;

#[cfg(feature = "ansible-builtin-slurp")]
pub mod slurp;

#[cfg(feature = "ansible-builtin-stat")]
pub mod stat;

#[cfg(feature = "ansible-builtin-subversion")]
pub mod subversion;

#[cfg(feature = "ansible-builtin-systemd")]
pub mod systemd;

#[cfg(feature = "ansible-builtin-systemd_service")]
pub mod systemd_service;

#[cfg(feature = "ansible-builtin-sysvinit")]
pub mod sysvinit;

#[cfg(feature = "ansible-builtin-tempfile")]
pub mod tempfile;

#[cfg(feature = "ansible-builtin-template")]
pub mod template;

#[cfg(feature = "ansible-builtin-unarchive")]
pub mod unarchive;

#[cfg(feature = "ansible-builtin-uri")]
pub mod uri;

#[cfg(feature = "ansible-builtin-user")]
pub mod user;

#[cfg(feature = "ansible-builtin-validate_argument_spec")]
pub mod validate_argument_spec;

#[cfg(feature = "ansible-builtin-wait_for")]
pub mod wait_for;

#[cfg(feature = "ansible-builtin-wait_for_connection")]
pub mod wait_for_connection;

#[cfg(feature = "ansible-builtin-yum_repository")]
pub mod yum_repository;
