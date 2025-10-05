//no importa donde esten las funciones, siempre y cuando esten declaradas

fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//declaraciones  y exprecioens

//declaraciones: no retornan valor
//expresiones: retornan valor, no devuelven ; al final

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

//funciones que retornan valor
// se puede retiornar con return
//pero por defecto retorna la ultima expresion sin ;
fn five() -> i32 {
    5
}