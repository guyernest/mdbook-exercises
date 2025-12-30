use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: cargo run --example lint_empty_tests -- <path> [<path> ...]");
        std::process::exit(2);
    }

    let mut violations = Vec::new();
    for arg in &args {
        let path = Path::new(arg);
        if path.is_file() && is_md(path) {
            lint_file(path, &mut violations);
        } else if path.is_dir() {
            for entry in walk_md(path) {
                lint_file(&entry, &mut violations);
            }
        }
    }

    if violations.is_empty() {
        println!("No empty tests blocks found.");
    } else {
        eprintln!("Found empty tests blocks in:");
        for v in &violations {
            eprintln!("- {}:{}", v.file.display(), v.line);
        }
        std::process::exit(1);
    }
}

fn is_md(p: &Path) -> bool {
    p.extension().map(|e| e == "md").unwrap_or(false)
}

fn walk_md(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(rd) = fs::read_dir(&dir) {
            for ent in rd.flatten() {
                let p = ent.path();
                if p.is_dir() { stack.push(p); }
                else if is_md(&p) { out.push(p); }
            }
        }
    }
    out
}

#[derive(Debug)]
struct Violation { file: PathBuf, line: usize }

fn lint_file(path: &Path, violations: &mut Vec<Violation>) {
    let Ok(text) = fs::read_to_string(path) else { return; };
    let mut lines = text.lines().enumerate().peekable();
    while let Some((i, line)) = lines.next() {
        if line.trim_start().starts_with("::: tests") {
            // Enter tests block until closing ':::'
            let mut in_block = true;
            let mut saw_fence = false;
            let mut code_empty = true;
            while let Some((_, l)) = lines.next() {
                let t = l.trim_end();
                if t.trim() == ":::" { in_block = false; break; }
                if t.trim_start().starts_with("```") {
                    if !saw_fence { saw_fence = true; }
                    // Collect code until next fence
                    while let Some(&(k, ln)) = lines.peek() {
                        let tt = ln.trim_end();
                        if tt.trim_start().starts_with("```") {
                            // end of code block
                            lines.next();
                            break;
                        }
                        if !tt.trim().is_empty() { code_empty = false; }
                        lines.next();
                    }
                }
                if !in_block { break; }
            }
            if !saw_fence || code_empty {
                violations.push(Violation { file: path.to_path_buf(), line: i + 1 });
            }
        }
    }
}

