fn kill_opgg()
{
    if std::env::consts::OS == "macos"
    {
        std::process::Command::new("killall")
        .args(["-9", "OP.GG"])
        .spawn()
        .expect("kill_opgg: OPGG not killed or already killed");
    }
    else if std::env::consts::OS == "windows"
    {
        std::process::Command::new("taskkill")
        .args(["/im", "OP.GG.exe", "/F"])
        .spawn()
        .expect("kill_opgg: OPGG not killed or already killed");
    }
}

fn asar_path() -> String
{
    let path: String;

    if std::env::consts::OS == "macos"
    {
        path = "/Applications/OP.GG.app/Contents/Resources/app.asar".to_string();
    }
    else if std::env::consts::OS == "windows"
    {
        match std::env::var("APP_DATA") {
            Ok(k) => path = format!("{}/Local/Programs/OP.GG/resources/app.asar", k),
            Err(e) => panic!("$APP_DATA is not set ({})", e)
        }
    }
    else
    {
        panic!("Your OS isn't compatible with this script");
    }
    return path;
}

pub fn remove_ads() -> bool
{
    let asar_file_path: String = asar_path();

    if std::path::Path::new(&asar_file_path).exists()
    {
        kill_opgg();
        return true;
    }
    return false;
}