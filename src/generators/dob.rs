use itertools::iproduct;

use super::Generator;

pub enum DobFormat {
    Us,
    Standard,
}

impl DobFormat {
    fn format(&self, month: i32, day: i32, year: i32) -> String {
        match self {
            DobFormat::Us => format!("{:02}/{:02}/{:04}", month, day, year),
            DobFormat::Standard => format!("{:04}-{:02}-{:02}", year, month, day),
        }
    }
}

pub struct Dob {
    format: DobFormat,
}

impl Dob {
    pub fn new(format: DobFormat) -> Self {
        Self { format }
    }
}

impl Generator for Dob {
    type Item = String;

    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(
            iproduct!((1..13), (1..32), (1900..2020))
                .map(|(month, day, year)| self.format.format(month, day, year)),
        )
    }
}
