// Vectores en Rust
fn main() {
    // Al igual que un Arreglo/Array los Vectores
    // nos permiten almacenar diferentes valores dentro de una colección

    // La principal diferencia entre un Arreglo/Array y un Vector
    // es que el Vector si puede modificar su longitud,
    // puede incrementarse o puede decrecer

    // Sin embargo al igual que los Arreglos/Arrays
    // los vectores solo pueden almacenar un mismo tipo de dato

    // Definir nuestro primer Vector

    // Comenzamos Definiendo una variable y le asignamos "vec!" como valor
    // seguido de unos Corchetes "[]" y finalizamos con un punto y coma ";"
                   // 0  1  2
    let mut vector = vec![1, 2, 3]; // Dentro de los Corchetes "[]" separados por
    // una coma "," vamos a colocar los elementos que
    // queremos que nuestro vector almacene

    // Al igual que los Arreglos/Arrays
    // los vectores se rigen por la regla de los indices

    // Imprimimos en consola

    println!("El valor del vector es: {:?}", vector); // Al igual que en los
    // Arreglos/Arrays es necesario utilizar ":?" dentro de nuestro juego de llaves
    // para poder imprimir nuestro vector dentro de nuestra función println!()

    // Añadiendo nuevos elementos al vector
    // Para ellos nos apoyaremos del método push

    // Colocamos el nombre de nuestro vector seguido de ".push"
    vector.push(4); // El método push recibe como argumento
    // el elemento el cual queremos agregar a nuestro vector
    vector.push(5);
    // Al igual que con los Arreglos/Arrays y las tuplas
    // los vectores requieren de variables mutables
    // por lo que haremos mutables nuestras variables
    // a través de let mut

    // Este elemento se agregara al final de nuestro índice

    // Y volvemos a imprimir
    println!("El nuevo valor del vector es: {:?}", vector);


    // Otra forma en la que podemos añadir elementos al vector es utilizando
    // el método insert

    // Colocamos el nombre de nuestro vector seguido de ".insert"
    vector.insert(0, -1); // El método insert recibe como argumento dos valores
    // el primer valor corresponde a la posición de la cual queramos agregar
    // un nuevo elemento. Por ejemplo: 0.

    // Y el segundo valor será el valor el cual queremos agregar al vector.
    // Por ejemplo: -1

    vector.insert(1,0);

    // Volvemos a imprimir
    println!("El nuevo valor del vector es: {:?}", vector); // el -1 se agrega
    // en el índice 0

    // Eliminar elementos del vector

    // Si queremos eliminar elementos de nuestro vector
    // podemos hacerlo mediante el método remove
    vector.remove(vector.len() -1); // El método remove recibe como argumento
    // El índice del elemento el cual queremos eliminar
    // Por ejemplo: el ultimo elemento

    println!("El nuevo valor del vector es: {:?}", vector);

    // Podemos obtener los elementos del vector mediante su índice
    // Por ejemplo:
    let primer_elemento = vector[0]; // Utilizamos Corchetes "[]"
    // e indicamos el índice

    let ultimo_elemento = vector[ vector.len() -1 ]; // Obtenemos la longitud
    // actual del vector -1

    // Y procedemos a imprimir

    println!("El primer elemento del vector es: {:?}", primer_elemento);
    println!("El ultimo elemento del vector es: {:?}", ultimo_elemento);

    // De igual forma podemos modificar el valor de un indice de un vector

    vector[0] = -10; // Hacemos la modificación antes de obtener los elementos

    let primer_elemento = vector[0];

    let ultimo_elemento = vector[ vector.len() -1 ];

    println!("El primer elemento del vector es: {:?}", primer_elemento);
    println!("El ultimo elemento del vector es: {:?}", ultimo_elemento);

    // También podemos usar los vectores como una pila
    // ya que los vectores contienen el método pop
    // este método se encarga de retornar y eliminar
    // el ultimo elemento del vector

    let ultimo_elemento = vector.pop().unwrap(); // El método pop retorna un objeto
    // una estructura de tipo option
    // Lo que vamos a hacer es aplicarle el método unwrap
    // con el método unwrap obtendremos el ultimo elemento

    // Y volvemos a imprimir
    println!("el valor es: {:?}", primer_elemento);
    println!("el valor es: {:?}", ultimo_elemento);
    // Podemos observar que el ultimo elemento era 4 pero que ahora
    // se ha eliminado del vector gracias al método pop

}
