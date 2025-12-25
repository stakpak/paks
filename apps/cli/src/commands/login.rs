//! Login/Logout commands - authenticate with the registry

use anyhow::{Result, bail};
use dialoguer::{Confirm, Input};
use paks_api::PaksClient;

use super::core::config::Config;

pub struct LoginArgs {
    pub token: Option<String>,
}

pub async fn run_login(args: LoginArgs) -> Result<()> {
    // Check if already logged in
    let mut config = Config::load()?;
    if let Some(existing_token) = config.get_auth_token() {
        // Verify existing token
        let mut client = PaksClient::new()?;
        client.set_token(existing_token);

        if let Ok(user) = client.get_current_user().await {
            println!("Already logged in as: {}", user.username);

            let overwrite = Confirm::new()
                .with_prompt("Do you want to log in with a different account?")
                .default(false)
                .interact()?;

            if !overwrite {
                return Ok(());
            }
        }
    }

    // Get token - either from args or prompt
    let token = if let Some(t) = args.token {
        t
    } else {
        println!("Get your API token from: https://stakpak.dev/settings/tokens");
        println!();
        Input::new().with_prompt("API Token").interact_text()?
    };

    if token.trim().is_empty() {
        bail!("Token cannot be empty");
    }

    // Validate token
    print!("Validating token... ");
    let mut client = PaksClient::new()?;
    client.set_token(&token);

    let user = client.get_current_user().await.map_err(|e| {
        println!("✗");
        anyhow::anyhow!("Invalid token: {}", e)
    })?;
    println!("✓");

    // Save token to config
    config.set_auth_token(token);
    config.save()?;

    println!();
    println!("✓ Logged in as: {}", user.username);

    Ok(())
}

pub async fn run_logout() -> Result<()> {
    let mut config = Config::load()?;

    if config.get_auth_token().is_none() {
        println!("Not logged in.");
        return Ok(());
    }

    // Confirm logout
    let confirm = Confirm::new()
        .with_prompt("Are you sure you want to log out?")
        .default(true)
        .interact()?;

    if !confirm {
        println!("Aborted.");
        return Ok(());
    }

    // Clear token
    config.clear_auth_token();
    config.save()?;

    println!("✓ Logged out successfully.");

    Ok(())
}
