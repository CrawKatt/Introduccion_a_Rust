fn main() {
    // Existen dos formas de hacer variables constantes en Rust
    // por convención vamos a colocar el nombre de la constante en mayúsculas
    const VALOR : i32 = 10; // una constante es diferente de una variable,
    // no puede ser mutable y se debe definir el tipo de variable en las
    // variables constantes

    // Mismas reglas en el uso de static
    static VAL : i32 = 5;

    let mut numero_uno = 15;
    let numero_dos : i32 = 10;

    numero_uno = 100;

    let resultado = numero_uno + numero_dos + VALOR;

    println!("El resultado es ({} + {} + {} = {})", numero_uno, numero_dos, VALOR, resultado);
}
