fn kill_opgg()
{

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
    return true;
}