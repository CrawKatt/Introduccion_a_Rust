// Otra forma de crear vectores utilizando una Estructura
fn main() {
    // Crear un vector usando la estructura Vec::new();


    // Creamos una variable mutable para nuestro vector

    // El método new va a generar un vector
    // completamente vacío
    let mut vector = Vec::new();
    // Colocamos la estructura seguido de "::" y ejecutamos el método new()

    vector.push(4);
    vector.push(5);

    println!("el valor del vector es: {:?}", vector);

    // También podemos indicar el tipo de dato de nuestro vector

    let mut vector : Vec<i32> = Vec::new();
    vector.push(4);

    // Imprimimos

    println!("el valor del vector es: {:?}", vector);


}
