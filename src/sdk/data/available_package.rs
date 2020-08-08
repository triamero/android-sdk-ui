use super::version::Version;

pub struct AvailablePackage {
    pub path: String,
    pub version: Version,
    pub description: String,
}

impl AvailablePackage {
    pub fn new(path: &str, version: &str, description: &str) -> AvailablePackage {
        AvailablePackage {
            path: String::from(path),
            version: Version::new(String::from(version)),
            description: String::from(description),
        }
    }
}
