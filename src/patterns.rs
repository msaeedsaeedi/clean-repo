use anyhow::{Context, Result};
use glob::Pattern;
use std::path::Path;

pub struct PatternMatcher {
    patterns: Vec<Pattern>,
}

impl PatternMatcher {
    /// Create a new pattern matcher from a list of glob patterns
    pub fn new(pattern_strings: &[String]) -> Result<Self> {
        let mut patterns = Vec::new();

        for pattern_str in pattern_strings {
            let pattern = Pattern::new(pattern_str)
                .with_context(|| format!("Invalid glob pattern: '{}'", pattern_str))?;
            patterns.push(pattern);
        }

        Ok(PatternMatcher { patterns })
    }

    /// Check if a path matches any of the exclude patterns
    pub fn matches(&self, path: &Path) -> bool {
        if self.patterns.is_empty() {
            return false;
        }

        let path_str = path.to_string_lossy();

        for pattern in &self.patterns {
            // Match against the full path
            if pattern.matches(&path_str) {
                return true;
            }

            // Also match against just the filename
            if let Some(filename) = path.file_name() {
                if pattern.matches(&filename.to_string_lossy()) {
                    return true;
                }
            }

            // Match against relative path components
            for component in path.components() {
                if pattern.matches(&component.as_os_str().to_string_lossy()) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_empty_patterns_no_match() {
        let matcher = PatternMatcher::new(&[]).unwrap();
        assert!(!matcher.matches(Path::new("test.log")));
    }

    #[test]
    fn test_simple_pattern_match() {
        let patterns = vec!["*.log".to_string()];
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        assert!(matcher.matches(Path::new("test.log")));
        assert!(matcher.matches(Path::new("error.log")));
        assert!(!matcher.matches(Path::new("test.txt")));
    }

    #[test]
    fn test_multiple_patterns() {
        let patterns = vec!["*.log".to_string(), "*.tmp".to_string()];
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        assert!(matcher.matches(Path::new("test.log")));
        assert!(matcher.matches(Path::new("cache.tmp")));
        assert!(!matcher.matches(Path::new("test.txt")));
    }

    #[test]
    fn test_directory_pattern() {
        let patterns = vec!["node_modules".to_string()];
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        assert!(matcher.matches(Path::new("node_modules")));
        assert!(matcher.matches(Path::new("project/node_modules")));
    }

    #[test]
    fn test_path_pattern() {
        let patterns = vec!["target/*".to_string()];
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        assert!(matcher.matches(Path::new("target/debug")));
        assert!(matcher.matches(Path::new("target/release")));
    }

    #[test]
    fn test_invalid_pattern() {
        let patterns = vec!["[".to_string()]; // Invalid glob pattern
        let result = PatternMatcher::new(&patterns);
        assert!(result.is_err());
    }
}
