# Purpose

- When installed from "cargo install", find the folder where the installation package is located.

## Usage

```rust
use package_own::{package_own, parent_folder};

fn main() {
    println!("{:?}", package_own!());
}
```

## Output

### Return type: std::path::PathBuf

```bash
 # Your OS: Windows

 > "{%USERPROFILE%}\\.cargo\\registry\\src\\github.com-{random number}\\
    {Your package. Case of this library<package_own-(version)>}\\"

 # Your OS: Linux

 $ "{$home}/.cargo/registry/src/github.com-{random number}/
    {Your package. Case of this library<package_own-(version)>}/"
```
