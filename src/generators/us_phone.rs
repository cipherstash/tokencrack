use itertools::iproduct;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::Generator;

static GLOBAL_DATA: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    HashMap::from([
        ("AL", vec!["205", "251", "256", "334", "659", "938"]),
        ("AK", vec!["907"]),
        ("AZ", vec!["480", "520", "602", "623", "928"]),
        ("AR", vec!["479", "501", "870"]),
        (
            "CA",
            vec![
                "209", "213", "279", "310", "323", "341", "350", "408", "415", "424", "442", "510",
                "530", "559", "562", "619", "626", "628", "650", "657", "661", "669", "707", "714",
                "747", "760", "805", "818", "820", "831", "840", "858", "909", "916", "925", "949",
                "951",
            ],
        ),
        ("CO", vec!["303", "719", "720", "970", "983"]),
        ("CT", vec!["203", "475", "860", "959"]),
        ("DE", vec!["302"]),
        ("DC", vec!["202"]),
        (
            "FL",
            vec![
                "239", "305", "321", "352", "386", "407", "561", "689", "727", "754", "772", "786",
                "813", "850", "863", "904", "941", "954",
            ],
        ),
        (
            "GA",
            vec![
                "229", "404", "470", "478", "678", "706", "762", "770", "912",
            ],
        ),
        ("HI", vec!["808"]),
        ("ID", vec!["208"]),
        (
            "IL",
            vec![
                "217", "224", "309", "312", "331", "447", "464", "618", "630", "708", "730", "773",
                "779", "815", "847", "872",
            ],
        ),
        (
            "IN",
            vec!["219", "260", "317", "463", "574", "765", "812", "930"],
        ),
        ("IA", vec!["319", "515", "563", "641", "712"]),
        ("KS", vec!["316", "620", "785", "913"]),
        ("KY", vec!["270", "364", "502", "606", "859"]),
        ("LA", vec!["225", "318", "337", "504", "985"]),
        ("ME", vec!["207"]),
        ("MD", vec!["227", "240", "301", "410", "443", "667"]),
        (
            "MA",
            vec![
                "339", "351", "413", "508", "617", "774", "781", "857", "978",
            ],
        ),
        (
            "MI",
            vec![
                "231", "248", "269", "313", "517", "586", "616", "679", "734", "810", "906", "947",
                "989",
            ],
        ),
        ("MN", vec!["218", "320", "507", "612", "651", "763", "952"]),
        ("MS", vec!["228", "601", "662", "769"]),
        (
            "MO",
            vec!["314", "417", "557", "573", "636", "660", "816", "975"],
        ),
        ("MT", vec!["406"]),
        ("NE", vec!["308", "402", "531"]),
        ("NV", vec!["702", "725", "775"]),
        ("NH", vec!["603"]),
        (
            "NJ",
            vec![
                "201", "551", "609", "640", "732", "848", "856", "862", "908", "973",
            ],
        ),
        ("NM", vec!["505", "575"]),
        (
            "NY",
            vec![
                "212", "315", "332", "347", "516", "518", "585", "607", "631", "646", "680", "716",
                "718", "838", "845", "914", "917", "929", "934",
            ],
        ),
        (
            "NC",
            vec![
                "252", "336", "704", "743", "828", "910", "919", "980", "984",
            ],
        ),
        ("ND", vec!["701"]),
        (
            "OH",
            vec![
                "216", "220", "234", "283", "330", "380", "419", "440", "513", "567", "614", "740",
                "937",
            ],
        ),
        ("OK", vec!["405", "539", "580", "918"]),
        ("OR", vec!["458", "503", "541", "971"]),
        (
            "PA",
            vec![
                "215", "223", "267", "272", "412", "445", "484", "570", "582", "610", "717", "724",
                "814", "835", "878",
            ],
        ),
        ("RI", vec!["401"]),
        ("SC", vec!["803", "839", "843", "854", "864"]),
        ("SD", vec!["605"]),
        ("TN", vec!["423", "615", "629", "731"]),
    ])
});

pub struct UsPhone {
    state: Option<&'static str>,
    pub format: PhoneNumberFormat,
}

#[derive(Clone, Copy, Debug)]
pub enum PhoneNumberFormat {
    Standard,
    WithDashes,
    WithDots,
    WithCountryCode,
    International,
}

impl PhoneNumberFormat {
    pub fn format_number(&self, area_code: String, exchange: i32, number: i32) -> String {
        match self {
            PhoneNumberFormat::Standard => format!("({}) {:03} {:04}", area_code, exchange, number),
            PhoneNumberFormat::WithDashes => format!("{}-{:03}-{:04}", area_code, exchange, number),
            PhoneNumberFormat::WithDots => format!("{}.{:03}.{:04}", area_code, exchange, number),
            PhoneNumberFormat::WithCountryCode => {
                format!("1-{}-{:03}-{:04}", area_code, exchange, number)
            }
            PhoneNumberFormat::International => {
                format!("+1 {} {:03} {:04}", area_code, exchange, number)
            }
        }
    }
}

impl UsPhone {
    pub fn new(format: PhoneNumberFormat) -> Self {
        Self {
            state: None,
            format,
        }
    }

    pub fn for_state(mut self, state: &'static str) -> Self {
        self.state = Some(state);
        self
    }

    pub fn area_codes(&self) -> Box<dyn Iterator<Item = &'static str>> {
        if let Some(state) = self.state {
            Box::new(
                GLOBAL_DATA
                    .get(state)
                    .expect("No such state")
                    .iter()
                    .copied(),
            )
        } else {
            Box::new(GLOBAL_DATA.values().flatten().copied())
        }
    }

    pub fn iter(&self) -> UsPhone2Iter {
        let area_codes = self.area_codes();

        UsPhone2Iter {
            format: self.format,
            iter: Box::new(
                iproduct!(area_codes, (200..999), (0..9999)).map(
                    |(area_code, district, number)| (String::from(area_code), district, number),
                ),
            ),
        }
    }
}

impl Generator for UsPhone {
    type Item = String;

    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(iproduct!(self.area_codes(), (200..999), (0..9999)).map(
            |(area_code, district, number)| {
                self.format
                    .format_number(area_code.to_string(), district, number)
            },
        ))
    }
}

pub struct UsPhone2Iter {
    format: PhoneNumberFormat,
    iter: Box<dyn Iterator<Item = (String, u32, u32)>>,
}

impl Iterator for UsPhone2Iter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(area_code, district, number)| {
            self.format
                .format_number(area_code, district as i32, number as i32)
        })
    }
}

impl IntoIterator for UsPhone {
    type Item = String;
    type IntoIter = UsPhone2Iter;

    fn into_iter(self) -> Self::IntoIter {
        UsPhone2Iter {
            format: self.format,
            iter: Box::new(
                iproduct!(self.area_codes(), (200..999), (0..9999)).map(
                    |(area_code, district, number)| (String::from(area_code), district, number),
                ),
            ),
        }
    }
}
