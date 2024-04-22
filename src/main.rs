use clap::{ArgMatches, Command};

fn main() {
    let matches = Command::new("rrfe-rs")
        .version("0.1.0")
        .author("sunsunmonkey")
        .about("redrockfe utils")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("create").about("create template"))
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => move_files(sub_matches),
        _ => unreachable!(),
    }
}

fn move_files(sub_matches: &ArgMatches) {
    print!("{:?}", sub_matches);
    print!("移动文件");
}
