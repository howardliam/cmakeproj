# cmakeproj
This is a personal tool to assist in creating cmake based C++ projects.

## Installation
```bash
git clone https://github.com/howardliam/cmakeproj
cd cmakeproj
cargo build --release
```

You'll then need to make cmakeproj binary available on path, for example:

```bash
ln -s cmakeproj/target/release/cmakeproj $HOME/scripts
```
