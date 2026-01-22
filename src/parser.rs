use crate::model::MakefileDoc;

pub fn parse(content: &str) -> MakefileDoc {
    let _cleaned = clean_content(content);
    
    MakefileDoc { categories: vec![] }
}

pub fn clean_content(content: &str) -> Result<Vec<&str>, &'static str> {
    if content.trim().is_empty() {
        return Err("The Makefile is empty");
    }
    
    let lines = content
        .lines()
        .filter(|line| !line.is_empty())
        .collect();
    
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn clean_content_return_error_if_empty() {
        let input = "";
        let result = clean_content(input);
        
        assert_eq!(result, Err("The Makefile is empty"))
    }
    
    #[test]
    fn clean_content_return_error_if_only_spaces() {
        let input = "     ";
        let result = clean_content(input);
        
        assert_eq!(result, Err("The Makefile is empty"))
    }
    
    #[test]
    fn clean_content_return_error_if_line_break() {
        let input = "\n";
        let result = clean_content(input);
        
        assert_eq!(result, Err("The Makefile is empty"))
    }
    
    #[test]
    fn clean_content_return_error_if_indent() {
        let input = "\t";
        let result = clean_content(input);
        
        assert_eq!(result, Err("The Makefile is empty"))
    }
    
    #[test]
    fn clean_content_return_error_if_spaces_indent_line_break() {
        let input = " \t\n ";w
        let result = clean_content(input);
        
        assert_eq!(result, Err("The Makefile is empty"))
    }
}