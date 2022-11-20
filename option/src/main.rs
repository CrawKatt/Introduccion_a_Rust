// Option

// El Enum Option nos permitirá trabajar con cualquier Tipo de Dato
// ya sea un entero, un flotante (Decimal), un booleano (true o false),
// un carácter (char), una Cadena/String, un Vector, un Arreglo/Array, etc

/*

enum Option<T> { // Hacemos uso de <> con una T dentro
    // El Enum posee dos opciones

    // Dentro de la Tupla almacenaremos el valor que deseemos (De cualquier tipo)
    Some(T), -> El valor
    // Que es una Tupla la cual almacena cualquier Tipo de Dato

    None
    // Esta opción nos sirve para representar la ausencia de algún valor
}

 */

fn obtener_valor(bandera: bool) -> Option<String> { // Esta función va a retornar un
                                       // Enum de Tipo Option, se utiliza la o con
                                       // mayúsculas y <> y dentro del diamante
                                       // vamos a definir el Tipo de Dato a usar
    if bandera {
        Some(String::from("Soy un mensaje para la tupla some!"))
    } else {
        None
    }

}

fn main() {
    // Muchos lenguajes de programación utilizan el "Null" o el "Undefined"
    // para representar la ausencia de algún valor y excepciones para
    // manejar los errores. Sin embargo en Rust, no existe nada de ello
    // esto especialmente para prever ciertos errores como lo puede ser, el
    // "Null Pointer Exception"

    // En lugar de ello, Rust implementa dos tipos especiales de Enums
    // Option y Result
    // Con estos Enums seremos capaces de trabajar con la ausencia de valores
    // y con los errores

    // El Enum Option nos permitirá conocer si existe o no algún valor
    // El Enum Result nos permitirá trabajar con errores. El Enum Result
    // lo podemos complementar junto con la función macro "panic!"
    // para finalizar el programa en caso que exista algún error

    let resultado = obtener_valor(true); // Resultado es un objeto
                                                                // de Tipo Option

    // Una muy buena forma de trabajar con este tipo de objetos
    // es a través de match

    //match resultado { // Dentro de resultado las posibles opciones Some y None
        //Some(valor) => println!("El valor es:{}", valor),
        //None => println!("No existe valor alguno")

    //}

    // Otra forma en la cual podemos obtener el valor de un objeto de tipo Option
    // es a través de métodos, siendo el más utilizado, unwrap()
    // unwrap() intenta obtener lo que la Tupla Some almacena

     let valor = resultado.unwrap(); // Si no hay valor, se ejecutará un panic!

    // Existe otro método llamado unwrap_or el cual nos permite definir
    // un valor por default en caso que la tupla Some no posea ninguno
    // let valor = resultado.unwrap_or("La tupla no almacena valor alguno".to_string());
    // Si no hay valor, se ejecutara un parámetro definido dentro de los paréntesis

    // Existe otro método llamado expect que al igual que
    // unwrap y unwrap_or, intentará obtener lo que la tupla Some almacene

    // En caso la Tupla no posea valor alguno y se ejecute el panic,
    // entonces podemos pasar como argumento, una Cadena de caracteres
    // la cual describa mas en detalle cual fue el error
    // let valor = resultado.expect("Se esperaba un String");

    println!("El valor es: {}", valor);
}
