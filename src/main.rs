use clap::{Parser, Subcommand};
use gameboy_dot_rs::rom::header;
use gameboy_dot_rs::rom::parse::Parse;
use std::io::Read;
use std::{error, fs, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut cli: Cli = Cli::parse();

    match cli.commands {
        Commands::Header { mut file } => header_command(&mut file),
    }?;

    Ok(())
}

fn header_command(file: &mut fs::File) -> io::Result<()> {
    let mut rom_bytes = Vec::new();
    file.read_to_end(&mut rom_bytes)?;

    let header = header::Header::parse(&rom_bytes[0x100..]);
    match header {
        Ok(header) => {
            println!("{:?}", header);
        }
        Err(message) => {
            eprintln!("ROM has an invalid header: {}", message);
        }
    }

    Ok(())
}

#[derive(Parser)]
#[clap(author = "Austin Bourgerie", about = "A GameBoy emulator in Rust")]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Parse, validate, and display the header without running")]
    Header {
        #[clap(parse(try_from_str = open_file))]
        file: fs::File,
    },
}

fn open_file(path: &str) -> Result<fs::File, String> {
    fs::File::open(path).map_err(|e| String::from(e.to_string()))
}
