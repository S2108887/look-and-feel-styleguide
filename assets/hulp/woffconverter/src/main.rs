use woofwoof::compress;
use std::env;

fn main() {
    let argumenten: Vec<String> = env::args().collect();
    if argumenten.len() < 3 {
        println!("Er zijn twee argumenten nodig: invoerbestand en uitvoerbestand");
        return;
    }

    let invoer = &argumenten[1];
    let uitvoer = &argumenten[2];

    println!("Invoer: {invoer}, uitvoer: {uitvoer}");

    let font_inhoud = std::fs::read(invoer)
        .expect("De invoer kon niet gelezen worden.");
    let woff2_data = compress(&font_inhoud, "", 8, true).expect("Compressiefout");

    std::fs::write(uitvoer, woff2_data).expect("Het resultaat kon niet weggeschreven worden.");
}
