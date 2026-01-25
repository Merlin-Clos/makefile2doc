# Usage

## 1. The Command Line Interface

The tool is designed to be smart about paths. You don't always need to specify arguments.

### Basic Usage
Run the command in the folder containing your Makefile:

```bash
makefile2doc
```

* **Input:** Looks for `Makefile` in the current directory.
* **Output:** Generates `MAKEFILE.md` in the same directory.

### Custom Input (`-i`)
If your Makefile is in a subfolder:

```bash
makefile2doc -i backend/Makefile
```

* **Input:** Reads `backend/Makefile`.
* **Output:** Automatically targets `backend/MAKEFILE.md`.

### Custom Output (`-o`)
If you want to save the documentation somewhere specific:

```bash
makefile2doc -i Makefile -o docs/development.md
```

* **Input:** Reads `Makefile` from the current directory.
* **Output:** Generates the documentation in `docs/development.md`.

## 2. Important Note: Output is Empty?
`makefile2doc` only documents targets that have a `## @description` tag. If your Makefile doesn't follow [The Convention](./convention.md), the generated file will be empty.

Try adding a description to one of your targets and run the tool again:
```makefile
## @description A simple hello world
hello:
	echo "hello"
```