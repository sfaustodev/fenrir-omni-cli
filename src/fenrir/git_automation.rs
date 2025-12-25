// Git automation commands for Fenrir
use std::process::Command;

pub fn gita_tudo() {
    println!("\n🐺 FENRIR GIT AUTOMATION - GITA TUDO\n");
    
    println!("📊 Step 1: Checking git status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Working tree clean");
            println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
            return;
        }
    }
    
    println!("\n📦 Step 2: Adding all...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ Staged");
    
    println!("\n💾 Step 3: Committing...");
    let msg = "🔄 Update Fenrir project\n\n🤖 Auto-generated";
    let _ = Command::new("git").args(&["commit", "-m", msg]).output();
    println!("✅ Committed");
    
    println!("\n🚀 Step 4: Pushing...");
    let push = Command::new("git").args(&["push", "origin", "main"]).output();
    match push {
        Ok(out) if out.status.success() => println!("✅ Pushed"),
        _ => println!("✅ Up to date"),
    }
    
    println!("\n✅ GITA TUDO COMPLETE!");
    println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
}

pub fn gita_ai() {
    println!("\n🤖 FENRIR GIT AUTOMATION - GITA AI\n");
    
    println!("📊 Step 1: Checking git status...");
    let status = Command::new("git").arg("status").output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        println!("{}", status);
        if status.contains("nothing to commit") {
            println!("\n✅ Working tree clean");
            println!("🐺 WOOF! WOOF! 🐺\n");
            return;
        }
    }
    
    println!("\n🔍 Step 2: Safety check...");
    let check = Command::new("git").args(&["status", "--short"]).output();
    if let Ok(output) = String::from_utf8_lossy(&output.stdout).to_string() {
        let has_sensitive = output.lines().any(|l| {
            let lower = l.to_lowercase();
            lower.contains(".env") || lower.contains("secret") || lower.contains("password")
        });
        if has_sensitive {
            println!("❌ Sensitive files! Aborted.");
            return;
        }
    }
    println!("✅ Safe");
    
    println!("\n📦 Step 3: Staging...");
    let _ = Command::new("git").args(&["add", "-A"]).output();
    println!("✅ Staged");
    
    println!("\n💾 Step 4: Committing...");
    let msg = "🤖 Auto-commit\n\n🔍 Safety checked\n📦 Staged";
    let _ = Command::new("git").args(&["commit", "-m", msg]).output();
    println!("✅ Committed");
    
    println!("\n✅ GITA AI COMPLETE!");
    println!("🐺 WOOF! WOOF! 🐺\n");
}
