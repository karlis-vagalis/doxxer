pub mod default {
    pub static CONFIG_FILE_NAME: &str = "doxxer";

    pub static DIRECTORY: &str = ".";
    pub static TAG_FILTER: &str = "";
    pub static OUTPUT_TEMPLATE: &str = "{version}";

    pub static INCREMENT: u64 = 1;

    pub static PRERELEASE_IDENTIFIER: &str = "build";
    pub static DEV_IDENTIFIER: &str = "dev";

    pub static PRERELEASE_TEMPLATE: &str = "{identifier}.{inc}";
    pub static DEV_PRERELEASE_TEMPLATE: &str = "{pre}.{identifier}.{distance}";

    pub static BUILD_METADATA_TEMPLATE: &str = "";
    pub static DEV_BUILD_METADATA_TEMPLATE: &str = "{hash}";
}