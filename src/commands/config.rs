use clap::Args;

#[derive(Args)]
pub struct ConfigArgs {
    #[arg(long, help = "Show config file paths being used")]
    pub paths: bool,
}

pub fn run(args: &ConfigArgs) -> Result<(), String> {
    if args.paths {
        print_paths();
        return Ok(());
    }
    let config = crate::config::get();
    print!("{}", config.to_toml());
    Ok(())
}

fn print_paths() {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());

    let global = std::env::var("XDG_CONFIG_HOME")
        .map(|d| std::path::PathBuf::from(d).join("tk").join("config.toml"))
        .unwrap_or_else(|_| {
            std::path::PathBuf::from(&home)
                .join(".config")
                .join("tk")
                .join("config.toml")
        });

    let project = find_project_config_path();

    println!(
        "Global config: {} ({})",
        global.display(),
        if global.exists() {
            "found"
        } else {
            "not found"
        }
    );
    match &project {
        Some(p) => println!("Project config: {} (found)", p.display()),
        None => println!("Project config: (none found)"),
    }
    println!("Config dir: {}", global.parent().unwrap().display());
}

fn find_project_config_path() -> Option<std::path::PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let candidate = dir.join(".tkconfig.toml");
        if candidate.exists() {
            return Some(candidate);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}
