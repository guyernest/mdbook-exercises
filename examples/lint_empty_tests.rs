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
        let t = line.trim_start();
        if t.starts_with("::: tests") {
            if block_has_empty_or_missing_fence(&mut lines) {
                violations.push(Violation { file: path.to_path_buf(), line: i + 1 });
            }
        } else if t.starts_with("::: starter") {
            if block_has_empty_or_missing_fence(&mut lines) {
                violations.push(Violation { file: path.to_path_buf(), line: i + 1 });
            }
        } else if t.starts_with("::: solution") {
            if block_has_empty_or_missing_fence(&mut lines) {
                violations.push(Violation { file: path.to_path_buf(), line: i + 1 });
            }
        }
    }
}

fn block_has_empty_or_missing_fence(lines: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Lines<'_>>>) -> bool {
    // Scan until closing ':::'; ensure at least one fenced code block is present
    // and that inside the code fence there is at least one non-empty line.
    let mut saw_fence = false;
    let mut code_empty = true;
    while let Some((_, l)) = lines.next() {
        let t = l.trim_end();
        let trimmed = t.trim();
        if trimmed == ":::" { break; }
        if trimmed.starts_with("```") {
            if !saw_fence { saw_fence = true; }
            // consume code lines until next fence
            while let Some(&(_, ln)) = lines.peek() {
                let tt = ln.trim_end();
                if tt.trim_start().starts_with("```") {
                    lines.next();
                    break;
                }
                if !tt.trim().is_empty() { code_empty = false; }
                lines.next();
            }
        }
    }
    !saw_fence || code_empty
}
