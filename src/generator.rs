use crate::model::MakefileDoc;

pub fn generate(doc: &MakefileDoc) -> String {
    let mut md = String::new();

    md.push_str("# Makefile Documentation\n\n");
    md.push_str(&generate_cheat_sheet(doc));

    md.push_str("\n---\n\n");

    md.push_str(&generate_workflow_graph(doc));

    md.push_str("---\n\n");

    md.push_str(&generate_section_details(doc));
    md
}

fn generate_cheat_sheet(doc: &MakefileDoc) -> String {
    let mut section = String::new();

    section.push_str("## Cheat Sheet\n");
    section.push_str("| Command | Category | Description |\n");
    section.push_str("| :--- | :--- | :--- |\n");

    for cat in &doc.categories {
        for cmd in &cat.commands {
            let link = cat.name.to_lowercase();
            let desc = cmd.description.replace("\\n", "<br>");

            section.push_str(&format!(
                "| [`make {}`](#{}) | {} | {} |\n",
                cmd.name, link, cat.name, desc
            ));
        }
    }
    section
}

fn generate_workflow_graph(doc: &MakefileDoc) -> String {
    let mut section = String::new();
    section.push_str("## Workflow Graph\n");
    section.push_str("```mermaid\n");
    section.push_str("graph TD;\n");

    for cat in &doc.categories {
        for cmd in &cat.commands {
            for dep in &cmd.dependencies {
                section.push_str(&format!("    {} --> {};\n", cmd.name, dep));
            }
        }
    }
    section.push_str("```\n\n");
    section
}

fn generate_section_details(doc: &MakefileDoc) -> String {
    let mut section = String::new();

    section.push_str("## Section Details\n");

    for cat in &doc.categories {
        section.push_str(&format!("\n### {}\n", cat.name));
        section.push_str("| Command | Description | Dependencies | Required Variables |\n");
        section.push_str("| :--- | :--- | :--- | :--- |\n");

        for cmd in &cat.commands {
            let desc = format_description(&cmd.description);
            let deps = format_list(&cmd.dependencies);
            let envs = format_list(&cmd.env);

            section.push_str(&format!(
                "| `make {}` | {} | {} | {} |\n",
                cmd.name, desc, deps, envs
            ));
        }
    }

    section
}

fn format_description(desc: &str) -> String {
    if desc.is_empty() {
        "-".to_string()
    } else {
        desc.replace("\\n", "<br>")
    }
}

fn format_list(items: &[String]) -> String {
    if items.is_empty() {
        "-".to_string()
    } else {
        items
            .iter()
            .map(|i| format!("`{}`", i))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
