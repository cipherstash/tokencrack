use super::Generator;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Email {
    name: String,
}

impl Email {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    fn generate_email_formats(&self, domain: &str) -> Vec<String> {
        let parts: Vec<&str> = self.name.split_whitespace().collect();
        if parts.len() < 2 {
            return vec![];
        }
    
        let first_name = parts[0];
        let last_name = parts[1];
    
        vec![
            format!("{}@{}", first_name.to_lowercase(), domain),
            format!("{}.{}@{}", first_name.to_lowercase(), last_name.to_lowercase(), domain),
            format!("{}{}@{}", first_name.to_lowercase(), last_name.to_lowercase(), domain),
            format!("{}{}@{}", first_name.to_lowercase().chars().next().unwrap(), last_name.to_lowercase(), domain),
            format!("{}@{}", last_name.to_lowercase(), domain),
        ]
    }
}

impl Generator for Email {
    type Item = String;

    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(Domain.generate().flat_map(move |domain| {
            self.generate_email_formats(&domain)
        }))
    }
}

pub struct Domain;

// TODO: The domain list should only include domains that have MX records
impl Generator for Domain {
    type Item = String;
    
    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let file = File::open("com_zone_fullf98195caa4").expect("domain name file not found");
        let reader = io::BufReader::new(file);

        Box::new(reader
            .lines()
            .map(|line| line.unwrap())
            .into_iter())
    }
}

