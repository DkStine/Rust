/* 
struct Temperature {
    degrees: f64,
}

impl Temperature {
    fn freezing_temp() -> Self {
        Self {
            degrees: 0.0,
        }
    }
    fn boiling_temp() -> Self {
        Self {
            degrees: 100.0,
        }
    }
    fn display_temp(&self) {
        println!("{:?} degrees", self.degrees);
    }
}
*/

enum Color {
    Black, 
    Brown,
    White,
}
struct Box {
    dimensions: i32,
    weight: f64,
    color: Color,
}

impl Box {
    fn new_box(d: i32, w: f64, c: Color) -> Self {
        Self {
            dimensions: d, 
            weight: w,
            color: c,
        }
    }
    fn print_characteristics(&self) {
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {:?}", self.weight);
        match self.color {
            Color::Black => println!("Color: Black"),
            Color::Brown => println!("Color: Brown"),
            Color::White => println!("Color: White"),
            _ => println!("Colorless"),
        }
    }
}

fn main() {
    /* 
    let temp = Temperature {
        degrees: 45.6,
    };
    temp.display_temp();

    let cold = Temperature::freezing_temp();
    cold.display_temp();

    Temperature::display_temp(&Temperature::boiling_temp());
    Temperature::display_temp(&Temperature::freezing_temp());
    */

    let box_1 = Box::new_box(41, 15.36, Color::Brown);
    box_1.print_characteristics();

    let box_2 = Box::new_box(32, 20.14, Color::Black);
    box_2.print_characteristics();

    let box_3 = Box::new_box(71, 18.2, Color::White);
    box_3.print_characteristics();
}