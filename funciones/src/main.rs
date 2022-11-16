// Funciones

// Definimos una nueva función que nos permita saludar a los usuarios
// para ellos utilizamos la palabra reservada "fn" y
// posteriormente asignamos un nombre a nuestra función

// Por convicción vamos a utilizar snake_case para el nombre de nuestra función

// Colocamos paréntesis "()" y luego creamos un bloque con un juego de llaves "{}"
fn saludar_usuarios() { // Dentro del bloque vamos a colocar todas las sentencias
    // las cuales queremos que se ejecuten cuando la función sea llamada

    // En este caso haremos una impresión en consola
    println!("Hola desde una función");
}

// Para llamar a nuestra función basta con llamar a la función utilizando el nombre
fn main() {
    println!("Hello, world!");
    saludar_usuarios(); // Llamamos a la función seguido de unos paréntesis "()"
    // y finalizamos con punto y coma ";"

    // Creamos la variable de resultado y utilizamos la función suma seguida
    // de paréntesis con nuestros parámetros dentro"()"
    let resultado = suma(10, 20);
    println!("El resultado es: {}", resultado);
}

// También podemos definir parámetros dentro de una función
// definiendo los parámetros dentro de los paréntesis de nuestra función

// Asignamos el parámetro y su tipo de dato y el tipo de dato del resultado
// seguido de un bloque con un juego de llaves "{}"
// Para asignar que la función va a retornar algo colocamos - y > dibujando una flecha "->"
fn suma(numero_uno : i32, numero_dos : i32) -> i32 { // "Suma" suma dos valores
    // y retorna un valor de tipo i32

    // Hacemos la suma de numero_uno y numero_dos
    numero_uno + numero_dos // No es necesario ";" ni la palabra reservada return
    // ya que la última línea que se ejecute en un bloque sera la que se retorne

}
