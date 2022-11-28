/// Unpack the asar archive located at `path`
///
/// # Arguments
///
/// * `path` - &str containing the path where the archive is located
///
/// # Example
/// ```
/// unpack_asar("/path/to/archive.asar");
/// ```
fn unpack_asar(_path: &str) -> bool
{
    return false
}

/// Spawn a process to kill OP.GG process
fn kill_opgg()
{
    use std::process::Command;

    let mut process: Command;

    if std::env::consts::OS == "macos"
    {
        process = Command::new("killall");
        process.args(&["-9", "OP.GG"]);
    }
    else if std::env::consts::OS == "windows"
    {
        // let CREATE_NO_WINDOW = 0x08000000;

        process = Command::new("taskkill");
        process.args(&["/im", "OP.GG.exe", "/F"]);

    } else { return; }

    process.spawn().expect("kill_opgg");
}

/// Returns the path to OP.GG based on which platform the user is using
///
/// ## Returns
///
/// * Success - returns the canonicalize version of the path to OP.GG app directory
/// * Failure - returns a String containing the error
fn format_asar_path() -> Result<std::path::PathBuf, String>
{
    let path: String;

    if std::env::consts::OS == "macos"
    {
        path = "/Applications/OP.GG.app/Contents/Resources/app.asar".to_string();
    }
    else if std::env::consts::OS == "windows"
    {
        let appdata: String;

        match std::env::var("APP_DATA") {
            Ok(val) => appdata = val,
            Err(e) => return Err(format!("format_asar_path: {e}"))
        };

        path = format!(
            "{}/Local/Programs/OP.GG/resources/app.asar",
            appdata
        );
    }
    else
    {
        return Err("format_asar_path: Platform not compatible.".to_string());
    }

    match std::fs::canonicalize(path) {
        Ok(val) => return Ok(val),
        Err(e) => return Err(format!("format_asar_path: {e}"))
    }
}

/// Main function for the patcher mod
///
/// ## Returns
///
/// * Success - returns `true`
/// * Failure - returns a String containing the error
pub fn remove_ads() -> Result<bool, String>
{
    let path: std::path::PathBuf;

    match format_asar_path() {
        Ok(val) => path = val,
        Err(e) => return Err(e)
    };

    if !path.exists()
    {
        return Err(
            "remove_ads: OP.GG not found, make sure the app is installed."
            .to_string()
        );
    }
    kill_opgg();
    if !unpack_asar(path.to_str().unwrap())
    {
        return Err(
            "unpack_asar: cannot unpack asar archive."
            .to_string()
        );
    }
    return Ok(true);
}