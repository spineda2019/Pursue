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
