use clap::Parser;

/// A command-line tool that deletes files and directories ignored by your project's .gitignore
#[derive(Parser, Debug)]
#[command(name = "clean-repo")]
#[command(version, about, long_about = None)]
#[command(after_help = "For detailed documentation, see: man clean-repo")]
pub struct Cli {
    /// Execute deletion (default: dry-run mode - shows what would be deleted)
    #[arg(short = 'x', long = "execute")]
    pub execute: bool,

    /// Exclude pattern (can be specified multiple times)
    /// 
    /// Examples: -i "*.log" -i "node_modules" -i "target/"
    #[arg(short = 'i', long = "ignore", value_name = "PATTERN")]
    pub ignore_patterns: Vec<String>,

    /// Verbose mode - show detailed information
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Quiet mode - suppress all output except errors
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }

    pub fn is_dry_run(&self) -> bool {
        !self.execute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_dry_run() {
        let cli = Cli {
            execute: false,
            ignore_patterns: vec![],
            verbose: false,
            quiet: false,
        };
        assert!(cli.is_dry_run());
    }

    #[test]
    fn test_execute_mode() {
        let cli = Cli {
            execute: true,
            ignore_patterns: vec![],
            verbose: false,
            quiet: false,
        };
        assert!(!cli.is_dry_run());
    }
}
