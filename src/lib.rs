use std::io::{self, Write};
use strum_macros::Display;
use std::process::exit;

// Global state mode
#[derive(Display, Debug)]
pub enum Mode {
    #[strum(serialize = "Command mode")]
    CommandMode,
    #[strum(serialize = "Insert mode")]
    InsertMode,
}

pub fn print_welcome(mode: &Mode) {
    print!("\
    # Welcome to rust-editor, a side-project by Dani Sancas.\n\
    # This simple CLI editor enables you to write some text to a file.\n\
    # It has two modes: Command mode and Insert mode.\n\
    # While using the Command mode you will see a prompt prepended by the \":\" character.\n\
    # On Insert mode, you can write your text, the prompt will be prepended by the \">\" character.\n\
    # The instructions (like this) are prepended by the \"#\" character.\n\
    # To see the commands type \"h\" or \"help\" + Enter on Command mode to display the full list.\n\
");
    self::print_current_mode(mode);
}

fn print_current_mode(mode: &Mode) {
    println!("# You are currently in {}.", mode.to_string());
}

fn print_help(mode: &Mode) {
    println!("<Help text placeholder>");
    self::print_current_mode(mode);
}

pub fn do_on_command_mode(mode: &mut Mode, buffer: &mut String) {
    print!(": ");
    io::stdout().flush().unwrap();

    let mut command: String = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("# Failed to read line. Exiting application...");

    //println!("Your command: {}", command);
    match command.trim() {
        "h" | "help" => print_help(mode),
        "i" | "insert" => enter_insert_mode(mode),
        "w" | "write" => write_to_file(),
        "q" | "quit" => exit_app(),
        _ => println!("Command not recognized. Type \"h\" or \"help\" + Enter to display the full list."),
    }
}

pub fn do_on_insert_mode(mode: &mut Mode, buffer: &mut String) {

}

fn enter_insert_mode(mode: &mut Mode) {
    *mode = Mode::InsertMode;
    print_current_mode(mode);
}

fn write_to_file() {

}

fn exit_app() {
    exit(0)
}