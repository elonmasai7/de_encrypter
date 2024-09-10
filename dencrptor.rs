use std::process::Command;
use std::io::{self, Write};

fn main() {
    print!("Enter password for the RAR file: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");
    
    let password = password.trim(); 

    
    let rar_file = "encrypted_file.rar";
    let output_dir = "./extracted";

   
    let status = Command::new("unrar")
        .arg("x")            // x: Extract files with full path
        .arg("-p")           // -p: Provide password
        .arg(password)       // Password entered by user
        .arg(rar_file)       // The RAR file to decrypt
        .arg(output_dir)     // Output directory for extraction
        .status()
        .expect("Failed to execute unrar");

    // Check if the extraction was successful
    if status.success() {
        println!("Successfully decrypted and extracted the RAR file!");
    } else {
        eprintln!("Failed to decrypt the RAR file. Please check the password and file.");
    }
}
