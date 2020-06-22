mod lib;

use lib::Mode;
use lib::print_welcome;

fn main() {
    // Init state
    let mut current_mode = Mode::CommandMode;
    let mut buffer: String = String::new();

    // Print welcome message
    print_welcome(&current_mode);

    // Begin app loop
    loop {
        match current_mode {
            Mode::CommandMode => lib::do_on_command_mode(&mut current_mode, &mut buffer),
            Mode::InsertMode => lib::do_on_insert_mode(&mut current_mode, &mut buffer),
        }
    }
}
