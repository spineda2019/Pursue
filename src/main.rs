use argparse::{ArgumentParser, StoreTrue};

const VERSION: &str = "0.0.1";

fn print_version_info() {
    println!("Trace version {}", VERSION);
}

fn main() {
    let mut logging: bool = false;
    let mut print_version: bool = false;

    {
        let mut argument_parser: ArgumentParser = ArgumentParser::new();
        argument_parser.set_description("Check your codebase FAST");

        argument_parser.refer(&mut logging).add_option(
            &["-l", "--log"],
            StoreTrue,
            "Output logging of file locations",
        );

        argument_parser.refer(&mut print_version).add_option(
            &["-v", "--version"],
            StoreTrue,
            "Print version and license information",
        );

        argument_parser.parse_args_or_exit();
    }

    if print_version {
        print_version_info();
        return;
    }

    println!("Logging: {}", logging);
}
