use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug,Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32
}

fn main() -> std::io::Result<()> {

    // let person = Person {
    //     name: "John Doe".to_string(),
    //     age: 30
    // };

    // Encodage de la variable sous forme de buffer
    // let encoded = serde_json::to_string(&person)?;
    // let mut file = File::create("persons.json")?;
    // file.write_all(encoded.as_bytes())?;

    // Lire et print des objets Rust
    let mut file = File::open("persons.json")?;
    let mut buffer = String::new();
    
    file.read_to_string(&mut buffer)?;
    let decoded_person: Person = serde_json::from_str(&buffer)?;

    println!("{:?}", decoded_person);
Ok(())
}

    // // Exemple pour un fichier ".bin"
    // // Gérer l'ouverture d'un fichier
    // let mut file = File::open("exemple.bin")?;

    // // Lire le contenu du fichier dans un vecteur de bytes
    // let mut buffer: Vec<u8> = Vec::new();
    // file.read_to_end(&mut buffer)?;

    // // Traiter les données binaires

    // Ok(())