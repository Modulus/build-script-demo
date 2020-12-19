use std::fs;

fn main() {
    let filename = String::from("resources/config.yaml");

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

}
