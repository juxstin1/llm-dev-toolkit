use clap::{Args, CommandFactory, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CompletionShell {
    #[value(name = "powershell")]
    PowerShell,
    Bash,
    Zsh,
    Fish,
}

#[derive(Args)]
pub struct CompletionsArgs {
    #[arg(value_enum)]
    shell: CompletionShell,
}

pub fn run(args: &CompletionsArgs) -> Result<(), String> {
    let shell = match args.shell {
        CompletionShell::PowerShell => clap_complete::Shell::PowerShell,
        CompletionShell::Bash => clap_complete::Shell::Bash,
        CompletionShell::Zsh => clap_complete::Shell::Zsh,
        CompletionShell::Fish => clap_complete::Shell::Fish,
    };
    let mut command = crate::Cli::command();
    clap_complete::generate(shell, &mut command, "tk", &mut std::io::stdout());
    Ok(())
}
