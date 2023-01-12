use asar::{AsarReader, AsarWriter};

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
fn patch_file(file: String) -> String
{
    let adsense_uri_patch: &str = "https://gist.githubusercontent.com/MidKnightXI/7ecf3cdd0a5804466cb790855e2524ae/raw/9b88cf64f3bb955edfff27bdfba72f5181d8748b/remover.txt";
    let na: &str = r#"["US","CA"].includes"#;
    let eu: &str = r#"["AD","AL","AT","AX","BA","BE","BG","BY","CH","CY","CZ","DE","DK","EE","ES","FI","FO","FR","GB","GG","GI","GR","HR","HU","IE","IM","IS","IT","JE","LI","LT","LU","LV","MC","MD","ME","MK","MT","NL","NO","PL","PT","RO","RS","RU","SE","SI","SJ","SK","SM","UA","VA","XK"].includes"#;

    let patched_file: String = file
        .replace("https://dtapp-player.op.gg/adsense.txt", adsense_uri_patch)
        .replace("google-analytics.com/mp/collect", "gist.githubusercontent.com")
        .replace(na, "[].includes")
        .replace(eu, "[].includes")
        .replace(r#"exports\.countryHasAds=\w;"#, "exports.countryHasAds=[];")
        .replace(r#"exports\.countryHasAdsAdsense=\w;"#, "exports.countryHasAdsAdsense=[];")
        .replace(r#"exports\.adsenseAds=\w;"#, "exports.adsenseAds=[];")
        .replace(r#"exports\.playwireAds=\w;"#, "exports.playwireAds=[];")
        .replace(r#"exports\.nitropayAds=\w;"#, "exports.nitropayAds=[];");
    return patched_file;
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
fn extract_all(asar_path: std::path::PathBuf) -> asar::Result<()>
{
    let asar_file: Vec<u8> = std::fs::read(asar_path.clone())?;
    let asar_r: AsarReader = AsarReader::new(&asar_file, asar_path.clone())?;
    let mut asar_w: AsarWriter = AsarWriter::new();

    for path in asar_r.files().keys()
    {
        let path_str = path.to_str().unwrap();
        let file = asar_r.files().get(path).unwrap();
        if path_str.starts_with("assets/react") && path_str.ends_with(".js")
        {
            let patched = patch_file(String::from_utf8(file.data().to_vec()).unwrap());
            asar_w.write_file(path.as_path(), patched.as_bytes(), false)?;

        }
        else
        {
            asar_w.write_file(path.as_path(), file.data(), false)?;
        }
    }
    asar_w.finalize(std::fs::File::create(asar_path)?)?;
    Ok(())
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
    match extract_all(path)
    {
        Ok(_) => {},
        Err(e) => return Err(format!("extract_all: {e}"))
    }
    Ok(true)
}