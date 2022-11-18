use std::str::FromStr;

#[derive(Debug)]
pub struct IpRangeBounds {
    pub lower: u8,
    pub upper: u8
}

impl FromStr for IpRangeBounds {
    type Err = String;

    fn from_str(range: &str) -> Result<Self, Self::Err> {
        let mut lower: u8 = 0;
        let mut upper: u8 = 255;
        if range.contains("-") {
            let hyphen_position: usize = range.find('-').unwrap();


        } else {
            match range.trim().parse::<u8>() {
                Ok(n) => {
                    lower = n;
                    upper = n;
                },
                Err(e) => return Err(String::from(format!("Range {} is not a valid u8 value",range))),
            }
        }

        if lower < 0 {
            return Err(String::from("Lower bound may not be lower 0"))
        }

        if upper > 255 {
            return Err(String::from("Upper bound may not be higher than 255"))
        }

        if upper < lower {
            return Err(String::from("Lower cannot be higher upper"));
        }

        Ok(IpRangeBounds { lower, upper })
    }
}

impl IpRangeBounds {
    pub fn validate_ip_range(range: &str) -> Result<(), String> {

        if range.trim().len() != range.len() {
            Err(String::from("package name cannot have leading and trailing space"))
        } else {
            Ok(())
        }
    }
}
