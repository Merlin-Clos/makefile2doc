# The Convention

To generate documentation, `makefile2doc` parses specific comments in your Makefile. We call this **"The Convention"**.

It is designed to be unobtrusive and readable, even without the tool.

## Core Rules

1.  **Description is Mandatory:** Any target *without* a description is considered **private** and is ignored.
2.  **Stateful Categories:** A category tag applies to all subsequent commands until a new one is defined.

## Supported Tags

### `## @description` (Required)
Describes what the command does.
* **Usage:** Must be placed immediately before the target.
* **Effect:** Adds the command to the documentation.
* **Multi-line support:** You can use `\n` or the `<br>` tag to force a line break in the generated Markdown tables.

```makefile
## @description Starts the server \n Warning: check your .env first!
start:
    ...
```

### `## @category` (Optional)
Groups commands into a section.

* **Usage:** Can be placed anywhere. It acts as a switch.
* **Default:** If omitted, commands go into a "General" category.

```makefile
## @category Database
# ... all commands below are now in "Database" ...
```

### `## @depends` (Optional)
Lists the dependencies of a command (what must run *before*).

* **Usage:** Comma-separated list of other make targets.
* **Effect:** Draws arrows in the generated **Workflow Graph** and lists them in the "Dependencies" column of the detailed tables.

```makefile
## @description Run database migrations
## @depends up, install
migrate:
    ...
```

### `## @env` (Optional)
Documents required environment variables.
* **Usage:** Comma-separated list of variables.
* **Effect:** Adds a "Required Variables" column in the details table.

```makefile
## @description Starts the server
## @env PORT, NODE_ENV
start:
    ...
```