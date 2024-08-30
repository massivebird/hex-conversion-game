use std::io::Write;

use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    loop {
        let answer_decimal: u32 = rng.gen_range(0..=255);
        let answer_hex: String = num_to_hex(answer_decimal);

        println!("Enter the decimal equivalent of {answer_hex}.");

        let user_guess = loop {
            print!("> ");

            // Aligns input cursor with input prompt
            std::io::stdout().flush().unwrap();

            let mut raw_input = String::new();
            std::io::stdin().read_line(&mut raw_input).unwrap();

            let Ok(decimal_value) = raw_input.trim_end().parse::<u32>() else {
                println!("Unable to parse number. Try again.");
                continue;
            };

            break decimal_value;
        };

        if user_guess == answer_decimal {
            println!("Correct!\n");
        } else {
            println!("Incorrect! The answer is #{answer_decimal}.\n");
        }
    }
}

fn num_to_hex(n: u32) -> String {
    let digit_converter = |digit: u32| -> char {
        match digit {
            // +48 maps range 0..=9 to ASCII range '0'..='9'
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

#[cfg(test)]
mod tests {
    use super::num_to_hex;

    #[test]
    fn hex_is_correct() {
        assert_eq!(num_to_hex(0), "x00");
        assert_eq!(num_to_hex(5), "x05");
        assert_eq!(num_to_hex(16), "x10");
        assert_eq!(num_to_hex(255), "xFF");
    }
}
