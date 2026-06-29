use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Runtime-evaluated config. Loaded once at startup from TOML files.
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub features: Option<FeatureFlags>,
    pub defaults: Option<DefaultFlags>,
    pub commands: Option<HashMap<String, HashMap<String, toml::Value>>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FeatureFlags {
    pub git: Option<bool>,
    pub fetch: Option<bool>,
    pub symbols: Option<bool>,
    pub detect: Option<bool>,
    pub context: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DefaultFlags {
    pub format: Option<String>,
    pub color: Option<String>,
}

impl Config {
    fn default_features() -> FeatureFlags {
        FeatureFlags {
            git: Some(true),
            fetch: Some(true),
            symbols: Some(true),
            detect: Some(true),
            context: Some(true),
        }
    }

    fn load() -> Self {
        let global_path = global_config_path();
        let project_path = find_project_config();

        let global = if global_path.exists() {
            load_toml(&global_path)
        } else {
            None
        };
        let project = project_path
            .as_ref()
            .filter(|p| p.exists())
            .and_then(load_toml);

        merge(global, project)
    }

    #[allow(dead_code)]
    pub fn is_feature_enabled(&self, name: &str) -> bool {
        match name {
            "git" => self.features.as_ref().and_then(|f| f.git).unwrap_or(true),
            "fetch" => self.features.as_ref().and_then(|f| f.fetch).unwrap_or(true),
            "symbols" => self
                .features
                .as_ref()
                .and_then(|f| f.symbols)
                .unwrap_or(true),
            "detect" => self
                .features
                .as_ref()
                .and_then(|f| f.detect)
                .unwrap_or(true),
            "context" => self
                .features
                .as_ref()
                .and_then(|f| f.context)
                .unwrap_or(true),
            _ => true,
        }
    }

    pub fn default_format(&self) -> Option<super::commands::OutputFormat> {
        self.defaults
            .as_ref()
            .and_then(|d| d.format.as_deref())
            .and_then(|s| match s.to_lowercase().as_str() {
                "json" => Some(super::commands::OutputFormat::Json),
                "text" => Some(super::commands::OutputFormat::Text),
                _ => None,
            })
    }

    pub fn default_color(&self) -> Option<super::commands::ColorChoice> {
        self.defaults
            .as_ref()
            .and_then(|d| d.color.as_deref())
            .and_then(|s| match s.to_lowercase().as_str() {
                "auto" => Some(super::commands::ColorChoice::Auto),
                "always" => Some(super::commands::ColorChoice::Always),
                "never" => Some(super::commands::ColorChoice::Never),
                _ => None,
            })
    }

    #[allow(dead_code)]
    pub fn get_cmd_default(&self, cmd: &str, key: &str) -> Option<toml::Value> {
        self.commands
            .as_ref()
            .and_then(|cmds| cmds.get(cmd))
            .and_then(|defaults| defaults.get(key))
            .cloned()
    }

    pub fn to_toml(&self) -> String {
        let mut buf = String::new();
        buf.push_str("# tk config — effective (merged)\n\n");
        buf.push_str("[features]\n");
        if let Some(ref f) = self.features {
            for (key, val) in [
                ("git", f.git),
                ("fetch", f.fetch),
                ("symbols", f.symbols),
                ("detect", f.detect),
                ("context", f.context),
            ] {
                buf.push_str(&format!("{} = {}\n", key, val.unwrap_or(true)));
            }
        }
        buf.push('\n');
        buf.push_str("[defaults]\n");
        if let Some(ref d) = self.defaults {
            if let Some(ref f) = d.format {
                buf.push_str(&format!("format = \"{}\"\n", f));
            }
            if let Some(ref c) = d.color {
                buf.push_str(&format!("color = \"{}\"\n", c));
            }
        }
        buf.push('\n');
        if let Some(ref cmds) = self.commands {
            for (cmd, opts) in cmds {
                buf.push_str(&format!("[commands.{}]\n", cmd));
                for (key, val) in opts {
                    buf.push_str(&format!("{} = {}\n", key, val));
                }
                buf.push('\n');
            }
        }
        buf
    }
}

/// Initialize the global config. Call once at startup.
pub fn init() {
    let config = Config::load();
    let _ = CONFIG.set(config);
}

/// Get the global config. Returns a default (all features on) if not initialized.
pub fn get() -> &'static Config {
    CONFIG.get().unwrap_or(&DEFAULT_CONFIG)
}

#[allow(dead_code)]
pub fn is_feature_enabled(name: &str) -> bool {
    CONFIG
        .get()
        .map(|c| c.is_feature_enabled(name))
        .unwrap_or(true)
}

#[allow(dead_code)]
pub fn require_feature(name: &str) -> Result<(), String> {
    if is_feature_enabled(name) {
        Ok(())
    } else {
        Err(format!(
            "'{}' is not enabled. Set [features].{} = true in your tk config.",
            name, name
        ))
    }
}

static DEFAULT_CONFIG: Config = Config {
    features: Some(FeatureFlags {
        git: Some(true),
        fetch: Some(true),
        symbols: Some(true),
        detect: Some(true),
        context: Some(true),
    }),
    defaults: None,
    commands: None,
};

fn global_config_path() -> PathBuf {
    let base = match std::env::var("XDG_CONFIG_HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".config")
        }
    };
    base.join("tk").join("config.toml")
}

fn find_project_config() -> Option<PathBuf> {
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

fn load_toml(path: &PathBuf) -> Option<Config> {
    let content = std::fs::read_to_string(path).ok()?;
    toml::from_str(&content)
        .map_err(|e| {
            eprintln!("Warning: failed to parse {}: {}", path.display(), e);
        })
        .ok()
}

fn merge(global: Option<Config>, project: Option<Config>) -> Config {
    let mut merged = Config {
        features: Some(Config::default_features()),
        defaults: None,
        commands: None,
    };

    if let Some(g) = global {
        merge_into(&mut merged, g);
    }
    if let Some(p) = project {
        merge_into(&mut merged, p);
    }

    merged
}

fn merge_into(target: &mut Config, source: Config) {
    if let Some(sf) = source.features {
        let tf = target.features.get_or_insert_with(Config::default_features);
        if let Some(v) = sf.git {
            tf.git = Some(v);
        }
        if let Some(v) = sf.fetch {
            tf.fetch = Some(v);
        }
        if let Some(v) = sf.symbols {
            tf.symbols = Some(v);
        }
        if let Some(v) = sf.detect {
            tf.detect = Some(v);
        }
        if let Some(v) = sf.context {
            tf.context = Some(v);
        }
    }
    if let Some(sd) = source.defaults {
        let td = target.defaults.get_or_insert(DefaultFlags {
            format: None,
            color: None,
        });
        if sd.format.is_some() {
            td.format = sd.format;
        }
        if sd.color.is_some() {
            td.color = sd.color;
        }
    }
    if let Some(sc) = source.commands {
        let tc = target.commands.get_or_insert_with(HashMap::new);
        for (cmd, opts) in sc {
            tc.insert(cmd, opts);
        }
    }
}
