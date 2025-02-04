use crate::core::{OptionUnset, TaskModule};
use serde::Serialize;

/// [ansible.builtin.file](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/file_module.html)
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Module {
    #[serde(rename = "ansible.builtin.file")]
    pub module: Args,
}

impl TaskModule for Module {}

/// [ansible.builtin.file parameters](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/file_module.html#parameters)
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Args {
    // path
    // aliases: dest, name
    // path / required
    // Path to the file being managed.
    pub path: String,
    #[serde(flatten)]
    pub options: OptArgs,
}

#[derive(Serialize, Clone, Debug, PartialEq, Default)]
pub struct OptArgs {
    // access_time
    // string
    // added in Ansible 2.7
    // This parameter indicates the time the file’s access time should be set to.
    // Should be preserve when no modification is required, YYYYMMDDHHMM.SS when using default time format, or now.
    // Default is None meaning that preserve is the default for state=[file,directory,link,hard] and now is default for state=touch.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub access_time: OptionUnset<String>,
    // access_time_format
    // string
    // added in Ansible 2.7
    // When used with access_time, indicates the time format that must be used.
    // Based on default Python format (see time.strftime doc).
    // Default: "%Y%m%d%H%M.%S"
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub access_time_format: OptionUnset<String>,
    // attributes
    // aliases: attr
    // string
    // The attributes the resulting filesystem object should have.
    // To get supported flags look at the man page for chattr on the target system.
    // This string should contain the attributes in the same order as the one displayed by lsattr.
    // The = operator is assumed as default, otherwise + or - operators need to be included in the string.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub attributes: OptionUnset<String>,
    // follow
    // boolean
    // This flag indicates that filesystem links, if they exist, should be followed.
    // follow=yes and state=link can modify src when combined with parameters such as mode.
    // Previous to Ansible 2.5, this was false by default.
    // While creating a symlink with a non-existent destination, set follow=false to avoid a warning message related to permission issues. The warning message is added to notify the user that we can not set permissions to the non-existent destination.
    // Choices:
    // false
    // true ← (default)
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub follow: OptionUnset<bool>,
    // force
    // boolean
    // Force the creation of the links in two cases: if the link type is symbolic and the source file does not exist (but will appear later); the destination exists and is a file (so, we need to unlink the path file and create a link to the src file in place of it).
    // Choices:
    // false ← (default)
    // true
    // group
    // string
    // Name of the group that should own the filesystem object, as would be fed to chown.
    // When left unspecified, it uses the current group of the current user unless you are root, in which case it can preserve the previous ownership.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub group: OptionUnset<String>,
    // mode
    // any
    // The permissions the resulting filesystem object should have.
    // For those used to /usr/bin/chmod remember that modes are actually octal numbers. You must give Ansible enough information to parse them correctly. For consistent results, quote octal numbers (for example, '644' or '1777') so Ansible receives a string and can do its own conversion from string into number. Adding a leading zero (for example, 0755) works sometimes, but can fail in loops and some other circumstances.
    // Giving Ansible a number without following either of these rules will end up with a decimal number which will have unexpected results.
    // As of Ansible 1.8, the mode may be specified as a symbolic mode (for example, u+rwx or u=rw,g=r,o=r).
    // If mode is not specified and the destination filesystem object does not exist, the default umask on the system will be used when setting the mode for the newly created filesystem object.
    // If mode is not specified and the destination filesystem object does exist, the mode of the existing filesystem object will be used.
    // Specifying mode is the best way to ensure filesystem objects are created with the correct permissions. See CVE-2020-1736 for further details.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub mode: OptionUnset<String>,
    // modification_time
    // string
    // added in Ansible 2.7
    // This parameter indicates the time the file’s modification time should be set to.
    // Should be preserve when no modification is required, YYYYMMDDHHMM.SS when using default time format, or now.
    // Default is None meaning that preserve is the default for state=[file,directory,link,hard] and now is default for state=touch.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub modification_time: OptionUnset<String>,
    // modification_time_format
    // string
    // added in Ansible 2.7
    // When used with modification_time, indicates the time format that must be used.
    // Based on default Python format (see time.strftime doc).
    // Default: "%Y%m%d%H%M.%S"
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub modification_time_format: OptionUnset<String>,
    // owner
    // string
    // Name of the user that should own the filesystem object, as would be fed to chown.
    // When left unspecified, it uses the current user unless you are root, in which case it can preserve the previous ownership.
    // Specifying a numeric username will be assumed to be a user ID and not a username. Avoid numeric usernames to avoid this confusion.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub owner: OptionUnset<String>,
    // recurse
    // boolean
    // Recursively set the specified file attributes on directory contents.
    // This applies only when state is set to directory.
    // Choices:
    // false ← (default)
    // true
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub recurse: OptionUnset<bool>,
    // selevel
    // string
    // The level part of the SELinux filesystem object context.
    // This is the MLS/MCS attribute, sometimes known as the range.
    // When set to _default, it will use the level portion of the policy if available.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub selevel: OptionUnset<String>,
    // serole
    // string
    // The role part of the SELinux filesystem object context.
    // When set to _default, it will use the role portion of the policy if available.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub serole: OptionUnset<String>,
    // setype
    // string
    // The type part of the SELinux filesystem object context.
    // When set to _default, it will use the type portion of the policy if available.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub setype: OptionUnset<String>,
    // seuser
    // string
    // The user part of the SELinux filesystem object context.
    // By default it uses the system policy, where applicable.
    // When set to _default, it will use the user portion of the policy if available.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub seuser: OptionUnset<String>,
    // src
    // path
    // Path of the file to link to.
    // This applies only to state=link and state=hard.
    // For state=link, this will also accept a non-existing path.
    // Relative paths are relative to the file being created (path) which is how the Unix command ln -s SRC DEST treats relative paths.
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub src: OptionUnset<String>,
    // state
    // string
    // If absent, directories will be recursively deleted, and files or symlinks will be unlinked. In the case of a directory, if diff is declared, you will see the files and folders deleted listed under path_contents. Note that absent will not cause ansible.builtin.file to fail if the path does not exist as the state did not change.
    // If directory, all intermediate subdirectories will be created if they do not exist. Since Ansible 1.7 they will be created with the supplied permissions.
    // If file, with no other options, returns the current state of path.
    // If file, even with other options (such as mode), the file will be modified if it exists but will NOT be created if it does not exist. Set to touch or use the ansible.builtin.copy or ansible.builtin.template module if you want to create the file if it does not exist.
    // If hard, the hard link will be created or changed.
    // If link, the symbolic link will be created or changed.
    // If touch (new in 1.4), an empty file will be created if the file does not exist, while an existing file or directory will receive updated file access and modification times (similar to the way touch works from the command line).
    // Default is the current state of the file if it exists, directory if recurse=yes, or file otherwise.
    // Choices:
    // "absent"
    // "directory"
    // "file"
    // "hard"
    // "link"
    // "touch"
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub state: OptionUnset<State>,
    // unsafe_writes
    // boolean
    // Influence when to use atomic operation to prevent data corruption or inconsistent reads from the target filesystem object.
    // By default this module uses atomic operations to prevent data corruption or inconsistent reads from the target filesystem objects, but sometimes systems are configured or just broken in ways that prevent this. One example is docker mounted filesystem objects, which cannot be updated atomically from inside the container and can only be written in an unsafe manner.
    // This option allows Ansible to fall back to unsafe methods of updating filesystem objects when atomic operations fail (however, it doesn’t force Ansible to perform unsafe writes).
    // IMPORTANT! Unsafe writes are subject to race conditions and can lead to data corruption.
    // Choices:
    // false ← (default)
    // true
    #[serde(
        default = "OptionUnset::default",
        skip_serializing_if = "OptionUnset::is_unset"
    )]
    pub unsafe_writes: OptionUnset<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Absent,
    Directory,
    File,
    Hard,
    Link,
    Touch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansible_builtin_file_task_module() {
        let task_module = Module {
            module: Args {
                path: "/path/to/file".to_string(),
                options: OptArgs {
                    state: OptionUnset::Some(State::Touch),
                    ..Default::default()
                },
            },
        };
        let json = serde_json::to_string(&task_module).unwrap();
        assert_eq!(
            json,
            r#"{"ansible.builtin.file":{"path":"/path/to/file","state":"touch"}}"#
        );
    }
}
