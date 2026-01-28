# Automation &amp; CI/CD

Documentation is misleading if it is not up-to-date.

Instead of remembering to run `makefile2doc` manually after every change, you should let your CI pipeline handle it.
The logic is simple: **if `Makefile` is modified, regenerate `MAKEFILE.md` and commit the result.**

## GitHub Actions Example

Here is a ready-to-use workflow using [checkout](https://github.com/marketplace/actions/checkout) and the [git-auto-commit Action](https://github.com/marketplace/actions/git-auto-commit) (check links for latest versions).

Create `.github/workflows/update-docs.yml`:

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
          VERSION: v0.1.2
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