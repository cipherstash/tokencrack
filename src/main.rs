use clap::{Parser, Subcommand};
use tokencrack::generators::{EmailForName, EnglishName, UsPhone};
use tokencrack::{generators, Cracker, Sha256};

#[derive(Debug, Subcommand)]
enum DataGenerator {
    UsPhone {
        target: String,
        // TODO: Use value_enum https://docs.rs/clap/4.5.21/clap/_derive/_tutorial/chapter_3/index.html#enumerated-values
        #[arg(long)]
        state: Option<String>,
    },
    //UsState,
    //Dob,
    Email {
        #[arg(short, long)]
        name: Option<String>,
        target: String,
    },
    EnglishName {
        target: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    data_generator: DataGenerator,
    #[arg(short = 'f', long, global = true, default_value = "hex")]
    target_format: Option<String>,
}

fn get_target(target: &str, _format: Option<String>) -> Vec<u8> {
    hex::decode(target).unwrap() // TODO: Error handling
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: miette
    let cli = Cli::parse();
    let target_format = cli.target_format;
    let result = match cli.data_generator {
        DataGenerator::UsPhone { state: _, target } => {
            // TODO: format
            let generator = UsPhone::new(generators::PhoneNumberFormat::WithDots);
            // TODO: State
            let cracker: Cracker<_, Sha256> = Cracker::new(generator);
            let target = get_target(&target, target_format);
            cracker.crack(&target)
        }
        DataGenerator::EnglishName { target } => {
            let generator = EnglishName;
            let cracker: Cracker<_, Sha256> = Cracker::new(generator);
            let target = get_target(&target, target_format);
            cracker.crack(&target)
        }
        DataGenerator::Email {
            name: Some(name),
            target,
        } => {
            let generator = EmailForName::new(name);
            let cracker: Cracker<_, Sha256> = Cracker::new(generator);
            let target = get_target(&target, target_format);
            cracker.crack(&target)
        }
        _ => unimplemented!(),
    };

    if let Some(found) = result {
        println!("Found: {}", found);
    } else {
        println!("Unable to reverse token");
    }

    Ok(())
}
