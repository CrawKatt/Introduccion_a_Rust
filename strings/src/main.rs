// Cadenas/Strings
fn main() {
    // Una Cadena/String es una cadena de caracteres

    // En Rust Existen dos formas de representar Cadenas/Strings

    // La primera es haciendo uso de "str"
    // La segunda es apoyándonos de la estructura String

    // La principal diferencia entre str y String es que str es una cadena de
    // caracteres inmutable, una vez nosotros definimos dicha Cadena/String
    // no podremos agregar o quitar caracteres a menos que se utilice mut
    // junto con una variable mutable

    // Por otro lado con String si es posible, String es una Cadena/String
    // mutable, podremos agregar o quitar caracteres, haciendo que
    // esta Cadena/String modifique su longitud, pueda incrementar o decrecer


    // Al tener comportamientos diferentes, estos tipos se almacenan en espacios
    // de memoria diferentes

    // El str se almacena en el Stack
    // El String se almacena en el Heap


    // Ahora definiremos un str

    // Hacemos uso de un &str
    let variable_str = "Tipo str"; // Hacemos uso de comillas dobles ""

    // Ahora definiremos una variable de tipo Cadena/String

    let variable_string = String::from("Tipo String"); // Hacemos uso de la estructura String con el método from dando como argumento nuestra Cadena/String de elementos
    // El método "new" Retorna una Cadena/String vacía "" mientras que el método "from" genera una Cadena/String a partir de una Cadena/String

    // Ahora imprimimos en consola, ambos valores

    println!("El str es: {}",variable_str);
    println!("El String es: {}", variable_string);

    // También podemos crear Cadenas/Strings mutables

    let mut variable_string_mutable = String::from("String mutable");

    // Utilizamos el método push

    variable_string_mutable.push(':'); // El método push recibe como argumento un carácter
    // ese carácter se agregara al final de nuestra Cadena/String

    variable_string_mutable.push(' '); // Utilizamos comillas simples ''

    variable_string_mutable.push('H');

    variable_string_mutable.push('O');

    variable_string_mutable.push('L');

    variable_string_mutable.push('A');

    variable_string_mutable.push(' ');

    // También podemos agregar un str en lugar de agregar
    // carácter por carácter a nuestra Cadena/String utilizando el método push_str

    variable_string_mutable.push_str("String Tipo push str"); // Como argumento vamos a colocar una Cadena/String de caracteres de tipo str

    println!("{}", variable_string_mutable);


    // También podemos convertir una Cadena/String de tipo str a tipo String

    let nuevo_string = "Cadena nueva".to_string(); // Utilizamos el método .to_string() para convertir la Cadena/String de tipo str a tipo String

    println!("{}", nuevo_string);

    // Con las Cadenas/Strings podemos utilizar
    // los operadores igual "==" y diferente "!="

    // Creamos una nueva variable, asignamos la variable anterior y
    // utilizamos el operador igual "=="
    let igual = nuevo_string == "Cadena nueva.".to_string();

    println!("Las Cadenas/Strings son iguales: {}", igual);

    let distinto = nuevo_string != "Cadena nueva.".to_string();

    println!("Las cadenas/Strings son distintas: {}", distinto);

}
