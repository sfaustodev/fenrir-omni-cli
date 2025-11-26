// 🎯 TAREFINHA MODE - Sistema profissional de pedidos
// Garçom Claudão atendendo o chefe com maestria

use crate::task_management::chain_coordinator::ChainOfCaralhoManager;
use crate::task_management::task::{Complexity, Priority};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TarefinhaOrder {
    pub order_id: String,
    pub customer_request: String,
    pub order_type: OrderType,
    pub complexity_level: Complexity,
    pub urgency: Priority,
    pub ingredients: Vec<String>,
    pub preparation_style: PreparationStyle,
    pub special_requests: Vec<String>,
    pub estimated_preparation_time: u16,
    pub chef_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Morder,  // Ataque externo
    Rosnar,  // Scan defensivo
    Devorar, // Engenharia reversa
    GodMode, // Modo divino completo
    Special, // Personalizado
    Combo,   // Vários juntos
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreparationStyle {
    Mild,       // Suave
    Medium,     // Moderado
    Spicy,      // Picante
    ExtraSpicy, // Extra picante
    HellFire,   // Fogo do inferno
    Custom,     // Personalizado
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTarefinha {
    pub id: String,
    pub title: String,
    pub description: String,
    pub assigned_to: String,
    pub estimated_time: u16,
    pub dependencies: Vec<String>,
    pub execution_order: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TarefinhaMenu {
    pub appetizers: Vec<String>,
    pub main_courses: Vec<String>,
    pub desserts: Vec<String>,
    pub special_combinations: Vec<String>,
    pub chef_recommendations: Vec<String>,
}

pub struct TarefinhaMode {
    pub chain_manager: ChainOfCaralhoManager,
    pub current_order: Option<TarefinhaOrder>,
    pub order_history: Vec<TarefinhaOrder>,
    pub menu: TarefinhaMenu,
}

impl TarefinhaMode {
    pub fn new() -> Self {
        Self {
            chain_manager: ChainOfCaralhoManager::new(),
            current_order: None,
            order_history: vec![],
            menu: Self::create_menu(),
        }
    }

    /// 🎯 INICIAR MODO GARÇOM CLAUDÃO
    pub async fn start_interactive_mode(&mut self) -> Result<()> {
        println!("\n🎯🍽️  FENRIR TAREFINHA MODE - GARÇOM CLAUDÃO 🍽️🎯");
        println!("👨‍🍳 Bem-vindo chefe! Claudão à sua disposição!");
        println!("💎 Cardápio exclusivo com as melhores tarefinhas da casa");
        println!("🔥 Nossa especialidade: decompor o impossível em unidades atômicas");
        println!("");

        self.chain_manager.start_weekly_scheduler();

        self.show_welcome_menu();

        loop {
            print!("🎯 Tarefinha> ");
            io::stdout().flush()?;

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() {
                        continue;
                    }

                    match input.to_lowercase().as_str() {
                        "cardapio" | "menu" | "menu" => {
                            self.show_menu();
                        }
                        "chef" | "chefe" | "garcom" => {
                            self.call_garcom();
                        }
                        "histórico" | "historico" | "orders" => {
                            self.show_order_history();
                        }
                        "sair" | "exit" | "quit" => {
                            println!("\n👋 Claudão: Foi um prazer servir, chefe! Volte sempre!");
                            println!("🔥 Tarefinha Mode encerrando com elegância...");
                            break;
                        }
                        _ if input.starts_with("nova")
                            || input.starts_with("-new")
                            || input.starts_with("new") =>
                        {
                            let description = input
                                .strip_prefix("nova")
                                .unwrap_or(input)
                                .strip_prefix("-new")
                                .unwrap_or(input)
                                .strip_prefix("new")
                                .unwrap_or("")
                                .trim();

                            if description.is_empty() {
                                self.take_detailed_order().await?;
                            } else {
                                self.process_quick_order(description).await?;
                            }
                        }
                        _ => {
                            // Tratar como pedido rápido
                            self.process_quick_order(input).await?;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Erro: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// 📋 MENU DE BOAS-VINDAS
    fn show_welcome_menu(&self) {
        println!("🎯 COMANDOS DISPONÍVEIS:");
        println!("   📋 cardapio        - Ver nosso menu especial");
        println!("   👨‍🍳 garçom          - Chamar Claudão para recomendações");
        println!("   📝 nova <pedido>  - Fazer novo pedido rápido");
        println!("   📝 -new <pedido>  - Fazer novo pedido detalhado");
        println!("   📚 histórico        - Ver pedidos anteriores");
        println!("   🚪 sair            - Encerrar atendimento");
        println!("");

        println!("💭 EXEMPLOS:");
        println!("   nova morder bitcoin2000 com extra invasao");
        println!("   -new devorar explorer.exe estilo godmode");
        println!("   rosnar sistema interno completo");
        println!("");
    }

    /// 🍽️ MOSTRAR CARDÁPIO COMPLETO
    fn show_menu(&self) {
        println!("\n🍽️  CARDÁPIO FENRIR TAREFINHA 🍽️");
        println!("{}", "═".repeat(60));

        println!("\n🥗 ENTRADAS (Appetizers):");
        for (i, item) in self.menu.appetizers.iter().enumerate() {
            println!("   {}. {}", i + 1, item);
        }

        println!("\n🍖 PRATOS PRINCIPAIS (Main Courses):");
        for (i, item) in self.menu.main_courses.iter().enumerate() {
            println!("   {}. {}", i + 1, item);
        }

        println!("\n🍰 SOBREMESAS (Desserts):");
        for (i, item) in self.menu.desserts.iter().enumerate() {
            println!("   {}. {}", i + 1, item);
        }

        println!("\n🔥 COMBOS ESPECIAIS:");
        for (i, item) in self.menu.special_combinations.iter().enumerate() {
            println!("   {}. {}", i + 1, item);
        }

        println!("\n👨‍🍳 RECOMENDAÇÕES DO CHEF:");
        for (i, item) in self.menu.chef_recommendations.iter().enumerate() {
            println!("   {}. {}", i + 1, item);
        }

        println!("{}", "═".repeat(60));
        println!("💭 Dica: Use 'nova <número>' para pedir do menu ou descreva seu pedido!");
    }

    /// 👨‍🍳 CHAMAR GARÇOM CLAUDÃO
    fn call_garcom(&self) {
        println!("\n👨‍🍳 CLAUDÃO se aproximando da mesa...");
        println!("🎯 Olá chefe! Claudão aqui para servir!");

        println!("\n💎 RECOMENDAÇÕES DE HOJE:");
        println!("   🔥 O 'Combo Devil's Breakfast' está excelente!");
        println!("   🥊 O 'Mordida Infernal' veio bem temperado hoje");
        println!("   💀 O 'God Mode Supremo' é nossa especialidade da casa");

        println!("\n🤔 O que o chefe está com vontade hoje?");
        println!("   🎯 Algo rápido e direto?");
        println!("   🍽️ Uma experiência completa?");
        println!("   🔥 Quer experimentar nosso nível máximo?");

        println!("\n💭 Apenas diga 'nova' seguido do que deseja!");
        println!("   Ex: nova combo devil com extra picante");
    }

    /// 📚 MOSTRAR HISTÓRICO DE PEDIDOS
    fn show_order_history(&self) {
        if self.order_history.is_empty() {
            println!("\n📚 Nenhum pedido no histórico ainda!");
            println!("🎯 Este é seu primeiro dia conosco, chefe!");
            return;
        }

        println!("\n📚 HISTÓRICO DE PEDIDOS");
        println!("{}", "═".repeat(50));

        for (i, order) in self.order_history.iter().enumerate() {
            println!("\n🎯 PEDIDO #{}", i + 1);
            println!("   📝 Requisição: {}", order.customer_request);
            println!("   🔥 Tipo: {:?}", order.order_type);
            println!("   ⚡ Urgência: {:?}", order.urgency);
            println!("   🌶️ Estilo: {:?}", order.preparation_style);
            println!(
                "   ⏱️ Tempo estimado: {} min",
                order.estimated_preparation_time
            );

            if !order.ingredients.is_empty() {
                println!("   🧪 Ingredientes: {}", order.ingredients.join(", "));
            }

            if !order.special_requests.is_empty() {
                println!(
                    "   🎯 Pedidos especiais: {}",
                    order.special_requests.join(", ")
                );
            }
        }

        println!("{}", "═".repeat(50));
    }

    /// 🎯 PROCESSAR PEDIDO RÁPIDO
    async fn process_quick_order(&mut self, description: &str) -> Result<()> {
        println!("\n🎯 Claudão anotando seu pedido...");
        println!("📝 '{}'", description);

        // Analisar e decompor o pedido
        let parsed_order = self.parse_customer_request(description)?;

        println!("🤔 Claudão analisando: {:?}", parsed_order.order_type);
        println!("⚡ Urgência detectada: {:?}", parsed_order.urgency);
        println!(
            "⏱️ Tempo estimado: {} minutos",
            parsed_order.estimated_preparation_time
        );

        // Confirmar com o chefe
        println!("\n✅ Pedido identificado! Confirma preparação? (s/n)");
        print!("🎯 Confirmar> ");
        io::stdout().flush()?;

        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;

        if confirmation.trim().to_lowercase() == "s" {
            println!("\n🔥 Claudão indo para a cozinha!");
            println!("👨‍🍳 Preparando '{}' com maestria!", description);

            // Preparar a tarefinha
            self.prepare_tarefinha(&parsed_order).await?;
        } else {
            println!("\n😕 Pedido cancelado. Claudão aguarda suas instruções!");
        }

        Ok(())
    }

    /// 🎯 PEGAR PEDIDO DETALHADO
    async fn take_detailed_order(&mut self) -> Result<()> {
        println!("\n🎯 Claudão pronto para anotar seu pedido detalhado!");

        let mut order = TarefinhaOrder {
            order_id: format!(
                "order_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            customer_request: String::new(),
            order_type: OrderType::Special,
            complexity_level: Complexity::Senior,
            urgency: Priority::Medium,
            ingredients: vec![],
            preparation_style: PreparationStyle::Medium,
            special_requests: vec![],
            estimated_preparation_time: 30,
            chef_notes: String::new(),
        };

        // Coletar informações detalhadas
        println!("📝 Descreva exatamente o que deseja:");
        print!("🎯 Requisição> ");
        io::stdout().flush()?;
        let mut request = String::new();
        io::stdin().read_line(&mut request)?;
        order.customer_request = request.trim().to_string();

        println!("\n🔥 Qual o nível de picância?");
        println!("   1. Mild (Suave)");
        println!("   2. Medium (Moderado)");
        println!("   3. Spicy (Picante)");
        println!("   4. ExtraSpicy (Extra picante)");
        println!("   5. HellFire (Fogo do inferno)");
        print!("🎯 Picância (1-5)> ");
        io::stdout().flush()?;
        let mut spice = String::new();
        io::stdin().read_line(&mut spice)?;

        order.preparation_style = match spice.trim() {
            "1" => PreparationStyle::Mild,
            "2" => PreparationStyle::Medium,
            "3" => PreparationStyle::Spicy,
            "4" => PreparationStyle::ExtraSpicy,
            "5" => PreparationStyle::HellFire,
            _ => PreparationStyle::Custom,
        };

        println!("\n⚡ Qual a urgência?");
        println!("   1. Low (Sem pressa)");
        println!("   2. Medium (Normal)");
        println!("   3. High (Urgente)");
        println!("   4. Critical (EMERGÊNCIA!)");
        print!("🎯 Urgência (1-4)> ");
        io::stdout().flush()?;
        let mut urgency = String::new();
        io::stdin().read_line(&mut urgency)?;

        order.urgency = match urgency.trim() {
            "1" => Priority::Low,
            "2" => Priority::Medium,
            "3" => Priority::High,
            "4" => Priority::Critical,
            _ => Priority::Medium,
        };

        println!("\n🎯 Algum pedido especial ou ingrediente extra?");
        print!("🎯 Especiais (separados por vírgula)> ");
        io::stdout().flush()?;
        let mut specials = String::new();
        io::stdin().read_line(&mut specials)?;

        if !specials.trim().is_empty() {
            order.special_requests = specials
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        // Analisar e preparar
        println!("\n🤔 Claudão analisando seu pedido detalhado...");
        let parsed_order = self.parse_detailed_order(&order);

        println!("✅ Pedido complexo identificado!");
        println!("🔥 Tipo: {:?}", parsed_order.order_type);
        println!("⚡ Urgência: {:?}", parsed_order.urgency);
        println!("🌶️ Estilo: {:?}", parsed_order.preparation_style);
        println!(
            "⏱️ Tempo estimado: {} minutos",
            parsed_order.estimated_preparation_time
        );

        if !parsed_order.special_requests.is_empty() {
            println!("🎯 Especiais: {}", parsed_order.special_requests.join(", "));
        }

        println!("\n✅ Confirma esta obra-prima? (s/n)");
        print!("🎯 Confirmar> ");
        io::stdout().flush()?;
        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;

        if confirmation.trim().to_lowercase() == "s" {
            println!("\n🔥 Claudão indo para a cozinha com maestria!");
            println!("👨‍🍳 Preparando obra-prima do chefe!");

            self.prepare_tarefinha(&parsed_order).await?;
        } else {
            println!("\n😕 Obra-prima cancelada. Claudão refaz quando quiser!");
        }

        Ok(())
    }

    /// 🧠 PARSER INTELIGENTE DE PEDIDOS
    fn parse_customer_request(&self, request: &str) -> Result<TarefinhaOrder> {
        let mut order = TarefinhaOrder {
            order_id: format!(
                "order_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            customer_request: request.to_string(),
            order_type: OrderType::Special,
            complexity_level: Complexity::Senior,
            urgency: Priority::Medium,
            ingredients: vec![],
            preparation_style: PreparationStyle::Medium,
            special_requests: vec![],
            estimated_preparation_time: 30,
            chef_notes: "Parseado automaticamente pelo Garçom Claudão".to_string(),
        };

        // Detectar tipo de operação
        if request.to_lowercase().contains("morder") {
            order.order_type = OrderType::Morder;
            order.complexity_level = Complexity::Pleno;
        } else if request.to_lowercase().contains("rosnar") {
            order.order_type = OrderType::Rosnar;
            order.complexity_level = Complexity::Junior;
        } else if request.to_lowercase().contains("devorar") {
            order.order_type = OrderType::Devorar;
            order.complexity_level = Complexity::Senior;
        } else if request.to_lowercase().contains("godmode")
            || request.to_lowercase().contains("god mode")
        {
            order.order_type = OrderType::GodMode;
            order.complexity_level = Complexity::GodMode;
        }

        // Detectar urgência e estilo
        if request.to_lowercase().contains("urgente") || request.to_lowercase().contains("rápido")
        {
            order.urgency = Priority::High;
            order.preparation_style = PreparationStyle::Spicy;
        } else if request.to_lowercase().contains("extra")
            || request.to_lowercase().contains("super")
        {
            order.preparation_style = PreparationStyle::ExtraSpicy;
        } else if request.to_lowercase().contains("máximo")
            || request.to_lowercase().contains("inferno")
        {
            order.preparation_style = PreparationStyle::HellFire;
        }

        // Extrair ingredientes (alvos)
        let words: Vec<&str> = request.split_whitespace().collect();
        for word in words {
            if word.len() > 3
                && ![
                    "morder", "rosnar", "devorar", "com", "para", "modo", "urgente",
                ]
                .contains(&word.to_lowercase().as_str())
            {
                order.ingredients.push(word.to_string());
            }
        }

        // Calcular tempo baseado na complexidade
        order.estimated_preparation_time = match order.complexity_level {
            Complexity::Junior => 15,
            Complexity::Pleno => 30,
            Complexity::Senior => 45,
            Complexity::GodMode => 60,
        };

        // Ajustar tempo baseado no estilo
        match order.preparation_style {
            PreparationStyle::HellFire => order.estimated_preparation_time *= 2,
            PreparationStyle::ExtraSpicy => {
                order.estimated_preparation_time =
                    (order.estimated_preparation_time as f32 * 1.5) as u16
            }
            _ => {}
        }

        Ok(order)
    }

    /// 🎯 PARSER DE PEDIDO DETALHADO
    fn parse_detailed_order(&self, detailed_order: &TarefinhaOrder) -> TarefinhaOrder {
        let mut order = detailed_order.clone();

        // Analisar a requisição principal para detectar tipo
        let request_lower = order.customer_request.to_lowercase();

        if request_lower.contains("morder") {
            order.order_type = OrderType::Morder;
            order.complexity_level = Complexity::Pleno;
        } else if request_lower.contains("rosnar") {
            order.order_type = OrderType::Rosnar;
            order.complexity_level = Complexity::Junior;
        } else if request_lower.contains("devorar") {
            order.order_type = OrderType::Devorar;
            order.complexity_level = Complexity::Senior;
        } else if request_lower.contains("god") || request_lower.contains("combo") {
            order.order_type = OrderType::Combo;
            order.complexity_level = Complexity::GodMode;
        }

        // Ajustar tempo baseado na urgência
        match order.urgency {
            Priority::Critical => order.estimated_preparation_time /= 2,
            Priority::High => {
                order.estimated_preparation_time =
                    (order.estimated_preparation_time as f32 * 0.75) as u16
            }
            Priority::Low => order.estimated_preparation_time *= 2,
            _ => {}
        }

        order
    }

    /// 🔥 PREPARAR A TAREFINHA NA COZINHA
    async fn prepare_tarefinha(&mut self, order: &TarefinhaOrder) -> Result<()> {
        println!("\n🔥 CLAUDÃO NA COZINHA!");
        println!("👨‍🍳 Preparando: {}", order.customer_request);

        // Criar batch de tarefinhas
        let goal = format!(
            "Executar: {} ({:?})",
            order.customer_request, order.order_type
        );

        let batch_id = self.chain_manager.create_batch_from_goal(goal)?;

        if let Some(id) = batch_id {
            if let Some(last) = self.chain_manager.caderninhos.last() {
                println!(
                    "📋 {} tarefinhas criadas pelo Chef Claudão!",
                    last.tarefinhas.len()
                );
            }

            println!("\n🎯 EXECUTANDO TAREFINHAS SINCRONAS:");
            self.chain_manager.process_batch(&id).await?;
        }

        println!("\n⚡ EXECUTANDO PILHA ASYNC PARA O RESTO:");
        self.chain_manager.process_pilha_async().await?;

        // Salvar no histórico
        self.order_history.push(order.clone());

        println!("\n✅ OBRA-PRIMA PRONTA!");
        println!("🔥 Claudão orgulhoso do resultado!");
        println!("📊 Performance: Ver dashboard completo");

        // Mostrar resultados finais
        self.chain_manager.show_dashboard();

        Ok(())
    }

    /// 🍽️ CRIAR MENU ESPECIALIZADO
    fn create_menu() -> TarefinhaMenu {
        TarefinhaMenu {
            appetizers: vec![
                "Rosnada Leve - Scan defensivo rápido".to_string(),
                "Mordida Teste - Ataque externo básico".to_string(),
                "Devorada Simples - Análise rápida".to_string(),
                "God Mode Light - Teste de divindade".to_string(),
            ],
            main_courses: vec![
                "Morder Agressiva - Invasão completa".to_string(),
                "Rosnada Profunda - Scan avançado".to_string(),
                "Devorada Total - Engenharia reversa".to_string(),
                "God Mode Pleno - Dominação absoluta".to_string(),
            ],
            desserts: vec![
                "Relatório do Sucesso - Documentação vitória".to_string(),
                "Cleanup Master - Limpeza perfeita".to_string(),
                "Persistência Eterna - Acesso garantido".to_string(),
            ],
            special_combinations: vec![
                "Combo Devil's Breakfast - Morder + Rosnar + Devorar".to_string(),
                "God Mode Supremo - Todas operações divinas".to_string(),
                "Triple Threat - Ataque em 3 fases".to_string(),
                "Inferno Complete - Pacote serviço completo".to_string(),
            ],
            chef_recommendations: vec![
                "O combo Devil's Breakfast é nossa especialidade".to_string(),
                "God Mode Supremo para clientes exigentes".to_string(),
                "Inferno Complete para projetos ambiciosos".to_string(),
                "Experimente nossos níveis de picância progressivos".to_string(),
            ],
        }
    }
}
