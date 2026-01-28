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

    index
}