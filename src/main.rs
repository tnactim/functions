fn main() {
    println!( "Hello, world!" );

    another_function( 5, 6.0 );

    let x = five();
    println!( "The value of x is: {}", x );
    println!("x + 1 is: {}", plus_one( x ) );
}

// must declare type of each parameter in function signature
fn another_function( x: i32, y: f64 ) {
    println!( "The value of x is: {}", x );
    println!( "The value of y is: {}", y );

    // cannot use statements as expressions
    // let x = (let y = 6)  or
    // x = y = 6            are not allowed
    // expressions evaluate to something, and can be a part of a statement
    // e.g. calling a function is an expression, same with a macro or
    // new scopes with a block {}
    let y = {
        let x = 3;
        x + 1       // no ; at the end of an expression
    };              // (would turn into statement)
    println!( "The value of y is: {}", y );
}

// functions with Return Values
// don't name return values, but declare their type after an arrow ->
fn five() -> i32 {
    5
}

fn plus_one( x: i32 ) -> i32 {
    x + 1
}
