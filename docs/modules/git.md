# std:git ⚡

Git repository operations.

## Import

```flowlang
circle git from "std:git"
```

## Functions

### `clone(url: Silk, dest: Silk) -> Pulse`
Clone a repository.

```flowlang
git.clone("https://github.com/user/repo.git", "./repo")
```

### `pull(repo_path: Silk) -> Pulse`
Pull latest changes (fetch + merge).

```flowlang
git.pull("./repo")
```

### `checkout(repo_path: Silk, ref: Silk) -> Pulse`
Checkout a branch, tag, or commit.

```flowlang
git.checkout("./repo", "main")
git.checkout("./repo", "v1.0.0")
```

### `status(repo_path: Silk) -> Relic`
Get repository status.

```flowlang
let status = git.status("./repo")
shout(status.branch) -- "main"
shout(status.dirty)  -- both! if modified
shout(status.files)  -- ["modified.txt"]
```

### `init(path: Silk) -> Pulse`
Initialize a new repository.

```flowlang
git.init("./new-repo")
```
