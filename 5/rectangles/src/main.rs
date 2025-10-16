struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}

///metodos


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

//FUNCIONES ASOCIADAS
//no estan asciadasa una instancia en particular
//suelen usa

impl Rectangle {
    // Constructor convencional: Rectangle::new(w, h)
    fn new(width: u32, height: u32) -> Self { 
        Self { width, height }
    }
}

let sq = Rectangle::square(3); // Creamos una instancia sin tener una previa
// sq.width es 3, sq.height es 3rse para crear instancias de la estructura