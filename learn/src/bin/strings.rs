/* 
struct LineItems {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    print!("Item: {:?} ", name);
}
*/

struct Person {
    name: String,
    age: i32, 
    fav_color: String,
}


fn display_name_color(peep: &Person) {
    println!("Name: {:?} -> Fav Color: {:?}", peep.name, peep.fav_color);
}


fn main() {
    /* 
    let items = vec![
        LineItems {
            name: "cereal".to_owned(),
            count: 3,
        },
        LineItems {
            name: "medicine".to_owned(),
            count: 5,
        }
    ];

    for item in &items {
        print_name(&item.name);
        println!("Qty: {:?}", item.count);
    }
    */

    let people = vec![
        Person {
            name: "Neil".to_owned(),
            age: 9,
            fav_color: "Black".to_owned(),
        },
        Person {
            name: "Nitin".to_owned(),
            age: 35,
            fav_color: "Green".to_owned(),
        },
        Person {
            name: "Mukesh".to_owned(),
            age: 6,
            fav_color: "Red".to_owned(),
        },
    ];

    for peep in &people {
        if peep.age <= 10 {
            display_name_color(peep);
        }
    }
}