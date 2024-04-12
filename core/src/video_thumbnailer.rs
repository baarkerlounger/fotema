// SPDX-FileCopyrightText: © 2024 David Bliss
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::repo;
use crate::Error::*;
use crate::Result;
use std::path;
use std::process::Command;
use tempfile;

#[derive(Debug, Clone)]
pub struct VideoThumbnailer {
    base_path: path::PathBuf,
}

impl VideoThumbnailer {
    pub fn build(base_path: &path::Path) -> Result<Self> {
        let base_path = path::PathBuf::from(base_path);
        std::fs::create_dir_all(base_path.join("square"))
            .map_err(|e| RepositoryError(e.to_string()))?;
        Ok(Self { base_path })
    }

    /// Computes a preview square for an image that has been inserted
    /// into the Repository. Preview image will be written to file system.
    pub fn square_preview(&self, vid: &repo::Video) -> Result<path::PathBuf> {
        if vid.thumbnail_path.as_ref().is_some_and(|p| p.exists()) {
            return Ok(vid.path.clone());
        }

        let mut cmd = Command::new("ls -l");
        let status = cmd.status();
        println!("Command status = {:?}", status);

        Err(ScannerError("None".to_string()))
    }
}
