use argparse::{ArgumentParser, StoreTrue};

fn main() {
    let mut logging: bool = false;

    {
        let mut argument_parser: ArgumentParser = ArgumentParser::new();
        argument_parser.set_description("Check your codebase FAST");

        argument_parser.refer(&mut logging).add_option(
            &["-l", "--log"],
            StoreTrue,
            "Output logging of file locations",
        );

        argument_parser.parse_args_or_exit();
    }

    println!("Logging: {}", logging);
}
