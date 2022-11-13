// Bloques en Rust
fn main() { // En una función main podemos encontrar un bloque

    // Un bloque en términos simples, es una colección de sentencias
    // las cuales se encuentran agrupadas dentro de un juego de llaves "{}"

    // Dentro de un bloque seremos capaces de definir
    // la cantidad de variables que necesitemos

    // Podemos utilizar ciclos, condiciones, estructuras, llamar a otras funciones,etc

    // Las variables tienen un ciclo de vida que esta
    // estrechamente relacionado con los bloques

    // Una variable se crea, se utiliza, se modifica y se destruye dentro de un bloque

    // En Rust, un bloque es la colección de
    // sentencias agrupadas por un juego de llaves

    // Esta variable solo puede utilizarse dentro del bloque principal
    // y dentro de los bloques hijos. Es decir, los bloques anidados
    let mensaje = "hola, soy una variable en el bloque main.";
    // En este bloque estamos creando, utilizando y destruyendo una variable

    // Una vez que un bloque es finalizado, todas las variables que se hayan
    // creado dentro de el, se destruyen para liberar espacio en memoria
    println!("{}", mensaje);


    // En Rust, podemos crear la cantidad de bloques que necesitemos
    // no limitándonos a utilizar ciclos, funciones o condiciones

    // Podemos crear bloques solamente con un juego de llaves "{}"
    // y dentro de ese bloque, podemos colocar la cantidad de
    // sentencias que queramos utilizar
    {
        // En este bloque podemos utilizar la variable de la función principal
        println!("{}", mensaje);

        // Pero si definimos una variable dentro de este bloque anidado
        // esta variable solo puede utilizarse dentro del bloque donde fue creada
        // y en los bloques anidados, NO en los bloques superiores
        let mensaje_dos = "Hola, soy una variable en un bloque anidado";
        // Al nosotros definir una variable dentro de un bloque, estamos limitando
        // su alcance. Es decir, donde podemos utilizar la variable

        println!("Hola desde un segundo bloque!");

        // Cuando la variable mensaje_dos llega a este punto,
        // mensaje_dos ya no existe
    }
}
