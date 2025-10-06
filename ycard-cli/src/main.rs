use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio::fs;
use tracing::{error, info};
use ycard_core::{self as ycard, PhonesStyle, ValidationMode};

#[derive(Parser)]
#[command(name = "ycard")]
#[command(about = "A CLI for parsing, formatting, and validating yCard files")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Default locale for parsing
    #[arg(long, global = true, default_value = "auto")]
    locale: String,

    /// Path to additional alias pack JSON file
    #[arg(long, global = true)]
    alias_pack: Option<PathBuf>,

    /// Disable bundled aliases (use only baked-in fallback)
    #[arg(long, global = true)]
    no_bundled_aliases: bool,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse yCard file and output result
    Parse {
        /// Input file path
        file: PathBuf,

        /// Output JSON AST instead of formatted YAML
        #[arg(long)]
        json_ast: bool,

        /// Use strict parsing mode (no field normalization)
        #[arg(long)]
        strict: bool,
    },

    /// Format yCard file
    Fmt {
        /// Input file path
        file: PathBuf,

        /// Write result back to file
        #[arg(long)]
        write: bool,

        /// Phone number formatting style
        #[arg(long, default_value = "canonical")]
        phones_style: String,

        /// Relocalize keys to specified locale
        #[arg(long)]
        relocalize_keys: Option<String>,
    },

    /// Check/validate yCard file
    Check {
        /// Input file path
        file: PathBuf,

        /// Use strict validation mode
        #[arg(long)]
        strict: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(level).init();

    // Initialize alias manager
    let mut alias_manager = ycard::AliasManager::new();

    if !cli.no_bundled_aliases {
        // Load bundled aliases (would be from installed location in real implementation)
        info!("Using bundled aliases");
    }

    if let Some(alias_pack_path) = &cli.alias_pack {
        info!("Loading alias pack from: {}", alias_pack_path.display());
        let pack_content = fs::read_to_string(alias_pack_path)
            .await
            .context("Failed to read alias pack file")?;

        alias_manager
            .load_pack(&pack_content)
            .context("Failed to load alias pack")?;
    }

    // Set default locale
    let locale = if cli.locale == "auto" {
        // Would detect from system locale in real implementation
        None
    } else {
        alias_manager.set_default_locale(&cli.locale);
        Some(cli.locale.as_str())
    };

    match cli.command {
        Commands::Parse {
            file,
            json_ast,
            strict,
        } => parse_command(file, json_ast, strict, locale, alias_manager).await,
        Commands::Fmt {
            file,
            write,
            phones_style,
            relocalize_keys,
        } => {
            fmt_command(
                file,
                write,
                phones_style,
                relocalize_keys,
                locale,
                alias_manager,
            )
            .await
        }
        Commands::Check { file, strict } => {
            check_command(file, strict, locale, alias_manager).await
        }
    }
}

async fn parse_command(
    file: PathBuf,
    json_ast: bool,
    strict: bool,
    locale: Option<&str>,
    alias_manager: ycard::AliasManager,
) -> Result<()> {
    let content = fs::read_to_string(&file)
        .await
        .context("Failed to read input file")?;

    let parser = ycard::Parser::with_alias_manager(alias_manager);

    let ycard = if strict {
        parser.parse_strict(&content)
    } else {
        // Use lenient parsing by default for human-friendly behavior
        parser.parse_lenient(&content, locale)
    }
    .context("Failed to parse yCard")?;

    if json_ast {
        let json = serde_json::to_string_pretty(&ycard).context("Failed to serialize to JSON")?;
        println!("{}", json);
    } else {
        let formatted = ycard::format(&ycard).context("Failed to format yCard")?;
        println!("{}", formatted);
    }

    Ok(())
}

async fn fmt_command(
    file: PathBuf,
    write: bool,
    phones_style: String,
    relocalize_keys: Option<String>,
    locale: Option<&str>,
    alias_manager: ycard::AliasManager,
) -> Result<()> {
    let content = fs::read_to_string(&file)
        .await
        .context("Failed to read input file")?;

    let parser = ycard::Parser::with_alias_manager(alias_manager);
    let ycard = parser
        .parse_lenient(&content, locale)
        .context("Failed to parse yCard")?;

    let phones_style = match phones_style.as_str() {
        "canonical" => PhonesStyle::Canonical,
        "shorthand" => PhonesStyle::Shorthand,
        "auto" => PhonesStyle::Auto,
        _ => {
            error!("Invalid phones-style: {}. Using canonical.", phones_style);
            PhonesStyle::Canonical
        }
    };

    let formatter = ycard::Formatter::new()
        .with_phones_style(phones_style)
        .with_relocalize_keys(relocalize_keys);

    let formatted = formatter.format(&ycard).context("Failed to format yCard")?;

    if write {
        fs::write(&file, formatted)
            .await
            .context("Failed to write formatted result")?;
        info!("Formatted {} in place", file.display());
    } else {
        println!("{}", formatted);
    }

    Ok(())
}

async fn check_command(
    file: PathBuf,
    strict: bool,
    locale: Option<&str>,
    alias_manager: ycard::AliasManager,
) -> Result<()> {
    let content = fs::read_to_string(&file)
        .await
        .context("Failed to read input file")?;

    let parser = ycard::Parser::with_alias_manager(alias_manager);
    let ycard = parser
        .parse_lenient(&content, locale)
        .context("Failed to parse yCard")?;

    let mode = if strict {
        ValidationMode::Strict
    } else {
        ValidationMode::Lenient
    };

    let diagnostics = ycard::validate(&ycard, mode).context("Failed to validate yCard")?;

    if diagnostics.is_empty() {
        println!("✅ {} is valid", file.display());
        Ok(())
    } else {
        println!("❌ {} has {} issues:", file.display(), diagnostics.len());

        for diagnostic in &diagnostics {
            let level_icon = match diagnostic.level {
                ycard::DiagnosticLevel::Error => "🔴",
                ycard::DiagnosticLevel::Warning => "🟡",
                ycard::DiagnosticLevel::Info => "🔵",
                ycard::DiagnosticLevel::Hint => "💡",
            };

            println!("  {} {}", level_icon, diagnostic.message);
            if let Some(code) = &diagnostic.code {
                println!("     Code: {}", code);
            }
        }

        let has_errors = diagnostics
            .iter()
            .any(|d| matches!(d.level, ycard::DiagnosticLevel::Error));

        if has_errors {
            std::process::exit(1);
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_parse() {
        // Would test CLI functionality
    }
}
