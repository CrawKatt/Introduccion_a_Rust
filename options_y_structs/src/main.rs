// Options y Structs

// Ahora vamos a ver un ejemplo de como podemos implementar el Enum Option
// en nuestras Estructuras/Struct
#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    edad: Option<u32> // Este atributo es completamente opcional, por lo
    // que utilizaremos Option seguido de su tipo de dato, en este caso, u32
}

fn main() {

    let usuario1 = User{
        username: String::from("Eduardo"),
        password: String::from("password"),
        email: String::from("eduardo@correo.com"),
        edad: None //Some(26)
    };

    println!("El usuario es: {:?}", usuario1);

    //let edad = usuario1.edad.unwrap(); // Match

    match usuario1.edad {
        Some(edad) => println!("Su edad es: {}", edad),
        None => { },
    }


}
