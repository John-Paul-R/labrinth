use crate::validate::{SupportedGameVersions, ValidationError, ValidationResult};
use std::io::Cursor;
use zip::ZipArchive;

pub struct BukkitValidator {}

impl super::Validator for BukkitValidator {
    fn get_file_extensions<'a>(&self) -> &'a [&'a str] {
        &["jar"]
    }

    fn get_project_types<'a>(&self) -> &'a [&'a str] {
        &["plugin"]
    }

    fn get_supported_loaders<'a>(&self) -> &'a [&'a str] {
        &["bukkit", "spigot", "paper", "purpur"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<&[u8]>>,
    ) -> Result<ValidationResult, ValidationError> {
        // TODO: Add schema validation in order to make sure that it won't error out
        archive.by_name("plugin.yml").map_err(|_| {
            ValidationError::InvalidInputError(
                "No plugin.yml file is present in your file.".to_string(),
            )
        })?;

        Ok(ValidationResult::Pass)
    }
}