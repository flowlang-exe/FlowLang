//! std:git - Git operations module using git2
//!
//! Provides Git repository operations powered by libgit2.

use crate::types::{NativeFn, Value};
use crate::error::FlowError;
use std::collections::HashMap;
use std::sync::Arc;

/// Load the git module
pub fn load_git_module() -> Vec<(&'static str, Value)> {
    vec![
        ("clone", Value::NativeFunction(NativeFn::new(git_clone))),
        ("pull", Value::NativeFunction(NativeFn::new(git_pull))),
        ("checkout", Value::NativeFunction(NativeFn::new(git_checkout))),
        ("status", Value::NativeFunction(NativeFn::new(git_status))),
        ("init", Value::NativeFunction(NativeFn::new(git_init))),
    ]
}

/// Clone a repository
/// git.clone(url, dest) -> Pulse
fn git_clone(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 {
        return Err(FlowError::runtime("git.clone() requires (url, destination)", 0, 0));
    }

    let url = args[0].to_string();
    let dest = args[1].to_string();

    match git2::Repository::clone(&url, &dest) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(FlowError::runtime(&format!("git clone failed: {}", e), 0, 0)),
    }
}

/// Pull latest changes (fetch + merge)
/// git.pull(repo_path) -> Pulse
fn git_pull(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("git.pull() requires repository path", 0, 0));
    }

    let repo_path = args[0].to_string();
    
    let repo = git2::Repository::open(&repo_path)
        .map_err(|e| FlowError::runtime(&format!("Failed to open repo: {}", e), 0, 0))?;

    // Fetch from origin
    let mut remote = repo.find_remote("origin")
        .map_err(|e| FlowError::runtime(&format!("No origin remote: {}", e), 0, 0))?;

    remote.fetch(&["HEAD"], None, None)
        .map_err(|e| FlowError::runtime(&format!("Fetch failed: {}", e), 0, 0))?;

    Ok(Value::Boolean(true))
}

/// Checkout a branch, tag, or commit
/// git.checkout(repo_path, ref) -> Pulse
fn git_checkout(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 {
        return Err(FlowError::runtime("git.checkout() requires (repo_path, ref)", 0, 0));
    }

    let repo_path = args[0].to_string();
    let git_ref = args[1].to_string();

    let repo = git2::Repository::open(&repo_path)
        .map_err(|e| FlowError::runtime(&format!("Failed to open repo: {}", e), 0, 0))?;

    // Try as branch first
    if let Ok(branch) = repo.find_branch(&format!("origin/{}", git_ref), git2::BranchType::Remote) {
        let reference = branch.into_reference();
        repo.set_head(reference.name().unwrap())
            .map_err(|e| FlowError::runtime(&format!("Failed to set HEAD: {}", e), 0, 0))?;
    } else if let Ok(reference) = repo.find_reference(&format!("refs/tags/{}", git_ref)) {
        repo.set_head(reference.name().unwrap())
            .map_err(|e| FlowError::runtime(&format!("Failed to set HEAD: {}", e), 0, 0))?;
    } else if let Ok(oid) = git2::Oid::from_str(&git_ref) {
        let commit = repo.find_commit(oid)
            .map_err(|e| FlowError::runtime(&format!("Commit not found: {}", e), 0, 0))?;
        repo.set_head_detached(commit.id())
            .map_err(|e| FlowError::runtime(&format!("Failed to checkout: {}", e), 0, 0))?;
    } else {
        return Err(FlowError::runtime(&format!("Ref '{}' not found", git_ref), 0, 0));
    }

    // Checkout files
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
        .map_err(|e| FlowError::runtime(&format!("Checkout failed: {}", e), 0, 0))?;

    Ok(Value::Boolean(true))
}

/// Get repository status
/// git.status(repo_path) -> Relic {branch, dirty, files}
fn git_status(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("git.status() requires repository path", 0, 0));
    }

    let repo_path = args[0].to_string();

    let repo = git2::Repository::open(&repo_path)
        .map_err(|e| FlowError::runtime(&format!("Failed to open repo: {}", e), 0, 0))?;

    // Get current branch
    let head = repo.head().ok();
    let branch = head.as_ref()
        .and_then(|h| h.shorthand())
        .unwrap_or("HEAD")
        .to_string();

    // Check for changes
    let statuses = repo.statuses(None)
        .map_err(|e| FlowError::runtime(&format!("Status failed: {}", e), 0, 0))?;

    let dirty = !statuses.is_empty();
    
    let mut files: Vec<Value> = Vec::new();
    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            files.push(Value::String(Arc::new(path.to_string())));
        }
    }

    let mut result = HashMap::new();
    result.insert("branch".to_string(), Value::String(Arc::new(branch)));
    result.insert("dirty".to_string(), Value::Boolean(dirty));
    result.insert("files".to_string(), Value::Array(Arc::new(files)));

    Ok(Value::Relic(Arc::new(result)))
}

/// Initialize a new repository
/// git.init(path) -> Pulse
fn git_init(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("git.init() requires path", 0, 0));
    }

    let path = args[0].to_string();

    match git2::Repository::init(&path) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(FlowError::runtime(&format!("git init failed: {}", e), 0, 0)),
    }
}
