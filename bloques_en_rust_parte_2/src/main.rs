// Bloques en Rust parte 2
fn main() {
// Los bloques tienen la capacidad de retornar valores

    // Creamos una variable que tendrá el valor de la
    // variable dentro del bloque anidado
    let resultado = { // Definimos el
        // bloque anidado como el valor de la variable "resultado"
        println!("Hola, nos encontramos en un bloque anidado");

        let variable : i32 = 200;

        println!("{}",variable);
        // Llamamos a la variable a secas, sin palabras reservadas ni definiciones
        variable // NO estamos retornando la variable, estamos retornando su valor
        // Ahora la variable retorna el valor

    }; // Debido a que estamos haciendo una asignación
       // debemos finalizar el bloque con punto y coma ";"

    // Imprimimos la nueva variable
    println!("El valor de resultado es: {}", resultado);

    let calificacion : i8 = 10;
    let mut mensaje = String::new();

    if calificacion == 10 {
        mensaje = String::from("Felicitaciones aprobaste la materia.");
        // El método from nos permite crear un String
        //  a partir de una cadena de caracteres
    } else {
        mensaje = String::from("Necesitas estudiar más");
    }

    println!("{}", mensaje);

    // Con los conocimientos ya adquiridos sobre los bloques, podemos refactorizar
    // haciendo mas simple el código que se encuentra arriba

    // Creamos la variable calificación
    let calificacion : i8 = 10;

    // Creamos una variable y le asignamos una condición como valor
    let mensaje = if calificacion == 10 {
        String::from("Felicitaciones aprobaste la materia.")
    } else {
        String::from("Necesitas estudiar más")
    }; // Estos dos bloques están retornando un valor

    println!("{}", mensaje);

}
