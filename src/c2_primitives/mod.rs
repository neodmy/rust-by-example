mod intro;
mod literals_and_operators;
mod tuples;
mod arrays_and_slices;

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

    intro::intro();

    print_title("Literals and Operators");
    literals_and_operators::literals();
    print_end_separator();

    print_title("Tuples");
    tuples::tuples();
    tuples::activity();
    print_end_separator();
    
    print_title("Arrays and slices");
    arrays_and_slices::arrays_slices();
    print_end_separator();
}