//! Generate JSON Schema from Rust types
//!
//! Run with: cargo run --bin generate-schema -p paks-api-schema > schema.json

use paks_api_schema::PaksApiSchema;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(PaksApiSchema);
    let json = serde_json::to_string_pretty(&schema).unwrap_or_else(|e| {
        eprintln!("Failed to serialize schema: {}", e);
        std::process::exit(1);
    });
    println!("{}", json);
}
