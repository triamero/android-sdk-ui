use ::std::fmt::{Display, Error, Formatter};
use ::std::cmp::{Ord, PartialEq, PartialOrd};
use ::std::cmp::Ordering;

#[derive(Clone, Eq)]
pub struct Version {
    major: u32,
    minor: u32,
    build: u32,
}

impl Version {
    pub fn new(version: String) -> Version {
        let mut splits: Vec<&str> = version.split(".").collect();

        if splits.len() < 2 {
            splits.push("0");
        }

        if splits.len() < 3 {
            splits.push("0");
        }

        return Version {
            major: parse_part_of_version(splits[0]),
            minor: parse_part_of_version(splits[1]),
            build: parse_part_of_version(splits[2]),
        };
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.build == other.build
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {

        let cmp_major = self.major.cmp(&other.major);

        if cmp_major != Ordering::Equal {
            return cmp_major;
        }

        let cmp_minor = self.minor.cmp(&other.minor);

        if cmp_minor != Ordering::Equal {
            return cmp_minor;
        }

        self.build.cmp(&other.build)
    }
}

fn parse_part_of_version(part: &str) -> u32 {
    return match part.trim().parse() {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            0
        }
    };
}
