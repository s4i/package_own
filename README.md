# Purpose

- When installed from "cargo install", find the folder where the installation package is located.

## Usage1 (Search own local package)

```rust
use package_own::parent_folder;

fn main() {
    println!("{:?}", parent_folder(""));
}
```

## Output1 (Return type: std::path::PathBuf)

```bash
 # Your OS: Windows

 > "{%USERPROFILE%}\\.cargo\\registry\\src\\github.com-{random number}\\
    {Your package. Case of this library<package_own-(version)>}\\"

 # Your OS: Linux

 $ "{$home}/.cargo/registry/src/github.com-{random number}/
    {Your package. Case of this library<package_own-(version)>}/"
```

## Usage2 (Add an arbitrary path to the searched package path)

```rust
use package_own::parent_folder;

fn main() {
    println!("{:?}", parent_folder("src/main.rs"));
}
```

## Output2 (Return type: std::path::PathBuf)

```bash
 # Your OS: Windows

 > "{%USERPROFILE%}\\.cargo\\registry\\src\\github.com-{random number}\\
    {Your package. Case of this library<package_own-(version)>}\\src\\main.rs"

 # Your OS: Linux

 $ "{$home}/.cargo/registry/src/github.com-{random number}/
    {Your package. Case of this library<package_own-(version)>}/src/main.rs"
```
