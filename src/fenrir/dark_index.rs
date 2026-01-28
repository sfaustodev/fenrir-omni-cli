// ============================================================================
// DARK INDEX - KEYWORD MAPPING FOR KALI TOOLS
// Índice Sombrio do Kali - Keyword matching for tool recommendations
// ============================================================================

use std::collections::HashMap;

/// Create the dark index mapping keywords to Kali tools
pub fn create_dark_index() -> HashMap<String, Vec<String>> {
    let mut index = HashMap::new();

    // RECONNAISSANCE TOOLS
    index.insert("nmap".to_string(), vec![
        "port scan", "network scan", "service detection", "os detection", "vulnerability scan",
        "recon", "discovery", "enumeration", "fingerprinting", "port scanning", "network mapping",
        "tcp scan", "udp scan", "syn scan", "stealth scan", "aggressive scan", "version detection",
        "script scan", "default scripts", "timing", "fragmentation", "spoofing", "decoy",
        "idle scan", "ftp bounce", "bypass firewall", "evade ids", "output format", "grepable",
        "xml output", "normal output", "scriptable output", "traceroute", "route discovery",
        "ip protocol scan", "sctp scan", "raw packet", "custom packet", "ipv6 scan"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("netdiscover".to_string(), vec![
        "arp scan", "passive discovery", "active discovery", "network discovery", "host discovery",
        "ip range scan", "subnet scan", "local network", "lan discovery", "mac address",
        "vendor lookup", "oui lookup", "network mapping", "recon", "enumeration", "passive",
        "active", "range scan", "interface scan", "promiscuous mode", "packet capture",
        "address resolution", "neighbor discovery", "ipv6 neighbor", "router discovery"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("theHarvester".to_string(), vec![
        "email harvest", "domain harvest", "subdomain harvest", "people search", "osint",
        "email enumeration", "domain enumeration", "subdomain enumeration", "google dorks",
        "bing search", "yahoo search", "linkedin search", "twitter search", "instagram search",
        "github search", "shodan search", "censys search", "virustotal search", "threatcrowd",
        "crtsh", "dnsdumpster", "passive reconnaissance", "information gathering",
        "email addresses", "phone numbers", "usernames", "social media", "public records"
    ].iter().map(|s| s.to_string()).collect());

    // SCANNING TOOLS
    index.insert("nikto".to_string(), vec![
        "web scan", "web server scan", "vulnerability scan", "cgi scan", "outdated software",
        "server misconfiguration", "insecure files", "dangerous files", "web vulnerabilities",
        "http methods", "headers", "cookies", "authentication", "ssl configuration",
        "subdomain scan", "proxy scan", "mutation scan", "evasion techniques", "reporting",
        "html output", "xml output", "csv output", "txt output", "json output"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("masscan".to_string(), vec![
        "fast scan", "mass scanning", "internet scale", "port scan", "tcp syn scan",
        "banner grab", "service detection", "rate limiting", "packet rate", "pps limit",
        "blacklist", "whitelist", "exclude file", "include file", "output format",
        "binary output", "grepable output", "json output", "list output", "resume scan",
        "paused scan", "distributed scanning", "cluster scanning", "shodan integration"
    ].iter().map(|s| s.to_string()).collect());

    // EXPLOITATION TOOLS
    index.insert("metasploit-framework".to_string(), vec![
        "exploit framework", "payload delivery", "meterpreter", "post exploitation",
        "auxiliary modules", "exploit modules", "payload modules", "encoder modules",
        "nop modules", "evasion modules", "database", "workspace", "session management",
        "job management", "resource scripts", "automation", "msfconsole", "msfvenom",
        "msfdb", "multi handler", "exploit development", "shellcode", "reverse shell",
        "bind shell", "staged payload", "stageless payload", "persistence", "privilege escalation"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("sqlmap".to_string(), vec![
        "sql injection", "sqli", "database dump", "table dump", "column dump", "data extraction",
        "blind injection", "error based", "union based", "time based", "boolean based",
        "stacked queries", "waf bypass", "tamper scripts", "level", "risk", "technique",
        "database enumeration", "user enumeration", "password hash", "privilege escalation",
        "file system access", "os command execution", "metasploit integration", "burp integration",
        "proxy support", "tor support", "user agent", "referer", "cookie", "authentication",
        "basic auth", "digest auth", "ntlm auth", "certificate auth", "form based",
        "crawl website", "google dorks", "shodan dorks", "censys dorks"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("exploitdb".to_string(), vec![
        "exploit search", "exploit database", "searchsploit", "cve search", "edb-id",
        "exploit code", "proof of concept", "poc", "vulnerability exploitation", "0day",
        "remote code execution", "rce", "local privilege escalation", "lpe", "denial of service",
        "dos", "distributed dos", "ddos", "buffer overflow", "heap overflow", "stack overflow",
        "format string", "race condition", "use after free", "double free", "integer overflow",
        "directory traversal", "path traversal", "command injection", "code injection",
        "xss exploitation", "csrf exploitation", "ssrf exploitation", "xxe exploitation"
    ].iter().map(|s| s.to_string()).collect());

    // PASSWORD ATTACKS
    index.insert("john".to_string(), vec![
        "password cracking", "john the ripper", "hash cracking", "dictionary attack",
        "brute force", "incremental mode", "external mode", "markov mode", "prince mode",
        "mask attack", "hybrid attack", "wordlist", "rules", "pot file", "session",
        "restore session", "show cracked", "unshadow", "single crack", "format detection",
        "opencl", "cuda", "gpu acceleration", "distributed cracking", "fork mode",
        "node mode", "mpi support", "regex mode", "subsets mode", "mangling rules"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("hashcat".to_string(), vec![
        "gpu cracking", "hashcat", "password recovery", "brute force gpu", "dictionary gpu",
        "mask attack gpu", "hybrid attack gpu", "rule based gpu", "combinator gpu",
        "prince gpu", "fingerprint attack", "permutation attack", "table lookup attack",
        "toggle case", "toggle at position", "reverse string", "duplicate string",
        "duplicate block", "rotate left", "rotate right", "append character", "prepend character",
        "truncate at position", "replace character", "purge character", "duplicate character",
        "extract memory", "memory collision", "context attack", "slow candidates",
        "distributed cracking", "oclhashcat", "cuda hashcat", "intel opencl"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("hydra".to_string(), vec![
        "online cracking", "brute force online", "password spraying", "credential stuffing",
        "parallel attacks", "http post", "http get", "https", "http proxy", "http basic auth",
        "http ntlm auth", "http digest auth", "form based auth", "cisco auth", "ftp auth",
        "ssh auth", "telnet auth", "smtp auth", "pop3 auth", "imap auth", "ldap auth",
        "smb auth", "mssql auth", "mysql auth", "postgres auth", "oracle auth", "vnc auth",
        "rdp auth", "socks5 auth", "teamspeak auth", "sip auth", "xmpp auth", "icq auth",
        "irc auth", "radmin auth", "pcanywhere auth", "rexec auth", "rlogin auth", "rsh auth",
        "ssl vpn", "owa auth", "adam6500 auth", "cisco enable", "cisco aaa", "afp auth",
        "ncp auth", "rexec auth", "rlogin auth", "rsh auth", "s7 auth", "firebird auth",
        "saas auth", "directadmin auth", "asterisk auth", "cvss auth", "dicom auth", "pilot auth",
        "iax2 auth", "tr064 auth", "sep auth", "vtiger auth", "cherokee auth", "chap auth",
        "crammd5 auth", "digestmd5 auth", "login auth", "plain auth", "scram auth", "ntlm auth",
        "oauth2 auth", "radius auth", "kerberos auth", "spnego auth", "gssapi auth"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("crunch".to_string(), vec![
        "wordlist generation", "password list", "character set", "pattern generation",
        "min length", "max length", "charset", "numeric", "lowercase", "uppercase",
        "symbols", "space", "custom charset", "pattern", "literal", "placeholder",
        "@ = lowercase", ", = uppercase", "% = numbers", "^ = symbols", "literal text",
        "duplicate removal", "sort output", "compress output", "bzip2", "gzip", "lzma",
        "stdout", "file output", "resume generation", "checkpoint", "progress indicator",
        "verbose mode", "quiet mode", "version", "help", "man page"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("patator".to_string(), vec![
        "multi-purpose brute", "brute force framework", "module system", "http brute",
        "ftp brute", "ssh brute", "telnet brute", "smtp brute", "pop brute", "imap brute",
        "ldap brute", "smb brute", "mssql brute", "mysql brute", "oracle brute", "postgres brute",
        "vnc brute", "rdp brute", "rlogin brute", "rexec brute", "rsh brute", "svn brute",
        "smbclient brute", "snmp brute", "ike brute", "unzip brute", "keystore brute",
        "sqlcipher brute", "zip brute", "pdf brute", "rar brute", "7z brute", "gzip brute",
        "bzip2 brute", "openssl brute", "tcp connect", "dummy module", "eval module",
        "file module", "dns module", "finger module", "ftp_client module", "http_fuzz module",
        "http_post module", "imap_login module", "imap_search module", "ip module", "ipv6 module",
        "ldap_search module", "ldap_bind module", "memcached module", "mssql_login module",
        "mysql_login module", "mysql_query module", "netbios_ns module", "netbios_ssn module",
        "oracle_login module", "oracle_query module", "pop_login module", "pop_passd module",
        "postgres_login module", "postgres_query module", "rdp_login module", "rlogin_login module",
        "rsync module", "rexec_login module", "rsh_login module", "rtsp_url module", "sip_invite module",
        "sip_register module", "sip_options module", "smb_login module", "smb_lookupsid module",
        "smb_version module", "smtp_vrfy module", "smtp_rcpt module", "smtp_login module",
        "snmp_login module", "socks_proxy module", "ssh_login module", "ssh_key module",
        "tcp_syn module", "telnet_login module", "tftp module", "unzip_pass module",
        "vmware_authd module", "vnc_login module", "x11 module", "zip_pass module"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("medusa".to_string(), vec![
        "parallel brute force", "massive threading", "login cracker", "password cracker",
        "online attacks", "http mod", "ftp mod", "ssh mod", "telnet mod", "smtp mod",
        "pop3 mod", "imap mod", "ldap mod", "smbnt mod", "mssql mod", "mysql mod",
        "postgres mod", "oracle mod", "vnc mod", "rsh mod", "rexec mod", "rlogin mod",
        "cvs mod", "svn mod", "icq mod", "xmpp mod", "sip mod", "pcanywhere mod",
        "teamspeak mod", "socks5 mod", "ncp mod", "afp mod", "s7 mod", "firebird mod",
        "saas mod", "directadmin mod", "asterisk mod", "cvss mod", "dicom mod",
        "pilot mod", "iax2 mod", "tr064 mod", "sep mod", "vtiger mod", "cherokee mod",
        "chap mod", "crammd5 mod", "digestmd5 mod", "login mod", "plain mod", "scram mod",
        "ntlm mod", "oauth2 mod", "radius mod", "kerberos mod", "spnego mod", "gssapi mod",
        "parallel processing", "threading", "resume attacks", "password file", "user file",
        "combo file", "host file", "port specification", "timeout", "delay", "retries",
        "verbose", "debug", "quiet", "progress", "statistics", "log file", "error log"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("ncrack".to_string(), vec![
        "network auth cracker", "parallel cracking", "service cracking", "rlogin cracking",
        "rsh cracking", "rexec cracking", "telnet cracking", "ftp cracking", "ssh cracking",
        "http cracking", "https cracking", "pop3 cracking", "imap cracking", "smtp cracking",
        "smb cracking", "vnc cracking", "sip cracking", "redis cracking", "postgresql cracking",
        "mysql cracking", "mssql cracking", "oracle cracking", "mongodb cracking", "cassandra cracking",
        "couchdb cracking", "couchbase cracking", "elasticsearch cracking", "memcached cracking",
        "clamav cracking", "irc cracking", "dict cracking", "daytime cracking", "echo cracking",
        "chargen cracking", "qotd cracking", "time cracking", "finger cracking", "ident cracking",
        "ntp cracking", "snmp cracking", "ldap cracking", "ldaps cracking", "radius cracking",
        "kerberos cracking", "rdp cracking", "winrm cracking", "wmi cracking", "rpc cracking",
        "java rmi cracking", "ajp cracking", "xmpp cracking", "dns cracking", "tftp cracking",
        "rtsp cracking", "ipp cracking", "cups cracking", "afp cracking", "bacnet cracking",
        "modbus cracking", "dnp3 cracking", "enip cracking", "fox cracking", "s7 cracking",
        "omron cracking", "pcworx cracking", "proconos cracking", "codesys cracking", "iec104 cracking",
        "multiprocessing", "timing templates", "connection limit", "rate limit", "service detection",
        "ssl support", "proxy support", "resume attacks", "save state", "load state", "verbose",
        "debug", "quiet", "statistics", "log file", "error log", "output format"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("cewl".to_string(), vec![
        "custom wordlist", "website crawling", "spider", "depth control", "external links",
        "internal links", "email addresses", "usernames", "metadata", "word extraction",
        "lowercase", "uppercase", "leet speak", "case insensitive", "no words", "min word length",
        "max word length", "wordlist output", "verbose", "debug", "quiet", "help", "version",
        "auth user", "auth pass", "proxy host", "proxy port", "proxy user", "proxy pass",
        "user agent", "cookie", "header", "keepalive", "timeout", "delay", "max connections",
        "url blacklist", "url whitelist", "save cookies", "load cookies", "follow redirects",
        "max redirects", "convert to utf8", "no robots", "allow robots", "meta temp dir",
        "meta file", "meta temp", "groups", "count", "offsite", "write", "depth", "show words",
        "with numbers", "ua", "auth_type", "auth", "proxy_auth", "proxy", "delay", "timeout",
        "max_page_size", "min_word_length", "max_word_length", "no_words", "lowercase", "uppercase",
        "capitalize", "leet", "offsite", "write", "depth", "show_words", "with_numbers", "ua",
        "auth_type", "auth", "proxy_auth", "proxy", "delay", "timeout", "max_page_size",
        "min_word_length", "max_word_length", "no_words", "lowercase", "uppercase", "capitalize",
        "leet", "offsite", "write", "depth", "show_words", "with_numbers"
    ].iter().map(|s| s.to_string()).collect());

    // WEB APPLICATIONS
    index.insert("burpsuite".to_string(), vec![
        "web proxy", "intercept proxy", "repeater", "intruder", "decoder", "comparer",
        "sequencer", "extender", "scanner", "collaborator", "clickbandit", "csrf poc",
        "dom invader", "macro recorder", "content discovery", "active scan", "passive scan",
        "sql injection scan", "xss scan", "csrf scan", "xxe scan", "ssrf scan", "command injection scan",
        "path traversal scan", "file inclusion scan", "deserialization scan", "expression language scan",
        "server side template scan", "client side template scan", "header injection scan",
        "http request smuggling scan", "http response splitting scan", "cache poisoning scan",
        "graphQL scan", "json web token scan", "oauth scan", "open redirection scan",
        "subdomain takeover scan", "cloud storage scan", "backup file scan", "directory listing scan",
        "exposed database scan", "exposed git scan", "exposed svn scan", "exposed configuration scan",
        "weak password scan", "default credentials scan", "exposed session scan", "session fixation scan",
        "insecure direct object reference scan", "mass assignment scan", "parameter pollution scan",
        "http parameter pollution scan", "race condition scan", "insecure deserialization scan",
        "xml external entity scan", "xpath injection scan", "ldap injection scan", "nosql injection scan",
        "crlf injection scan", "host header injection scan", "user agent injection scan",
        "referer injection scan", "origin injection scan", "x forwarded for injection scan",
        "x real ip injection scan", "x orig url injection scan", "x rewrite url injection scan",
        "forwarded injection scan", "x host injection scan", "x forwarded host injection scan",
        "x forwarded proto injection scan", "x url scheme injection scan", "x original url injection scan",
        "x rewrite url injection scan", "x original host injection scan", "x forwarded server injection scan",
        "x http method override injection scan", "x method override injection scan",
        "x json injection scan", "x www form urlencoded injection scan", "x xml injection scan",
        "x yaml injection scan", "x toml injection scan", "x ini injection scan", "x csv injection scan",
        "x tsv injection scan", "x properties injection scan", "x query injection scan",
        "x fragment injection scan", "x path injection scan", "x scheme injection scan",
        "x authority injection scan", "x userinfo injection scan", "x host injection scan",
        "x port injection scan", "x path injection scan", "x query injection scan",
        "x fragment injection scan", "authorization injection scan", "proxy authorization injection scan",
        "cookie injection scan", "set cookie injection scan", "content type injection scan",
        "content length injection scan", "transfer encoding injection scan", "content encoding injection scan",
        "accept injection scan", "accept charset injection scan", "accept encoding injection scan",
        "accept language injection scan", "accept datetime injection scan", "cache control injection scan",
        "pragma injection scan", "via injection scan", "from injection scan", "max forwards injection scan",
        "range injection scan", "if match injection scan", "if none match injection scan",
        "if modified since injection scan", "if unmodified since injection scan",
        "if range injection scan", "connection injection scan", "upgrade injection scan",
        "trailer injection scan", "te injection scan", "expect injection scan", "host injection scan",
        "x requested with injection scan", "dnt injection scan", "x csrf token injection scan",
        "x xsrf token injection scan", "csrf token injection scan", "xsrf token injection scan",
        "x frame options injection scan", "x content type options injection scan",
        "x xss protection injection scan", "content security policy injection scan",
        "x content security policy injection scan", "strict transport security injection scan",
        "public key pins injection scan", "x powered by injection scan", "server injection scan",
        "x aspnet version injection scan", "x runtime injection scan", "x version injection scan",
        "x powered by injection scan", "server injection scan", "x aspnet version injection scan",
        "x runtime injection scan", "x version injection scan", "x rack cache injection scan",
        "x runtime injection scan", "x version injection scan", "x rack cache injection scan",
        "x sentry trace injection scan", "x request id injection scan", "x correlation id injection scan",
        "x trace id injection scan", "x span id injection scan", "x parent span id injection scan",
        "x sampled injection scan", "x b3 traceid injection scan", "x b3 spanid injection scan",
        "x b3 parentspanid injection scan", "x b3 sampled injection scan", "x b3 flags injection scan",
        "x ot span context injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection pan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan", "x b3 traceid injection scan",
        "x b3 spanid injection scan", "x b3 parentspanid injection scan", "x b3 sampled injection scan",
        "x b3 flags injection scan", "x ot span context injection scan", "x request id injection scan",
        "x correlation id injection scan", "x trace id injection scan", "x span id injection scan",
        "x parent span id injection scan", "x sampled injection scan"
    ].iter().map(|s| s.to_string()).collect());


    // ============================================================================
    // MISSING KALI TOOLS - ADDING FOR COMPLETE INDEX
    // ============================================================================

    // Information Gathering - Additional Tools
    index.insert("amass".to_string(), vec![
        "subdomain enumeration", "attack surface mapping", "dns enumeration", "asset discovery",
        "domain reconnaissance", "passive reconnaissance", "active reconnaissance",
        "dns record collection", "certificate transparency", "search engine discovery",
        "graph database", "network topology", "asn discovery", "internet scanning",
        "subdomain brute force", "alteration discovery", "cert log", "ct log"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("dnsenum".to_string(), vec![
        "dns enumeration", "subdomain brute force", "zone transfer", "dns record",
        "dns query", "dns lookup", "name server", "mx record", "txt record",
        "soa record", "ns record", "cname record", "dns reconnaissance",
        "domain information", "host discovery", "network mapping"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("dnsrecon".to_string(), vec![
        "dns reconnaissance", "dns enumeration", "zone transfer", "axfr",
        "dns record query", "srv record", "dnssec", "wildcard resolution",
        "subdomain brute force", "google scraping", "dns dictionary",
        "std query", "rrl query", "dns cache snooping", "host lookup",
        "bind version", "dns chaos", "dns fingerprinting"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("fierce".to_string(), vec![
        "dns enumeration", "subdomain brute force", "zone transfer", "dns reconnaissance",
        "locate non-contiguous ip space", "host discovery", "domain scan",
        "dns probe", "name server lookup", "mx record lookup", "ptr lookup",
        "recursive query", "dns shift", "permutation scan", "wordlist scan"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("maltego".to_string(), vec![
        "osint", "open source intelligence", "link analysis", "visual analysis",
        "data mining", "reconnaissance", "information gathering", "relationship mapping",
        "social network analysis", "entity resolution", "data visualization",
        "transform", "machine", "pattern recognition", "threat intelligence",
        "investigation", "forensics", "domain correlation", "email correlation"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("sherlock".to_string(), vec![
        "username search", "osint", "social media enumeration", "user investigation",
        "username lookup", "account discovery", "platform search", "username checker",
        "social network", "digital footprint", "user reconnaissance", "profile finder",
        "username enumeration", "account lookup", "social platforms", "identity search"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("dnsx".to_string(), vec![
        "dns toolkit", "dns query", "dns lookup", "subdomain enumeration", "dns resolver",
        "dns bruteforce", "dns wildcard", "dns record", "dns probe",
        "fast dns", "multi threaded dns", "dns enumeration", "dns reconnaissance",
        "dns validation", "dns monitoring", "dns stats", "dns output"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("email2phonenumber".to_string(), vec![
        "email to phone", "osint", "phone enumeration", "email investigation",
        "phone lookup", "number discovery", "contact enumeration", "email reconnaissance",
        "phone osint", "number search", "email search", "phone number finder",
        "contact discovery", "mobile number", "phone lookup", "email to mobile"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("nuclei".to_string(), vec![
        "vulnerability scanner", "template based scanning", "yaml templates", "vuln detection",
        "security scanner", "cve scan", "exposure scan", "misconfig detection",
        "port scan", "web scan", "network scan", "automated scanning",
        "vulnerability assessment", "security assessment", "bug bounty", "penetration testing",
        "scan engine", "template engine", "headless browser", "protocol detection"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("openvas".to_string(), vec![
        "vulnerability scanner", "vulnerability assessment", "security testing",
        "compliance scanning", "full scan", "comprehensive scan", "vuln detection",
        "cve detection", "security audit", "penetration testing", "security assessment",
        "network security", "web application scan", "database scan", "service detection",
        "os detection", "vulnerability management", "scan report", "risk assessment"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("wpscan".to_string(), vec![
        "wordpress scanner", "wordpress security", "vulnerability scan", "wp plugin scan",
        "wp theme scan", "wordpress enumeration", "wp user enumeration", "wp brute force",
        "wordpress audit", "wp security audit", "wordpress penetration testing",
        "wp vuln", "wordpress vulnerabilities", "plugin enumeration", "theme enumeration",
        "timthumbs scan", "wp config backup", "wp xmlrpc", "wp login brute"
    ].iter().map(|s| s.to_string()).collect());

    // Wireless Attacks
    index.insert("aircrack-ng".to_string(), vec![
        "wifi audit", "wpa crack", "wep crack", "wpa2 crack", "handshake capture",
        "wireless cracking", "wifi password", "wpa handshake", "wps crack", "wifi attack",
        "monitor mode", "packet capture", "injection", "deauth", "airplay",
        "wifi key", "wireless security", "wpa enterprise", "pmkid", "hcxdumptool"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("airgeddon".to_string(), vec![
        "wifi attack", "wpa attack", "wep attack", "wps attack", "evil twin",
        "wifi phishing", "captive portal", "deauth attack", "beacon flood",
        "wireless penetration", "wifi hacking", "wpa2 attack", "handshake capture",
        "pmkid attack", "wps pin", "dos attack", "channel hopping", "air crack"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("bully".to_string(), vec![
        "wps crack", "wps pin", "wps brute force", "wifi attack", "wps attack",
        "pixie dust", "wps pin recovery", "wps brute", "wireless attack",
        "wifi password", "wps exploit", "wps vulnerability", "pin crack",
        "wps pixie", "wps null pin", "wifi cracking", "router pin"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("wifite".to_string(), vec![
        "wifi attack", "automated wifi", "wpa attack", "wep attack", "wps attack",
        "wifi hacking", "wireless attack", "wpa2 attack", "handshake capture",
        "wps capture", "wifi cracking", "automated cracking", "wifi audit",
        "wpa enterprise", "eviltwin", "wifi password", "masscan wifi", "wifi scanner"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("pixiewps".to_string(), vec![
        "wps pixie dust", "wps crack", "wps offline", "wps pin recovery",
        "wifi attack", "wps exploit", "pixie dust attack", "wps vulnerability",
        "wireless crack", "wps offline crack", "pin recovery", "wps brute",
        "wifi pixie", "wps pin crack", "offline wps", "wps attack"
    ].iter().map(|s| s.to_string()).collect());

    // Web Applications - Additional Tools
    index.insert("dirsearch".to_string(), vec![
        "directory scanner", "dir brute force", "path discovery", "web enumeration",
        "directory enumeration", "file enumeration", "web scanner", "hidden files",
        "web directory", "path scan", "dir fuzzing", "web discovery",
        "directory listing", "hidden path", "web enumeration", "url brute",
        "extension search", "recursive scan", "multi threading"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("ffuf".to_string(), vec![
        "web fuzzer", "directory fuzzing", "parameter fuzzing", "vhost discovery",
        "web fuzzing", "fast fuzzer", "http fuzzing", "brute force",
        "fuzzing tool", "web discovery", "directory brute", "parameter brute",
        "header fuzzing", "post fuzzing", "get fuzzing", "vhost enumeration",
        "rate limit", "multi threading", "wordlist filtering"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("gobuster".to_string(), vec![
        "directory brute force", "dns brute force", "subdomain enumeration", "web scanner",
        "dir busting", "directory enumeration", "subdomain busting", "vhost busting",
        "web discovery", "path discovery", "url brute", "dir scanner",
        "dns scanner", "subdomain scanner", "s3 bucket", "azure storage", "gcs bucket"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("xsser".to_string(), vec![
        "xss scanner", "cross site scripting", "xss detection", "xss exploitation",
        "web vulnerability", "xss attack", "reflected xss", "stored xss",
        "dom xss", "xss payload", "xss injection", "javascript injection",
        "xss testing", "xss audit", "xss brute", "xss fuzzing", "blind xss"
    ].iter().map(|s| s.to_string()).collect());

    // Exploitation - Additional Tools
    index.insert("msfvenom".to_string(), vec![
        "payload generator", "metasploit payload", "shellcode", "reverse shell",
        "bind shell", "meterpreter", "payload creation", "exe payload",
        "dll payload", "apk payload", "macho payload", "elf payload",
        "script payload", "web payload", "shellcode generator", "encoder",
        "msf payload", "exploit payload", "backdoor", "trojan"
    ].iter().map(|s| s.to_string()).collect());

    // Sniffing & Spoofing - Additional Tools
    index.insert("wireshark".to_string(), vec![
        "packet analyzer", "network analyzer", "packet capture", "protocol analysis",
        "network sniffer", "traffic analysis", "packet inspection", "network forensics",
        "protocol decoder", "deep inspection", "network troubleshooting", "traffic capture",
        "pcap", "network monitor", "protocol analyzer", "network security"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("tcpdump".to_string(), vec![
        "packet capture", "network sniffer", "packet analyzer", "command line capture",
        "network capture", "traffic capture", "packet filtering", "bpf",
        "network troubleshooting", "protocol analysis", "packet inspection",
        "network monitor", "traffic analysis", "pcap", "network forensics"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("ettercap".to_string(), vec![
        "mitm", "man in the middle", "arp spoofing", "dns spoofing", "ssl strip",
        "packet sniffer", "network injection", "arp poison", "mitm attack",
        "network sniffing", "password sniffing", "session hijacking", "arp cache poison",
        "dns spoof", "packet filter", "network bridging", "content filtering"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("bettercap".to_string(), vec![
        "mitm", "network attack", "wifi attack", "bluetooth attack", "hid attack",
        "network sniffer", "proxy", "arp spoof", "dns spoof", "packet injection",
        "wifi monitoring", "bluetooth monitoring", "network reconnaissance", "network manipulation",
        "blueSnap", "ble module", "wifi module", "ethernet module", "http proxy"
    ].iter().map(|s| s.to_string()).collect());

    // Forensics
    index.insert("autopsy".to_string(), vec![
        "forensic platform", "disk analysis", "file recovery", "deleted files",
        "digital forensics", "forensic analysis", "timeline analysis", "keyword search",
        "web cache", "cookie analysis", "history analysis", "email extraction",
        "exif data", "file carving", "registry analysis", "memory dump", "thumbnail cache"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("sleuthkit".to_string(), vec![
        "forensic tools", "disk analysis", "file system analysis", "digital forensics",
        "volume analysis", "file recovery", "deleted files", "timeline",
        "filesystem forensics", "partition analysis", "inode analysis", "data carving",
        "file metadata", "mactime", "fls", "ils", "blkls"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("volatility".to_string(), vec![
        "memory analysis", "ram forensics", "memory dump", "process list", "network connections",
        "memory forensics", "volatile memory", "ram analysis", "memory extraction",
        "process memory", "kernel memory", "memory dump analysis", "malware analysis",
        "windows memory", "linux memory", "memory image", "timeline", "dll injection"
    ].iter().map(|s| s.to_string()).collect());

    // Post-Exploitation
    index.insert("impacket".to_string(), vec![
        "windows exploitation", "active directory", "pass the hash", "pass the ticket",
        "windows attack", "smb attack", "ldap attack", "kerberos attack",
        "wmi execution", "mssql exec", "secretsdump", "golden ticket", "silver ticket",
        "psexec", "wmiexec", "smbexec", "atexec", "mimikatz"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("responder".to_string(), vec![
        "llmnr poisoning", "nbtns poisoning", "mitm", "credential capture", "hash capture",
        "windows network", "smb relay", "http relay", "ldap relay", "password capture",
        "ntlm capture", "network poisoning", "wpad spoof", "nbns spoof",
        "multi replay", "smb attack", "windows authentication", "ad attack"
    ].iter().map(|s| s.to_string()).collect());

    // Reverse Engineering
    index.insert("ghidra".to_string(), vec![
        "reverse engineering", "decompiler", "disassembler", "binary analysis", "malware analysis",
        "code analysis", "binary reverse", "patch analysis", "firmware analysis",
        "disassembly", "decompilation", "assembly code", "machine code",
        "function graph", "control flow", "data flow", "symbolic execution"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("radare2".to_string(), vec![
        "reverse engineering", "disassembler", "debugger", "hex editor", "binary analysis",
        "malware analysis", "code analysis", "binary patching", "shellcode analysis",
        "disassembly", "assembly", "debugging", "hexdump", "binary exploitation",
        "scriptable", "r2 pipe", "visual mode", "graph view"
    ].iter().map(|s| s.to_string()).collect());

    // Hardware
    index.insert("proxmark3".to_string(), vec![
        "rfid", "nfc", "rfid cloning", "contactless", "rfid sniffing",
        "card cloning", "mifare", "hf tag", "lf tag", "rfid attack",
        "nfc relay", "rfid emulation", "card reader", "rfid write", "rfid read",
        "proxmark", "rfid tool", "nfc tool", "access control", "hardware hack"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("hackrf".to_string(), vec![
        "sdr", "software defined radio", "rf", "radio", "frequency", "spectrum",
        "rf capture", "rf transmit", "signal analysis", "frequency scanner",
        "gsm", "lte", "rfid", "gps", "fm radio", "digital radio", "amateur radio",
        "antenna", "rf hack", "sdr tool", "signal intelligence"
    ].iter().map(|s| s.to_string()).collect());

    // ============================================================================
    // ADDITIONAL ESSENTIAL KALI TOOLS
    // ============================================================================

    // Network Utilities
    index.insert("netcat".to_string(), vec![
        "netcat", "nc", "network connector", "port listener", "reverse shell",
        "bind shell", "port scanner", "file transfer", "backdoor",
        "network utility", "tcp udp", "network connection", "banner grab",
        "port forwarding", "chat server", "network debugging", "socket"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("socat".to_string(), vec![
        "socat", "network relay", "port forward", "bidirectional", "data transfer",
        "reverse shell", "bind shell", "proxy", "network bridge",
        "encrypt tunnel", "ssl tunnel", "socket relay", "protocol converter",
        "serial port", "pty", "network pipe", "data interceptor"
    ].iter().map(|s| s.to_string()).collect());

    // Privilege Escalation
    index.insert("linpeas".to_string(), vec![
        "privilege escalation", "linux privesc", "enumeration", "misconfiguration",
        "linux audit", "security audit", "vulnerability scan", "suid scan",
        "cron jobs", "path hijack", "kernel exploit", "permission check",
        "linux post-exploitation", "reconnaissance", "information gathering", "privesc"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("winpeas".to_string(), vec![
        "privilege escalation", "windows privesc", "enumeration", "misconfiguration",
        "windows audit", "security audit", "vulnerability scan", "registry scan",
        "services", "permissions", "kernel exploit", "post exploitation",
        "windows enumeration", "information gathering", "privesc", "active directory"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("pspy".to_string(), vec![
        "process monitoring", "linux monitoring", "cron monitoring", "uid less",
        "privilege escalation", "process enumeration", "silent monitoring", "forensics",
        "background process", "automation", "cron jobs", "systemd timer",
        "linux security", "process sniffing", "no root required"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("enum4linux".to_string(), vec![
        "smb enumeration", "windows enumeration", "samba", "active directory",
        "network share", "smb audit", "windows recon", "share enumeration",
        "user enumeration", "group enumeration", "password policy", "smb vulnerability",
        "linux smb", "windows network", "network reconnaissance"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("linux-exploit-suggester".to_string(), vec![
        "linux exploit", "kernel exploit", "privilege escalation", "vulnerability suggest",
        "exploit recommendation", "linux privesc", "kernel version", "security advisory",
        "cve exploit", "privilege escalation suggester", "exploit database", "linux kernel"
    ].iter().map(|s| s.to_string()).collect());

    // Exploitation Tools
    index.insert("searchsploit".to_string(), vec![
        "exploit search", "exploit database", "edb", "cve search", "vulnerability search",
        "exploit code", "poc", "proof of concept", "shellcode", "exploit download",
        "security exploit", "vulnerability exploit", "0day", "privilege escalation",
        "remote exploit", "local exploit", "shellcode database"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("mimikatz".to_string(), vec![
        "password extraction", "windows credential", "pass the hash", "golden ticket",
        "silver ticket", "lsass", "credential dump", "windows security", "kerberos",
        "ntlm hash", "plaintext password", "memory dump", "windows authentication",
        "active directory", "lsa", "sam database", "wdigest"
    ].iter().map(|s| s.to_string()).collect());

    // Web Analysis
    index.insert("whatweb".to_string(), vec![
        "web scanner", "web fingerprint", "technology detection", "cms detection",
        "framework detection", "web enumeration", "website analysis", "plugin detection",
        "server version", "web tech stack", "web reconnaissance", "cms identification",
        "javascript library", "analytics", "tracking", "web server identification"
    ].iter().map(|s| s.to_string()).collect());

    index.insert("dotdotpwn".to_string(), vec![
        "path traversal", "directory traversal", "lfi", "rfi", "file inclusion",
        "web vulnerability", "fuzzer", "traversal fuzzing", "dot dot slash",
        "directory traversal scanner", "path traversal exploitation", "file disclosure",
        "lfi scanner", "rfi scanner", "web fuzzing"
    ].iter().map(|s| s.to_string()).collect());

    // Archive Tools
    index.insert("fcrackzip".to_string(), vec![
        "zip crack", "password recovery", "archive crack", "zip password",
        "brute force", "dictionary attack", "zip encryption", "file recovery",
        "password unlock", "archive unlock", "zip file", "password recovery tool"
    ].iter().map(|s| s.to_string()).collect());

    // Password/Wordlist Tools
    index.insert("keys.txt".to_string(), vec![
        "wordlist", "password list", "dictionary", "credential wordlist",
        "brute force wordlist", "password dictionary", "common passwords",
        "rockyou", "password file", "wordlist collection", "default passwords",
        "leaked passwords", "credential list", "password database"
    ].iter().map(|s| s.to_string()).collect());

    index
}