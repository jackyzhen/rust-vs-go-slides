// Generic type T with trait bounds
fn max<T: Ord + Clone>(xs: &Vec<T>) -> T {
    let mut max = &xs[0];
    
    for v in xs.iter() {
        if v > max {
            max = v;
        }
    } 
    // no semicolon = expression
    max.clone()
}

fn main() {
    let xs_int = vec![3,4,1,7,8,324];
    let xs_str = vec!["aaa", "bbb", "ccc", "ddd"];
    println!("{} {}", max(&xs_int), max(&xs_str));

    // std lib max function for generic vector
    println!("max = {:?}", xs_str.iter().max());
}
