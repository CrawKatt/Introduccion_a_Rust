// Métodos en Rust

// Para que podamos implementar métodos a nuestras Estructuras/Struct haremos uso
// de la palabra reservada "impl"

// Posteriormente vamos a definir a que Estructura/Struct
// le agregaremos los siguientes métodos en nuestro caso,
// la Estructura/Struct User

// Para este ejemplo, vamos a indicar que nuestra Estructura/Struct puede ser
// examinada, esto con fines de Debug
#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

// Ejemplo 1
// Nos apoyamos de un nuevo bloque a través de un juego de llaves "{}"
impl User {
    // Dentro del bloque vamos a definir los diferentes métodos
    // que tendrá nuestra Estructura/Struct a través de una función

    // self hace referencia al objeto en si mismo
    // Aplicamos una referencia de tipo mutable con &mut
    fn saluda(&mut self) { // Todos los métodos deben recibir
        // el parámetro "self dentro de los paréntesis

        // Como self es el objeto, podemos acceder a sus diferentes atributos
        // y a sus diferentes métodos
        println!("Hola, soy el usuario {}", self.username);
    }
// Ejemplo 2
    fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }
    // Un método es una función que le pertenece a una clase,
    // en este caso, a una Estructura/Struct
    // todos nuestros métodos van a recibir
    // como primer parámetro obligatoriamente "self" el cual
    // es una referencia al objeto mismo y colocamos mut para que
    // podamos modificar los atributos si asi lo deseamos
}

fn main() {
    let mut usuario1 = User {
        username: String::from("usuario1"),
        password: String::from("Password"),
    };
    // Podemos colocar nuestro objeto. y ejecutar su método saluda();
    usuario1.saluda();
    usuario1.change_password("Nuevo Password".to_string());

    println!("El usuario es: {:?}", usuario1); // Para obtener la información de
    // un objeto, de una Estructura/Struct hacemos uso de {:?}
}
