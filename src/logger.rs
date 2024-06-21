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
    collections::{HashMap, VecDeque},
    fs::{DirEntry, File},
    io::{BufRead, BufReader, ErrorKind},
    num::NonZero,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use crate::filetype::FileType;

pub struct Logger<'a> {
    root_directory: &'a Path,
    verbose: bool,
    key_comment_table: HashMap<&'a str, usize>,
}

impl<'a> Logger<'a> {
    const KEY_COMMENTS: [&'a str; 4] = ["TODO", "HACK", "BUG", "FIXME"];
    const CORE_NUM_ERROR: &'a str = "Could not properly deduce number of cpu cores!";

    pub fn new(directory: &'a PathBuf, verbose_printing: bool) -> Self {
        let mut comment_table: HashMap<&'a str, usize> = HashMap::new();
        for comment in Self::KEY_COMMENTS {
            comment_table.insert(comment, 0);
        }

        Self {
            root_directory: directory,
            verbose: verbose_printing,
            key_comment_table: comment_table,
        }
    }

    fn classify_file(file: &Path) -> FileType {
        return match file.extension() {
            Some(extension) => match extension.to_str() {
                Some("c") => FileType::C,
                Some("cpp") => FileType::Cpp,
                Some("py") => FileType::Python,
                Some("zig") => FileType::Zig,
                Some("rs") => FileType::Rust,
                Some("js") => FileType::Javascript,
                Some("ts") => FileType::Typescript,
                _ => FileType::Unknown,
            },
            None => match file.to_str() {
                Some("Makefile") => FileType::Makefile,
                None => return FileType::Unknown,
                _ => FileType::Unknown,
            },
        };
    }

    fn process_line(line: &str) {
        if line.is_empty() {
            return;
        }
    }

    fn parse_file(file: &Path) {
        let filetype = Self::classify_file(file);
        if let FileType::Unknown = filetype {
            return;
        }

        println!("Parsing File: {:?}", file);

        let file_handle: File = match File::open(file) {
            Ok(file_handle) => file_handle,
            Err(error) => {
                eprintln!(
                    "Error occurred while opening file {:?}\nError: {:?}",
                    file, error
                );
                return;
            }
        };

        let file_reader: BufReader<File> = BufReader::new(file_handle);

        for line in file_reader.lines() {
            Self::process_line(match &line {
                Ok(good_line) => good_line,
                Err(_) => "",
            });
        }
    }

    fn recursively_log(entry: DirEntry) -> Result<(), std::io::Error> {
        if entry.path().is_dir() {
            for sub_entry in entry.path().read_dir()? {
                let sub_entry = sub_entry?;
                Self::recursively_log(sub_entry)?;
            }
        } else {
            Self::parse_file(&entry.path());
        }

        Ok(())
    }

    fn waiting_room(data: Arc<Mutex<VecDeque<DirEntry>>>, abort: Arc<Mutex<bool>>) {
        loop {
            let entry = (*data).lock().unwrap().pop_front();
            match entry {
                None => {
                    if *abort.lock().unwrap() {
                        return;
                    } else {
                        continue;
                    }
                }
                Some(found_file) => Self::recursively_log(found_file),
            };
        }
    }

    pub fn log(&mut self) -> Result<(), std::io::Error> {
        let worker_queue: Arc<Mutex<VecDeque<DirEntry>>> = Arc::new(Mutex::new(VecDeque::new()));

        let worker_count = NonZero::new(num_cpus::get_physical());
        let worker_count = match worker_count {
            Some(number) => number,
            None => {
                eprintln!("{}", Self::CORE_NUM_ERROR);
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    Self::CORE_NUM_ERROR,
                ));
            }
        };

        let abort: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

        let mut jobs = vec![];

        for _ in 0..worker_count.get() {
            let data = worker_queue.clone();
            let abort = abort.clone();
            jobs.push(thread::spawn(|| {
                Self::waiting_room(data, abort);
            }));
        }

        if self.root_directory.is_dir() {
            for entry in self.root_directory.read_dir()? {
                let entry = entry?;
                (*worker_queue).lock().unwrap().push_back(entry);
            }
        } else {
            Self::parse_file(&self.root_directory);
        }

        while (*worker_queue).lock().unwrap().len() > 0 {
            continue;
        }

        *abort.lock().unwrap() = true;

        for job in jobs {
            job.join();
        }

        Ok(())
    }
}
