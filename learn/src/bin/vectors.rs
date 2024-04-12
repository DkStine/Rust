fn main() {
    let numbers = vec![10, 20, 30, 40];
    for num in &numbers {
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}", num),
        }
    }

    println!("The no. of elements is: {:?}", numbers.len());

}