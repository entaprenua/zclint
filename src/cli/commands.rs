use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "zclint")]
#[command(version = "0.1.0")]
#[command(about = "Zero-code linter for codeless components", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Check files or directories for violations
    Check {
        /// Files or directories to lint (default: current directory)
        #[arg(default_value = ".", num_args = 0..)]
        files: Vec<PathBuf>,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Install pre-commit hook
    Install,

    /// Initialize zclint config file
    Init,
}

pub fn install_hook() -> Result<()> {
    let hook_content = r#"#!/bin/sh
# zclint pre-commit hook
npx zclint --staged
"#;

    let hook_path = PathBuf::from(".git/hooks/pre-commit");

    if hook_path.exists() {
        println!(
            "Warning: pre-commit hook already exists at {}",
            hook_path.display()
        );
        println!("Please manually add 'npx zclint --staged' to your existing hook.");
    } else {
        std::fs::create_dir_all(".git/hooks")?;
        std::fs::write(&hook_path, hook_content)?;
        println!("Installed pre-commit hook at {}", hook_path.display());
    }

    Ok(())
}

pub fn init_config() -> Result<()> {
    let config_content = r#"# zclint configuration

include:
  - "**/*.{tsx,jsx}"

exclude:
  - "node_modules/**"
  - "dist/**"
  - "build/**"
  - ".git/**"

rules:
  no-event-handlers: error
  no-reactive-primitives: error
  no-ternary: error
  no-logical-and: error
  no-plain-ts: error
"#;

    std::fs::write("zclint.yaml", config_content)?;
    println!("Created zclint.yaml configuration file.");

    Ok(())
}
