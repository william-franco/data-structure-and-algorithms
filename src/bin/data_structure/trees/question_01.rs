//! Binary Search Tree (BST)
//!
//! Implements a BST with insert, search, and remove operations using `Option<Box<No>>`.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

struct ArvoreBinariaBusca {
    raiz: Option<Box<No>>,
}

impl ArvoreBinariaBusca {
    fn new() -> Self {
        ArvoreBinariaBusca { raiz: None }
    }

    fn inserir(&mut self, valor: i32) {
        self.raiz = Self::inserir_rec(self.raiz.take(), valor);
    }

    fn inserir_rec(no: Option<Box<No>>, valor: i32) -> Option<Box<No>> {
        match no {
            None => Some(Box::new(No {
                valor,
                esquerda: None,
                direita: None,
            })),
            Some(mut n) => {
                if valor < n.valor {
                    n.esquerda = Self::inserir_rec(n.esquerda.take(), valor);
                } else if valor > n.valor {
                    n.direita = Self::inserir_rec(n.direita.take(), valor);
                }
                Some(n)
            }
        }
    }

    fn buscar(&self, valor: i32) -> bool {
        Self::buscar_rec(self.raiz.as_ref(), valor)
    }

    fn buscar_rec(no: Option<&Box<No>>, valor: i32) -> bool {
        match no {
            None => false,
            Some(n) => {
                if valor == n.valor {
                    true
                } else if valor < n.valor {
                    Self::buscar_rec(n.esquerda.as_ref(), valor)
                } else {
                    Self::buscar_rec(n.direita.as_ref(), valor)
                }
            }
        }
    }

    fn remover(&mut self, valor: i32) {
        self.raiz = Self::remover_rec(self.raiz.take(), valor);
    }

    fn remover_rec(no: Option<Box<No>>, valor: i32) -> Option<Box<No>> {
        match no {
            None => None,
            Some(mut n) => {
                if valor < n.valor {
                    n.esquerda = Self::remover_rec(n.esquerda.take(), valor);
                } else if valor > n.valor {
                    n.direita = Self::remover_rec(n.direita.take(), valor);
                } else {
                    match (n.esquerda.take(), n.direita.take()) {
                        (None, None) => return None,
                        (Some(l), None) | (None, Some(l)) => return Some(l),
                        (Some(l), Some(mut r)) => {
                            let mut curr = &mut r;
                            while curr.esquerda.is_some() {
                                curr = curr.esquerda.as_mut().unwrap();
                            }
                            let min_val = curr.valor;
                            n.valor = min_val;
                            n.esquerda = Some(l);
                            n.direita = Self::remover_rec(Some(r), min_val);
                        }
                    }
                }
                Some(n)
            }
        }
    }
}

fn main() {
    let mut bst = ArvoreBinariaBusca::new();
    for v in [50, 30, 70, 20, 40, 60, 80] {
        bst.inserir(v);
    }
    assert!(bst.buscar(40));
    assert!(!bst.buscar(100));
    bst.remover(30);
    assert!(!bst.buscar(30));
    println!("BST operations completed");
}
