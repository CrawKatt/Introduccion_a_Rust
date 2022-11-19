// Panic en Rust
fn main() {
    // En Rust existe una función macro llamada "panic" la cual nos permite
    // finalizar el programa en caso que exista algún tipo de error

    println!("Hola soy un mensaje de la linea 6");

    println!("Hola soy un mensaje de la linea 8");
    // Para simular un error entre estas dos lineas haremos
    // uso de la función macro panic!

    panic!("El programa finaliza de forma inesperada!!!"); // Esta función
    // recibe como argumento el mensaje el cual queremos mostrar al usuario,
    // el mensaje de error

    // En Rust no existen las excepciones/null

    println!("Hola soy un mensaje de la linea 10");

    println!("Hola soy un mensaje de la linea 12");

}
