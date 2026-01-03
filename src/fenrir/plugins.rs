use libloading::Library;
use std::collections::HashMap;
use std::path::Path;

/// Interface base de plugins Fenrir.
pub trait FenrirPlugin: Send + Sync {
    /// Nome do plugin.
    fn name(&self) -> &str;
    /// Descrição curta.
    fn description(&self) -> &str;
    /// Execução do plugin.
    fn run(&self, input: &str) -> anyhow::Result<String>;
}

type PluginCreate = unsafe fn() -> *mut dyn FenrirPlugin;

/// Registry simples de plugins.
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn FenrirPlugin>>,
    _libs: Vec<Library>,
}

impl PluginRegistry {
    /// Cria registry vazio.
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            _libs: Vec::new(),
        }
    }

    /// Registra plugin estático.
    pub fn register(&mut self, plugin: Box<dyn FenrirPlugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    /// Carrega plugin dinâmico.
    pub unsafe fn load_dynamic(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let lib = Library::new(path.as_ref())?;
        let constructor: libloading::Symbol<PluginCreate> = lib.get(b"fenrir_plugin_create")?;
        let boxed_raw = constructor();
        let plugin = Box::from_raw(boxed_raw);
        self.plugins
            .insert(plugin.name().to_string(), plugin);
        self._libs.push(lib);
        Ok(())
    }

    /// Lista plugins.
    pub fn list(&self) -> Vec<(String, String)> {
        self.plugins
            .values()
            .map(|p| (p.name().to_string(), p.description().to_string()))
            .collect()
    }

    /// Executa plugin.
    pub fn run(&self, name: &str, input: &str) -> anyhow::Result<String> {
        let plugin = self
            .plugins
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("plugin não encontrado"))?;
        plugin.run(input)
    }
}
