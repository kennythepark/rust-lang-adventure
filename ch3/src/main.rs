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

fn main() {
    mutability_example();
    shadowing_example();
}
