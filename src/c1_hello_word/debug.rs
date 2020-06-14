/*
    All types which want to use std::fmt formatting traits require an implementation to be printable. Automatic implementations
    are only provided for types such as in the std library. All others must be manually implemented somehow.

    The fmt::Debug trait makes this very straightforward. All types can derive (automatically create) the fmt::Debug implementation.
    This is not true for fmt::Display which must be manually implemented.
*/
pub fn debug_1() {
    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    #[allow(dead_code)]
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct DebugPrintable(i32);
}

// All std library types automatically are printable with {:?} too:
pub fn debug_2() {
    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    println!("This `{:?}` is a struct", Structure(3));

    // Put a `Structure` inside of the structure `Deep`. Make it printable also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    //  Rust also provides "pretty printing" with {:#?}.
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}