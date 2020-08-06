pub struct Version {
    major: u16,
    minor: u16,
    build: u16,
}

impl Version {
    pub fn new(version: String) -> Version {
        let splits: Vec<&str> = version.split(".").collect();

        let ver = Version {
            major: splits[0].parse().unwrap(),
            minor: splits[1].parse().unwrap(),
            build: splits[2].parse().unwrap(),
        };

        return ver;
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

pub struct Package {
    pub path: String,
    pub version: Version,
    pub description: String,
}

pub struct InstalledPackage {
    pub path: String,
    pub version: Version,
    pub description: String,
    pub location: String,
}

impl InstalledPackage {
    pub fn new(path: &str, version: &str, description: &str, location: &str) -> InstalledPackage {
        InstalledPackage {
            path: String::from(path),
            version: Version::new(String::from(version)),
            description: String::from(description),
            location: String::from(location),
        }
    }
}
