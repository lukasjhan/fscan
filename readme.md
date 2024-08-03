# FS-analyzer

This is a simple tool to analyze the filesystem.

## Installation

```bash
cargo install fscan
```

## Usage

```bash
USAGE:
    fscan [FLAGS] [OPTIONS] [DIRECTORY]

FLAGS:
    -s               Follow any symbolic links encountered
    -h, --help       Prints help information
    -r               Display raw size in bytes
    -V, --version    Prints version information

OPTIONS:
    -d <max-depth>        Maximal directory depth to recurse, or -1 for infinite [default: 5]

ARGS:
    <DIRECTORY>    Directory to list [default: .]
```

## Example

```bash
fcsan . -d 2
```

```bash
.                                      219.9  MB  (D)
├── target                             219.8  MB  (D)
│   ├── debug                          219.8  MB  (D)
│   │   ├── deps                       141.3  MB  (D)
│   │   ├── build                      24.9   MB  (D)
│   │   ├── incremental                23.6   MB  (D)
│   │   ├── fsan                       14.4   MB
│   │   ├── fs-analyzer                13.9   MB
│   │   ├── libfs_analyzer.rlib        857.4  KB
│   │   ├── libfsan.rlib               849.2  KB
│   │   ├── .fingerprint               70.8   KB  (D)
│   │   ├── fsan.d                     209    B
│   │   ├── fs-analyzer.d              127    B
│   │   ├── libfs_analyzer.d           97     B
│   │   ├── libfsan.d                  90     B
│   │   ├── examples                   0      B   (D)
│   │   └── .cargo-lock                0      B
│   ├── .rustc_info.json               1.9    KB
│   └── CACHEDIR.TAG                   177    B
├── .git                               36.1   KB  (D)
│   ├── hooks                          22.9   KB  (D)
│   │   ├── pre-rebase.sample          4.8    KB
│   │   ├── fsmonitor-watchman.sample  4.5    KB
│   │   ├── update.sample              3.6    KB
│   │   ├── push-to-checkout.sample    2.7    KB
│   │   ├── pre-commit.sample          1.6    KB
│   │   ├── prepare-commit-msg.sample  1.5    KB
│   │   ├── pre-push.sample            1.3    KB
│   │   ├── commit-msg.sample          896    B
│   │   ├── pre-receive.sample         544    B
│   │   ├── applypatch-msg.sample      478    B
│   │   ├── pre-applypatch.sample      424    B
│   │   ├── pre-merge-commit.sample    416    B
│   │   └── post-update.sample         189    B
│   ├── objects                        11.4   KB  (D)
│   │   ├── 94                         1.6    KB  (D)
│   │   ├── 39                         1.5    KB  (D)
│   │   ├── c0                         1.1    KB  (D)
│   │   ├── 91                         1.1    KB  (D)
│   │   ├── 3a                         899    B   (D)
│   │   ├── 80                         883    B   (D)
│   │   ├── 31                         743    B   (D)
│   │   ├── a4                         720    B   (D)
│   │   ├── 15                         655    B   (D)
│   │   ├── 64                         593    B   (D)
│   │   ├── 4e                         411    B   (D)
│   │   ├── ec                         368    B   (D)
│   │   ├── 8f                         206    B   (D)
│   │   ├── c1                         205    B   (D)
│   │   ├── 74                         149    B   (D)
│   │   ├── f5                         148    B   (D)
│   │   ├── fc                         145    B   (D)
│   │   ├── ac                         82     B   (D)
│   │   ├── 9a                         81     B   (D)
│   │   ├── ea                         23     B   (D)
│   │   ├── info                       0      B   (D)
│   │   └── pack                       0      B   (D)
│   ├── logs                           718    B   (D)
│   │   ├── HEAD                       359    B
│   │   └── refs                       359    B   (D)
│   ├── index                          681    B
│   ├── info                           240    B   (D)
│   │   └── exclude                    240    B
│   ├── config                         92     B
│   ├── description                    73     B
│   ├── COMMIT_EDITMSG                 48     B
│   ├── refs                           41     B   (D)
│   │   ├── heads                      41     B   (D)
│   │   └── tags                       0      B   (D)
│   └── HEAD                           23     B
├── src                                11.0   KB  (D)
│   ├── lib                            9.0    KB  (D)
│   │   ├── print.rs                   4.5    KB
│   │   ├── mod.rs                     2.8    KB
│   │   └── tree.rs                    1.7    KB
│   └── main.rs                        2.0    KB
├── Cargo.lock                         3.2    KB
├── readme.md                          500    B
├── Cargo.toml                         261    B
└── .gitignore                         8      B
```

## Features

- Display directory tree
- Display size of each directory and files
- Display size in human readable format
- Display size in raw format
- Sort by size
- Display with colors
  - Executables
  - Directories
  - hidden files
  - symbolic links
