// Return

// Definimos una función y le asignamos sus parámetros
fn factorial(numero: u32) -> u32 {
    if numero == 1 {
        numero; // Si nuestro parámetro se encuentra en la ultima linea del bloque
        // podemos ahorrarnos el uso de return
    } else {
        factorial(numero - 1) * numero
    }
}

fn main() {
    let resultado = factorial(5);

    println!("El factorial es: {}", resultado);
}