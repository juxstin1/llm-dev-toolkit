# Ticket Drafts

These are GitHub issue drafts for bugs found during the repo deep dive.

The GitHub CLI is installed, but `gh auth status` currently reports an invalid token for `github.com` user `juxstin1`. After re-authenticating with GitHub, these can be created as issues with:

```powershell
gh issue create --title "`tree -a` includes `.git` despite git-aware contract" --body-file tickets/001-tree-a-includes-git.md
gh issue create --title "`stats -d` overcounts files for directories" --body-file tickets/002-stats-directory-overcounts-files.md
gh issue create --title "`info -f` likely misclassifies symlinks" --body-file tickets/003-info-symlink-misclassified.md
gh issue create --title "`checksum` exits successfully for unsupported algorithms" --body-file tickets/004-checksum-invalid-algorithm-exits-zero.md
gh issue create --title "Validate enum-like CLI options instead of silently accepting invalid values" --body-file tickets/005-validate-enum-like-cli-options.md
gh issue create --title "`search -e .rs` returns no matches while `search -e rs` works" --body-file tickets/006-search-extension-filter-leading-dot.md
gh issue create --title "`ltd -L` shows hidden entries by default unlike `tree -L`" --body-file tickets/007-ltd-shows-hidden-entries-by-default.md
```
