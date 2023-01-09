/// Replace the content of the specified file to delete advertisements' links.
///
/// ## Arguments
///
/// * `path` - `std::path::PathBuf` containing the path of the file to patch.
///
/// ## Example
/// ```
/// let path = std::path::PathBuf::from("/path/to/file");
///
/// patch_file(path);
///```
fn patch_file(path: std::path::PathBuf) -> std::io::Result<()>
{
    let mut content: String = std::fs::read_to_string(path.as_path())?;

    let adsense_uri_patch: &str = "https://gist.githubusercontent.com/MidKnightXI/7ecf3cdd0a5804466cb790855e2524ae/raw/9b88cf64f3bb955edfff27bdfba72f5181d8748b/remover.txt";
    let na: &str = r#"["US","CA"].includes"#;
    let eu: &str = r#"["AD","AL","AT","AX","BA","BE","BG","BY","CH","CY","CZ","DE","DK","EE","ES","FI","FO","FR","GB","GG","GI","GR","HR","HU","IE","IM","IS","IT","JE","LI","LT","LU","LV","MC","MD","ME","MK","MT","NL","NO","PL","PT","RO","RS","RU","SE","SI","SJ","SK","SM","UA","VA","XK"].includes"#;

    let patched_content: String = content
        .replace("https://dtapp-player.op.gg/adsense.txt", adsense_uri_patch)
        .replace("google-analytics.com/mp/collect", "gist.githubusercontent.com")
        .replace(na, "[].includes")
        .replace(eu, "[].includes")
        .replace(r#"exports\.countryHasAds=\w;"#, "exports.countryHasAds=[];")
        .replace(r#"exports\.countryHasAdsAdsense=\w;"#, "exports.countryHasAdsAdsense=[];")
        .replace(r#"exports\.adsenseAds=\w;"#, "exports.adsenseAds=[];")
        .replace(r#"exports\.playwireAds=\w;"#, "exports.playwireAds=[];")
        .replace(r#"exports\.nitropayAds=\w;"#, "exports.nitropayAds=[];");

    std::fs::write(path, patched_content)?;
    Ok(())
}

fn scan_dir(asar_file_path: &str) -> std::io::Result<()> {
    let asset_dir: std::fs::ReadDir = std::fs::read_dir("opgg_unpacked/assets/react")?;

    for file in asset_dir {
        let file = file?;
        if file.path().extension().unwrap_or_default() == "js" {
            patch_file(file.path())?;
        }
    }
    Ok(())
}


/// Unpack the asar archive located at `path`
///
/// ## Arguments
///
/// * `path` - `std::path::PathBuf` containing the path to the asar archive.
///
/// # Example
/// ```
/// let path = std::path::PathBuf::from("/path/to/file");
///
/// unpack_asar(path);
/// ```
fn extract_all(path: std::path::PathBuf) -> bool
{
    // dest: "./opgg_unpacked"
    return false;
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
    }
    else
    {
        return;
    }

    println!("kill_opgg: killing opgg process.");
    process.spawn().expect("kill_opgg: spawn error");
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

    println!("format_asar_path: determining platform.");
    if std::env::consts::OS == "macos"
    {
        path = "/Applications/OP.GG.app/Contents/Resources/app.asar".to_string();
    }
    else if std::env::consts::OS == "windows"
    {
        match std::env::var("APP_DATA") {
            Ok(v) => path = format!(
                "{}/Local/Programs/OP.GG/resources/app.asar",
                v),
            Err(e) => return Err(format!("format_asar_path: {e}"))
        };
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
        return Err("remove_ads: OP.GG not found, make sure the app is installed.".to_string());
    }
    kill_opgg();
    if !extract_all(path)
    {
        return Err("unpack_asar: cannot unpack asar archive.".to_string());
    }
    return Ok(true);
}