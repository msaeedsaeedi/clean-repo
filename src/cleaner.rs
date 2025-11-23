use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;

use crate::cli::Cli;
use crate::git::GitRepo;
use crate::patterns::PatternMatcher;

pub struct CleanResult {
    pub deleted: Vec<PathBuf>,
    pub excluded: Vec<PathBuf>,
    pub failed: Vec<(PathBuf, String)>,
}

impl CleanResult {
    fn new() -> Self {
        CleanResult {
            deleted: Vec::new(),
            excluded: Vec::new(),
            failed: Vec::new(),
        }
    }
}

pub struct Cleaner<'a> {
    cli: &'a Cli,
    repo: GitRepo,
    matcher: PatternMatcher,
}

impl<'a> Cleaner<'a> {
    pub fn new(cli: &'a Cli) -> Result<Self> {
        let repo = GitRepo::open()?;

        if !repo.has_gitignore() {
            anyhow::bail!(
                ".gitignore not found in repository root ({})",
                repo.root().display()
            );
        }

        let matcher = PatternMatcher::new(&cli.ignore_patterns)?;

        Ok(Cleaner { cli, repo, matcher })
    }

    fn log_info(&self, msg: &str) {
        if !self.cli.quiet {
            println!("{} {}", "ℹ".blue(), msg);
        }
    }

    fn log_ok(&self, msg: &str) {
        if !self.cli.quiet {
            println!("{} {}", "✓".green(), msg);
        }
    }

    fn log_warn(&self, msg: &str) {
        if !self.cli.quiet {
            println!("{} {}", "⚠".yellow(), msg);
        }
    }

    fn log_error(&self, msg: &str) {
        eprintln!("{} {}", "✗".red(), msg);
    }

    fn log_debug(&self, msg: &str) {
        if self.cli.verbose {
            println!("{} {}", "DEBUG:".cyan(), msg);
        }
    }

    pub fn run(&self) -> Result<CleanResult> {
        // Change to repository root
        std::env::set_current_dir(self.repo.root())
            .context("Failed to change to repository root")?;

        self.log_debug(&format!("Changed to repository root: {}", self.repo.root().display()));

        // Get all ignored files
        let ignored_files = self.repo.get_ignored_files()
            .context("Failed to get ignored files")?;

        let total = ignored_files.len();
        self.log_debug(&format!("Ignored files found: {}", total));

        if total == 0 {
            self.log_ok("No ignored files found — repository is clean.");
            return Ok(CleanResult::new());
        }

        // Filter files based on exclude patterns
        let mut result = CleanResult::new();
        
        for file in ignored_files {
            // Make path relative to repo root for display
            let relative_path = file.strip_prefix(self.repo.root())
                .unwrap_or(&file)
                .to_path_buf();

            if self.matcher.matches(&relative_path) {
                result.excluded.push(relative_path.clone());
                self.log_debug(&format!("Excluded by pattern: {}", relative_path.display()));
            } else {
                result.deleted.push(file);
            }
        }

        if result.deleted.is_empty() {
            self.log_ok("No (non-excluded) ignored files to delete.");
            if !result.excluded.is_empty() {
                self.log_info(&format!("Excluded {} files.", result.excluded.len()));
            }
            return Ok(result);
        }

        // Dry run or execute
        if self.cli.is_dry_run() {
            self.display_dry_run(&result);
        } else {
            self.execute_deletion(&mut result)?;
        }

        Ok(result)
    }

    fn display_dry_run(&self, result: &CleanResult) {
        self.log_info("DRY RUN — the following files/dirs would be removed:");
        
        for path in &result.deleted {
            let relative = path.strip_prefix(self.repo.root()).unwrap_or(path);
            println!("  {}", relative.display().to_string().cyan());
        }

        if !result.excluded.is_empty() {
            println!();
            self.log_info(&format!("Excluded files ({}):", result.excluded.len()));
            for path in &result.excluded {
                println!("  {}", path.display().to_string().yellow());
            }
        }

        println!();
        self.log_warn("Run with -x to actually delete the listed files.");
    }

    fn execute_deletion(&self, result: &mut CleanResult) -> Result<()> {
        let total = result.deleted.len();
        
        let progress = if self.cli.quiet || total < 10 {
            None
        } else {
            let pb = ProgressBar::new(total as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );
            Some(pb)
        };

        let files_to_delete = result.deleted.clone();
        result.deleted.clear();

        for path in files_to_delete {
            let relative = path.strip_prefix(self.repo.root()).unwrap_or(&path).to_path_buf();
            
            if let Some(ref pb) = progress {
                pb.set_message(format!("{}", relative.display()));
            }

            let path_for_error = path.clone();
            let deletion_result = if path.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            };
            
            match deletion_result {
                Ok(_) => {
                    result.deleted.push(path);
                    self.log_debug(&format!("Deleted: {}", relative.display()));
                }
                Err(e) => {
                    result.failed.push((path_for_error, e.to_string()));
                    self.log_error(&format!("Failed to delete {}: {}", relative.display(), e));
                }
            }

            if let Some(ref pb) = progress {
                pb.inc(1);
            }
        }

        if let Some(pb) = progress {
            pb.finish_with_message("Done");
        }

        if !result.failed.is_empty() {
            self.log_warn(&format!(
                "Deletion completed with {} failure(s).",
                result.failed.len()
            ));
        } else {
            self.log_ok(&format!("Successfully deleted {} item(s).", result.deleted.len()));
        }

        Ok(())
    }
}
