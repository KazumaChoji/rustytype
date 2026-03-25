

<h1 align=center>
rustytype
</h1>

<p align=center>
a fork of rustytype. still a work in progress. here is rustytype's readme until i write my own:
</p>

<p align=center>
<a href="https://crates.io/crates/rustytype"><img alt="Crates.io badge" src="https://img.shields.io/crates/v/rustytype"></a>
<a href="https://github.com/Samyak2/rustytype/actions/workflows/lints-and-checks.yml"><img src="https://github.com/Samyak2/rustytype/actions/workflows/lints-and-checks.yml/badge.svg" alt="Lints and checks badge" /></a>
<a href="https://docs.rs/rustytype/latest/rustytype/"><img alt="docs.rs badge" src="https://img.shields.io/docsrs/rustytype"></a>
</p>

<p align=center>
<img src=https://raw.githubusercontent.com/Samyak2/rustytype/main/images/rustytype.gif>
</p>

# Usage

## Install

### From GitHub

Go to the [latest release](https://github.com/Samyak2/rustytype/releases/latest), scroll down to "Assets" and download the correct file for your platform (`.zip` in case of Mac OS, `.tar.gz` in case of Linux). Unzip the file and run the `rustytype` binary inside.

### From Cargo

Alternatively, if you have the `cargo` tool (part of the Rust toolchain) installed on your system, you can use:

```
cargo install rustytype
```

## Run typing test

rustytype looks best on a nice terminal (such as Alacritty) with color and style support.

If installed through GitHub, run the binary (found inside the zip/tar.gz file after extracting) directly:
```
./rustytype
```

If installed through `cargo`, use:
```
rustytype
```

## Keyboard shortcuts

See `rustytype --help` for a list of keyboard shortcuts (the list can also be found [here](https://github.com/Samyak2/rustytype/blob/main/src/config.rs#L10)).

## Show less or more text

To change the number of words shown in each test, use the `-n` flag (default: 30):

```
rustytype -n 10
```

```
rustytype -n 100
```

## Use a different word list

By default, a list of top 250 English words (`top250`) is used and random words are selected from it. See `rustytype -h` for a list of available built-in word lists.

To use the OS provided word list instead, use:
```
rustytype -w os
```
Note: the OS word list varies a lot from system to system and usually has more than 100,000 words. This can lead to difficult and esoteric words appearing in the test, reducing your typing speed.

You can provide your own word list too (Note: the word list must meet [these assumptions](https://docs.rs/rustytype/latest/rustytype/textgen/struct.RawWordSelector.html#assumptions)):
```
rustytype -f /path/to/word/list
```

## Add punctuation to test

By default, only lowercase words are shown. To add punctuation and sentence case, use the `-p` flag:

```
rustytype -p
```

# Platform support

- rustytype was only tested on Linux and Mac OS. If you find any problems, please [open an issue](https://github.com/Samyak2/rustytype/issues).
- Windows is not supported yet. Follow [this issue](https://github.com/Samyak2/rustytype/issues/14) for updates. It should work on WSL though.

# License

MIT
