use csv::ReaderBuilder;
fn main() -> Result<(), csv::Error>  {

    // ouvrir le fichier CSV
    let file = std::fs::File::open("data.csv")?;

    // utiliser le reader pour parcourir les enregistrements du CSV
    let mut reader = ReaderBuilder::new().from_reader(file);

    // itérer sur chaque ligne et afficher les valeurs
    for result in reader.records()  {
        let record = result?;
        for field in record.iter() {
            print!("{field}")
        }
        println!("")
    }
  Ok(())
}
