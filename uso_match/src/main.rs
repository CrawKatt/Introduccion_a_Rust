// Match
fn main() {
    // "Match" es el equivalente a "Switch" en otros lenguajes
    // Con Match podemos evaluar un valor en diferentes casos

    // Ejemplo

    // Definimos una nueva variable de tipo Entero (i32)
    let numero : i32 = 55;

    // Usando Match, vamos a definir todos los posibles casos para nuestra variable

    // Utilizamos la palabra reservada "Match" seguida de nuestra
    // variable y un juego de llaves "{}"
    match numero {
        // Para definir los casos vamos a seguir la siguiente estructura

        // valor => sentencia a ejecutar / tarea a realizar

        // Si vamos a trabajar con mas de un caso entonces vamos a finalizar con coma ","

        // Evaluamos si numero es igual a 1

        // y colocamos la tarea a realizar, en este caso, un println!()
        // En caso de que numero tenga valor 1 se procederá a imprimir
        1 => println!("El numero es uno."),
        // En caso de que numero tenga valor 2 se procederá a imprimir
        2 => println!("El numero es dos."),
        // En caso de que numero tenga valor 3 se procederá a imprimir
        3 => println!("El numero es tres."),
        // En caso de que numero tenga valor 4, 5 o 6 se procederá a imprimir
        4 | 5 | 6 => println!("El numero esta entre cuatro y seis"),
        // Si queremos ejecutar mas de una tarea o mas de una sentencia
        // podemos utilizar los bloques
        7..=100 => {
            println!("El numero es mayor o igual a 7");
            println!("El numero se evalúa mediante un rango");
        },

        // En caso de que numero tenga otro valor se procederá a imprimir
        _ => println!("{}", numero) // _ Es el equivalente a Default en otros lenguajes
        // En el caso de Match, no es necesario finalizar con punto y coma ";"
    }
    // También podemos usar match dentro de una variable y
    // evitar crear múltiples println()

    let mensaje = match numero {
        1 => "El numero es uno",
        2 => "El numero es dos",
        3 => "El numero es tres",
        4 | 5 | 6 => "El numero se encuentra entre cuatro y seis",
        7..=100 => {
            let mensaje = "El numero se evalúa mediante un rango del 7 al 100";

            mensaje
        }
        _ => "numero"
    };

    println!("El resultado es: {}", mensaje);

}
