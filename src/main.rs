use rand::prelude::*;
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Enter the maximum value to be generated.");

    let range_max = prompt_user_for_u32();

    loop {
        let answer_decimal: u32 = rng.gen_range(0..=range_max);
        let answer_hex: String = num_to_hex(answer_decimal);

        println!("Enter the decimal equivalent of {answer_hex}.");

        let user_guess = prompt_user_for_u32();

        if user_guess == answer_decimal {
            println!("Correct!\n");
        } else {
            println!("Incorrect! The answer is #{answer_decimal}.\n");
        }
    }
}

fn num_to_hex(n: u32) -> String {
    let largest_power_of_16 = if n == 0 { 0 } else { n.ilog(16) };

    let mut hex_chars: Vec<char> = Vec::new();

    // This copy of n will be decremented until it reaches zero.
    // This represents the remaining values that have to be converted to hex.
    let mut remainder = n;

    for power in (0..=largest_power_of_16).rev() {
        // Compute next digit value 0..=15.
        // "How many times does 16^{power} fit into the remainder?"
        let digit_value = remainder / 16u32.pow(power);

        hex_chars.push(match digit_value {
            // +48 maps range 0..=9 to ASCII range '0'..='9'
            n @ 0..=9 => char::from_u32(n + 48).unwrap(),
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => unreachable!(),
        });

        // compute working remainder of n to prepare for next iteration
        remainder -= digit_value * 16u32.pow(power);
    }

    format!("x{}", hex_chars.iter().collect::<String>())
}

/// Generates a prompt for user input. Retries until the input is successfully
/// parsed into an unsigned integer.
fn prompt_user_for_u32() -> u32 {
    loop {
        print!("> ");

        // Positions input cursor after input prompt
        std::io::stdout().flush().unwrap();

        let mut raw_input = String::new();
        std::io::stdin().read_line(&mut raw_input).unwrap();

        let Ok(decimal_value) = raw_input.trim_end().parse::<u32>() else {
            println!("Unable to parse unsigned integer. Try again.");
            continue;
        };

        break decimal_value;
    }
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
