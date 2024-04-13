
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(5.23, "Billy".to_owned()),
        Ticket::Standard(10.50),
        Ticket::Vip(80.65, "Gates".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, name) => {
                println!("Backstage -> Name: {:?}, Price: {:?}", name, price)
            },
            Ticket::Standard(price) => println!("Standard -> Price: {:?}", price),
            Ticket::Vip(price, name) => {
                println!("VIP -> Name: {:?}, Price: {:?}", name, price)
            },
        }
    }
}