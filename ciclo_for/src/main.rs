// Ciclo for en Rust
fn main() {
    // En Rust, el ciclo for nos permitirá iterar sobre una colección de datos.
    // Ya sea un vector, un Arreglo/Array, una tupla, etc.

    // El ciclo for funcionara como un for each

    // Comenzamos creando un Arreglo/Array

    let numeros : [i32; 5] = [1, 2, 3, 4, 5];

    // Iteramos nuestro Arreglo/Array utilizando el ciclo for

    // Utilizamos la palabra reservada for y definimos un nuevo bloque

    // Aplicamos el método iter a nuestro Arreglo/Array que
    // devuelve una estructura iterable
    for numero in numeros.iter() { // Este bloque se ejecutara por cada elemento
                                   // dentro de la colección
        println!("El valor de número: {:?}", numero);
    }

    // Podemos utilizar el ciclo for junto con un rango
    // El rango lo podemos hacer colocando un numero de inicio y un numero final
    // separando estos números con dos puntos seguidos ".."
    for numero in 1..101 {
        println!("{:?}", numero);
    }

    // Algoritmo Fizz Buzz

    // El algoritmo dicta lo siguiente:

    // Si un número es divisible entre 3 entonces imprimimos "Fizz" en consola
    // Si un número es divisible entre 5 vamos a imprimir en consola "Buzz"
    // Si un numero es divisible entre 3 y 5 vamos a imprimir "Fizz Buzz"

    // Creamos un ciclo for y creamos dentro de su bloque una condición
    for numero in 1..101 {

        if numero  % 3 == 0 && numero % 5 == 0 {
            println!("Fizz Buzz");

        } else if numero % 3 == 0 {
            println!("Fizz");

        } else if numero % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", numero);
        }
    }
}
