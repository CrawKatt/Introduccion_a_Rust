// Enums
fn main() {
    // Un Enum es un tipo que almacena
    // diferentes variantes, almacena diferentes opciones

    // Para hacer uso de un Enum utilizamos la palabra reservada "Enum"
    // y le asignamos un nombre a nuestro Enum, por convención, haremos uso del
    // CamelCase (primera letra mayúscula sin uso de guiones bajos)

    enum Response { // Creamos un bloque con un juego de llaves "{}"
    // Dentro del bloque vamos a colocar todas las
    // posibles opciones para el Enum

        Success, // Se completo correctamente
        Error(u32, String), // Podemos indicar un código de Error a través de una Tupla
        // Esta Tupla poseerá un elemento de tipo u32 y un String
        // (Los Errores son números positivos) -> 403, 404, 500

    }
    // Definimos una variable de tipo Success

    // Para acceder a la opción de nuestro enum
    // seguiremos la siguiente estructura dentro de nuestra variable
    let respuesta = Response::Error(501, String::from("No es posible completar la operacion!!!"));
        // Estructura: ^ Enum::Valor(tupla en caso de haber una)
    // (Cambiamos Enum y valor por el nombre de nuestro
    // Enum y el Valor al que queremos acceder

    // Ahora nuestra variable respuesta es Success

    // Unimos el Enum junto con el Match
    // para evaluar la variable

    match respuesta {
        // En este caso existen únicamente dos opciones Success y Error

        // Imprimimos en consola ambos casos
        Response::Success => println!("La petición se realizo correctamente"),
        Response::Error(403, _) => println!("Forbidden"),
        Response::Error(404, _) => println!("Not Found"),
        Response::Error(500, _) => println!("Internal Server Error"),
        Response::Error(_, mensaje) => println!("{}", mensaje),
    }

}
