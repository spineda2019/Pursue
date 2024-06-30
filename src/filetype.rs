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

macro_rules! stringify_filetype {
    ($ft: ident) => {
        match $ft {
            FileType::C {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "C",
            FileType::CHeader {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "C Header",
            FileType::Cpp {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "C++",
            FileType::CppHeader {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "C++ Header",
            FileType::CSharp {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "C#",
            FileType::Java {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "Java",
            FileType::Python {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "Python",
            FileType::Go {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "Go",
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
            FileType::Json {
                inline_comment_format: _,
                multiline_comment_start_format: _,
                multiline_comment_end_format: _,
            } => "JSON",
        }
    };
}

macro_rules! destructure_filetype {
    ($ft: ident) => {
        match $ft {
            FileType::C {
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            } => (
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            ),
            FileType::CHeader {
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
            FileType::CppHeader {
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            } => (
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            ),
            FileType::CSharp {
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            } => (
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            ),
            FileType::Java {
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
            FileType::Go {
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
            FileType::Json {
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            } => (
                inline_comment_format,
                multiline_comment_start_format,
                multiline_comment_end_format,
            ),
        }
    };
}

pub(crate) use destructure_filetype;
pub(crate) use stringify_filetype;

pub enum FileType<'b> {
    C {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    CHeader {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Cpp {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    CppHeader {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    CSharp {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Java {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Python {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
    Go {
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
    Json {
        inline_comment_format: Option<&'b str>,
        multiline_comment_start_format: Option<&'b str>,
        multiline_comment_end_format: Option<&'b str>,
    },
}
