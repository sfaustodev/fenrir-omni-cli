# TODO: Test Pentest Stealthy Discovery of Netflix Credentials

## Step 1: Resolve Build Issues
- [ ] Fix dependency conflicts (solana-client vs zcash packages)
- [ ] Attempt to build the project successfully

## Step 2: Test Stealthy Network Scanning
- [ ] Run `scan` command with stealth mode on local network (e.g., 192.168.1.0/24)
- [ ] Verify nmap stealth options (-sS, -T1) are used to avoid detection
- [ ] Document discovered devices and open ports

## Step 3: Test Stealthy Penetration Testing
- [ ] Run `bite` command with cautious intensity on discovered targets
- [ ] Check for any credential-related findings or vulnerabilities
- [ ] Ensure no exploitation is performed (auto_exploit=false)

## Step 4: Analyze for Netflix Credential Discovery
- [ ] Review scan/bite output for potential Netflix-related services (ports 80/443, etc.)
- [ ] Note limitations: No built-in Netflix-specific tools; OSINT is stub-only
- [ ] Suggest manual use of tools like Wireshark for credential sniffing if needed

## Step 5: Document Results
- [ ] Record stealth effectiveness and detection avoidance
- [ ] Note any issues with dependency conflicts impacting functionality
- [ ] Provide summary of testing outcomes

## Improvements and Fixes Checklist
- [x] Add real Netflix credential discovery logic (plugin added)
- [ ] Implement Tor-based OSINT in osint.rs
- [ ] Add more Kali tools for credential hunting (e.g., hydra, john)
- [ ] Improve plugin system error handling
- [ ] Add configuration for stealth levels
- [ ] Implement credential storage and reporting
- [ ] Add unit tests for kali_tools functions
- [ ] Optimize build times by selective dependency inclusion
