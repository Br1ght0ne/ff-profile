// ff-profile - prints path to your default Firefox profile
// Copyright (C) 2019 Oleksii Filonenko <brightone@protonmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use dirs::home_dir;
use ini::Ini;
use snafu::{OptionExt, ResultExt, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("user home directory doesn't exist"))]
    NoHomeDir,
    #[snafu(display("profiles.ini doesn't exist"))]
    NoProfilesIni,
    #[snafu(display("parsing INI failed: {}", source))]
    ParseIniError { source: ini::ini::Error },
    #[snafu(display("no Install* section in profiles.ini"))]
    NoInstallSection,
    #[snafu(display("no default profile in profiles.ini"))]
    NoDefaultProfile,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

fn default_profile_path() -> Result<PathBuf> {
    let ini = Ini::load_from_file(find_profiles_ini()?).context(ParseIniError)?;
    let default_profile_name = find_default_profile(&ini)?;
    Ok(firefox_home()?.join(default_profile_name))
}

fn find_default_profile(ini: &Ini) -> Result<&String> {
    ini.iter()
        // remove the Option layer from section names
        .filter_map(|(section, props): (&Option<String>, _)| section.as_ref().map(|s| (s, props)))
        .find(|(section, _): &(&String, _)| section.starts_with("Install"))
        .context(NoInstallSection)
        .and_then(|(_, props)| props.get("Default".into()).context(NoDefaultProfile))
}

fn firefox_home() -> Result<PathBuf> {
    let home = home_dir().context(NoHomeDir)?;
    Ok(home.join(".mozilla/firefox"))
}

fn find_profiles_ini() -> Result<PathBuf> {
    let path = firefox_home()?.join("profiles.ini");
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::NoProfilesIni)
    }
}

fn main() {
    match default_profile_path() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
