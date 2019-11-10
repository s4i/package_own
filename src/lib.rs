use regex::Regex;
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

// $home/.cargo/registry/src/github.com-*/cargo-rls-install-{version}/
fn push_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(env!("CARGO_HOME"));
    path.push("registry");
    path.push("src");
    match github_folder(&path) {
        // $home/.cargo/registry/src/github.com-*/
        Ok(dir) => path.push(dir),
        Err(e) => println!("{:?}", e),
    }
    path.to_path_buf()
}

fn github_folder(path: &PathBuf) -> Result<String, String> {
    if path.is_dir() {
        let paths = fs::read_dir(path).unwrap();
        let re_get_github = Regex::new(r"github.com-\b.+").unwrap();

        let mut dirs: Vec<String> = Vec::new();
        for path in paths {
            dirs.push(path.unwrap().path().display().to_string());
        }

        if re_get_github.is_match(&dirs[0]) {
            return Ok(re_get_github.find(&dirs[0]).unwrap().as_str().to_owned());
        }
    }
    Err("Not found github.com-{random_number} directory".to_owned())
}
