fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// constantes
// las constantes son inmutables por defecto y deben tener un tipo declarado
// las constantes pueden ser declaradas en cualquier ámbito, incluso fuera de funciones
// las constantes solo pueden ser asignadas a valores constantes, no pueden ser el resultado de una
//escribir con mayusculas y guiones bajos por convención
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// shadowing
//puedes declarar un nueva variable con el mismo nombre que una variable anterior
//la segunda variable eclipsa a la primera, tomando cualquier uso de la nombre 
//de la variable a sí misma hasta que se sombree o finalice el alcance

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

//tipos de datos y anotaciones de tipos

// numeros enteros
// i8, i16, i32, i64, i128, isize 

//numeros con punto flotante
//f32, f64
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
// operaciones numericas
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

// booleanos (normal)

//char
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}


// Tipos compuestos

//tuplas
// una tupla es un grupo de valores con tipos potencialmente diferentes
// las tuplas tienen un tamaño fijo: una vez declaradas, no pueden crecer o reducirse

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
//desestructuración de tuplas
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

//se puede acceder con un punto y el índice

//tupla sin valor se llama unit

//arrays
// los arrays tienen un tamaño y tipo fijo
let a: [i32; 5] = [1, 2, 3, 4, 5];

//se puede acceder con corchetes y el índice