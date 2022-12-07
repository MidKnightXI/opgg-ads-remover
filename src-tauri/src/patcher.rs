

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
fn extract_all(path: &str, _dest: &str) -> bool
{
    use std::io::prelude::*;
    use std::io::SeekFrom;

    // Maybe consider using bufreader later
    let mut file: std::fs::File;
    let mut buf: Vec<u8> = vec![0;4];
    let mut header_size: i32;
    let h_string: String;

    match std::fs::File::open(&path) {
        Ok(f) => file = f,
        Err(e) => panic!("{}", e),
    };

    // Skip the 4 first bytes of 32-bit unsigned integer
    file.seek(SeekFrom::Start(4))
        .expect("unpack_asar: cannot go to offset 4.");

    // Reading header size
    file.read_exact(&mut buf).expect("unpack_asar: cannot read header size.");
    header_size = i32::from_ne_bytes(buf.try_into().unwrap());
    if header_size < 1
    {
        eprintln!("unpack_asar: header_size is < 1 => index out of range.");
        return false;
    }
    header_size -= 10;

    // Get the header content
    file.seek(SeekFrom::Current(8))
        .expect("unpack_asar: cannot read further than header.");
    buf = vec![0; header_size as usize];
    file.read_exact(&mut buf).expect("unpack_asar: cannot read header.");
    h_string = String::from_utf8(buf).expect("unpack_asar: cannot convert to utf8");
    let header: serde_json::Value = serde_json::from_str(h_string.as_str()).unwrap();

    return true
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
    if !extract_all(path.to_str().unwrap(), "./")
    {
        return Err("unpack_asar: cannot unpack asar archive.".to_string());
    }
    return Ok(true);
}