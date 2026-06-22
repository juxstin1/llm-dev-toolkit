use clap::{Args, Subcommand, ValueEnum};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SPEC0: &str = include_str!("../../spec0/commands/spec0.md");
const SPEC0_PLAN: &str = include_str!("../../spec0/commands/spec0-plan.md");
const SPEC0_EXEC: &str = include_str!("../../spec0/commands/spec0-exec.md");
const SPEC0_REVIEW: &str = include_str!("../../spec0/commands/spec0-review.md");
const SPEC0_HANDOFF: &str = include_str!("../../spec0/commands/spec0-handoff.md");
const CODEX_SKILL: &str = include_str!("../../spec0/skills/codex/spec0/SKILL.md");

#[derive(Args)]
pub struct Spec0Args {
    #[command(subcommand)]
    action: Spec0Action,
}

#[derive(Subcommand)]
enum Spec0Action {
    #[command(about = "List bundled Spec0 commands and install targets")]
    List,
    #[command(about = "Print a bundled Spec0 command prompt")]
    Print {
        #[arg(help = "Command name, such as spec0-plan. Omit to print all commands")]
        command: Option<String>,
    },
    #[command(about = "Install Spec0 commands for Claude Code, Codex, or OpenCode")]
    Install {
        #[arg(
            long,
            value_enum,
            default_value = "all",
            help = "Agent surface to install"
        )]
        agent: Spec0Agent,
        #[arg(
            long,
            value_enum,
            default_value = "user",
            help = "Install globally for the user or into a project directory"
        )]
        scope: Spec0Scope,
        #[arg(
            long,
            value_name = "DIR",
            help = "Project root for --scope project (default: current directory)"
        )]
        dir: Option<PathBuf>,
        #[arg(long, help = "Overwrite existing command files")]
        force: bool,
        #[arg(long, help = "Show files that would be written without writing them")]
        dry_run: bool,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Spec0Agent {
    All,
    Claude,
    Codex,
    Opencode,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Spec0Scope {
    User,
    Project,
}

struct Template {
    name: &'static str,
    description: &'static str,
    content: &'static str,
}

struct InstallTarget {
    path: PathBuf,
    content: &'static str,
}

const TEMPLATES: &[Template] = &[
    Template {
        name: "spec0",
        description: "run the full orient, frame, plan, execute, verify, handoff loop",
        content: SPEC0,
    },
    Template {
        name: "spec0-plan",
        description: "convert a rough request into a bounded implementation plan",
        content: SPEC0_PLAN,
    },
    Template {
        name: "spec0-exec",
        description: "execute the next planned slice and verify it",
        content: SPEC0_EXEC,
    },
    Template {
        name: "spec0-review",
        description: "review current work against the Spec0 frame",
        content: SPEC0_REVIEW,
    },
    Template {
        name: "spec0-handoff",
        description: "summarize status for the next agent session",
        content: SPEC0_HANDOFF,
    },
];

pub fn run(args: &Spec0Args) -> Result<(), String> {
    match &args.action {
        Spec0Action::List => {
            list_commands();
            Ok(())
        }
        Spec0Action::Print { command } => print_command(command.as_deref()),
        Spec0Action::Install {
            agent,
            scope,
            dir,
            force,
            dry_run,
        } => install(*agent, *scope, dir.as_deref(), *force, *dry_run),
    }
}

fn list_commands() {
    println!("Spec0 commands:");
    for template in TEMPLATES {
        println!("  /{} - {}", template.name, template.description);
    }

    println!();
    println!("Install targets:");
    println!("  claude   user: ~/.claude/commands/*.md");
    println!("  claude   project: .claude/commands/*.md");
    println!("  opencode user: ~/.config/opencode/commands/*.md");
    println!("  opencode project: .opencode/commands/*.md");
    println!("  codex    user: ~/.agents/skills/spec0/SKILL.md + ~/.codex/prompts/*.md");
    println!("  codex    project: .agents/skills/spec0/SKILL.md");
    println!();
    println!("Examples:");
    println!("  tk spec0 install --agent all --scope user");
    println!("  tk spec0 install --agent all --scope project --dir .");
    println!("  tk spec0 print spec0-plan");
}

fn print_command(command: Option<&str>) -> Result<(), String> {
    match command {
        Some(name) => {
            let template = find_template(name)?;
            print!("{}", template.content);
        }
        None => {
            for (idx, template) in TEMPLATES.iter().enumerate() {
                if idx > 0 {
                    println!();
                }
                println!("--- {} ---", template.name);
                print!("{}", template.content);
            }
        }
    }
    Ok(())
}

fn find_template(name: &str) -> Result<&'static Template, String> {
    TEMPLATES
        .iter()
        .find(|template| template.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| {
            let names = TEMPLATES
                .iter()
                .map(|template| template.name)
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "unknown Spec0 command '{}'; expected one of: {}",
                name, names
            )
        })
}

fn install(
    agent: Spec0Agent,
    scope: Spec0Scope,
    dir: Option<&Path>,
    force: bool,
    dry_run: bool,
) -> Result<(), String> {
    let agents = selected_agents(agent);
    for selected in agents {
        let targets = targets_for(selected, scope, dir)?;
        for target in targets {
            write_target(&target, force, dry_run)?;
        }
    }

    if agent == Spec0Agent::Codex || agent == Spec0Agent::All {
        println!(
            "Codex note: custom prompts are deprecated; the installed skill is the durable path."
        );
    }

    Ok(())
}

fn selected_agents(agent: Spec0Agent) -> Vec<Spec0Agent> {
    match agent {
        Spec0Agent::All => vec![Spec0Agent::Claude, Spec0Agent::Codex, Spec0Agent::Opencode],
        other => vec![other],
    }
}

fn targets_for(
    agent: Spec0Agent,
    scope: Spec0Scope,
    dir: Option<&Path>,
) -> Result<Vec<InstallTarget>, String> {
    match agent {
        Spec0Agent::All => unreachable!("expanded before target resolution"),
        Spec0Agent::Claude => command_targets(agent_base(agent, scope, dir)?),
        Spec0Agent::Opencode => command_targets(agent_base(agent, scope, dir)?),
        Spec0Agent::Codex => codex_targets(scope, dir),
    }
}

fn agent_base(agent: Spec0Agent, scope: Spec0Scope, dir: Option<&Path>) -> Result<PathBuf, String> {
    let base = match (agent, scope) {
        (Spec0Agent::Claude, Spec0Scope::User) => home_dir()?.join(".claude").join("commands"),
        (Spec0Agent::Claude, Spec0Scope::Project) => {
            project_dir(dir)?.join(".claude").join("commands")
        }
        (Spec0Agent::Opencode, Spec0Scope::User) => home_dir()?
            .join(".config")
            .join("opencode")
            .join("commands"),
        (Spec0Agent::Opencode, Spec0Scope::Project) => {
            project_dir(dir)?.join(".opencode").join("commands")
        }
        _ => unreachable!("Codex uses codex_targets"),
    };
    Ok(base)
}

fn command_targets(base: PathBuf) -> Result<Vec<InstallTarget>, String> {
    Ok(TEMPLATES
        .iter()
        .map(|template| InstallTarget {
            path: base.join(format!("{}.md", template.name)),
            content: template.content,
        })
        .collect())
}

fn codex_targets(scope: Spec0Scope, dir: Option<&Path>) -> Result<Vec<InstallTarget>, String> {
    let mut targets = Vec::new();

    match scope {
        Spec0Scope::User => {
            let home = home_dir()?;
            targets.push(InstallTarget {
                path: home
                    .join(".agents")
                    .join("skills")
                    .join("spec0")
                    .join("SKILL.md"),
                content: CODEX_SKILL,
            });

            let prompts_dir = env::var_os("CODEX_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| home.join(".codex"))
                .join("prompts");
            targets.extend(TEMPLATES.iter().map(|template| InstallTarget {
                path: prompts_dir.join(format!("{}.md", template.name)),
                content: template.content,
            }));
        }
        Spec0Scope::Project => {
            targets.push(InstallTarget {
                path: project_dir(dir)?
                    .join(".agents")
                    .join("skills")
                    .join("spec0")
                    .join("SKILL.md"),
                content: CODEX_SKILL,
            });
        }
    }

    Ok(targets)
}

fn project_dir(dir: Option<&Path>) -> Result<PathBuf, String> {
    match dir {
        Some(path) => Ok(path.to_path_buf()),
        None => env::current_dir().map_err(|e| format!("failed to read current directory: {}", e)),
    }
}

fn home_dir() -> Result<PathBuf, String> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .ok_or_else(|| "HOME or USERPROFILE is required for user-scoped install".to_string())
}

fn write_target(target: &InstallTarget, force: bool, dry_run: bool) -> Result<(), String> {
    if dry_run {
        println!("would write {}", target.path.display());
        return Ok(());
    }

    if target.path.exists() && !force {
        println!("skip {} (exists; use --force)", target.path.display());
        return Ok(());
    }

    if let Some(parent) = target.path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create {}: {}", parent.display(), e))?;
    }

    fs::write(&target.path, target.content)
        .map_err(|e| format!("failed to write {}: {}", target.path.display(), e))?;
    println!("wrote {}", target.path.display());
    Ok(())
}
