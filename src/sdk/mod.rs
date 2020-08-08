pub mod data;
pub mod manager;

pub use data::available_package::AvailablePackage;
pub use data::installed_package::InstalledPackage;
pub use data::package::Package;
pub use data::version::Version;

pub fn list_packages() -> Vec<Package> {
    manager::list_packages()
}
