mod discovery;
mod models;
mod output;

use models::{Options, OutputFormat};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = Options::parse(&args);

    if !options.nologo {
        println!("VSCWhere version {}", VERSION);
        println!();
    }

    if options.help {
        print_help();
        return;
    }

    let mut instances = discovery::discover_installations(options.prerelease);

    if options.sort {
        instances.sort_by(|a, b| {
            let va = parse_version(&a.installation_version);
            let vb = parse_version(&b.installation_version);
            vb.cmp(&va) // Descending order
        });
    }

    if options.latest && !instances.is_empty() {
        instances.truncate(1);
    }

    let output = match options.format {
        OutputFormat::Json => output::format_json(&instances, options.property.as_deref()),
        OutputFormat::Text => output::format_text(&instances, options.property.as_deref()),
    };

    println!("{}", output);
}

fn parse_version(version: &str) -> Vec<u32> {
    version
        .split('.')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn print_help() {
    println!(
        r#"Usage: vscwhere [options]

Options:
  -all            Find all instances (default)
  -prerelease     Include prerelease (Insiders) builds
  -latest         Return only the latest version
  -format <type>  Output format: text (default), json
  -property <name> Return value of specified property
  -nologo         Suppress version banner
  -sort           Sort instances by version (descending)
  -help, -?       Display this help message

Properties:
  installationPath     Installation directory
  installationVersion  Version number
  productPath          Path to Code.exe
  productId            Product identifier (stable/insider)
  isPrerelease         True for Insiders builds
  displayName          Human-readable product name
  extensionsPath       User extensions directory
  userDataPath         User settings/data directory

Examples:
  vscwhere                           List all VS Code installations
  vscwhere -prerelease               Include Insiders builds
  vscwhere -latest -format json      Latest install as JSON
  vscwhere -property installationPath  Just the install paths"#
    );
}
