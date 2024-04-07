/* 
fn first_name() {
    print!("Deepak ");
}
fn last_name() {
    println!("Kumar");
}
*/

// a2
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result);
}
fn main() {
    // a1
    /* 
    first_name();
    last_name();
    */

    // a2
    let result = add(45, 32);
    display_result(result);

    
}