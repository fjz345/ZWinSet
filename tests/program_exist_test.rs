#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*; // imports items from parent module
    use ZWinSet::{
        utils::path_exists,
        windows::{
            does_program_exist, does_program_path_exist_on_any_drive, does_program_registry_exist,
        },
    };

    /* Tests that can not be included in regular test runs */
    #[test]
    #[ignore]
    fn test_does_program_registry_exist() {
        assert!(does_program_registry_exist("Steam"));
        assert!(does_program_registry_exist("Chrome"));
        assert!(does_program_registry_exist("Spotify"));
    }
    #[test]
    #[ignore]
    fn test_does_path_exist() {
        assert!(path_exists(Path::new("C:\\Program Files (x86)\\Steam")));
        assert!(path_exists(Path::new(
            "C:\\Program Files\\Google\\Chrome\\Application"
        )));
    }
    #[test]
    #[ignore]
    fn test_does_program_path_exist_on_any_drive() {
        assert!(does_program_path_exist_on_any_drive(
            "Program Files (x86)\\Steam"
        ));
        assert!(does_program_path_exist_on_any_drive(
            "Program Files\\Google\\Chrome\\Application"
        ));
    }
    #[test]
    #[ignore]
    fn test_does_program_exist() {
        assert!(does_program_exist("Steam"));
        assert!(does_program_exist("Spotify"));
        assert!(does_program_exist("Chrome"));
    }
}
