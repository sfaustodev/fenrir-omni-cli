use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub description: String,
    pub accepted: bool,
    pub reason: String,
    pub timestamp: u64,
}

struct PendingDecision {
    id: String,
    description: String,
    decider: Box<dyn FnOnce() -> bool + Send>,
}

pub struct DecisionEngine {
    pub log: Vec<Decision>,
    pending: VecDeque<PendingDecision>,
}

impl DecisionEngine {
    pub fn new() -> Self {
        Self {
            log: Vec::new(),
            pending: VecDeque::new(),
        }
    }

    pub fn propose<F>(&mut self, id: &str, desc: &str, decider: F)
    where
        F: FnOnce() -> bool + Send + 'static,
    {
        self.pending.push_back(PendingDecision {
            id: id.to_string(),
            description: desc.to_string(),
            decider: Box::new(decider),
        });
    }

    pub fn commit(&mut self, id: String, desc: String, accepted: bool, reason: String) {
        let decision = Decision {
            id: id.clone(),
            description: desc,
            accepted,
            reason,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.log.push(decision);
        log::info!(
            "DECISION -> ({}) {}",
            id,
            if accepted { "OK" } else { "NOK" }
        );
    }

    pub fn run_pending(&mut self) -> (usize, usize) {
        let mut accepted = 0;
        let mut rejected = 0;

        while let Some(pending) = self.pending.pop_front() {
            let result = (pending.decider)();
            if result {
                accepted += 1;
            } else {
                rejected += 1;
            }

            self.commit(
                pending.id,
                pending.description,
                result,
                "auto-run".to_string(),
            );
        }

        (accepted, rejected)
    }

    pub fn stats(&self) -> (usize, usize) {
        let accepted = self.log.iter().filter(|d| d.accepted).count();
        let rejected = self.log.len().saturating_sub(accepted);
        (accepted, rejected)
    }
}

#[cfg(test)]
mod tests {
    use super::DecisionEngine;

    #[test]
    fn test_decision_engine() {
        let mut brain = DecisionEngine::new();
        brain.propose("test1", "primeira decisão", || true);
        brain.propose("test2", "segunda decisão", || false);
        brain.run_pending();
        assert_eq!(brain.stats(), (1, 1));
    }
}
