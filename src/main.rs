use quaternion::vector::Vector;
use std::io::{stdin, stdout, Write};

fn main() {
    print!("Provide vector in format x, y, z: ");
    let mut vector = read_vector();

    loop {
        print!("Provide rotation_axis in format x, y, z: ");
        let rotation_axis = read_vector();

        print!("Provide rotation angle: ");
        let rotation_angle = read_f64();

        vector.rotate(&rotation_axis, rotation_angle);
        println!("Resultant vector: {:?}", vector);

        println!("Want to rotate it further? Y/N: ");

        match read_yorn().as_str() {
            "N" | "n" => {
                break;
            }
            "Y" | "y" => {
                continue;
            }
            &_ => {
                break;
            }
        }
    }
}

fn read_yorn() -> String {
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    input.trim().to_string()
}

fn read_f64() -> f64 {
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    let rotation_angle = input.trim().parse::<f64>().unwrap().to_radians();
    rotation_angle
}

fn read_vector() -> Vector {
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    let vector_input: Vec<&str> = input.trim().split(",").collect();
    let vector_input: Vec<f64> = vector_input
        .into_iter()
        .map(|s| s.trim())
        .map(|s| s.parse::<f64>().unwrap())
        .collect();

    Vector {
        x: vector_input[0],
        y: vector_input[1],
        z: vector_input[2],
    }
}
