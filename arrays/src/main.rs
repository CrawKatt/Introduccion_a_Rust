// Arreglos/Arrays en Rust
// un Arreglo/Array nos permite almacenar múltiples valores
// dentro de una colección

fn main() {
    // En Rust, un Arreglo/Array debe almacenar el mismo tipo de dato
    // Solo enteros, solo Strings, solo booleanos, etc

    // Ejemplos
    // Creamos una variable

    // En Rust, los Arreglos/Arrays se rigen por la regla de los indices
    // a cada elemento le corresponde un indice y los indices comienzan en cero
                           // 0  1  2  3  4
    let numeros =    [1, 2, 3, 4, 5]; // Tamaño/Size -> 5
    // Mediante corchetes [] vamos a definir un nuevo Arreglo/Array

    // Dentro de los [] separados de ; colocamos los elementos que
    // queramos que conformen el Arreglo/Array

    // Imprimimos
    println!("El valor del arreglo es: {:?}", numeros);
    // En el caso de los Arreglos/Arrays estos necesitan incluir un ":?"
    // dentro de el juego de llaves para poder imprimirse mediante la
    // función println!() debido a su formato


    // También podemos ser más específicos
    // con el tipo y el tamaño del Arreglo/Array
                           // 0  1  2  3  4  5
    let mut numeros : [i32; 6] = [1, 2, 3, 4, 5, 6];
    // Utilizamos : seguido de [i32] para especificar que solo
    // almacenaremos números enteros dentro del Arreglo/Array
    // e indicamos su longitud
    // Utilizamos ; luego de i32 para especificar el tamaño máximo de elementos

    println!("El valor del arreglo es: {:?}", numeros);

    // Otra forma de definir Arreglos/Arrays es mediante valores por defecto/default

    // El primer hace referencia al elemento que queremos almacenar
    // dentro del Arreglo/Array ya sea un numero, un flotante/decimal un String
    // un Carácter/char, un booleano, etc
    let valores = [5.5; 10]; // Arreglo/Array de máximo 10 elementos
    // el segundo valor hace referencia a el
    // tamaño máximo de nuestro Arreglo/Array

    println!("el valor del arreglo es: {:?}", valores);


    // Obtener elementos de los Arreglos/Arrays

    // Definimos una variable y asignamos un indice de
    // nuestro Arreglo/Array como valor utilizando la variable anterior
    let primer_elemento = numeros[0];
    let ultimo_elemento = numeros[numeros.len() -1 ]; // El método len nos
    // permite conocer la cantidad de elementos que posee nuestro arreglo
    // en este caso numeros posee un tamaño de 6 elementos y 5 indices
    // ya que los indices comienzan en 0 por lo tanto, restamos uno

    println!("El primer elemento es: {:?}", primer_elemento);
    println!("El ultimo elemento es: {:?}", ultimo_elemento);


    // Modificar elementos de un Arreglo/Array

    // utilizamos la variable y la posición del Arreglo/Array
    // que queramos modificar
    numeros[2] = 30; // Reemplazamos el 3 por 30 e imprimimos nuevamente

    println!("El valor del arreglo es: {:?}", numeros);
    // En este caso, estamos modificando el valor del indice 2
    // Aviso: Como estamos modificando un valor dentro de una variable,
    // dicha variable debe ser mutable utilizando let mut concepto de Rust
    // aprendido anteriormente
}
