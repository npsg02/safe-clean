use anyhow::{anyhow, Result};
use tokio::process::Command as AsyncCommand;
use dialoguer::Confirm;

pub async fn cleanup(dry_run: bool) -> Result<()> {
    println!("🐳 Docker Safe Cleanup");
    println!("======================");

    if !is_docker_available().await? {
        return Err(anyhow!("Docker is not available or not running"));
    }

    // Check for unused containers
    cleanup_containers(dry_run).await?;
    
    // Check for unused images
    cleanup_images(dry_run).await?;
    
    // Check for unused volumes
    cleanup_volumes(dry_run).await?;
    
    // Check for unused networks
    cleanup_networks(dry_run).await?;

    println!("\n✅ Docker cleanup completed!");
    Ok(())
}

async fn is_docker_available() -> Result<bool> {
    let output = AsyncCommand::new("docker")
        .args(["version", "--format", "{{.Server.Version}}"])
        .output()
        .await;

    match output {
        Ok(output) => Ok(output.status.success()),
        Err(_) => Ok(false),
    }
}

async fn cleanup_containers(dry_run: bool) -> Result<()> {
    println!("\n📦 Checking for stopped containers...");
    
    let output = AsyncCommand::new("docker")
        .args(["ps", "-a", "--filter", "status=exited", "--format", "table {{.ID}}\\t{{.Image}}\\t{{.Status}}"])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    if lines.len() <= 1 {
        println!("   No stopped containers found.");
        return Ok(());
    }

    println!("{}", stdout);
    
    if dry_run {
        println!("   [DRY RUN] Would remove {} stopped containers", lines.len() - 1);
        return Ok(());
    }

    if Confirm::new()
        .with_prompt(&format!("Remove {} stopped containers?", lines.len() - 1))
        .interact()?
    {
        let result = AsyncCommand::new("docker")
            .args(["container", "prune", "-f"])
            .output()
            .await?;

        if result.status.success() {
            println!("   ✅ Stopped containers removed successfully");
        } else {
            println!("   ❌ Failed to remove containers: {}", String::from_utf8_lossy(&result.stderr));
        }
    }

    Ok(())
}

async fn cleanup_images(dry_run: bool) -> Result<()> {
    println!("\n🖼️  Checking for unused images...");
    
    let output = AsyncCommand::new("docker")
        .args(["images", "--filter", "dangling=true", "--format", "table {{.ID}}\\t{{.Repository}}\\t{{.Tag}}\\t{{.Size}}"])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    if lines.len() <= 1 {
        println!("   No dangling images found.");
        return Ok(());
    }

    println!("{}", stdout);
    
    if dry_run {
        println!("   [DRY RUN] Would remove {} dangling images", lines.len() - 1);
        return Ok(());
    }

    if Confirm::new()
        .with_prompt(&format!("Remove {} dangling images?", lines.len() - 1))
        .interact()?
    {
        let result = AsyncCommand::new("docker")
            .args(["image", "prune", "-f"])
            .output()
            .await?;

        if result.status.success() {
            println!("   ✅ Dangling images removed successfully");
        } else {
            println!("   ❌ Failed to remove images: {}", String::from_utf8_lossy(&result.stderr));
        }
    }

    Ok(())
}

async fn cleanup_volumes(dry_run: bool) -> Result<()> {
    println!("\n💾 Checking for unused volumes...");
    
    let output = AsyncCommand::new("docker")
        .args(["volume", "ls", "--filter", "dangling=true", "--format", "table {{.Name}}\\t{{.Driver}}\\t{{.Scope}}"])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    if lines.len() <= 1 {
        println!("   No unused volumes found.");
        return Ok(());
    }

    println!("{}", stdout);
    
    if dry_run {
        println!("   [DRY RUN] Would remove {} unused volumes", lines.len() - 1);
        return Ok(());
    }

    if Confirm::new()
        .with_prompt(&format!("Remove {} unused volumes?", lines.len() - 1))
        .interact()?
    {
        let result = AsyncCommand::new("docker")
            .args(["volume", "prune", "-f"])
            .output()
            .await?;

        if result.status.success() {
            println!("   ✅ Unused volumes removed successfully");
        } else {
            println!("   ❌ Failed to remove volumes: {}", String::from_utf8_lossy(&result.stderr));
        }
    }

    Ok(())
}

async fn cleanup_networks(dry_run: bool) -> Result<()> {
    println!("\n🌐 Checking for unused networks...");
    
    let output = AsyncCommand::new("docker")
        .args(["network", "ls", "--filter", "dangling=true", "--format", "table {{.ID}}\\t{{.Name}}\\t{{.Driver}}"])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    if lines.len() <= 1 {
        println!("   No unused networks found.");
        return Ok(());
    }

    println!("{}", stdout);
    
    if dry_run {
        println!("   [DRY RUN] Would remove {} unused networks", lines.len() - 1);
        return Ok(());
    }

    if Confirm::new()
        .with_prompt(&format!("Remove {} unused networks?", lines.len() - 1))
        .interact()?
    {
        let result = AsyncCommand::new("docker")
            .args(["network", "prune", "-f"])
            .output()
            .await?;

        if result.status.success() {
            println!("   ✅ Unused networks removed successfully");
        } else {
            println!("   ❌ Failed to remove networks: {}", String::from_utf8_lossy(&result.stderr));
        }
    }

    Ok(())
}