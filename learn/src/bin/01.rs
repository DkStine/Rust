/* 
fn first_name() {
    print!("Deepak ");
}
fn last_name() {
    println!("Kumar");
}



// a2
 
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result);
}


 
enum Colors {
    Red,
    Green,
    Blue,
    Black,
    White,
    Violet,
    Indigo,
}

fn print_color(color : Colors) {
    match color {
        Colors::Black => println!("It's Black"),
        Colors::Blue => println!("It's Blue"),
        Colors::Red => println!("It's red"),
        _ => println!("Invalid"),
    }
}


 
struct GroceryItems {
    items: i32,
    stock: i32,
}


enum Flavor {
    Sparkling,
    Orange,
    Mango,
    Coke,
}

struct Drink {
    flavor: Flavor,
    fl_oz: i64,
}

fn print_drinks(drink: Drink) {
    match drink.flavor {
        Flavor::Coke => println!("This is Coke!"),
        Flavor::Sparkling => println!("This is Sparkling!"),
        Flavor::Orange => println!("This is Orange!"),
        Flavor::Mango => println!("This is Mango!"),
    }
    println!("Quantity: {:?} floz", drink.fl_oz);
}


fn cart_coordinates() -> (i32, i32) {
    (4, 10)
}


struct Book {
    pages: i32,
    rating: i32,
}

fn display_pages(book: &Book) {
    println!("Pages: {:?}", book.pages);
}
fn display_rating(book: &Book) {
    println!("Rating: {:?}", book.rating);
}


struct GroceryItems {
    qty: i32,
    id: i32,
}

fn display_qty(item: &GroceryItems) {
    println!("Quantity: {:?}", item.qty);
}
fn display_id(item: &GroceryItems) {
    println!("Id: {:?}", item.id);
}
*/

fn main() {
    // a1
    /* 
    first_name();
    last_name();
    

    // a2
    // let result = add(45, 32);
    // display_result(result);

    // 3a
    
    let _bool = true;
    if _bool == true {
        println!("hello");
    } else {
        println!("Goodbye");
    }
    

    // 3b
    
    let n = 7;

    if n > 5 {
        println!(">5");
    } else if n < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
    
    
    // 4a
    
    let n = 2;

    match n {
        1 => println!("It's one"),
        2 => println!("It's two"),
        3 => println!("It's three"),
        _ => println!("Something else"),
    }


    // loop
    
    let mut i = 5;

    loop {
        if i == 0 {
            break;
        }
        println!("{:?}", i);
        i -= 1;
    }
    println!("Done");
    

    // while
    
    let mut j = 10;

    while j > 0 {
        println!("{:?}", j);
        j -= 1;
    }
    println!("Done!");
    

    // a7 -> Enum
    
    let color = Colors::Black;
    print_color(color);
    

    // Struct
    
    let groc = GroceryItems {
        items: 45,
        stock: 9,
    };

    println!("No. of items: {:?}", groc.items);
    println!("How much?: {:?}", groc.stock);
    

    let spark = Drink {
        flavor: Flavor::Sparkling,
        fl_oz: 45,
    };
    print_drinks(spark);
    

    // Tuple
    let (x, y) = cart_coordinates();
    if y > 5 {
        println!("{:?} > 5", y);
    }
    else if y < 5 {
        println!("{:?} < 5", y);
    }
    else {
        println!("{:?} = 5", y);
    }
    


    // Ownership and Borrowing
    let book = Book {
        pages: 53,
        rating: 5,
    };

    // display_pages(book) --> Referencing so the ownership transfers to display_pages fn
    display_pages(&book);  // --> Borrowing so the ownership remains with main() fn only
    display_rating(&book);
    

    let item = GroceryItems {
        id: 42563,
        qty: 80,
    };

    display_id(&item);
    display_qty(&item);
    */


}