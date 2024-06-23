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
    fs::File,
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
    const CORE_NUM_ERROR: &'a str = "ERROR: Could not properly deduce number of cpu cores!";

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
                Some("c") => Some(FileType::C {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                Some("cpp") => Some(FileType::Cpp {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                Some("py") => Some(FileType::Python {
                    inline_comment_format: Some("#"),
                    multiline_comment_start_format: None,
                    multiline_comment_end_format: None,
                }),
                Some("zig") => Some(FileType::Zig {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: None,
                    multiline_comment_end_format: None,
                }),
                Some("rs") => Some(FileType::Rust {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                Some("js") => Some(FileType::Javascript {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                Some("ts") => Some(FileType::Typescript {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                _ => None,
            },
            None => match file.to_str() {
                Some("Makefile") => Some(FileType::Makefile {
                    inline_comment_format: Some("#"),
                    multiline_comment_start_format: None,
                    multiline_comment_end_format: None,
                }),
                None => None,
                _ => None,
            },
        };
    }

    fn process_line(
        line: &str,
        filetype: &FileType,
        inside_multiline_comment: &mut bool,
        file_path: &Path,
    ) {
        if line.is_empty() {
            return;
        }

        let (inline_comment_format, multiline_comment_start_format, multiline_comment_end_format) =
            match *filetype {
                FileType::C {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
                FileType::Cpp {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
                FileType::Python {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
                FileType::Rust {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
                FileType::Zig {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
                FileType::Javascript {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
                FileType::Typescript {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
                FileType::Makefile {
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                } => (
                    inline_comment_format,
                    multiline_comment_start_format,
                    multiline_comment_end_format,
                ),
            };

        let comment_portion: &str = match (*inside_multiline_comment, inline_comment_format) {
            (true, _) => line,
            (false, None) => return,
            (false, Some(comment_pattern)) => {
                let position: Option<usize> = line.find(comment_pattern);
                if let Some(comment_position) = position {
                    &line[comment_position..]
                } else {
                    return;
                }
            }
        };

        for keyword in LogResult::KEY_COMMENTS {
            if comment_portion.contains(keyword) {
                println!(
                    "{} Found!\nFile: {:?}\nLine: {}\n",
                    keyword, file_path, line
                );

                result.lock().unwrap().increment_keyword(keyword);
            }
        }
    }

    fn parse_file(file_path: &Path, result: &Arc<Mutex<LogResult>>) {
        // println!("Parsing File: {:?}", file);

        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(_) => return,
        };

        let file_type = match Self::classify_file(file_path) {
            Some(t) => t,
            None => return,
        };

        let file_reader: BufReader<File> = BufReader::new(file);
        let mut inside_multiline_comment: bool = false;

        for line in file_reader.lines() {
            Self::process_line(
                match &line {
                    Ok(good_line) => good_line,
                    Err(_) => "",
                },
                &file_type,
                &mut inside_multiline_comment,
                file_path,
            );
        }
    }

    fn waiting_room(data: Arc<Mutex<VecDeque<PathBuf>>>, abort: Arc<Mutex<bool>>) {
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
                Some(found_file) => Self::parse_file(&found_file),
            };
        }
    }

    fn populate_queue(
        worker_queue: Arc<Mutex<VecDeque<PathBuf>>>,
        root: &Path,
    ) -> Result<(), std::io::Error> {
        if root.is_dir() {
            for entry in root.read_dir()? {
                let entry = entry?;
                if entry.path().is_dir() {
                    Self::populate_queue(worker_queue.clone(), &entry.path())?;
                } else {
                    (*worker_queue).lock().unwrap().push_back(entry.path());
                }
            }
        } else {
            (*worker_queue)
                .lock()
                .unwrap()
                .push_back(root.to_path_buf());
        }

        Ok(())
    }

    pub fn log(&mut self) -> Result<(), std::io::Error> {
        let worker_queue: Arc<Mutex<VecDeque<PathBuf>>> = Arc::new(Mutex::new(VecDeque::new()));

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

        Self::populate_queue(worker_queue.clone(), self.root_directory)?;

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
