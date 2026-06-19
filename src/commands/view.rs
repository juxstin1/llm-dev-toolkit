use crate::commands::ansi;
use crate::{CatArgs, HeadArgs, PreviewArgs};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

fn style_ansi(style: &syntect::highlighting::Style) -> String {
    let mut s = String::from("\x1b[0");
    if style.font_style.contains(FontStyle::BOLD) {
        s.push_str(";1");
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        s.push_str(";3");
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        s.push_str(";4");
    }
    s.push_str(&format!(
        ";38;2;{};{};{}",
        style.foreground.r, style.foreground.g, style.foreground.b
    ));
    if style.background.a > 0 {
        s.push_str(&format!(
            ";48;2;{};{};{}",
            style.background.r, style.background.g, style.background.b
        ));
    }
    s.push('m');
    s
}

pub fn run_cat(args: &CatArgs) -> Result<(), String> {
    let multiple = args.files.len() > 1;
    for path in &args.files {
        let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path, e))?;
        if multiple {
            print!("\n{}{}{}\n", ansi::BOLD, path, ansi::RESET);
            if content.lines().count() > 0 {
                println!();
            }
        }
        if args.number {
            for (i, line) in content.lines().enumerate() {
                println!("{:>6}\t{}", i + 1, line);
            }
        } else {
            print!("{}", content);
            if !content.ends_with('\n') {
                println!();
            }
        }
    }
    Ok(())
}

pub fn run_preview(args: &PreviewArgs) -> Result<(), String> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    let multiple = args.files.len() > 1;

    for path in &args.files {
        let p = Path::new(path);
        if super::is_binary(p) {
            if multiple {
                print!("\n{}{}{}\n\n", ansi::BOLD, path, ansi::RESET);
            }
            println!("[binary file]");
            continue;
        }

        let syntax = if let Some(lang) = &args.language {
            ss.find_syntax_by_token(lang)
                .unwrap_or_else(|| ss.find_syntax_plain_text())
        } else {
            match ss.find_syntax_for_file(p) {
                Ok(Some(s)) => s,
                _ => ss.find_syntax_plain_text(),
            }
        };

        let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path, e))?;

        if multiple {
            print!("\n{}{}{}\n", ansi::BOLD, path, ansi::RESET);
            if content.lines().count() > 0 {
                println!();
            }
        }

        if args.number {
            let mut highlighter = HighlightLines::new(syntax, theme);
            for (i, line) in LinesWithEndings::from(&content).enumerate() {
                let ranges = highlighter
                    .highlight_line(line, &ss)
                    .map_err(|e| format!("highlight error: {}", e))?;
                print!("{:>6}\t", i + 1);
                for (style, text) in &ranges {
                    let trimmed = text.trim_end_matches('\n').trim_end_matches('\r');
                    if !trimmed.is_empty() {
                        print!("{}{}\x1b[0m", style_ansi(style), trimmed);
                    }
                }
                println!();
            }
        } else {
            let mut highlighter = HighlightLines::new(syntax, theme);
            for line in LinesWithEndings::from(&content) {
                let ranges = highlighter
                    .highlight_line(line, &ss)
                    .map_err(|e| format!("highlight error: {}", e))?;
                for (style, text) in &ranges {
                    if !text.is_empty() {
                        print!("{}{}\x1b[0m", style_ansi(style), text);
                    }
                }
            }
            if !content.ends_with('\n') && !content.is_empty() {
                println!();
            }
        }
    }
    Ok(())
}

pub fn run_head(args: &HeadArgs) -> Result<(), String> {
    let multiple = args.files.len() > 1;
    for path in &args.files {
        let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path, e))?;
        if multiple {
            println!("\n==> {} <==\n", path);
        }
        for line in content.lines().take(args.lines) {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn run_tail(args: &HeadArgs) -> Result<(), String> {
    let multiple = args.files.len() > 1;
    for path in &args.files {
        let content = fs::read_to_string(path).map_err(|e| format!("{}: {}", path, e))?;
        if multiple {
            println!("\n==> {} <==\n", path);
        }
        if args.lines == 0 {
            continue;
        }
        let mut buf: VecDeque<&str> = VecDeque::with_capacity(args.lines);
        for line in content.lines() {
            if buf.len() == args.lines {
                buf.pop_front();
            }
            buf.push_back(line);
        }
        for line in &buf {
            println!("{}", line);
        }
    }
    Ok(())
}
