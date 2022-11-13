// Librerías
use std::io;

// Condicionales en Rust
fn main() {
    // Definimos una nueva variable de tipo &str (String)

    let color = "Verde";

    // Para crear condiciones haremos uso de la palabra reservada "if"

    // Para este ejemplo, simularemos un semáforo
    // mediante el uso de condicionales

    // Usamos un juego de llaves "{}" al igual que en nuestra función main
    if color == "Verde" { // La condición debe retornar un valor booleano (true/false)
        println!("Puede continuar.");
        // Dependiendo del resultado se ejecutara o no, un bloque del código

        // Dependiendo del color vamos a enviar un mensaje en consola
    } else { // En caso de que no se cumpla la condición utilizamos else y
        // damos paso al siguiente bloque de código cuando la condición no se cumpla
        println!("El color no es verde");
    }
    // También podemos usar else if para crear mas bloques de código
    // dentro de una condición

    let color = "Verde";

    if color == "Verde" {
        println!("Puede continuar.");
    // Evaluamos la condición si es amarillo
    } else if color == "Amarillo" { // usamos if después de else
                                    // para crear otra condición
        println!("Alto parcial.");
        // Y hacemos lo mismo si la condición si la condición es rojo
    } else if color == "Rojo" {
        println!("Alto total.");
        // Si ninguna de las condiciones se cumplen
        // enviamos a consola que el color no es válido
    } else {
        println!("El color no es válido: {}", color);
    }
    // Ahora utilizando lo aprendido anteriormente
    // haremos que el usuario pueda introducir un color a través de la consola

    let mut color = String::new();

    println!("Ingresa un color para el semáforo:");

// Uso de let _ para evitar Warnings
    let _ = io::stdin().read_line(&mut color);

    let color = color.trim().to_lowercase(); // utilizamos to_lowercase para
    // que el dato introducido sea integrado en minúsculas

    if color == "verde" {
        println!("Puede continuar.");

    } else if color == "amarillo" {
        println!("Alto parcial.");

    } else if color == "rojo" {
        println!("Alto total.");

    } else {
        println!("El color no es valido: {}", color);
    }

}
