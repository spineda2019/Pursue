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
    fs::{File, Metadata},
    io::{BufRead, BufReader, ErrorKind},
    num::NonZero,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use crate::{filebundle::FileBundle, filetype::FileType};

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

    fn classify_file(file: &Path) -> Option<FileType> {
        return match file.extension() {
            Some(extension) => match extension.to_str() {
                Some("c") => Some(FileType::C),
                Some("cpp") => Some(FileType::Cpp),
                Some("py") => Some(FileType::Python),
                Some("zig") => Some(FileType::Zig),
                Some("rs") => Some(FileType::Rust),
                Some("js") => Some(FileType::Javascript),
                Some("ts") => Some(FileType::Typescript),
                _ => None,
            },
            None => match file.to_str() {
                Some("Makefile") => Some(FileType::Makefile),
                None => None,
                _ => None,
            },
        };
    }

    fn process_line(line: &str) {
        if line.is_empty() {
            return;
        }
    }

    fn parse_file(file_bundle: FileBundle) {
        let FileBundle(file, file_type) = file_bundle;

        println!("Parsing File: {:?}", file);

        let file_reader: BufReader<File> = BufReader::new(file);

        for line in file_reader.lines() {
            Self::process_line(match &line {
                Ok(good_line) => good_line,
                Err(_) => "",
            });
        }
    }

    fn waiting_room(data: Arc<Mutex<VecDeque<FileBundle>>>, abort: Arc<Mutex<bool>>) {
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
                Some(found_file) => Self::parse_file(found_file),
            };
        }
    }

    fn populate_queue(
        worker_queue: Arc<Mutex<VecDeque<FileBundle>>>,
        root: &Path,
    ) -> Result<(), std::io::Error> {
        if root.is_dir() {
            for entry in root.read_dir()? {
                let entry = entry?;
                if entry.path().is_dir() {
                    Self::populate_queue(worker_queue.clone(), &entry.path());
                } else {
                    let file_type: FileType = match Self::classify_file(&entry.path()) {
                        Some(x) => x,
                        None => continue,
                    };

                    let file: File = match File::open(entry.path()) {
                        Ok(x) => x,
                        Err(_) => continue,
                    };

                    (*worker_queue)
                        .lock()
                        .unwrap()
                        .push_back(FileBundle(file, file_type));
                }
            }
        } else {
            let file_type: FileType = match Self::classify_file(&root) {
                Some(x) => x,
                None => return Ok(()),
            };

            let file: File = File::open(root)?;

            (*worker_queue)
                .lock()
                .unwrap()
                .push_back(FileBundle(file, file_type));
        }

        Ok(())
    }

    pub fn log(&mut self) -> Result<(), std::io::Error> {
        let worker_queue: Arc<Mutex<VecDeque<FileBundle>>> = Arc::new(Mutex::new(VecDeque::new()));

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

        Self::populate_queue(worker_queue.clone(), self.root_directory);

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
