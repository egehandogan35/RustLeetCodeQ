use clap::{Arg, ArgAction, Command};
use std::io;
use std::fs;
use ring::digest;

fn main() {
    let matches = Command::new("testing")
        .version("0.1")
        .author("ED")
        .arg(
            Arg::new("fileconv")
                .short('f')
                .long("fileconv")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("delfile")
                .short('d')
                .long("delete")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    if matches.args_present() {
        if let Some(path) = matches.get_one::<String>("fileconv") {
            let content = fs::read_to_string(path).expect("Not a txt file");
            println!("Content of the file is: {}", content);
            println!("Conversion starting... to ");
            let startof: &[u8] = content.as_bytes();
            let digest = digest::digest(&digest::SHA512, startof);
            let result = format!("Hash text: {:?}", digest);
            println!("Enter the path of the file to save the hash"); 
            let mut hash_path = String::new();
            io::stdin().read_line(&mut hash_path).unwrap();
            fs::write(hash_path.trim(), result).expect("Unable to write file");
            println!("File converted successfully"); 

            if matches.get_flag("delfile") {
                fs::remove_file(&path).expect("Failed to delete file");
                println!("File deleted successfully");
            }
        }
    }
}