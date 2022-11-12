// En Rust existe el concepto de "shadowing"
// que dicta que podemos usar el nombre de una variable la
// cantidad de veces que necesitemos
fn main() {

    let valor : i32 = 10;

    println!("El valor de la variable es: {}", valor);
    // Si compilamos y ejecutamos el valor que dará en consola será 10
    // pero podemos hacer que cambie de valor sin hacer que la variable sea mutable

    // Después de la impresión, podemos volver a crear la variable usando
    // el mismo nombre pero con otro valor
    let valor : i32 = 15;

    // Imprimimos la nueva variable
    println!("El nuevo valor de la variable es: {}", valor); // Esto dara como resultado, dos impresiones en consola
    // En este caso, la nueva variable tiene el mismo nombre que la variable anterior
    // pero su valor es diferente, lo que Rust hará es liberar el nombre destruyendo
    // la variable anterior para reemplazarlo con el valor de la nueva variable

    // ATENCIÓN, aunque estemos creando nuevas variables, estas siguen siendo inmutables
}
