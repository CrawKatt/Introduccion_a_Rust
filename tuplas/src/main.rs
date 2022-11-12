// Tuplas en Rust
fn main() {
    // Al igual que con los Arreglos/Arrays las tuplas
    // nos permiten almacenar diferentes elementos dentro de una colección

    // Definimos nuestra primera tupla

    // En las tuplas ya no hacemos uso de Corchetes "[]"
                            // 0     1     2
    let tupla = (10, false, 5.5); // Hacemos uso de Paréntesis "()"
    // La principal diferencia entre una tupla y un Arreglo/Array
    // es que las tuplas pueden almacenar diferentes tipos de datos,
    // cosa que con los Arreglos/Arrays no es posible

    // Al igual que los Arreglos/Arrays las tuplas
    // se rigen por indices comenzando por cero

    // Imprimimos la tupla
    println!("El valor de la tupla es: {:?}", tupla);
    // Al igual que en los Arreglos/Arrays las tuplas requieren
    // el uso de ":?" dentro del juego de llaves "{}" para poder imprimirse
    // dentro de la función println!()


    // Otra forma de definir una tupla

    // Definimos los tipos de datos que contendrá la tupla
    let mut tupla : (i32, bool, f64, i32) = (10, false, 5.5, 99);

    // Y volvemos a imprimir
    println!("El valor de la tupla es: {:?}", tupla);


    // También podemos obtener elementos de una tupla

    // Para obtener elementos de una tupla no usaremos ni Corchetes "[]"
    // ni usaremos Paréntesis "()"
    let primer_elemento = tupla.0; // Utilizamos el nombre de la tupla
    // seguido de un punto "." seguido de
    // el indice de la tupla a la que queramos acceder. Por ejemplo: 0

    let ultimo_elemento = tupla.3;

    println!("El primer elemento es: {:?}", primer_elemento);
    println!("El ultimo elemento es: {:?}", ultimo_elemento);


    // Ahora modificaremos los elementos de la tupla

    // Al igual que los Arreglos/Arrays y las variables,
    // las tuplas en Rust por defecto son inmutables,
    // por lo que necesitaremos volverlas mutables a traves de let mut
    tupla.1 = true; // Utilizamos el nombre de nuestra tupla
    // seguido de un punto "." seguido del indice a modificar
    // y modificamos su valor

    // E imprimimos nuevamente
    println!("El nuevo valor de la tupla es {:?}", tupla);
    // La principal diferencia entre un Arreglo/Array y una tupla
    // es que la tupla si permite almacenar diferentes tipos de datos
    // cosa que los Arreglos/Arrays no permiten.

    // En este caso estamos almacenando un numero entero (i32),
    // un valor booleano (bool),
    // un valor flotante/decimal (f64) y
    // otro valor entero (i32)

}
