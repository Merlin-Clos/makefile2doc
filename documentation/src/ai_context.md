# AI-Ready Documentation

`makefile2doc` bridges the gap between raw code and AI comprehension.

## 1. Context Window Efficiency
When you provide a raw Makefile to an LLM (ChatGPT, Claude, etc.), it has to parse implementation details like `docker compose` flags, `sed` commands, or complex environment variable expansions.

By providing the generated `MAKEFILE.md`, you offer a **high-level summary**:
* **Token Savings:** Only the intent and dependencies are sent, not the bash implementation.
* **Better Reasoning:** LLMs can understand the *Workflow Graph* (Mermaid) to see how commands relate, preventing it from suggesting commands in the wrong order.

## 2. Accuracy Enforcement
Using `makefile2doc` acts as a **Forcing Function** for your team:
* If the documentation is wrong, your AI will hallucinate.
* This incentivizes developers to keep `## @description` and `## @depends` tags accurate.

## 3. How to use with LLMs

To give your AI the full context, don't copy-paste files one by one.

👉 **[Click here for the Full Context (Print View)](print.html)**

Copy the content of that page and paste it into your AI. It contains the entire documentation in one go.
