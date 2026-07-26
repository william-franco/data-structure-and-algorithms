//! Browser History Simulation
//!
//! Simulates browser navigation history using two stacks (visit, back, forward).

struct HistoricoNavegador {
    atual: String,
    voltar: Vec<String>,
    avancar: Vec<String>,
}

impl HistoricoNavegador {
    fn new(pagina_inicial: &str) -> Self {
        HistoricoNavegador {
            atual: pagina_inicial.to_string(),
            voltar: Vec::new(),
            avancar: Vec::new(),
        }
    }

    fn visitar(&mut self, url: &str) {
        self.voltar.push(self.atual.clone());
        self.atual = url.to_string();
        self.avancar.clear();
    }

    fn voltar(&mut self) -> Option<&str> {
        if self.voltar.is_empty() {
            return None;
        }
        self.avancar.push(self.atual.clone());
        self.atual = self.voltar.pop().unwrap();
        Some(&self.atual)
    }

    fn avancar(&mut self) -> Option<&str> {
        if self.avancar.is_empty() {
            return None;
        }
        self.voltar.push(self.atual.clone());
        self.atual = self.avancar.pop().unwrap();
        Some(&self.atual)
    }

    fn pagina_atual(&self) -> &str {
        &self.atual
    }
}

fn main() {
    let mut browser = HistoricoNavegador::new("home.com");
    browser.visitar("google.com");
    browser.visitar("github.com");
    assert_eq!(browser.pagina_atual(), "github.com");
    browser.voltar();
    assert_eq!(browser.pagina_atual(), "google.com");
    browser.avancar();
    assert_eq!(browser.pagina_atual(), "github.com");
    println!("Current page: {}", browser.pagina_atual());
}
