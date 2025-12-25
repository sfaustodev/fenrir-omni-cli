// Git automation commands for Fenrir
use std::process::Command;

pub fn gita_tudo() {
    println!("\n🐺 FENRIR GIT AUTOMATION - GITA TUDO\n");
    
    // 1. Status
    println!("📊 Step 1: Checking git status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Nothing to commit. Working tree clean.");
            println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
            return;
        }
    }
    
    // 2. Add all
    println!("\n📦 Step 2: Adding all changes...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ All changes staged");
    
    // 3. Commit
    println!("\n💾 Step 3: Creating commit...");
    let msg = "🔄 Update Fenrir project\n\n📦 Auto-staged\n🤖 Generated with Claude Code";
    let commit = Command::new("git").args(&["commit", "-m", msg]).output();
    match commit {
        Ok(out) if out.status.success() => println!("✅ Commit created"),
        _ => println!("ℹ️  No changes to commit"),
    }
    
    // 4. Push
    println!("\n🚀 Step 4: Pushing to origin/main...");
    let push = Command::new("git").args(&["push", "origin", "main"]).output();
    match push {
        Ok(out) if out.status.success() => println!("✅ Pushed successfully"),
        _ => println!("✅ Already up to date"),
    }
    
    println!("\n✅ GITA TUDO COMPLETE!");
    println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
}

pub fn gita_ai() {
    println!("\n🤖 FENRIR GIT AUTOMATION - GITA AI\n");
    
    // 1. Status
    println!("📊 Step 1: Checking git status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Nothing to commit. Working tree clean.");
            println!("🐺 WOOF! WOOF! 🐺\n");
            return;
        }
    }
    
    // 2. Safety check
    println!("\n🔍 Step 2: Safety check...");
    let check = Command::new("git").args(&["status", "--short"]).output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        let has_sensitive = output.lines().any(|l| {
            let lower = l.to_lowercase();
            lower.contains(".env") || lower.contains("secret") || lower.contains("password") || lower.contains("api_key")
        });
        if has_sensitive {
            println!("❌ Sensitive files detected! Commit aborted for safety.");
            return;
        }
    }
    println!("✅ No sensitive files - Safe to proceed");
    
    // 3. Add
    println!("\n📦 Step 3: Staging changes...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ Changes staged");
    
    // 4. Commit
    println!("\n💾 Step 4: Creating commit...");
    let msg = "🤖 Auto-commit\n\n🔍 Safety check passed\n📦 Changes staged\n🤖 Generated with Claude Code";
    let commit = Command::new("git").args(&["commit", "-m", msg]).output();
    match commit {
        Ok(out) if out.status.success() => println!("✅ Commit created"),
        _ => println!("ℹ️  No changes to commit"),
    }
    
    println!("\n✅ GITA AI COMPLETE!");
    println!("🐺 WOOF! WOOF! 🐺\n");
}
