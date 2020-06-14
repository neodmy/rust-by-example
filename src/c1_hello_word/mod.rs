mod hello_world;
mod comments;
mod formatted_print;
mod debug;
mod display;
mod formatting;

const TITLE_MAX_SIZE: usize = 80;

fn print_title(title: &str) {
    let whitespaced_title = format!(" {} ", title);
    println!("{1:-^0$}", TITLE_MAX_SIZE, whitespaced_title);
}

fn print_end_separator() {
    println!("{1:-^0$}\n", TITLE_MAX_SIZE, '-');
}

#[allow(dead_code)]
pub fn run() {
    // hello world
    print_title("Hello World");
    hello_world::hello();
    hello_world::rustacean_greeting();
    print_end_separator();

    // comments
    print_title("Comments");
    comments::comments();
    print_end_separator();

    // formatted_print
    print_title("Formatted Print");
    formatted_print::formatted_print();
    formatted_print::formatted_print_activity();
    print_end_separator();

    // debug
    print_title("Debug");
    debug::debug_1();
    debug::debug_2();
    print_end_separator();

    // display
    print_title("Display");
    display::display_1();
    display::display_2();
    display::display_activity();
    display::testcase_list();
    display::testcase_list_activity();
    print_end_separator();

    // formatting
    print_title("Formatting");
    formatting::formatting();
    print_end_separator();
}