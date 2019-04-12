use std::default::Default;

struct Lotus {
    symbol: String,
    precision: u8,
    thousand_str: String,
    decimal_str: String,
    formatting: String
}

impl Lotus {
    fn new(symbol: String, precision: u8) -> Lotus {
        Lotus {
            symbol,
            precision,
            thousand_str: ",".into(),
            decimal_str: ".".into(),
            formatting: "%s %v".into()
        }
    }

    fn format<T: >(&self, number: T) -> String {

    }
}

impl Default for Lotus {
    fn default() -> Self {
        Lotus {
            symbol: "$",
            precision: 2,
            thousand_str: ",",
            decimal_str: ".",
            formatting: "%s %v",
        }
    }
}
