use std::fmt;
use std::process::ExitStatus;

use failure::Fail;
use notion_fail::{ExitCode, NotionFail};

use crate::tool::ToolSpec;
use crate::version::VersionSpec;

#[derive(Debug, Fail)]
pub enum ErrorDetails {
    /// Thrown when package tries to install a binary that is already installed.
    BinaryAlreadyInstalled {
        bin_name: String,
        package: String,
        version: String,
    },

    BinaryExecError {
        error: String,
    },

    /// Thrown when a user tries to `notion pin` something other than node/yarn/npm.
    CannotPinPackage,

    CliParseError,

    CommandNotImplemented {
        command_name: String,
    },

    CouldNotDetermineTool,

    CreateDirError {
        dir: String,
        error: String,
    },

    DepPackageReadError {
        error: String,
    },

    DeprecatedCommandError {
        command: String,
        advice: String,
    },

    DownloadToolNetworkError {
        tool: ToolSpec,
        from_url: String,
        error: String,
    },

    DownloadToolNotFound {
        tool: ToolSpec,
    },

    InvalidHookCommand {
        command: String,
    },

    /// Thrown when BinConfig (read from file) does not contain Platform info.
    NoBinPlatform {
        binary: String,
    },

    /// Thrown when there is no Node version matching a requested semver specifier.
    NodeVersionNotFound {
        matching: String,
    },

    NoGlobalInstalls,

    NoHomeEnvironmentVar,

    NoLocalDataDir,

    /// Thrown when a user tries to install or fetch a package with no executables.
    NoPackageExecutables,

    /// Thrown when there is no package version matching a requested semver specifier.
    NoPackageFound {
        name: String,
        matching: VersionSpec,
    },

    /// Thrown when a user tries to pin a Yarn version before pinning a Node version.
    NoPinnedNodeVersion,

    NoSuchTool {
        tool: String,
    },

    /// Thrown when the user tries to pin Node or Yarn versions outside of a package.
    NotInPackage,

    NoToolChain {
        shim_name: String,
    },

    NoVersionsFound,

    NpxNotAvailable {
        version: String,
    },

    /// Thrown when package install command is not successful.
    PackageInstallFailed {
        cmd: String,
        status: ExitStatus,
    },

    /// Thrown when package install command fails to execute.
    PackageInstallIoError {
        error: String,
    },

    PackageReadError {
        error: String,
    },

    /// Thrown when a package has been unpacked but is not formed correctly.
    PackageUnpackError,

    PathError,

    /// Thrown when the public registry for Node or Yarn could not be downloaded.
    RegistryFetchError {
        error: String,
    },

    SymlinkError {
        error: String,
    },

    ToolNotImplemented,

    /// Thrown when the shell name specified in the Notion environment is not supported.
    UnrecognizedShell {
        name: String,
    },

    /// Thrown when the postscript file was not specified in the Notion environment.
    UnspecifiedPostscript,

    /// Thrown when the shell name was not specified in the Notion environment.
    UnspecifiedShell,

    VersionParseError {
        error: String,
    },

    /// Thrown when there is no Yarn version matching a requested semver specifier.
    YarnVersionNotFound {
        matching: String,
    },
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorDetails::BinaryAlreadyInstalled { bin_name, package, version } => write!(f, "Conflict with bin '{}' already installed by '{}' version {}", bin_name, package, version),
            ErrorDetails::BinaryExecError { error } => write!(f, "{}", error),
            ErrorDetails::CannotPinPackage => {
                write!(f, "Only node, yarn, and npm can be pinned in a project")
            }
            ErrorDetails::CliParseError => write!(f, "There was a problem parsing the command line input"),
            ErrorDetails::CommandNotImplemented { command_name } => write!(f, "command `{}` is not yet implemented", command_name),
            ErrorDetails::CouldNotDetermineTool => write!(f, "Tool name could not be determined"),
            ErrorDetails::CreateDirError { dir, error } => {
                write!(f, "Could not create directory {}: {}", dir, error)
            }
            ErrorDetails::DepPackageReadError { error } => {
                write!(f, "Could not read dependent package info: {}", error)
            }
            ErrorDetails::DeprecatedCommandError { command, advice } => {
                write!(f, "The subcommand `{}` is deprecated.\n{}", command, advice)
            }
            ErrorDetails::DownloadToolNetworkError {
                tool,
                from_url,
                error,
            } => write!(
                f,
                "Failed to download {} from {}\n{}",
                tool, from_url, error
            ),
            ErrorDetails::DownloadToolNotFound { tool } => write!(f, "{} not found", tool),
            ErrorDetails::InvalidHookCommand { command } => write!(f, "Invalid hook command: '{}'", command),
            ErrorDetails::NoBinPlatform { binary } => {
                write!(f, "Platform info for executable `{}` is missing", binary)
            }
            ErrorDetails::NodeVersionNotFound { matching } => {
                write!(f, "No Node version found for {}", matching)
            }
            ErrorDetails::NoGlobalInstalls => write!(f, r#"
Global package installs are not recommended.

Consider using `notion install` to add a package to your toolchain (see `notion help install` for more info)."#),
            ErrorDetails::NoHomeEnvironmentVar => {
                write!(f, "environment variable 'HOME' is not set")
            }
            ErrorDetails::NoLocalDataDir => write!(f, "Windows LocalAppData directory not found"),
            ErrorDetails::NoPackageExecutables => write!(f, "Package has no binaries or executables - nothing to do"),
            ErrorDetails::NoPackageFound { name, matching } => write!(f, "No version of '{}' found for {}", name, matching),
            ErrorDetails::NoPinnedNodeVersion => {
                write!(f, "There is no pinned node version for this project")
            }
            ErrorDetails::NoSuchTool { tool } => write!(f, r#"
No {} version selected.

See `notion help pin` for help adding {} to a project toolchain.

See `notion help install` for help adding {} to your personal toolchain."#, tool, tool, tool),
            ErrorDetails::NotInPackage => write!(f, "Not in a node package"),
            ErrorDetails::NoToolChain { shim_name } => {
                write!(f, "No toolchain available to run {}", shim_name)
            }
            ErrorDetails::NoVersionsFound => write!(f, "no versions found"),
            ErrorDetails::NpxNotAvailable { version } => write!(f, r#"
'npx' is only available with npm >= 5.2.0

This project is configured to use version {} of npm."#, version),
            ErrorDetails::PackageInstallFailed { cmd, status } => write!(f, "Command `{}` failed with status {}", cmd, status),
            ErrorDetails::PackageInstallIoError { error } => write!(f, "Error executing package install command: {}", error),
            ErrorDetails::PackageReadError { error } => {
                write!(f, "Could not read package info: {}", error)
            }
            ErrorDetails::PackageUnpackError => write!(f, "Package unpack error: Could not determine unpack directory name"),
            ErrorDetails::PathError => write!(f, "`path` internal error"),
            ErrorDetails::RegistryFetchError { error } => {
                write!(f, "Could not fetch public registry\n{}", error)
            }
            ErrorDetails::SymlinkError { error } => write!(f, "{}", error),
            ErrorDetails::ToolNotImplemented => write!(f, "this tool is not yet implemented"),
            ErrorDetails::UnrecognizedShell { name } => write!(f, "Unrecognized shell: {}", name),
            ErrorDetails::UnspecifiedPostscript => {
                write!(f, "Notion postscript file not specified")
            }
            ErrorDetails::UnspecifiedShell => write!(f, "Notion shell not specified"),
            ErrorDetails::VersionParseError { error } => write!(f, "{}", error),
            ErrorDetails::YarnVersionNotFound { matching } => {
                write!(f, "No Yarn version found for {}", matching)
            }
        }
    }
}

impl NotionFail for ErrorDetails {
    fn exit_code(&self) -> ExitCode {
        match self {
            ErrorDetails::BinaryAlreadyInstalled { .. } => ExitCode::FileSystemError,
            ErrorDetails::BinaryExecError { .. } => ExitCode::ExecutionFailure,
            ErrorDetails::CannotPinPackage => ExitCode::InvalidArguments,
            ErrorDetails::CliParseError => ExitCode::UnknownError,
            ErrorDetails::CommandNotImplemented { .. } => ExitCode::NotYetImplemented,
            ErrorDetails::CouldNotDetermineTool => ExitCode::UnknownError,
            ErrorDetails::CreateDirError { .. } => ExitCode::FileSystemError,
            ErrorDetails::DepPackageReadError { .. } => ExitCode::FileSystemError,
            ErrorDetails::DeprecatedCommandError { .. } => ExitCode::InvalidArguments,
            ErrorDetails::DownloadToolNetworkError { .. } => ExitCode::NetworkError,
            ErrorDetails::DownloadToolNotFound { .. } => ExitCode::NoVersionMatch,
            ErrorDetails::InvalidHookCommand { .. } => ExitCode::UnknownError,
            ErrorDetails::NoBinPlatform { .. } => ExitCode::ExecutionFailure,
            ErrorDetails::NodeVersionNotFound { .. } => ExitCode::NoVersionMatch,
            ErrorDetails::NoGlobalInstalls => ExitCode::InvalidArguments,
            ErrorDetails::NoHomeEnvironmentVar => ExitCode::EnvironmentError,
            ErrorDetails::NoLocalDataDir => ExitCode::EnvironmentError,
            ErrorDetails::NoPackageExecutables { .. } => ExitCode::InvalidArguments,
            ErrorDetails::NoPackageFound { .. } => ExitCode::NoVersionMatch,
            ErrorDetails::NoPinnedNodeVersion => ExitCode::ConfigurationError,
            ErrorDetails::NoSuchTool { .. } => ExitCode::NoVersionMatch,
            ErrorDetails::NotInPackage => ExitCode::ConfigurationError,
            ErrorDetails::NoToolChain { .. } => ExitCode::ExecutionFailure,
            ErrorDetails::NoVersionsFound => ExitCode::NoVersionMatch,
            ErrorDetails::NpxNotAvailable { .. } => ExitCode::ExecutableNotFound,
            ErrorDetails::PackageInstallFailed { .. } => ExitCode::FileSystemError,
            ErrorDetails::PackageInstallIoError { .. } => ExitCode::FileSystemError,
            ErrorDetails::PackageReadError { .. } => ExitCode::FileSystemError,
            ErrorDetails::PackageUnpackError => ExitCode::ConfigurationError,
            ErrorDetails::PathError => ExitCode::UnknownError,
            ErrorDetails::RegistryFetchError { .. } => ExitCode::NetworkError,
            ErrorDetails::SymlinkError { .. } => ExitCode::FileSystemError,
            ErrorDetails::ToolNotImplemented => ExitCode::ExecutableNotFound,
            ErrorDetails::UnrecognizedShell { .. } => ExitCode::EnvironmentError,
            ErrorDetails::UnspecifiedPostscript => ExitCode::EnvironmentError,
            ErrorDetails::UnspecifiedShell => ExitCode::EnvironmentError,
            ErrorDetails::VersionParseError { .. } => ExitCode::NoVersionMatch,
            ErrorDetails::YarnVersionNotFound { .. } => ExitCode::NoVersionMatch,
        }
    }

    fn is_user_friendly(&self) -> bool {
        true
    }
}
