// Match Parte 2
fn main() {
    // Con Match no estamos limitados únicamente a evaluar sobre números enteros
    // podemos evaluar estructuras mucho mas complejas

    // Crearemos nuevamente el algoritmo Fizz Buzz pero esta vez, usaremos Match

    for numero in 1..31 {
    // Evaluamos una Tupla en donde el primer valor será el numero,
        // el resultado módulo de tres
    // El segundo valor numero, el resultado modulo de 5
        match (numero % 3, numero % 5) {
        // Definimos las condiciones dentro de nuestro bloque
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", numero) // También es posible re factorizar el ultimo
            // elemento comentando la linea de código 17 y
            // des comentando la linea de código 20
            //_ => println!("{}", numero) // Des comenta esta linea
            // para ver resultado re factorizado
        }
    }
}
