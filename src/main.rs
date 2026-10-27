mod algebra;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        println!();
        println!("==========================================");
        println!("  CFKATG - Generator");
        println!("==========================================");
        println!("  [1] Generate test");
        println!("  [2] Configure control-flow guards");
        println!("  [0] Exit");
        println!("==========================================");

        print!("Select an option: ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        match input.trim() {
            "1" => {
                let expression = algebra::bExp::boolean_expression();
                println!("  {}", expression);
            }
            "2" => println!("WIP"),
            "0" => {
                println!("Exiting");
                break;
            }
            _ => println!("Invalid selection. Try again."),
        }
    }

    Ok(())
}
