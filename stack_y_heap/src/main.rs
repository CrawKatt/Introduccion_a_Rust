// Stack y Heap

// En el Stack se encuentran alojadas todas aquellas variables las cuales de
// antemano ya conozcamos su longitud, es decir, su tamaño
fn main() {
    // En este caso estamos creando una variable llamada x
    // como la variable necesita ser almacenada en alguna parte y al ser de tipo
    // entero con un tamaño ya definido (i32 {tamaño 32 bytes} )
    // se procede a almacenar la variable en el Stack.

    let mut x : i32 = 10;

}

// El Stack tiene la ventaja de ser muy rápido, ya que al conocer de antemano
// el tamaño de todas las variables, se asignará el espacio en memoria
// más optimo para esta

// El Stack lo veremos como una pila, una pila donde se almacenan los valores
// conforme estos se van agregando

// Esto hace al Stack muy rápido en cuanto a lectura y escritura de valores


// En este caso definimos 3 variables

// Dos en la función foo
fn foo() {
    // Si seguimos el orden de ejecución un orden secuencial, un orden de arriba
    // hacia abajo, la primera variable en almacenarse en el Stack sera A para
    // posteriormente agregarse B y C
    let b = 10; // Se agrega al Stack
    let c = 20; // Se agrega al Stack
}

// Uno en la función main
fn main_2() {
    let a = 5; // Se agrega al Stack

    foo();
}
// Cuando la función foo finaliza, las variables asignadas dentro de su Scope
// serán destruidas

// tanto B y C al encontrarse en la parte superior de la pila, serán
// Fácilmente removidas, evitando asi, recalcular toda la pila

// Esto sucede cuando las variables se desconoce su tamaño, sin embargo
// cuando esto no es así, las variables serán almacenadas en el Heap


// En el Heap encontraremos todas aquellas variables las cuales su tamaño
// pueda variar en tiempo de ejecución, ya sea que incremente o decrezca como
// por ejemplo, los vectores y los Strings,
// estructuras que como sabemos son mutables
// podemos agregar o quitar elementos de ellos

// A diferencia del Stack, el Heap es mucho
// más lento en cuanto a lectura y escritura de variables
// esto debido a su naturaleza, no es lo mismo trabajar con datos con un tamaño
// definido a otros que puedan variar en cualquier momento

// Podemos decir que Heap es mucho menos organizado que el Stack