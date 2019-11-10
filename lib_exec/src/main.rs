use package_own::{package_own, parent_folder};

fn main() {
    println!("{:?}", package_own!());
    println!("{:?}", package_own!("src\\main.rs"));
    println!("{:?}", parent_folder(""));
    println!("{:?}", parent_folder("src/main.rs"));
}
