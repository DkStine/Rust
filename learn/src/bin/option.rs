struct Locker {
    name: String,
    locker_assgn: Option<i32>
}

fn main() {
    let s1_locker = Locker {
        name: "Akhil".to_owned(),
        locker_assgn: Some(10)
    };
    let s2_locker = Locker {
        name: "Sukhbir".to_owned(),
        locker_assgn: None
    };
    
    println!("Name: {:?}", s1_locker.name);
    match s1_locker.locker_assgn {
        Some(num) => println!("Locker no.: {:?}", num),
        None => println!("No locker assigned")
    }
    
    println!("Name: {:?}", s2_locker.name);
    match s2_locker.locker_assgn {
        Some(num) => println!("Locker no.: {:?}", num),
        None => println!("No locker assigned")
    }
}