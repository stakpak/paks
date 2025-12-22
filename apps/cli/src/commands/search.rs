//! Search command - search for skills in the registry

use anyhow::Result;

pub struct SearchArgs {
    pub query: String,
    pub limit: usize,
}

pub async fn run(args: SearchArgs) -> Result<()> {
    println!("Searching for: {} (limit: {})", args.query, args.limit);

    // TODO: Implement skill search
    // 1. Query registry API
    // 2. Display results in table format
    // 3. Show name, description, version, author

    Ok(())
}
