use std::fs::File;
use std::io::Write;

use lazycell::LazyCell;
use readext::ReadExt;
use semver::Version;

use crate::distro::node::NodeVersion;
use crate::fs::touch;
use crate::path::user_platform_file;
use crate::platform::PlatformSpec;

use notion_fail::{Fallible, ResultExt};

pub(crate) mod serial;

/// Lazily loaded toolchain
pub struct LazyToolchain {
    toolchain: LazyCell<Toolchain>,
}

impl LazyToolchain {
    /// Creates a new `LazyToolchain`
    pub fn new() -> Self {
        LazyToolchain {
            toolchain: LazyCell::new(),
        }
    }

    /// Forces loading of the toolchain and returns an immutable reference to it
    pub fn get(&self) -> Fallible<&Toolchain> {
        self.toolchain.try_borrow_with(|| Toolchain::current())
    }

    /// Forces loading of the toolchain and returns a mutable reference to it
    pub fn get_mut(&mut self) -> Fallible<&mut Toolchain> {
        self.toolchain.try_borrow_mut_with(|| Toolchain::current())
    }
}

pub struct Toolchain {
    platform: Option<PlatformSpec>,
}

impl Toolchain {
    fn current() -> Fallible<Toolchain> {
        let path = user_platform_file()?;
        let src = touch(&path)?.read_into_string().unknown()?;
        Ok(Toolchain {
            platform: serial::Platform::from_json(src)?.into_image()?,
        })
    }

    pub fn platform_ref(&self) -> Option<&PlatformSpec> {
        self.platform.as_ref()
    }

    /// Set the active Node version in the user platform file.
    pub fn set_active_node(&mut self, node_version: NodeVersion) -> Fallible<()> {
        let mut dirty = false;

        if let Some(ref mut platform) = self.platform {
            if platform.node_runtime != node_version.runtime {
                platform.node_runtime = node_version.runtime;
                dirty = true;
            }

            if platform.npm != Some(node_version.npm.clone()) {
                platform.npm = Some(node_version.npm);
                dirty = true;
            }
        } else {
            self.platform = Some(PlatformSpec {
                node_runtime: node_version.runtime,
                npm: Some(node_version.npm),
                yarn: None,
            });
            dirty = true;
        }

        if dirty {
            self.save()?;
        }

        Ok(())
    }

    /// Set the active Yarn version in the user platform file.
    pub fn set_active_yarn(&mut self, yarn_version: Version) -> Fallible<()> {
        let mut dirty = false;

        if let &mut Some(ref mut platform) = &mut self.platform {
            if platform.yarn != Some(yarn_version.clone()) {
                platform.yarn = Some(yarn_version);
                dirty = true;
            }
        }

        if dirty {
            self.save()?;
        }

        Ok(())
    }

    /// Set the active Npm version in the user platform file.
    pub fn set_active_npm(&mut self, npm_version: Version) -> Fallible<()> {
        let mut dirty = false;

        if let &mut Some(ref mut platform) = &mut self.platform {
            if platform.npm != Some(npm_version.clone()) {
                platform.npm = Some(npm_version);
                dirty = true;
            }
        }

        if dirty {
            self.save()?;
        }

        Ok(())
    }

    pub fn save(&self) -> Fallible<()> {
        let path = user_platform_file()?;
        let mut file = File::create(&path).unknown()?;
        match &self.platform {
            &Some(ref platform) => {
                let src = platform.to_serial().to_json()?;
                file.write_all(src.as_bytes()).unknown()?;
            }
            &None => {
                file.write_all(b"{}").unknown()?;
            }
        }
        Ok(())
    }
}
