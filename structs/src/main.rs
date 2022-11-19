// Estructuras/Struct

// Para que podamos definir nuestras propias Estructuras/Struct Haremos uso
// de la palabra reservada "Struct" y posteriormente colocamos el nombre de
// nuestra Estructura/Struct.
// Por convicción haremos uso del CamelCase

// Ejemplo 1
struct User {
    // Hacemos uso de un Bloque a través de un juego de llaves "{}"
    username: String, // Dentro del bloque vamos a definir todos los
    password: String  // atributos que poseerá nuestra Estructura/Struct
    // Atributo + Tipo de Dato
}

// Ejemplo 2
struct UserDos {
    usuario:String,
    contrasena:String // No se permite el uso de caracteres especiales como"ñ"
}

// Ejemplo 3
struct UserTres {
    userr:String,
    pass:String // No se permite el uso de caracteres especiales como"ñ"
}

// También podemos recibir Estructuras/Struct como parámetros // Descomenta esta función para probar
// fn create_user(username : String, password: String) -> User {
    //User { username, password }
//}

fn main() {
    // Definimos una nueva variable y le asignamos
    // nuestra Estructura/Struct como valor
    let usuario = User { // Utilizamos un juego de llaves "{}" y posteriormente
                         // los valores para sus atributos
        username: String::from("Usuario"), // Utilizamos el método "from"
        password: String::from("password123"),
    }; // Cuando hacemos uso de asignación "="
       // es obligatorio utilizar punto y coma ";"

    // Recibiendo Estructura/Struct  como parámetro de la función create_user
    // let usuario = create_user(username, password); // Descomenta esta linea para probar

    // Accedemos al atributo que queramos utilizar a través de .atributo
    println!("El username es: {}", usuario.username);
    println!("El password es: {}", usuario.password);

    // También podemos definir variables y utilizarlas
    // en nuestra Estructura/Struct

    let usuario= String::from("Usuario");
    let contrasena= String::from("contraseña");

    let user = UserDos {
        usuario, contrasena // El compilador intuirá que a la
        // variable llamada "usuario" su valor
        // le corresponde al atributo de "usuario" al igual que con contrasena
    };
    println!("User dos: {}", user.usuario);
    println!("Contraseña dos: {}", user.contrasena);

    // También podemos definir nuestra variable usuario con una sola linea
    let userr = String::from("Usuario 3");
    let pass = String::from("Contraseña");

    let userr = UserTres { userr, pass };

    println!("Usuario tres: {}", userr.userr);
    println!("Contraseña tres: {}", userr.pass);

}
