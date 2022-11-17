// Return

// Si tenemos una función la cual por ciertos motivos necesita
// finalizar y retornar un valor antes de tiempo podemos utilizar
// la palabra reservada "Return"

// Ejemplo

// Definimos una función y le asignamos sus parámetros
fn factorial(numero: u32) -> u32 {
    if numero == 1 {
        // Si nuestro parámetro se encuentra en la ultima linea del bloque
        // podemos ahorrarnos el uso de return
         numero // Si nuestro parámetro se encuentra en la ultima linea del bloque
        // podemos ahorrarnos el uso de return
    } else { // no finalizaremos numero con punto y coma ";"
        factorial(numero - 1) * numero
    }
}
// Llamamos a la función factorial en nuestra función main
fn main() {
    let resultado = factorial(5); // Asignamos el numero del cual
    // queramos saber su factorial

    // Imprimimos el Resultado
    println!("El factorial es: {}", resultado);
}