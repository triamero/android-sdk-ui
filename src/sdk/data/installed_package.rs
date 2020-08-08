use super::version::Version;

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
