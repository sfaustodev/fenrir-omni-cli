#!/usr/bin/env python3
"""
FENRIR - Update kali_tools_comprehensive.rs for macOS
Replaces Linux apt commands with macOS Homebrew equivalents
"""

import re
import sys

def update_kali_tools_file(input_file, output_file):
    """Update install commands in kali_tools_comprehensive.rs for macOS"""

    with open(input_file, 'r') as f:
        content = f.read()

    # Pattern to find install_command lines
    # Match: install_command: Some("sudo apt install <tool>")
    # Replace with: install_command: Some("brew install <tool>")

    # Pattern 1: Simple apt install commands (with .to_string())
    pattern1 = r'install_command: Some\("sudo apt install ([^"]+)"\.to_string\(\)\)'
    replacement1 = r'install_command: Some("brew install \1".to_string())'

    # Apply replacements
    updated_content = re.sub(pattern1, replacement1, content)

    # Pattern 2: Handle multiple tools in one command
    # Some lines have: install_command: Some("sudo apt install tool1 tool2 tool3")
    # Keep them as-is but use brew instead

    # Pattern 3: apt-get instead of apt (with .to_string())
    pattern2 = r'install_command: Some\("sudo apt-get install ([^"]+)"\.to_string\(\)\)'
    replacement2 = r'install_command: Some("brew install \1".to_string())'
    updated_content = re.sub(pattern2, replacement2, updated_content)

    # Count changes
    original_apt_count = len(re.findall(pattern1, content) + re.findall(pattern2, content))
    new_brew_count = len(re.findall(r'install_command: Some\("brew install', updated_content))

    # Write updated content
    with open(output_file, 'w') as f:
        f.write(updated_content)

    print(f"✅ Updated {output_file}")
    print(f"   - Replaced {original_apt_count} 'apt' commands with 'brew'")
    print(f"   - Total brew install commands: {new_brew_count}")

    return original_apt_count

if __name__ == "__main__":
    input_file = "src/fenrir/kali_tools_comprehensive.rs"
    output_file = "src/fenrir/kali_tools_comprehensive.rs"

    try:
        changes = update_kali_tools_file(input_file, output_file)
        print(f"\n🎯 Successfully updated {changes} install commands for macOS!")
    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)
