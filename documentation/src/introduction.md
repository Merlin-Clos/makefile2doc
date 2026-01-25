# Introduction

**Unlock the full potential of your Makefiles.**

`makefile2doc` is a CLI tool that turns your raw Makefiles into clear, categorized, and visual documentation.

## The Problem
Makefiles are powerful, but they often become "Black Boxes" as projects grow:
* **The "Bus Factor":** Often, only the author knows all the available commands.
* **Partial Onboarding:** New developers usually learn just the 2 or 3 commands needed to start, missing out on powerful utility scripts.
* **Cryptic Syntax:** Reading a raw Makefile requires mentally parsing bash syntax, which is inefficient.

## The Solution
Treat your Makefile as the **Single Source of Truth**.
By adding simple comments (The Convention), you generate a `MAKEFILE.md` that is always up-to-date with your code.

## Why use it?
* 🧠 **Kill the "Makefile Anxiety":** No more scrolling through 300 lines of obscure Bash at 2 AM just to find the right deploy command.
* 🚀 **Instant Onboarding:** Give new developers a "Cheat Sheet" that actually makes sense, letting them use the project before they fully understand its internals.
* 🤖 **AI Optimization:** Provides clean context for LLMs, saving tokens and preventing hallucinations (see [AI Context](./ai_context.md)).

## Why NOT use it?
* 🦀 You don't like Rust for some reason.
* 😨 You are afraid to re-open your Makefile because it currently works and you don't want to jinx it.