use super::version::Version;

pub struct Package {
    id: String,
    description: String,
    location: Option<String>,
    installed_version: Option<Version>,
    available_version: Option<Version>,
    available_update: Option<Version>,
}

impl Package {
    pub fn new(
        id: String,
        description: String,
        location: Option<String>,
        installed: Option<Version>,
        available: Option<Version>,
        update: Option<Version>,
    ) -> Package {
        Package {
            id: id,
            description: description,
            location: location,
            installed_version: installed,
            available_version: available,
            available_update: update
        }
    }
}
