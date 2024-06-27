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
 *  You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/> */

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
    num::NonZero,
    path::{Path, PathBuf},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use crate::{
    filetype::{destructure_filetype, FileType},
    log_result::LogResult,
};

pub struct Logger {
    data: Arc<Mutex<VecDeque<PathBuf>>>,
    finish_flag: Arc<Mutex<bool>>,
    root_directory: PathBuf,
    verbose: bool,
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            finish_flag: self.finish_flag.clone(),
            root_directory: self.root_directory.clone(),
            verbose: self.verbose,
        }
    }
}

impl<'a> Logger {
    const CORE_NUM_ERROR: &'a str = "ERROR: Could not properly deduce number of cpu cores!";
    const CPP_FILE_EXTENSIONS: [&'a str; 3] = ["cpp", "cxx", "cc"];

    pub fn new(directory: PathBuf, verbose_printing: bool) -> Self {
        Self {
            data: Arc::new(Mutex::new(VecDeque::new())),
            finish_flag: Arc::new(Mutex::new(false)),
            root_directory: directory,
            verbose: verbose_printing,
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
                Some(ext) if Self::CPP_FILE_EXTENSIONS.contains(&ext) => Some(FileType::Cpp {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                Some("cs") => Some(FileType::CSharp {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                Some("java") => Some(FileType::Java {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
                }),
                Some("py") => Some(FileType::Python {
                    inline_comment_format: Some("#"),
                    multiline_comment_start_format: None,
                    multiline_comment_end_format: None,
                }),
                Some("go") => Some(FileType::Go {
                    inline_comment_format: Some("//"),
                    multiline_comment_start_format: Some("/*"),
                    multiline_comment_end_format: Some("*/"),
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
            None => match file.file_name()?.to_str() {
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
        &self,
        line: &str,
        filetype: &FileType,
        inside_multiline_comment: &mut bool,
        file_path: &Path,
        result: &Arc<Mutex<LogResult>>,
    ) {
        if line.is_empty() {
            return;
        }

        let (inline_comment_format, multiline_comment_start_format, multiline_comment_end_format) =
            destructure_filetype!(filetype);

        let comment_portion: &str = match (inside_multiline_comment, inline_comment_format) {
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
                {
                    result.lock().unwrap().increment_keyword(keyword);
                }

                if self.verbose {
                    println!(
                        "{} Found!\nFile: {:?}\nLine: {}\n",
                        keyword, file_path, line
                    );
                }
            }
        }
    }

    fn parse_file(&self, file_path: &Path, result: &Arc<Mutex<LogResult>>) {
        // println!("Parsing File: {:?}", file);

        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(_) => return,
        };

        let file_type = match Self::classify_file(file_path) {
            Some(t) => t,
            None => return,
        };

        {
            result
                .lock()
                .unwrap()
                .increment_filetype_frequency(&file_type);
        }

        let file_reader: BufReader<File> = BufReader::new(file);
        let mut inside_multiline_comment: bool = false;

        for line in file_reader.lines() {
            self.process_line(
                match &line {
                    Ok(good_line) => good_line,
                    Err(_) => "",
                },
                &file_type,
                &mut inside_multiline_comment,
                file_path,
                result,
            );

            {
                result.lock().unwrap().increment_line_count();
            }
        }
    }

    fn waiting_room(&self, result: Arc<Mutex<LogResult>>, producer: Sender<()>) {
        loop {
            let entry: Option<PathBuf>;
            {
                entry = self.data.lock().unwrap().pop_front();
            }

            match entry {
                None => {
                    if *self.finish_flag.lock().unwrap() {
                        drop(producer);
                        return;
                    } else {
                        continue;
                    }
                }
                Some(found_file) => self.parse_file(&found_file, &result),
            };
        }
    }

    fn populate_queue(&self, root: &Path) -> Result<(), std::io::Error> {
        if root.is_dir() {
            for entry in root.read_dir()? {
                let entry = entry?;
                if entry.path().is_dir() {
                    self.populate_queue(&entry.path())?;
                } else {
                    self.data.lock().unwrap().push_back(entry.path());
                }
            }
        } else {
            self.data.lock().unwrap().push_back(root.to_path_buf());
        }

        Ok(())
    }

    pub fn log(&self) -> Result<(), std::io::Error> {
        let result: Arc<Mutex<LogResult>> = Arc::new(Mutex::new(LogResult::new()));

        let worker_count = NonZero::new(num_cpus::get());
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

        println!(
            "Number of CPUs supported for Trace's file I/O: {}\n",
            worker_count
        );

        let (producer, consumer): (Sender<()>, Receiver<()>) = mpsc::channel();

        for _ in 0..worker_count.get() {
            let self_clone = self.clone();
            let result = result.clone();
            let producer = producer.clone();
            thread::spawn(move || {
                self_clone.waiting_room(result, producer);
            });
        }

        self.populate_queue(&self.root_directory)?;

        {
            *self.finish_flag.lock().unwrap() = true;
        }

        drop(producer);

        for _ in consumer {}

        {
            result.lock().unwrap().print_result();
        }

        Ok(())
    }
}
