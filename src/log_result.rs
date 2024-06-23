/*
 *  logg_result.rs - store ongoing results for the logger
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

use std::collections::HashMap;

use crate::filetype::FileType;

pub struct LogResult<'a> {
    line_count: usize,
    keyword_table: HashMap<&'a str, usize>,
    filetype_table: HashMap<&'a str, usize>,
}

impl<'a> LogResult<'a> {
    pub const KEY_COMMENTS: [&'a str; 4] = ["TODO", "HACK", "BUG", "FIXME"];

    pub fn new() -> Self {
        let mut comment_table = HashMap::new();
        for comment in Self::KEY_COMMENTS {
            comment_table.insert(comment, 0);
        }

        Self {
            line_count: 0,
            keyword_table: comment_table,
            filetype_table: HashMap::new(),
        }
    }

    pub fn increment_line_count(&mut self) {
        self.line_count += 1;
    }

    pub fn increment_keyword(&mut self, keyword: &'a str) {
        if let Some(value) = self.keyword_table.get_mut(keyword) {
            *value += 1;
        } else {
            self.keyword_table.insert(keyword, 1);
        }
    }

    pub fn increment_filetype_frequency(&mut self, filetype: &FileType) {
        let name = match filetype {
            FileType::C {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "C",
            FileType::Cpp {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "C++",
            FileType::Python {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "Python",
            FileType::Rust {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "Rust",
            FileType::Zig {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "Zig",
            FileType::Javascript {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "JavaScript",
            FileType::Typescript {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "TypeScript",
            FileType::Makefile {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "Makefile",
        };

        if let Some(value) = self.filetype_table.get_mut(name) {
            *value += 1;
        } else {
            self.filetype_table.insert(name, 1);
        }
    }

    pub fn print_result(&self) {
        println!("-----------------------------------");
        println!("{: <20} | {: <10}\n", "Lines processed", self.line_count);

        println!("-----------------------------------");
        println!("{: <20} | {: <15}", "Key Comment", "Frequency");
        println!("-----------------------------------");
        for (key, frequency) in self.keyword_table.iter() {
            println!("{: <20} | {: <15}", key, frequency);
        }

        println!("\n-----------------------------------");
        println!("{: <20} | {: <15}", "File Type", "Frequency");
        println!("-----------------------------------");
        for (key, frequency) in self.filetype_table.iter() {
            println!("{: <20} | {: <15}", key, frequency);
        }
    }
}
