extern crate tabwriter;

use self::tabwriter::TabWriter;
use std::fmt;
use std::io::Write;

use super::tree::FSNode;

#[allow(dead_code)]
pub enum ANSIColor {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
    RESET,
}

impl ANSIColor {
    pub fn as_string(&self) -> &str {
        match self {
            &ANSIColor::BLACK => "\u{001B}[0;30m",
            &ANSIColor::RED => "\u{001B}[0;31m",
            &ANSIColor::GREEN => "\u{001B}[0;32m",
            &ANSIColor::YELLOW => "\u{001B}[0;33m",
            &ANSIColor::BLUE => "\u{001B}[0;34m",
            &ANSIColor::MAGENTA => "\u{001B}[0;35m",
            &ANSIColor::CYAN => "\u{001B}[0;36m",
            &ANSIColor::WHITE => "\u{001B}[0;37m",
            &ANSIColor::RESET => "\u{001B}[0;0m",
        }
    }
}

pub fn print_tree(tree: &FSNode, max_depth: i64, size_format: SizeFormat) {
    let mut tw = TabWriter::new(Vec::new());

    print_tree_impl(tree, &mut tw, "", 0, max_depth, size_format);

    tw.flush().unwrap();
    let bytes = tw.into_inner().unwrap();
    let tabulated = String::from_utf8_lossy(&bytes);

    print!("{}", tabulated);
}

const SUM: &'static str = "(D)";
const BRANCH: &'static str = "├── ";
const LAST_BRANCH: &'static str = "└── ";
const INDENT: &'static str = "    ";
const NESTED_INDENT: &'static str = "│   ";

fn print_tree_impl<T: Write>(
    node: &FSNode,
    mut tw: &mut TabWriter<T>,
    prefix: &str,
    depth: i64,
    max_depth: i64,
    size_format: SizeFormat,
) {
    let sum_suffix = if node.is_dir() { SUM } else { "" };

    let color = get_file_color(node);

    writeln!(
        &mut tw,
        "{}{}{}\t{}\t{}",
        color.as_string(),
        node.name(),
        ANSIColor::RESET.as_string(),
        size_format.human_readable_byte_size(node.size()),
        sum_suffix,
    )
    .unwrap();

    if max_depth < 0 || max_depth >= depth {
        for (idx, item) in node.children().enumerate() {
            let last = idx == (node.children().count() - 1);
            let (branch, nested) = if last {
                (LAST_BRANCH, INDENT)
            } else {
                (BRANCH, NESTED_INDENT)
            };

            write!(&mut tw, "{}{}", prefix, branch).unwrap();

            let nested_prefix = format!("{}{}", prefix, nested);
            print_tree_impl(
                item,
                &mut tw,
                &nested_prefix,
                depth + 1,
                max_depth,
                size_format,
            );
        }
    }
}

fn get_file_color(node: &FSNode) -> ANSIColor {
    let color = if node.is_dir() {
        ANSIColor::BLUE
    } else if node.is_symlink() {
        ANSIColor::YELLOW
    } else if node.is_hidden() {
        ANSIColor::CYAN
    } else if node.is_executable() {
        ANSIColor::GREEN
    } else {
        ANSIColor::WHITE
    };
    color
}

fn get_size_folor(size: u64) -> ANSIColor {
    let color = if size > 1024 * 1024 * 1024 {
        ANSIColor::RED
    } else if size > 100 * 1024 * 1024 {
        ANSIColor::MAGENTA
    } else {
        ANSIColor::WHITE
    };
    color
}

#[derive(Copy, Clone)]
pub enum SizeFormat {
    PRETTY,
    Raw,
}

impl SizeFormat {
    pub fn human_readable_byte_size(self, bytes: u64) -> SizeFormatter {
        SizeFormatter(self, bytes)
    }
}

pub struct SizeFormatter(SizeFormat, u64);

impl fmt::Display for SizeFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        static PREFIX: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];
        let color = get_size_folor(self.1);
        let (power, prefix, which) = match self.0 {
            SizeFormat::PRETTY => (1024, PREFIX, log2(self.1) / 10),
            SizeFormat::Raw => return self.fmt_raw(f),
        };

        if which == 0 {
            return self.fmt_raw(f);
        }

        let decimal = self.1 as f64 / (power as f64).powf(which as f64);
        write!(
            f,
            "{}{:.1}\t{}{}",
            color.as_string(),
            decimal,
            prefix[which as usize],
            ANSIColor::RESET.as_string()
        )
    }
}

impl SizeFormatter {
    fn fmt_raw(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = get_size_folor(self.1);
        write!(
            f,
            "{}{}\tB{}",
            color.as_string(),
            self.1,
            ANSIColor::RESET.as_string()
        )
    }
}

fn log2(mut x: u64) -> u64 {
    let mut n: u64 = 0;

    while (x >> 1) > 0 {
        x >>= 1;
        n += 1;
    }
    n
}
