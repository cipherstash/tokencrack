use super::Generator;
use itertools::iproduct;
use once_cell::sync::Lazy;

static FREE_EMAIL_DOMAINS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    Vec::from([
        "gmail.com",
        "yahoo.com",
        "hotmail.com",
        "outlook.com",
        "aol.com",
        "icloud.com",
        "protonmail.com",
        "zoho.com",
        "yandex.com",
        "mail.com",
        "gmx.com",
        "tutanota.com",
        "fastmail.com",
        "hushmail.com",
        "runbox.com",
        "disroot.org",
        "mailbox.org",
        "posteo.de",
        "kolabnow.com",
        "mailbox.org",
        "tutanota.com",
        "disroot.org",
        "protonmail.com",
        "kolabnow.com",
    ])
});

static TLDS: Lazy<Vec<&'static str>> =
    Lazy::new(|| Vec::from(["com", "biz", "info", "net", "org"]));

pub struct EmailForName {
    name: String,
}

impl EmailForName {
    pub fn new(name: String) -> Self {
        Self { name }
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
            format!(
                "{}.{}@{}",
                first_name.to_lowercase(),
                last_name.to_lowercase(),
                domain
            ),
            format!(
                "{}{}@{}",
                first_name.to_lowercase(),
                last_name.to_lowercase(),
                domain
            ),
            format!(
                "{}{}@{}",
                first_name.to_lowercase().chars().next().unwrap(),
                last_name.to_lowercase(),
                domain
            ),
            format!("{}@{}", last_name.to_lowercase(), domain),
        ]
    }
}

impl Generator for EmailForName {
    type Item = String;

    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(
            Domain
                .generate()
                .flat_map(move |domain| self.generate_email_formats(&domain)),
        )
    }
}

pub struct Domain;

// TODO: The domain list should only include domains that have MX records
impl Generator for Domain {
    type Item = String;

    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(
            // TODO: This follows the structure used by python faker and is only useful as an exercise
            iproduct!(
                FREE_EMAIL_DOMAINS
                    .iter()
                    .map(|&s| s.to_string())
                    .chain(
                        super::name::LAST_NAMES
                            .iter()
                            .map(|x| x.to_string().to_lowercase())
                    )
                    .chain(
                        iproduct!(
                            super::name::LAST_NAMES
                                .iter()
                                .map(|x| x.to_string().to_lowercase()),
                            super::name::LAST_NAMES
                                .iter()
                                .map(|x| x.to_string().to_lowercase())
                        )
                        .map(|(first, last)| format!("{}-{}", first, last))
                    ),
                TLDS.iter().map(|&s| s.to_string())
            )
            .map(|(domain, tld)| format!("{}.{}", domain, tld)),
        )
    }
}
