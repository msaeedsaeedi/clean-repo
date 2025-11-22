use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions};
use std::path::{Path, PathBuf};

pub struct GitRepo {
    repo: Repository,
    workdir: PathBuf,
}

impl GitRepo {
    /// Open a git repository from the current directory or environment variables
    pub fn open() -> Result<Self> {
        let repo = Repository::open_from_env()
            .or_else(|_| Repository::discover("."))
            .context("Not inside a Git repository")?;

        let workdir = repo.workdir()
            .context("Repository has no working directory (bare repository?)")?
            .to_path_buf();

        Ok(GitRepo { repo, workdir })
    }

    /// Get the repository root directory
    pub fn root(&self) -> &Path {
        &self.workdir
    }

    /// Check if .gitignore exists in the repository root
    pub fn has_gitignore(&self) -> bool {
        self.workdir.join(".gitignore").exists()
    }

    /// Get all ignored files and directories
    pub fn get_ignored_files(&self) -> Result<Vec<PathBuf>> {
        let mut opts = StatusOptions::new();
        opts.include_ignored(true)
            .include_untracked(true)
            .recurse_ignored_dirs(true);

        let statuses = self.repo.statuses(Some(&mut opts))
            .context("Failed to get git status")?;

        let mut ignored_files = Vec::new();

        for entry in statuses.iter() {
            let status = entry.status();
            
            // We only want files that are ignored (not tracked, not in index)
            if status.contains(Status::IGNORED) {
                if let Some(path) = entry.path() {
                    let full_path = self.workdir.join(path);
                    ignored_files.push(full_path);
                }
            }
        }

        Ok(ignored_files)
    }
}


