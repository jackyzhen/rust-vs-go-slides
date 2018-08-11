fn main() {
    // _Stack_ allocated integer
    let x = 5;

    // *Copy* 'x' into 'y' - no resources are 'moved'.
    // Primitive types are copy by default because their size is
    // known at compile time and allocated on stack.
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y); // x is 5, and y is 5

    // 'a' is a _heap_ allocated, growable list
    let a = vec![1, 2, 3, x, y];

    // The pointer address of 'a' is copied (not the data) into 'b'.
    // Both are now pointers to the same heap allocated data, but
    // 'b' now owns it. Move 'a' into 'b'.
    let b = a;

    // println!("a contains: {:?}", a); // ERROR

    take_ownership_and_destroy_list(b);

    // println!("b contains: {:?}", b); // ERROR
}

fn take_ownership_and_destroy_list(xs: Vec<isize>) {
    println!("Destroying a list {:?}", xs);
} // 'xs' is dropped at end of function scope and the memory freed

