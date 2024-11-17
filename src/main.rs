use itertools::Itertools;
use rayon::prelude::*;
use tokencrack::{Cracker, generators, Sha256};
use tokencrack::generators::{UsState, UsPhone, Dob, DobFormat, EnglishName, Email};
use tokencrack::token::{tokenize, tokenize_with_salt};
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;
use tokencrack::generators::Generator;

const CHUNK_SIZE: usize = 100_000;

fn main() -> io::Result<()> {
    let token = tokenize::<Sha256>("CA");
    /*println!("{}", hex::encode(&tokenize::<Sha256>("555-800-1000")[..4]));
    println!("{}", hex::encode(&tokenize::<Sha256>("123-590-7666")[..4]));
    println!("{}", hex::encode(&tokenize::<Sha256>("123-456-7890")[..4]));
    println!("{}", hex::encode(&tokenize::<Sha256>("123-333-0000")[..4]));*/

    let cracker: Cracker<_, Sha256> = Cracker::new(UsState);
    cracker.crack(&token).map(|found| println!("Found: {}", found));

    /*Email::new("Daniel Draper".to_string()).generate().for_each(|line| {
        println!("{}", line);
    });
    //let target = tokenize::<Sha256>("347-554-2635");
    //let target = tokenize::<Sha256>("1981-01-01");
    let target = tokenize::<Sha256>("Daniel Draper");*/

    //let salt = hex::decode("aabb771100").unwrap();
    //let cracker: Cracker<_, Sha256> = Cracker::new(UsPhone::new(generators::PhoneNumberFormat::WithDashes).for_state("NY"));
    //let cracker: Cracker<_, Sha256> = Cracker::new(Dob::new(DobFormat::Standard));
    /*let cracker: Cracker<_, Sha256> = Cracker::new(EnglishName);

    if let Some(found) = cracker.crack(&target) {
        println!("Found: {}", found);
    } else {
        println!("Unable to reverse token");
    }*/

    /*let file = File::open("com_zone_fullf98195caa4")?;
    let reader = io::BufReader::new(file);

    let target = hash("cipherstash.com");

    println!("Target: {:x}", target);

    reader
        .lines()
        .chunks(CHUNK_SIZE)
        .into_iter()
        .for_each(|chunk| {
            // FIXME: This line probably slows things down
            let chunk: Vec<String> = chunk.map(|line| line.unwrap()).collect();
            if let Some(found) = search_chunk(&chunk, &target) {
                println!("Found: {}", found);
                exit(0);
            }
        });*/

    Ok(())
}