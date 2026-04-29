use std::io::{self, Write};

use anyhow::{Context, Error, bail};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

pub fn get_input_num() -> Result<usize, Error> {
    let input_str = get_input("Enter number (0 is exit): ");
    let num: usize = input_str.parse().with_context(|| "invalid input")?;
    if num == 0 {
        bail!("exit");
    }

    Ok(num)
}
