#[allow(dead_code)]
pub fn comments() {
    /*
        You can manipulate expressions more easily with block comments
        than with line comments. Try deleting the comment delimiters
        to change the result:
    */

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}