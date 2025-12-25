//! Search command - search for skills in the registry

use anyhow::{Context, Result};
use paks_api::{PaksClient, SearchPaksQuery};

pub struct SearchArgs {
    pub query: String,
    pub limit: usize,
}

pub async fn run(args: SearchArgs) -> Result<()> {
    // Create API client
    let client = PaksClient::builder()
        .base_url("https://apiv2.stakpak.dev")
        .build()
        .context("Failed to create API client")?;

    // Build search query
    let query = SearchPaksQuery {
        query: Some(args.query.clone()),
        limit: Some(args.limit as u32),
        ..Default::default()
    };

    // Execute search
    let mut results = client
        .search_paks(query)
        .await
        .context("Failed to search registry")?;

    if results.is_empty() {
        println!("\n  No skills found matching '{}'\n", args.query);
        return Ok(());
    }

    // Sort by downloads (descending)
    results.sort_by(|a, b| b.total_downloads.cmp(&a.total_downloads));

    println!();
    for pak in results {
        // First line: owner/name + stats
        let downloads = format_count(pak.total_downloads);
        print!(
            "  \x1b[1;36m{}\x1b[0m/\x1b[1m{}\x1b[0m \x1b[2m↓{}\x1b[0m",
            pak.owner_name, pak.name, downloads
        );

        // Tags inline (up to 3)
        if let Some(ref tags) = pak.tags
            && !tags.is_empty()
        {
            let tags_str: String = tags
                .iter()
                .take(3)
                .map(|t| format!("\x1b[33m#{}\x1b[0m", t))
                .collect::<Vec<_>>()
                .join(" ");
            print!("  {}", tags_str);
        }
        println!();

        // Description on second line
        if let Some(desc) = &pak.description {
            let truncated: String = desc.chars().take(72).collect();
            let suffix = if desc.len() > 72 { "…" } else { "" };
            println!("    \x1b[2m{}{}\x1b[0m", truncated, suffix);
        }
    }

    println!("\n  \x1b[2mInstall: paks install <owner>/<skill>\x1b[0m\n");

    Ok(())
}

/// Format large numbers with K/M suffixes
fn format_count(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
