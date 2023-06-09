use std::{
    path::PathBuf,
    env::consts::OS
};

use asar::{
    AsarReader,
    AsarWriter,
    reader::AsarFile
};

/// Remove ads from the file.
///
/// ## Arguments
/// * `file` - [`String`] containing the content of the file to patch.
///
/// ## Example
/// ```
/// let file_path = "path/to/file.js";
/// let contents = fs::read_to_string(file_path).unwrap();
///
/// patch_file(content);
///```
#[cfg(target_os = "windows")]
fn patch_file(file: String) -> String {
    const GIST: &str = "https://gist.githubusercontent.com";
    const CLOUDFLARE: &str = "https://op-gg-remove-ads.shyim.workers.dev";

    let patched_file: String = file
        .replace(
            r#"checkIfChromeDirectoryExists("Default")"#,
            r#"checkIfChromeDirectoryExists("NoChrome:((((??")"#,
        )
        .replace(
            r#"AppData\Local\Google\Chrome\User Data"#,
            r#"AppData\Local\Google\Carbon\Privacy?"#,
        )
        .replace("https://desktop.op.gg/api/tracking/ow", GIST)
        .replace("https://geo-internal.op.gg/api/current-ip", GIST)
        .replace("app.labs.sydney", "op-gg-remove-ads.shyim.workers.dev")
        .replace("https://opgg-desktop-data.akamaized.net", CLOUDFLARE);

    return patched_file;
}

/// Replace the content of the specified file to delete advertisements' links.
///
/// ## Arguments
/// * `file` - [`String`] containing the content of the file to patch.
///
/// ## Example
/// ```
/// let file_path = "path/to/file.js";
/// let contents = fs::read_to_string(file_path).unwrap();
///
/// patch_file(content);
///```
#[cfg(target_os = "macos")]
fn patch_file(file: String) -> String {
    const ADSENSE_URI: &str = "https://gist.githubusercontent.com/MidKnightXI/7ecf3cdd0a5804466cb790855e2524ae/raw/9b88cf64f3bb955edfff27bdfba72f5181d8748b/remover.txt";
    const NA: &str = r#"["US","CA"].includes"#;
    const EU: &str = r#"["AD","AL","AT","AX","BA","BE","BG","BY","CH","CY","CZ","DE","DK","EE","ES","FI","FO","FR","GB","GG","GI","GR","HR","HU","IE","IM","IS","IT","JE","LI","LT","LU","LV","MC","MD","ME","MK","MT","NL","NO","PL","PT","RO","RS","RU","SE","SI","SJ","SK","SM","UA","VA","XK"].includes"#;

    let patched_file: String = file
        .replace(
            "google-analytics.com/mp/collect",
            "gist.githubusercontent.com",
        )
        .replace(
            r#"exports\.countryHasAdsAdsense=\w;"#,
            "exports.countryHasAdsAdsense=[];",
        )
        .replace("https://dtapp-player.op.gg/adsense.txt", ADSENSE_URI)
        .replace(NA, "[].includes")
        .replace(EU, "[].includes")
        .replace(r#"exports\.countryHasAds=\w;"#, "exports.countryHasAds=[];")
        .replace(r#"exports\.adsenseAds=\w;"#, "exports.adsenseAds=[];")
        .replace(r#"exports\.playwireAds=\w;"#, "exports.playwireAds=[];")
        .replace(r#"exports\.nitropayAds=\w;"#, "exports.nitropayAds=[];");

    return patched_file;
}

/// Scan the asar archive located at `asar_path`
///
/// ## Arguments
/// * `asar_path` - [`PathBuf`] containing the path to the asar archive.
///
/// ## Example
/// ```
/// let asar_path = std::path::PathBuf::from("/path/to/archive.asar");
///
/// scan_all(asar_path);
/// ```
#[cfg(target_os = "windows")]
fn scan_all(asar_path: PathBuf) -> asar::Result<()> {
    let asar_file: Vec<u8> = std::fs::read(asar_path.clone())?;
    let asar_r = AsarReader::new(&asar_file, asar_path.clone())?;
    let mut asar_w = AsarWriter::new();
    let paths = asar_r.files();

    println!("scan_all: scanning files from the archive.");
    for path in paths.keys() {
        let file: &AsarFile;

        file = match asar_r.files().get(path) {
            Some(v) => v,
            None => continue
        };

        if path.to_str().unwrap().eq(r"assets\main\main.js")
        {
            let patched: String;

            println!("scan_all: removing ads of {:?}", path);
            patched = patch_file(String::from_utf8(file.data().to_vec()).unwrap());
            asar_w.write_file(path.as_path(), patched.as_bytes(), false)?;
        }
        else
        {
            asar_w.write_file(path.as_path(), file.data(), false)?;
        }
    }

    println!("scan_all: rebuilding asar archive");
    asar_w.finalize(std::fs::File::create(asar_path)?)?;
    Ok(())
}

/// Scan the asar archive located at `asar_path`
///
/// ## Arguments
/// * `asar_path` - [`PathBuf`] containing the path to the asar archive.
///
/// ## Example
/// ```
/// let asar_path = std::path::PathBuf::from("/path/to/archive.asar");
///
/// scan_all_old(asar_path);
/// ```
#[cfg(target_os = "macos")]
fn scan_all(asar_path: PathBuf) -> asar::Result<()> {
    let asar_file: Vec<u8> = std::fs::read(asar_path.clone())?;
    let asar_r = AsarReader::new(&asar_file, asar_path.clone())?;
    let mut asar_w = AsarWriter::new();
    let files = asar_r.files();

    println!("scan_all: scanning files of the archive.");
    for path in files.keys() {
        let file: &AsarFile;

        file = match asar_r.files().get(path) {
            Some(v) => v,
            None => continue
        };

        if path.starts_with("assets/react") && path.ends_with(".js")
        {
            let patched: String;

            println!("patch_file: removing ads from {}", path.to_str().unwrap());
            patched = patch_file(String::from_utf8(file.data().to_vec()).unwrap());
            asar_w.write_file(path.as_path(), patched.as_bytes(), false)?;
        }
        else
        {
            asar_w.write_file(path.as_path(), file.data(), false)?;
        }
    }

    println!("scan_all: rebuilding asar archive");
    asar_w.finalize(std::fs::File::create(asar_path)?)?;
    Ok(())
}

/// Spawn a process to kill OP.GG process
fn kill_opgg() {
    use std::process::Command;

    let mut process: Command;

    if OS == "macos"
    {
        process = Command::new("killall");
        process.args(&["-9", "OP.GG"]);
    }
    else if OS == "windows"
    {
        // const CREATE_NO_WINDOW = 0x08000000;
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
/// * Success - returns the canonicalize version of the path to OP.GG app directory
/// * Failure - returns a String containing the error
fn format_asar_path() -> Result<PathBuf, String> {
    let path: PathBuf;

    println!("format_asar_path: determining platform.");
    if OS == "macos" {
        path = PathBuf::from("/Applications/OP.GG.app/Contents/Resources/app.asar");
    }
    else if OS == "windows" {
        let appdata: String;

        appdata = match std::env::var("LOCALAPPDATA") {
            Ok(v) => v,
            Err(e) => return Err(format!("format_asar_path: {e}")),
        };

        path = PathBuf::from(appdata).join("Programs/OP.GG/resources/app.asar");
    }
    else {
        return Err("format_asar_path: Platform not compatible.".to_string());
    }

    match std::fs::canonicalize(path) {
        Ok(val) => return Ok(val),
        Err(e) => return Err(format!("format_asar_path: {e}")),
    }
}

/// Main function for the patcher mod
///
/// ## Returns
/// * Success - returns `true`
/// * Failure - returns a String containing the error
pub fn remove_ads() -> Result<bool, String> {
    let path: PathBuf;

    path = match format_asar_path() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    if !path.exists() {
        return Err("remove_ads: OP.GG not found, make sure the app is installed.".to_string());
    }

    kill_opgg();

    match scan_all(path) {
        Ok(_) => {}
        Err(e) => return Err(format!("extract_all: {e}")),
    }

    Ok(true)
}
