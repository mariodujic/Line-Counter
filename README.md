<h1 align="center">Line Counter</h1>
<p align="center">Reads lines of code in a given directory, and outputs line count per file type in a table form.</p>

## Preview

<img src="/preview/preview.gif"/>

## Install

```
clone https://github.com/mariodujic/Line-Counter
cd Line-Counter
cargo run <directory-path>
```

## Config

Directories and file types can be excluded from the table by editing root `config.toml` file.

##### Example

To exclude `build` and `idea` directory, and `txt` file type, `config.toml` should contain:
```toml
[excluded]
dir = ['build', '.idea']
ext = ['txt']
```