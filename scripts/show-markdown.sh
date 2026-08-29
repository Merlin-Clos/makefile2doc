#!/bin/sh

set -eu

MDCAT_URL="https://github.com/BIRSAx2/mdcat"

if [ "$#" -ne 1 ]; then
    printf 'Usage: %s <markdown-file>\n' "$0" >&2
    exit 2
fi

markdown_file=$1

if [ ! -f "$markdown_file" ]; then
    printf 'Markdown file not found: %s\n' "$markdown_file" >&2
    exit 1
fi

if command -v mdcat >/dev/null 2>&1; then
    exec mdcat "$markdown_file"
fi

printf 'Tip: install mdcat for rendered Markdown: %s\n\n' "$MDCAT_URL"

if command -v cat >/dev/null 2>&1; then
    exec cat "$markdown_file"
fi

if command -v pwsh >/dev/null 2>&1; then
    MARKDOWN_FILE=$markdown_file exec pwsh -NoProfile \
        -Command 'Get-Content -Raw -LiteralPath $env:MARKDOWN_FILE'
fi

if command -v powershell.exe >/dev/null 2>&1; then
    MARKDOWN_FILE=$markdown_file exec powershell.exe -NoProfile \
        -Command 'Get-Content -Raw -LiteralPath $env:MARKDOWN_FILE'
fi

printf 'Unable to display %s. Install mdcat: %s\n' \
    "$markdown_file" "$MDCAT_URL" >&2
exit 1
