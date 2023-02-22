use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Read the reminder from the user
    println!("Enter your reminder:");
    let mut reminder = String::new();
    std::io::stdin().read_line(&mut reminder)?;

    // Create a new file in the current directory
    let mut file = File::create("reminder.txt")?;

    // Write the reminder to the file
    file.write_all(reminder.as_bytes())?;

    Ok(())
}

