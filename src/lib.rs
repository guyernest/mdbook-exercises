//! # mdbook-exercises
//!
//! A preprocessor for [mdBook](https://rust-lang.github.io/mdBook/) that adds
//! interactive exercise blocks with hints, solutions, and test execution.
//!
//! ## Library Usage
//!
//! Use `mdbook-exercises` as a library to parse exercise markdown files:
//!
//! ````rust
//! use mdbook_exercises::{parse_exercise, ParsedExercise};
//!
//! let markdown = r#"
//! # Exercise: Hello World
//!
//! ::: exercise
//! id: hello-world
//! difficulty: beginner
//! :::
//!
//! Write a greeting function.
//!
//! ::: starter
//! ```rust
//! fn greet() { todo!() }
//! ```
//! :::
//! "#;
//!
//! let parsed = parse_exercise(markdown).expect("Failed to parse");
//! match parsed {
//!     ParsedExercise::Code(exercise) => {
//!         assert_eq!(exercise.metadata.id, "hello-world");
//!     }
//!     ParsedExercise::UseCase(_) => panic!("Expected code exercise"),
//! }
//! ````
//!
//! ## Feature Flags
//!
//! - `default` - Full mdBook preprocessor
//! - `preprocessor` - mdBook integration (requires `render`)
//! - `render` - HTML rendering
//! - (no features) - Parser only, minimal dependencies

pub mod parser;
pub mod types;

#[cfg(feature = "render")]
pub mod render;

#[cfg(feature = "preprocessor")]
pub mod preprocessor;

// Re-export main types for convenience
pub use parser::{parse_exercise, ParseError};
pub use types::*;

#[cfg(feature = "render")]
pub use render::{render_exercise, render_exercise_with_config, RenderError};
