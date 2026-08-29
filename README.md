# makefile2doc

**Unlock the full potential of your Makefiles.**

`makefile2doc` is a zero-config CLI tool that turns your raw Makefiles into clear, categorized, and visual Markdown documentation.

---

## 📚 Documentation & Usage

Everything you need to know is in the official documentation:

👉 **[Read the Full Documentation](https://merlin-clos.github.io/makefile2doc/)**

* **[Installation Guide](https://merlin-clos.github.io/makefile2doc/installation.html)** (Binaries & Cargo)
* **[Real-World Example](https://merlin-clos.github.io/makefile2doc/example.html)** (See the generated output)
* **[The Convention](https://merlin-clos.github.io/makefile2doc/convention.html)** (How to comment your Makefile)

---

## 📥 Installation

You don't need Rust installed. Simply download the binary for your OS (Windows, macOS, Linux) from the Releases page:

**[Download Latest Release](https://github.com/Merlin-Clos/makefile2doc/releases/latest)**

---

## Development

The repository Makefile is the official interface for development commands.
It is documented by `makefile2doc` itself in [MAKEFILE.md](MAKEFILE.md).

```bash
make check-tools
make check
```

Use `make test-one TEST=module::tests::name` to run one exact unit test and
`make docs` after changing the Makefile annotations.

---

## 📄 License

This project is licensed under either of:

* [MIT License](LICENSE)
* [Apache License, Version 2.0](LICENSE)

at your option.
