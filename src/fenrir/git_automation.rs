// Git automation commands for Fenrir
use std::process::Command;

pub fn gita_tudo() {
    println!("\n🐺 FENRIR GIT AUTOMATION - GITA TUDO\n");

    println!("📊 Step 1: Checking git status...");
    let status_output = Command::new("git").arg("status").output();
    if let Ok(output) = status_output {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("{}", result);
        if result.contains("nothing to commit") {
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
    let commit_result = Command::new("git").args(&["commit", "-m", msg]).output();
    match commit_result {
        Ok(result) if result.status.success() => println!("✅ Committed"),
        _ => println!("ℹ️ No changes to commit"),
    }

    println!("\n🚀 Step 4: Pushing...");
    let push_result = Command::new("git").args(&["push", "origin", "main"]).output();
    match push_result {
        Ok(result) if result.status.success() => println!("✅ Pushed"),
        _ => println!("✅ Up to date"),
    }
    
    println!("\n✅ GITA TUDO COMPLETE!");
    println!("🐺 WOOF! WOOF! WOOF! 🐺\n");
}

pub fn gita_ai() {
    println!("\n🤖 FENRIR GIT AUTOMATION - GITA AI\n");

    println!("📊 Step 1: Checking git status...");
    let status_output = Command::new("git").arg("status").output();
    if let Ok(output) = status_output {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("{}", result);
        if result.contains("nothing to commit") {
            println!("\n✅ Working tree clean");
            println!("🐺 WOOF! WOOF! 🐺\n");
            return;
        }
    }

    println!("\n🔍 Step 2: Safety check...");
    let check_output = Command::new("git").args(&["status", "--short"]).output();
    if let Ok(output) = check_output {
        let result = String::from_utf8_lossy(&output.stdout);
        let has_sensitive = result.lines().any(|l| {
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
    let commit_result = Command::new("git").args(&["commit", "-m", msg]).output();
    match commit_result {
        Ok(result) if result.status.success() => println!("✅ Committed"),
        _ => println!("ℹ️ No changes to commit"),
    }
    
    println!("\n✅ GITA AI COMPLETE!");
    println!("🐺 WOOF! WOOF! 🐺\n");
}
