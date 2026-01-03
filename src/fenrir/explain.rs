use std::fs;
use std::path::Path;
use std::process::Command;

/// Módulo para explicar arquivos e trechos de código antes de ações.
pub struct CodeExplainer {
    ai_enabled: bool,
}

impl CodeExplainer {
    pub fn new() -> Self {
        // Check if AI is available (for now, just check if we can run basic commands)
        let ai_enabled = Command::new("which")
            .arg("curl")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        Self { ai_enabled }
    }

    /// Explica um arquivo antes de modificá-lo.
    pub fn explain_file(&self, file_path: &Path) -> anyhow::Result<String> {
        if !file_path.exists() {
            return Ok(format!("❌ Arquivo {} não encontrado", file_path.display()));
        }

        let content = fs::read_to_string(file_path)?;
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("txt");

        let language = self.detect_language(extension);

        let mut explanation = format!(
            "📄 {} Análise do arquivo: {}\n\n",
            "[EXPLAIN]".cyan(),
            file_path.display()
        );

        explanation.push_str(&format!("🔍 Tipo detectado: {}\n", language));
        explanation.push_str(&format!("📏 Tamanho: {} linhas\n\n", content.lines().count()));

        // Basic structural analysis
        explanation.push_str(&self.analyze_structure(&content, language));

        // Risk assessment
        explanation.push_str(&self.assess_risks(&content, language));

        // AI-powered explanation (if available)
        if self.ai_enabled {
            explanation.push_str("\n🤖 Análise IA:\n");
            explanation.push_str(&self.ai_explain(&content, language));
        }

        Ok(explanation)
    }

    /// Explica um trecho específico de código.
    pub fn explain_code_snippet(&self, code: &str, language: Option<&str>) -> String {
        let lang = language.unwrap_or("unknown");

        let mut explanation = format!("💻 Análise do trecho de código ({}):\n\n", lang);

        // Basic analysis
        explanation.push_str(&self.analyze_code_structure(code, lang));

        // Potential issues
        explanation.push_str(&self.identify_potential_issues(code, lang));

        // AI explanation
        if self.ai_enabled {
            explanation.push_str("\n🤖 Explicação IA:\n");
            explanation.push_str(&self.ai_explain_code(code, lang));
        }

        explanation
    }

    /// Detecta linguagem baseada na extensão do arquivo.
    fn detect_language(&self, extension: &str) -> &'static str {
        match extension {
            "rs" => "Rust",
            "py" => "Python",
            "js" => "JavaScript",
            "ts" => "TypeScript",
            "java" => "Java",
            "cpp" | "cc" | "cxx" => "C++",
            "c" => "C",
            "go" => "Go",
            "rb" => "Ruby",
            "php" => "PHP",
            "sh" | "bash" => "Shell Script",
            "yml" | "yaml" => "YAML",
            "json" => "JSON",
            "xml" => "XML",
            "md" => "Markdown",
            "sql" => "SQL",
            "html" => "HTML",
            "css" => "CSS",
            _ => "Texto simples",
        }
    }

    /// Analisa estrutura básica do arquivo.
    fn analyze_structure(&self, content: &str, language: &str) -> String {
        let lines = content.lines().collect::<Vec<_>>();
        let mut analysis = String::new();

        analysis.push_str("🏗️  Estrutura do arquivo:\n");

        match language {
            "Rust" => {
                let mut structs = 0;
                let mut functions = 0;
                let mut mods = 0;

                for line in &lines {
                    if line.trim().starts_with("struct ") { structs += 1; }
                    if line.trim().starts_with("fn ") { functions += 1; }
                    if line.trim().starts_with("mod ") { mods += 1; }
                }

                analysis.push_str(&format!("  • {} structs\n", structs));
                analysis.push_str(&format!("  • {} funções\n", functions));
                analysis.push_str(&format!("  • {} módulos\n", mods));
            }
            "Python" => {
                let mut classes = 0;
                let mut functions = 0;
                let mut imports = 0;

                for line in &lines {
                    if line.trim().starts_with("class ") { classes += 1; }
                    if line.trim().starts_with("def ") { functions += 1; }
                    if line.trim().starts_with("import ") || line.trim().starts_with("from ") { imports += 1; }
                }

                analysis.push_str(&format!("  • {} classes\n", classes));
                analysis.push_str(&format!("  • {} funções\n", functions));
                analysis.push_str(&format!("  • {} imports\n", imports));
            }
            "JavaScript" | "TypeScript" => {
                let mut functions = 0;
                let mut classes = 0;
                let mut imports = 0;

                for line in &lines {
                    if line.contains("function ") || line.contains("=>") { functions += 1; }
                    if line.trim().starts_with("class ") { classes += 1; }
                    if line.trim().starts_with("import ") || line.trim().starts_with("require(") { imports += 1; }
                }

                analysis.push_str(&format!("  • {} funções\n", functions));
                analysis.push_str(&format!("  • {} classes\n", classes));
                analysis.push_str(&format!("  • {} imports\n", imports));
            }
            _ => {
                analysis.push_str("  • Análise estrutural não disponível para este tipo de arquivo\n");
            }
        }

        analysis.push_str("\n");
        analysis
    }

    /// Avalia riscos potenciais no código.
    fn assess_risks(&self, content: &str, language: &str) -> String {
        let mut risks = Vec::new();

        // Check for hardcoded secrets
        if content.to_lowercase().contains("password") ||
           content.to_lowercase().contains("secret") ||
           content.to_lowercase().contains("token") {
            risks.push("⚠️  Possível presença de credenciais hardcoded");
        }

        // Check for unsafe operations
        match language {
            "Rust" => {
                if content.contains("unsafe ") {
                    risks.push("⚠️  Uso de código unsafe");
                }
            }
            "C" | "C++" => {
                if content.contains("malloc(") || content.contains("free(") {
                    risks.push("⚠️  Gerenciamento manual de memória");
                }
            }
            _ => {}
        }

        // Check for large files
        if content.lines().count() > 1000 {
            risks.push("⚠️  Arquivo muito grande - considere dividir em módulos menores");
        }

        let mut assessment = "🛡️  Avaliação de riscos:\n".to_string();
        if risks.is_empty() {
            assessment.push_str("  ✅ Nenhum risco óbvio identificado\n");
        } else {
            for risk in risks {
                assessment.push_str(&format!("  • {}\n", risk));
            }
        }

        assessment.push_str("\n");
        assessment
    }

    /// Analisa estrutura de um trecho de código.
    fn analyze_code_structure(&self, code: &str, language: &str) -> String {
        let mut analysis = "🔧 Análise estrutural:\n".to_string();

        match language {
            "Rust" => {
                if code.contains("fn ") {
                    analysis.push_str("  • Define uma função\n");
                }
                if code.contains("struct ") {
                    analysis.push_str("  • Define uma estrutura de dados\n");
                }
                if code.contains("impl ") {
                    analysis.push_str("  • Implementa métodos para um tipo\n");
                }
                if code.contains("match ") {
                    analysis.push_str("  • Usa pattern matching\n");
                }
            }
            "Python" => {
                if code.contains("def ") {
                    analysis.push_str("  • Define uma função\n");
                }
                if code.contains("class ") {
                    analysis.push_str("  • Define uma classe\n");
                }
                if code.contains("if ") {
                    analysis.push_str("  • Contém lógica condicional\n");
                }
                if code.contains("for ") || code.contains("while ") {
                    analysis.push_str("  • Contém loops\n");
                }
            }
            _ => {
                analysis.push_str("  • Análise específica não disponível\n");
            }
        }

        analysis.push_str("\n");
        analysis
    }

    /// Identifica possíveis problemas no código.
    fn identify_potential_issues(&self, code: &str, _language: &str) -> String {
        let mut issues = Vec::new();

        // Check for common issues
        if code.contains("TODO") || code.contains("FIXME") {
            issues.push("📝 Contém TODOs ou FIXMEs pendentes");
        }

        if code.contains("println!") || code.contains("console.log") {
            issues.push("🔍 Contém statements de debug (print/console.log)");
        }

        if code.lines().any(|line| line.len() > 120) {
            issues.push("📏 Algumas linhas são muito longas (>120 caracteres)");
        }

        let mut issue_str = "⚠️  Possíveis problemas:\n".to_string();
        if issues.is_empty() {
            issue_str.push_str("  ✅ Nenhum problema óbvio identificado\n");
        } else {
            for issue in issues {
                issue_str.push_str(&format!("  • {}\n", issue));
            }
        }

        issue_str.push_str("\n");
        issue_str
    }

    /// Explicação usando IA (simulada por enquanto).
    fn ai_explain(&self, content: &str, language: &str) -> String {
        // This is a placeholder for actual AI integration
        // In a real implementation, this would call an AI service

        match language {
            "Rust" => {
                if content.contains("async") {
                    "Este código usa async/await para operações assíncronas, permitindo execução não-bloqueante.\n".to_string()
                } else if content.contains("unsafe") {
                    "Contém blocos unsafe - cuidado com gerenciamento manual de memória e ponteiros.\n".to_string()
                } else {
                    "Código Rust que segue princípios de ownership e borrowing para segurança de memória.\n".to_string()
                }
            }
            "Python" => {
                if content.contains("import") {
                    "Importa bibliotecas externas - verifique dependências no requirements.txt.\n".to_string()
                } else {
                    "Código Python - lembre-se da importância da indentação correta.\n".to_string()
                }
            }
            _ => "Análise IA não disponível para este tipo de arquivo.\n".to_string()
        }
    }

    /// Explicação de trecho de código usando IA.
    fn ai_explain_code(&self, code: &str, language: &str) -> String {
        // Placeholder for AI code explanation
        match language {
            "Rust" => {
                if code.contains("Result<") {
                    "Retorna Result para tratamento de erros - boa prática de programação defensiva.\n".to_string()
                } else {
                    "Trecho de código Rust - foca em zero-cost abstractions e segurança.\n".to_string()
                }
            }
            _ => "Explicação IA detalhada não disponível.\n".to_string()
        }
    }
}

/// Explica um arquivo antes de modificá-lo.
pub fn explain_file(file_path: &str) -> anyhow::Result<String> {
    let explainer = CodeExplainer::new();
    explainer.explain_file(Path::new(file_path))
}

/// Explica um trecho de código.
pub fn explain_code_snippet(code: &str, language: Option<&str>) -> String {
    let explainer = CodeExplainer::new();
    explainer.explain_code_snippet(code, language)
}
