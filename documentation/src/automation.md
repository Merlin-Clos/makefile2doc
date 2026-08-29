# Automation &amp; CI/CD

Documentation is misleading if it is not up-to-date.

Instead of remembering to run `makefile2doc` manually after every change, you should let your CI pipeline handle it.

There are two useful strategies:

- regenerate `MAKEFILE.md` and commit the result automatically;
- generate a temporary file and fail the CI job when the committed documentation is stale.

The first strategy is convenient when generated commits are acceptable. The second keeps CI read-only and requires contributors to commit the generated file with their Makefile change.

## GitHub Actions Example

Here is a ready-to-use workflow using [checkout](https://github.com/marketplace/actions/checkout) and the [git-auto-commit Action](https://github.com/marketplace/actions/git-auto-commit) (check links for latest versions).

Create `.github/workflows/makefile2doc.yml`:

```yaml
name: Update Makefile Doc

on:
  push:
    paths:
      - "Makefile" # Trigger only when the Makefile changes

permissions:
  contents: write # Required to push the new commit

jobs:
  update-doc:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - name: Install makefile2doc
        env:
          # Check the latest version: https://github.com/Merlin-Clos/makefile2doc/releases
          VERSION: v0.1.3
        run: |
          curl -L -o makefile2doc https://github.com/Merlin-Clos/makefile2doc/releases/download/${VERSION}/makefile2doc-linux-amd64
          chmod +x makefile2doc
          sudo mv makefile2doc /usr/local/bin/

      - name: Generate Documentation
        run: makefile2doc

      - name: Commit & Push changes
        uses: stefanzweifel/git-auto-commit-action@v7
        with:
          commit_message: "docs: auto-update MAKEFILE.md"
          file_pattern: MAKEFILE.md
```

## Blocking Drift Check

This repository uses the read-only strategy on every pull request. The workflow generates documentation with the binary from the current checkout, compares it with the committed `MAKEFILE.md`, and fails without modifying tracked files when they differ. Contributors fix the check by running `make docs` and committing the result.
