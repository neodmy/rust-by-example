/*
    Rust provides access to a wide variety of primitives. A sample includes:

    Scalar Types
        - signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
        - unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
        - floating point: f32, f64
        - char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
        - bool either true or false
        - and the unit type (), whose only possible value is an empty tuple: ()

    Despite the value of a unit type being a tuple, it is not considered a compound 
    type because it does not contain multiple values.

    Compound Types
        - arrays like [1, 2, 3]
        - tuples like (1, true)

    Variables can always be type annotated. Numbers may additionally be annotated via 
    a suffix or by default. Integers default to i32 and floats to f64. Note that Rust 
    can also infer types from context.
*/

pub fn intro() {
    // Variables can be type annotated.
    let _logical: bool = true;

    // Regular annotation
    let _a_float: f64 = 1.0;
    // Suffix annotation
    let _an_intenger = 5i32;

    // A type can also be inferred from context
    let mut _inferred_type = 12; // Type i64 is inferred from another line
    _inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut _mutable = 12; // Mutable 'i32'
    _mutable = 21;

    // Error! The type of a variable can't be changed.
    // _mutable = true;

    // Variables can be overwritten with shadowing.
    let _mutable = true;
}