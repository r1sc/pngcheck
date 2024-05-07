use clap::Parser;
use pngcheck::parse_file;
use pngcheck::png::{Chunk, Png};
use pngcheck::view::view_image;
use std::error::Error;

mod pretty_assert_printing;

//PNG check
#[derive(Parser)]
#[clap(author, about, version, long_about = None)]
enum Args {
    ///Check a PNG file
    Check {
        ///The PNG file to check
        file: String,
    },
    ///View a PNG file
    View {
        ///The PNG file to view
        file: String,
    },
}

fn print_chunks(chunks: &Vec<Chunk>) {
    for chunk in chunks {
        println!("Chunk type: {:?}", chunk.chunk_type);
        println!("Chunk length: {:?}", chunk.length);
        println!(
            "CRC: {:?}, Valid: {:?}",
            chunk.crc,
            chunk.validate_checksum()
        );
    }
}

fn read_file(file: &str) -> Result<Png, Box<dyn Error>> {
    let file = std::fs::File::open(file).expect("Failed to open file");

    parse_file(file)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args {
        Args::Check { file } => {
            let data = read_file(&file)?;
            println!("IHDR: {:?}", data.ihdr());
            println!("PLTE: {:?}", data.plte());
            println!("tRNS: {:?}", data.trns());
            println!("pHYs: {:?}", data.phys());
            println!("sRGB: {:?}", data.srgb());
            println!("gAMA: {:?}", data.gama());
            print_chunks(&data.chunks);
            println!("Extra bytes: {:?}", data.extra_bytes);
        }
        Args::View { file } => {
            let data = read_file(&file)?;
            match data.ihdr() {
                Some(ihdr) => {
                    view_image(&data.get_scanlines()?, &ihdr);
                }
                None => eprintln!("IHDR chunk not found"),
            }
        }
    };

    Ok(())
}
