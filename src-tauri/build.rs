/// Build script entry point for the Mycelium application
/// 
/// This function is called during the build process to generate necessary
/// Tauri configuration files and perform any required build-time setup.
/// 
/// # Returns
/// 
/// This function does not return as it exits after build completion.
/// 
/// # Errors
/// 
/// If the build process fails, it will log an error and exit with a non-zero code.
fn main() {
    tauri_build::build()
}
