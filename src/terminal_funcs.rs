// Clears the terminal screen and moves the cursor to the top-left corner
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

// Prompts the user for input, displaying a given prompt message.
// If the user provides valid input, it is parsed and returned as a f64.
// If the input is invalid or empty, a default value is returned.
pub fn get_input(prompt: &str, default: f64) -> f64 {
    use std::io::{self, Write};
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(default)
}
