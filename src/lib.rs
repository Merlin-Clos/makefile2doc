pub fn search<'a>(contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() { // Faire un test qui vérifie si le fichier est vide
        let contents = "\
test           
";
    
    assert_eq!(vec!["test"], search(contents));
    }
}