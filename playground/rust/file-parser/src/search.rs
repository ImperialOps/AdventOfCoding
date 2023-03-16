
#[derive(Debug)]
pub struct Search {
    directory: String,
    recursive: bool,
    files: Vec<String>,
}

impl Search {
    pub fn new(directory: String, recursive: bool) -> Self {
        Self {
            directory,
            recursive,
            files: Vec::new(),
        }
    }

    pub fn directory(&self) -> &String {
        &self.directory
    }

    pub fn recursive(&self) -> &bool {
        &self.recursive
    }

    pub fn files_mut(&mut self) -> &mut Vec<String> {
        &mut self.files
    }
}
