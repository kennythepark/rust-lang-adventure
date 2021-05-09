const MAX_POINTS: u32 = 100_000_000;

fn mutability_example() {
    /*

    There are multiple trade-offs to consider in addition to the
    prevention of bugs. For example, in cases where you’re using
    large data structures, mutating an instance in place may be faster
    than copying and returning newly allocated instances. With smaller
    data structures, creating new instances and writing in a more
    functional programming style may be easier to think through, so lower
    performance might be a worthwhile penalty for gaining that clarity.

    Link: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

    */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = MAX_POINTS;
    println!("The value of x is: {}", x);
}

fn shadowing_example() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn expression_in_block_example() {
    /*

    Expressions do not include ending semicolons. If you add
    a semicolon to the end of an expression, you turn it into
    a statement, which will then not return a value.

    */
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// This function will cause a panic past
// n_pos value of 46.
fn fibonacci(n_pos: u32) -> u32 {
    let mut buffer: (u32, u32) = (0, 1);
    let mut iter = 1;

    while iter <= n_pos {
        let (n_minus_2, n_minus_1) = buffer;
        let n = n_minus_2 + n_minus_1;
        println!("Result at iteration {}: {}", iter, n);

        buffer = (n_minus_1, n);
        iter += 1;
    }

    return buffer.1;
}

fn main() {
    mutability_example();
    shadowing_example();
    expression_in_block_example();

    println!("Fibonacci result: {}", fibonacci(40));
}
