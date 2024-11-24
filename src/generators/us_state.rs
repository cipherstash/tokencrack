use super::Generator;

const STATE_ABBREVIATIONS: [&str; 51] = [
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "DC", "FL", "GA", "HI", "ID", "IL", "IN", "IA",
    "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM",
    "NY", "US", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA",
    "WV", "WI", "WY",
];

pub struct UsState;

impl Generator for UsState {
    type Item = String;

    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(STATE_ABBREVIATIONS.iter().map(|state| state.to_string()))
    }
}
