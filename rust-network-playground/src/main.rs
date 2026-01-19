fn main() {
    //step 1 - hello, world
    //'rustup' is managing Rust
    //rust always end '.rs'
    //using more than one word in filename, use an underscore to seprate.
    //main function is fist of pices. always do first front code.
    //if you have parameters, must be use in ().
    //function declaire use to {}.
    //easy to standard with 'rustfmt', is auto fomatting.
    //'println' is expression text function.
    //1.Rust 매크로 - 문자열을 인수로 전달 - println 화면에 출력
    //must be use ; when you end the line.
    println!("Hello, world!");
    //compile sepearte execution.
    //it's similar C/C++.
    //'./main' is done with compile.
    //Rust is ahead-of-time compiled language.
    //'rustc' is enough simple compile but we have to do study 'Cargo'
    //using 'rustc' is not make Cargo.toml.
    // [package]
    // name = "hello_cargo"
    // version = "0.1.0"
    // edition = "2024"
    // [dependencies]
    // it's Tom's Obvious, Minimal Language style.

    //step 2 - primitives
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    
}
