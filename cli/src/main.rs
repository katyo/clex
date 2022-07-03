use anyhow::Result;
use clex::{Lexeme, Lexer, Token};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

fn is_c(path: &Path) -> bool {
    path.extension().map(|ext| ext == "c").unwrap_or(false)
}

fn lex_file(stats: &mut Stats, args: &Args, path: &Path) -> Result<()> {
    use std::io::Read;

    if args.print_files {
        println!("@@ {}", path.display());
    }

    let mut file = File::open(&path)?;
    let mut src = String::default();

    file.read_to_string(&mut src)?;

    fn print_extracted(args: &Args, name: &str, data: impl std::fmt::Debug) {
        if args.print_extracted {
            println!("    >> {}: {:?}", name, data);
        }
    }

    fn print_failed<'l>(path: &Path, name: &str, lexeme: &Lexeme<'l>) {
        eprintln!(
            "    !! {}: {:?} {:?} ({})",
            name,
            lexeme.span,
            lexeme.slice,
            path.display()
        );
    }

    let lexer = Lexer::from(src.as_ref());

    for lexeme in lexer {
        if lexeme.token == Token::Unknown {
            eprintln!(
                "  ?? {:?} {:?} ({})",
                lexeme.span,
                lexeme.slice,
                path.display()
            );
        } else {
            if args.print_tokens {
                println!(
                    "  -- {:?} {:?} {:?}",
                    lexeme.token, lexeme.span, lexeme.slice
                );
            }

            macro_rules! extract_data {
                (($path:ident, $args:ident, $lexeme:ident) { $($token:ident, $name:ident $(::<$type:ident>)*, $arg:ident;)* }) => {
                    match $lexeme.token {
                        $(
                            Token::$token => {
                                    if args.$arg {
                                        if let Some(val) = lexeme.$name$(::<$type>)*() {
                                            print_extracted(args, stringify!($name), val);
                                        } else {
                                            print_failed($path, stringify!($name), &lexeme);
                                        }
                                    }
                            }
                        )*
                            _ => {},
                    }
                };
            }

            extract_data! {
                (path, args, lexeme) {
                    Identifier, keyword, extract_keywords;
                    Comment, comment, extract_comments;
                    Char, char, extract_chars;
                    String, string, extract_strings;
                    Int, int::<i128>, extract_ints;
                    Float, float::<f64>, extract_floats;
                }
            }
        }
    }

    stats.files += 1;

    Ok(())
}

fn lex_dir(stats: &mut Stats, args: &Args, path: &Path) -> Result<()> {
    if args.print_dirs {
        println!("** {}", path.display());
    }

    for entry in path.read_dir()? {
        let path = entry?.path();
        if path.is_file() {
            if is_c(&path) {
                lex_file(stats, args, &path)?;
            }
        } else if path.is_dir() {
            lex_dir(stats, args, &path)?;
        }
    }

    stats.dirs += 1;

    Ok(())
}

#[derive(structopt::StructOpt)]
struct Args {
    /// Extract keywords
    #[structopt(short = "k", long)]
    pub extract_keywords: bool,

    /// Extract comment texts
    #[structopt(short = "C", long)]
    pub extract_comments: bool,

    /// Extract character literals
    #[structopt(short = "c", long)]
    pub extract_chars: bool,

    /// Extract string literals
    #[structopt(short = "s", long)]
    pub extract_strings: bool,

    /// Extract integer literals
    #[structopt(short = "i", long)]
    pub extract_ints: bool,

    /// Extract floating-point literals
    #[structopt(short = "f", long)]
    pub extract_floats: bool,

    /// Print extracted data
    #[structopt(short = "x", long)]
    pub print_extracted: bool,

    /// Print token data
    #[structopt(short = "t", long)]
    pub print_tokens: bool,

    /// Print file paths
    #[structopt(short = "p", long)]
    pub print_files: bool,

    /// Print directory paths
    #[structopt(short = "d", long)]
    pub print_dirs: bool,

    /// C source file or directory
    #[structopt()]
    pub source_path: PathBuf,
}

impl AsRef<Args> for Args {
    fn as_ref(&self) -> &Args {
        self
    }
}

#[derive(Default)]
struct Stats {
    pub files: usize,
    pub dirs: usize,
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let mut stats = Stats::default();
    let path = &args.source_path;

    if path.is_file() {
        if is_c(path) {
            lex_file(&mut stats, &args, path)?;
        } else {
            eprintln!("Not a C source: {}", path.display());
        }
    } else if path.is_dir() {
        lex_dir(&mut stats, &args, path)?;
    }

    println!("** processed {} dirs and {} files", stats.dirs, stats.files);

    Ok(())
}
