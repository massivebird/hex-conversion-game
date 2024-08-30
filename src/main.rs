use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let number: u32 = rng.gen_range(0..=255);

    let as_hex: String = num_to_hex(number);

    println!("{number} -> {as_hex}");
}

fn num_to_hex(n: u32) -> String {
    let digit_converter = |digit: u32| -> char {
        match digit {
            n @ 0..=9 => char::from_u32(n + 48).unwrap(),
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => panic!("dude"),
        }
    };

    let sixteens = n / 16;
    let ones = n - (16 * sixteens);

    format!("x{}{}", digit_converter(sixteens), digit_converter(ones))
}
