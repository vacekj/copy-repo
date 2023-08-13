use std::fs::File;
use clap::Parser;
use std::io::{self, Read, stdout, Stdout, Write};
use std::path::Path;
use ignore::Walk;

#[derive(Parser, Debug)]
struct Opts {
    /// Path to the git repository
    #[arg(short, long)]
    repo_path: String,

    /// Path to the preamble text file
    #[arg(short, long, default_value = "")]
    preamble: String,

    /// Path to the output file
    #[arg(short, long)]
    output: Option<String>,
}

fn process_repository(repo_path: &Path, mut output: Stdout) -> io::Result<()> {
    for result in Walk::new(repo_path) {
        if let Ok(entry) = result {
            if entry.path().is_file() {
                let relative_file_path = entry.path().strip_prefix(repo_path).unwrap().to_string_lossy().to_string();
                let mut file = File::open(relative_file_path.clone())?;
                let mut buf = vec![];
                file.read_to_end (&mut buf)?;
                let contents = String::from_utf8_lossy (&buf);
                writeln!(&mut output, "----")?;
                writeln!(output, "{}", relative_file_path)?;
                writeln!(output, "{}", contents)?;
            }
        }
    }
    writeln!(output, "--END--")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();

    let repo_path = Path::new(&opts.repo_path);
    let mut stdout = io::stdout();

    if !opts.preamble.is_empty() {
        writeln!(stdout, "{}", std::fs::read_to_string(opts.preamble)?)?;
    } else {
        writeln!(stdout, "The following text is a Git repository with code. ")?;
    }

    process_repository(&repo_path, stdout)?;

    Ok(())
}
