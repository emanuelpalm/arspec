//! Arrowhead specification project management.
//!
//! This module contains tools useful for managing a folder containing a
//! specification project.

mod settings;

pub use self::settings::Settings;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Project {
    root: Box<Path>,
    settings: Box<Settings>,
}

impl Project {
    pub fn create<P>(path: P) -> io::Result<Project>
        where P: Into<PathBuf>
    {
        let mut path = path.into();
        path.push(".ahfs");
        fs::create_dir_all(&path)?;
        path.pop();
        Self::read(path)
    }

    pub fn locate<P>(path: P) -> io::Result<Project>
        where P: Into<PathBuf>
    {
        let mut path = path.into();
        loop {
            path.push(".ahfs");
            let is_dir = path.is_dir();
            path.pop();
            if is_dir {
                break;
            }
            if !path.pop() {
                return Err(io::ErrorKind::NotFound.into());
            }
        }
        Self::read(path)
    }

    fn read<P>(root: P) -> io::Result<Project>
        where P: Into<Box<Path>>
    {
        let root = root.into();
        let settings = Settings::read(root.join(".ahfs").join("settings.txt"))?;

        Ok(Project { root, settings: settings.into() })
    }

    #[inline]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[inline]
    pub fn settings(&self) -> &Settings {
        &self.settings
    }
}