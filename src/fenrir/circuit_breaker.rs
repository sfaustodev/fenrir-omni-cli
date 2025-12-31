use std::time::{Duration, Instant};

/// Circuit breaker simples para RPCs.
#[derive(Debug)]
pub struct CircuitBreaker {
    failures: u32,
    threshold: u32,
    open_until: Option<Instant>,
    cooldown: Duration,
}

impl CircuitBreaker {
    /// Cria um breaker com limite e cooldown.
    pub fn new(threshold: u32, cooldown: Duration) -> Self {
        Self {
            failures: 0,
            threshold,
            open_until: None,
            cooldown,
        }
    }

    /// Retorna true se pode executar.
    pub fn allow(&self) -> bool {
        match self.open_until {
            Some(until) => Instant::now() >= until,
            None => true,
        }
    }

    /// Marca sucesso e fecha o circuito.
    pub fn success(&mut self) {
        self.failures = 0;
        self.open_until = None;
    }

    /// Marca falha e abre o circuito se necessário.
    pub fn failure(&mut self) {
        self.failures = self.failures.saturating_add(1);
        if self.failures >= self.threshold {
            self.open_until = Some(Instant::now() + self.cooldown);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breaker_opens_and_closes() {
        let mut breaker = CircuitBreaker::new(2, Duration::from_millis(10));
        assert!(breaker.allow());
        breaker.failure();
        assert!(breaker.allow());
        breaker.failure();
        assert!(!breaker.allow());
        std::thread::sleep(Duration::from_millis(12));
        assert!(breaker.allow());
        breaker.success();
        assert!(breaker.allow());
    }
}
