/*
    fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output
    appearance. This is done by manually implementing fmt::Display, which uses the {} print marker.
    Implementing it looks like this:
*/

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

pub fn display_1() {
    // Define a structure for which `fmt::Display` will be implemented. This is
    // a tuple struct named `Structure` that contains an `i32`.
    struct Structure(i32);

    // To use the `{}` marker, the trait `fmt::Display` must be implemented
    // manually for the type.
    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }

    println!(
        "Now Structure(i32) implementing fmt::Dispaly looks like this: {}",
        Structure(32)
    );
}

/*
    fmt::Display may be cleaner than fmt::Debug but this presents a problem for the std library.
    How should ambiguous types be displayed? For example, if the std library implemented a single
    style for all Vec<T>, what style should it be? Would it be either of these two?
        - Vec<path>: /:/etc:/home/username:/bin (split on :)
        - Vec<number>: 1,2,3 (split on ,)

    No, because there is no ideal style for all types and the std library doesn't presume to dictate one.
    fmt::Display is not implemented for Vec<T> or for any other generic containers. fmt::Debug must then
    be used for these generic cases.

    This is not a problem though because for any new container type which is not generic,fmt::Display can be implemented.
*/

pub fn display_2() {
    // A structure holding two numbers. `Debug` will be derived so the results can
    // be contrasted with `Display`.
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "{}, {}", self.0, self.1)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
}

/*
    So, fmt::Display has been implemented but fmt::Binary has not, and therefore
    cannot be used. std::fmt has many such traits and each requires its own implementation.
    This is detailed further in std::fmt.
*/

/*
    Activity
    After checking the output of the above example, use the Point2D struct as a guide to add a Complex
    struct to the example. When printed in the same way, the output should be:

    Display: 3.3 + 7.2i
    Debug: Complex { real: 3.3, imag: 7.2 }
*/

pub fn display_activity() {
    #[derive(fmt::Debug)]
    struct Complex {
        real: f64,
        imaginary: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imaginary)
        }
    }

    let complex = Complex {
        real: 3.3,
        imaginary: 7.2,
    };

    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

/*
    Implementing fmt::Display for a structure where the elements must each be handled sequentially
    is tricky. The problem is that each write! generates a fmt::Result. Proper handling of this
    requires dealing with all the results. Rust provides the ? operator for exactly this purpose.

    Using ? on write! looks like this:

    // Try `write!` to see if it errors. If it errors, return
    // the error. Otherwise continue.
    write!(f, "{}", value)?;

    Alternatively, you can also use the try! macro, which works the same way. This is a bit more verbose
    and no longer recommended, but you may still see it in older Rust code. Using try! looks like this:

    try!(write!(f, "{}", value));
*/

pub fn testcase_list() {
    // With ? available, implementing fmt::Display for a Vec is straightforward:

    // Define a structure named `List` containing a `Vec`.
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and create a reference to `vec`.
            let vec = &self.0;

            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator, or try!, to return on errors.
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    let v = List(vec!(1, 2, 3));
    println!("{}", v);
}


pub fn testcase_list_activity() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for(count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
