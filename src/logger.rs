/*
 *  logger.rs - log codebase given info from command line
 *  Copyright (C) 2024  Sebastian Pineda (spineda.wpi.alum@gmail.com)
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program. If not, see <https://www.gnu.org/licenses/>
 */

use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

pub struct Logger<'a> {
    root_directory: &'a Path,
    verbose: bool,
}

impl<'a> Logger<'a> {
    pub fn new(directory: &'a str, current_dir: &'a PathBuf, verbose_printing: bool) -> Self {
        let path = Path::new(directory);
        if path.exists() {
            Self {
                root_directory: path,
                verbose: verbose_printing,
            }
        } else {
            Self {
                root_directory: current_dir,
                verbose: verbose_printing,
            }
        }
    }

    fn parse_file(&self, file: &'a Path) {
        if self.verbose {
            println!("Parsing File: {:?}", file);
        }
    }

    fn recursively_log(&self, entry: &'a DirEntry) -> Result<(), std::io::Error> {
        if entry.path().is_dir() {
            for sub_entry in entry.path().read_dir()? {
                let sub_entry = sub_entry?;
                self.recursively_log(&sub_entry)?;
            }
        } else {
            self.parse_file(&entry.path());
        }

        Ok(())
    }

    pub fn log(&self) -> Result<(), std::io::Error> {
        println!("Root: {:?}", self.root_directory);
        println!("Verbose?: {}\n", self.verbose);

        if self.root_directory.is_dir() {
            for entry in self.root_directory.read_dir()? {
                let entry = entry?;
                self.recursively_log(&entry)?;
            }
        } else {
            println!("File: {:?}", self.root_directory);
        }

        Ok(())
    }
}
