// Result

// Result puede almacenar dos valores: <T y E>.

// Estos valores son mutuamente excluyentes,
// no pueden existir los dos al mismo tiempo.

// Ambos valores pueden ser de cualquier tipo, ya sea un entero,
// un flotante (decimal), un booleano (true o false), un carácter (char)
// una Cadena/String, un vector, un Arreglo/Array, una Tupla, etc.

// Pueden almacenar cualquier Tipo de Dato

// T hace referencia al valor el cual queremos establecer
// E hace referencia al error mismo


// El Enum Result posee dos opciones:

// Ok(T) Se usa cuando no exista ningún error y
// podamos establecer algún valor, en este caso T

// Err(E) Se usa cuando exista algún error, en este caso almacenará E


// Ejemplo:

/*

enum Result<T, E> {
    Ok(T),
    Err(E)
}

 */

fn division(numero1: i32, numero2: i32) -> Result<i32, String> {
    if numero2 == 0 {
        return Err(String::from("No es posible dividir por cero!!!"));
    }

    Ok(numero1 / numero2)
}

fn main() {

    let resultado = division(10, 0); // Result

    match resultado {
        Ok(valor) => println!("El resultado es: {}", valor),
        Err(error) => println!("El error es: {}", error)
    }



}
