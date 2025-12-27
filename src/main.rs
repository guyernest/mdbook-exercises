//! mdbook-exercises preprocessor binary.
//!
//! This binary is invoked by mdBook during the build process.

use std::env;
use std::io;
use std::process;

#[cfg(feature = "preprocessor")]
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
#[cfg(feature = "preprocessor")]
use mdbook_exercises::preprocessor::FullExercisesPreprocessor;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle mdBook preprocessor commands
    if args.len() > 1 {
        match args[1].as_str() {
            "supports" => {
                // Check if we support the given renderer
                let renderer = args.get(2).map(|s| s.as_str()).unwrap_or("");
                handle_supports(renderer);
            }
            _ => {
                // Unknown command, try to process as preprocessor
                #[cfg(feature = "preprocessor")]
                run_preprocessor();

                #[cfg(not(feature = "preprocessor"))]
                {
                    eprintln!("Error: preprocessor feature not enabled");
                    process::exit(1);
                }
            }
        }
    } else {
        // No arguments - run as preprocessor (stdin/stdout mode)
        #[cfg(feature = "preprocessor")]
        run_preprocessor();

        #[cfg(not(feature = "preprocessor"))]
        {
            eprintln!("Error: preprocessor feature not enabled");
            process::exit(1);
        }
    }
}

/// Handle the `supports` command.
fn handle_supports(renderer: &str) {
    // We only support HTML output
    if renderer == "html" {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

/// Run the preprocessor, reading from stdin and writing to stdout.
#[cfg(feature = "preprocessor")]
fn run_preprocessor() {
    let preprocessor = FullExercisesPreprocessor::new();

    if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Process the book from stdin and write the result to stdout.
#[cfg(feature = "preprocessor")]
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), mdbook::errors::Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    // Warn if mdBook version differs from build version
    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against mdbook version {}, \
             but is being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
