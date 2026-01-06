use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VSCodeInstance {
    pub installation_path: String,
    pub installation_version: String,
    pub product_path: String,
    pub product_id: String,
    pub is_prerelease: bool,
    pub display_name: String,
    pub extensions_path: String,
    pub user_data_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInfo {
    #[allow(dead_code)]
    pub name_short: Option<String>,
    pub name_long: Option<String>,
    pub quality: Option<String>,
    pub data_folder_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PackageInfo {
    pub version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug)]
pub struct Options {
    pub all: bool,
    pub prerelease: bool,
    pub latest: bool,
    pub format: OutputFormat,
    pub property: Option<String>,
    pub nologo: bool,
    pub sort: bool,
    pub help: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            all: true,
            prerelease: false,
            latest: false,
            format: OutputFormat::Text,
            property: None,
            nologo: false,
            sort: false,
            help: false,
        }
    }
}

impl Options {
    pub fn parse(args: &[String]) -> Self {
        let mut opts = Self::default();
        let mut i = 0;

        while i < args.len() {
            let arg = args[i].to_lowercase();
            match arg.as_str() {
                "-all" => opts.all = true,
                "-prerelease" => opts.prerelease = true,
                "-latest" => opts.latest = true,
                "-format" => {
                    if i + 1 < args.len() {
                        i += 1;
                        opts.format = match args[i].to_lowercase().as_str() {
                            "json" => OutputFormat::Json,
                            _ => OutputFormat::Text,
                        };
                    }
                }
                "-property" => {
                    if i + 1 < args.len() {
                        i += 1;
                        opts.property = Some(args[i].clone());
                    }
                }
                "-nologo" => opts.nologo = true,
                "-sort" => opts.sort = true,
                "-help" | "-?" | "/?" => opts.help = true,
                _ => {}
            }
            i += 1;
        }

        opts
    }
}
