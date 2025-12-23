//! Package Manager for FlowLang
//!
//! Handles downloading and managing packages from Git repositories.
//! Packages are stored locally in .flowlang/pkg/<host>/<user>/<repo>

use crate::config::ProjectConfig;
use crate::error::FlowError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Parsed package specification from URL
#[derive(Debug, Clone)]
pub struct PackageSpec {
    pub host: String,      // e.g., "github.com"
    pub owner: String,     // e.g., "flowlang-exe"
    pub repo: String,      // e.g., "http"
    pub git_ref: String,   // branch, tag, or commit SHA
}

impl PackageSpec {
    /// Parse a package URL like "github.com/user/repo@ref"
    pub fn parse(url: &str) -> Result<Self, FlowError> {
        // Split by @ to get ref
        let (base, git_ref) = if let Some(idx) = url.find('@') {
            (&url[..idx], url[idx + 1..].to_string())
        } else {
            (url, "main".to_string()) // Default to main branch
        };

        // Split by / to get host/owner/repo
        let parts: Vec<&str> = base.split('/').collect();
        if parts.len() != 3 {
            return Err(FlowError::runtime(
                &format!("Invalid package URL '{}'. Expected format: host/owner/repo[@ref]", url),
                0, 0
            ));
        }

        Ok(PackageSpec {
            host: parts[0].to_string(),
            owner: parts[1].to_string(),
            repo: parts[2].to_string(),
            git_ref,
        })
    }

    /// Get the Git clone URL
    pub fn clone_url(&self) -> String {
        format!("https://{}/{}/{}.git", self.host, self.owner, self.repo)
    }

    /// Get local path relative to .flowlang/pkg/
    pub fn local_path(&self) -> PathBuf {
        PathBuf::from(&self.host)
            .join(&self.owner)
            .join(&self.repo)
    }
}

/// Package Manager handles downloading and resolving packages
pub struct PackageManager {
    project_root: PathBuf,
    pkg_dir: PathBuf,
}

impl PackageManager {
    /// Create a new PackageManager for the given project root
    pub fn new(project_root: PathBuf) -> Self {
        let pkg_dir = project_root.join(".flowlang").join("pkg");
        PackageManager { project_root, pkg_dir }
    }

    /// Get the package directory path
    pub fn pkg_dir(&self) -> &Path {
        &self.pkg_dir
    }

    /// Resolve a package alias to its local path
    pub fn resolve_package(&self, alias: &str, config: &ProjectConfig) -> Option<PathBuf> {
        let url = config.packages.get(alias)?;
        let spec = PackageSpec::parse(url).ok()?;
        let path = self.pkg_dir.join(spec.local_path());
        
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    /// Fetch a single package
    pub fn fetch_package(&self, spec: &PackageSpec) -> Result<PathBuf, FlowError> {
        let target_path = self.pkg_dir.join(spec.local_path());

        // Create parent directories
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                FlowError::runtime(&format!("Failed to create package directory: {}", e), 0, 0)
            })?;
        }

        // If already exists, try to update
        if target_path.exists() {
            return self.update_package(spec, &target_path);
        }

        // Clone the repository
        println!("📦 Downloading {}...", spec.clone_url());

        let repo = git2::Repository::clone(&spec.clone_url(), &target_path)
            .map_err(|e| FlowError::runtime(&format!("Failed to clone package: {}", e), 0, 0))?;

        // Checkout the specified ref
        self.checkout_ref(&repo, &spec.git_ref)?;

        // Validate package has config.flowlang.json
        self.validate_package(&target_path, spec)?;

        println!("✅ Installed {}/{}", spec.owner, spec.repo);
        Ok(target_path)
    }

    /// Update an existing package
    fn update_package(&self, spec: &PackageSpec, path: &Path) -> Result<PathBuf, FlowError> {
        println!("🔄 Updating {}/{}...", spec.owner, spec.repo);

        let repo = git2::Repository::open(path)
            .map_err(|e| FlowError::runtime(&format!("Failed to open package repo: {}", e), 0, 0))?;

        // Fetch updates
        let mut remote = repo.find_remote("origin")
            .map_err(|e| FlowError::runtime(&format!("Failed to find remote: {}", e), 0, 0))?;

        remote.fetch(&[&spec.git_ref], None, None)
            .map_err(|e| FlowError::runtime(&format!("Failed to fetch updates: {}", e), 0, 0))?;

        // Checkout the ref
        self.checkout_ref(&repo, &spec.git_ref)?;

        println!("✅ Updated {}/{}", spec.owner, spec.repo);
        Ok(path.to_path_buf())
    }

    /// Checkout a specific git ref (branch, tag, or commit)
    fn checkout_ref(&self, repo: &git2::Repository, git_ref: &str) -> Result<(), FlowError> {
        // Try as branch first
        let reference = if let Ok(branch) = repo.find_branch(&format!("origin/{}", git_ref), git2::BranchType::Remote) {
            branch.into_reference()
        } else if let Ok(reference) = repo.find_reference(&format!("refs/tags/{}", git_ref)) {
            reference
        } else if let Ok(oid) = git2::Oid::from_str(git_ref) {
            // Direct commit SHA
            let commit = repo.find_commit(oid)
                .map_err(|e| FlowError::runtime(&format!("Commit not found: {}", e), 0, 0))?;
            repo.set_head_detached(commit.id())
                .map_err(|e| FlowError::runtime(&format!("Failed to checkout commit: {}", e), 0, 0))?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                .map_err(|e| FlowError::runtime(&format!("Failed to checkout: {}", e), 0, 0))?;
            return Ok(());
        } else {
            return Err(FlowError::runtime(
                &format!("Git ref '{}' not found (tried branch, tag, and commit)", git_ref),
                0, 0
            ));
        };

        // Set HEAD to the reference
        repo.set_head(reference.name().unwrap())
            .map_err(|e| FlowError::runtime(&format!("Failed to set HEAD: {}", e), 0, 0))?;

        // Checkout
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .map_err(|e| FlowError::runtime(&format!("Failed to checkout: {}", e), 0, 0))?;

        Ok(())
    }

    /// Validate that a package has config.flowlang.json
    fn validate_package(&self, path: &Path, spec: &PackageSpec) -> Result<(), FlowError> {
        let config_path = path.join("config.flowlang.json");
        if !config_path.exists() {
            return Err(FlowError::runtime(
                &format!(
                    "Package {}/{} is not a valid FlowLang package (missing config.flowlang.json)",
                    spec.owner, spec.repo
                ),
                0, 0
            ));
        }
        Ok(())
    }

    /// Install all packages from config
    pub fn install_all(&self, config: &ProjectConfig) -> Result<HashMap<String, PathBuf>, FlowError> {
        let mut installed = HashMap::new();

        for (alias, url) in &config.packages {
            let spec = PackageSpec::parse(url)?;
            let path = self.fetch_package(&spec)?;
            installed.insert(alias.clone(), path);
        }

        Ok(installed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_url() {
        let spec = PackageSpec::parse("github.com/flowlang-exe/http@main").unwrap();
        assert_eq!(spec.host, "github.com");
        assert_eq!(spec.owner, "flowlang-exe");
        assert_eq!(spec.repo, "http");
        assert_eq!(spec.git_ref, "main");
    }

    #[test]
    fn test_parse_package_url_with_tag() {
        let spec = PackageSpec::parse("github.com/user/repo@v1.0.0").unwrap();
        assert_eq!(spec.git_ref, "v1.0.0");
    }

    #[test]
    fn test_parse_package_url_default_ref() {
        let spec = PackageSpec::parse("github.com/user/repo").unwrap();
        assert_eq!(spec.git_ref, "main");
    }
}