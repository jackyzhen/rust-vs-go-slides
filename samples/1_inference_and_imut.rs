fn main() {

    // explicit type annotation
    let a_float: f64 = 1.0;

    // type inference
    let an_integer = 2;

    // opt in mutability
    let mut can_change = false;
    can_change = true;

    // variable shadowing
    let can_change = false;
    
    print_things(a_float, an_integer, can_change);
}

fn print_things(float: f64, integer: usize, boolean: bool) {
    print!("{}-{}-{}", float, integer, boolean);
}
