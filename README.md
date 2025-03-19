# cmakeproj
This is a personal tool to assist in creating cmake based C++ projects.

## Installation
```bash
git clone https://github.com/howardliam/cmakeproj
cd cmakeproj
cargo build --release
```

You'll then need to make cmakeproj binary available on path, for example (I am yet to make this a crate):

```bash
sudo ln -s /home/liam/Programming/cmakeproj/target/release/cmakeproj /usr/bin/cmakeproj
```

## Usage
To see commands and parameters:
```bash
cmakeproj help
cmakeproj <COMMAND> --help
```
