use std::fs;
use std::path::PathBuf;
use std::result::Result;

#[macro_export]
macro_rules! package_own {
    () => {
        parent_folder("")
    };
    ($fmt:expr) => {
        parent_folder($fmt)
    };
}

pub fn parent_folder(file_name: &str) -> PathBuf {
    let mut own_path = PathBuf::new();
    own_path.push(push_path());
    own_path.push(format!(
        "{}{}{}",
        env!("CARGO_PKG_NAME"),
        '-',
        env!("CARGO_PKG_VERSION")
    ));
    if !file_name.is_empty() {
        let mut f = file_name.replace(r"\\", r"\");
        f = f.replace(r"\", "/");
        let s: Vec<_> = f.split('/').collect();
        let tail_path: PathBuf = s.iter().collect();
        own_path.push(tail_path);
    }
    own_path
}

fn push_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(env!("CARGO_HOME"));
    path.push("registry");
    path.push("src");
    match github_folder(&path) {
        Ok(dir) => path.push(dir),
        Err(e) => println!("{:?}", e),
    }
    path.to_path_buf()
}

fn github_folder(path: &PathBuf) -> Result<String, String> {
    if path.is_dir() {
        let paths = fs::read_dir(path).unwrap();

        let mut dirs: Vec<String> = Vec::new();
        for path in paths {
            dirs.push(path.unwrap().path().display().to_string());
        }

        let s: Vec<_> = if cfg!(target_os = "windows") {
            dirs[0].split('\\').collect()
        } else {
            dirs[0].split('/').collect()
        };

        if s[s.len() - 1].starts_with("github.com-") {
            return Ok(s[s.len() - 1].to_owned());
        }
    }
    Err("Not found github.com-{random_number} directory".to_owned())
}
