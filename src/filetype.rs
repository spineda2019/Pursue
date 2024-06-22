/*
 *  filetype.rs - Store grammar information about different filetypes
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

pub enum FileType<'b> {
    C {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Cpp {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Python {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Rust {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Zig {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Javascript {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Typescript {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Makefile {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
}
