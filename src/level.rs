use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Level {
    pub name: String,
    pub readme: String,
    pub options: Vec<String>,
    pub correct: Vec<String>,
}

impl Level {
    pub fn load_all(path: &str) -> Option<Vec<Level>> {
        let base_path = PathBuf::from(path);
        let mut out = Vec::new();

        let folders = fs::read_dir(&base_path)
            .ok()?
            .map(|x| x.unwrap())
            .filter(|x| x.path().is_dir())
            .collect::<Vec<DirEntry>>();

        for i in folders {
            let name = i.file_name().to_str()?.to_owned();
            let readme = fs::read_to_string(i.path().join("readme.md")).ok()?;
            let options = fs::read_to_string(i.path().join("options.txt"))
                .ok()?
                .lines()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let correct = fs::read_to_string(i.path().join("correct.txt"))
                .ok()?
                .lines()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();

            // Valadate Regex
            Regex::new(&correct.join(",")).unwrap();

            out.push(Level {
                name,
                readme,
                options,
                correct,
            });
        }

        Some(out)
    }
}
