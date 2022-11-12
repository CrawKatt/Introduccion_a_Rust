// Leer datos por Consola

// Usamos una librería standard de Rust
use std::io; // use standard library, input output;

fn main() {
    // Imprimimos un mensaje en consola
    println!("Ingresa el nombre de usuario");

    // Usamos una variable mutable que almacenará un string vacío
    let mut username = String::new(); // Static -> ""

    // Usamos la librería para leer datos desde la consola (en este caso username)
             // read_line retorna un valor de tipo Result (hablaremos de el más adelante)
    io::stdin().read_line(&mut username); // Introducimos la variable username
    // como argumento dentro de los paréntesis en read_line
    // mediante referencia/préstamo usando & con permisos de mutabilidad
    // (hablaremos sobre los prestamos/referencia mas adelante)

    // para imprimir sin saltos de linea creamos una nueva variable del mismo nombre
    // que la anterior y le damos el valor de la anterior variable y usamos el método
    //.trim(); que elimina los saltos de linea
    let username = username.trim();

    // obtener dato mediante método Result
    println!("Ingresa la edad del usuario");

    let mut edad = String::new();

    io::stdin().read_line(&mut edad);

    // al usar shadowing estamos usando una nueva variable
    let edad = edad.trim(); // convertimos el String en un i32 (entero)

    // utilizamos el método .parse() y el método unwrap() para obtener
    // el valor ya parseado
    // (hablaremos mas en detalle de estas funciones mas adelante)
    let edad : i32 = edad.parse().unwrap();

    // Imprimimos en consola lo que el usuario ingrese utilizando la variable username
    println!("Hola {} con edad {}", username, edad); // AVISO: el salto de línea
    // se aplica por defecto al imprimir
}
