use std::io::Read;

fn read_input(file: &Option<String>) -> Result<String, String> {
    match file {
        Some(path) => std::fs::read_to_string(path).map_err(|e| e.to_string()),
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| e.to_string())?;
            Ok(buf)
        }
    }
}

pub fn run(args: &crate::JsonArgs) -> Result<(), String> {
    match &args.action {
        crate::JsonAction::Format { file, spaces } => {
            let input = read_input(file)?;
            let value: serde_json::Value =
                serde_json::from_str(&input).map_err(|e| format!("Invalid JSON: {}", e))?;
            let indent = spaces.unwrap_or(2);
            let formatted = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
            if indent == 2 {
                println!("{}", formatted);
            } else {
                for line in formatted.lines() {
                    let trimmed = line.trim_start();
                    let leading = line.len() - trimmed.len();
                    let new_leading = " ".repeat(leading / 2 * indent);
                    println!("{}{}", new_leading, trimmed);
                }
            }
        }
        crate::JsonAction::Validate { file } => {
            let input = read_input(file)?;
            match serde_json::from_str::<serde_json::Value>(&input) {
                Ok(_) => println!("valid"),
                Err(e) => println!("invalid: {}", e),
            }
        }
        crate::JsonAction::Keys { file } => {
            let input = read_input(file)?;
            let value: serde_json::Value =
                serde_json::from_str(&input).map_err(|e| format!("Invalid JSON: {}", e))?;
            match value {
                serde_json::Value::Object(map) => {
                    for key in map.keys() {
                        println!("{}", key);
                    }
                }
                _ => return Err("JSON value is not an object".to_string()),
            }
        }
    }
    Ok(())
}
