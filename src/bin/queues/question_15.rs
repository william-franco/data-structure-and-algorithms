// Implemente uma fila que alterna atendimento entre dois tipos de clientes (VIP e normal).

use std::collections::VecDeque;

#[derive(Debug)]
struct Customer {
    id: usize,
}

fn main() {
    let mut vip: VecDeque<Customer> = VecDeque::new();
    let mut normal: VecDeque<Customer> = VecDeque::new();

    for i in 1..=3 {
        normal.push_back(Customer { id: i });
    }
    for i in 101..=103 {
        vip.push_back(Customer { id: i });
    }

    let mut serve_vip_next = true;
    let mut tick = 0;

    while !vip.is_empty() || !normal.is_empty() {
        tick += 1;
        if serve_vip_next {
            if let Some(c) = vip.pop_front() {
                println!("tick {}: atendendo VIP {}", tick, c.id);
            } else if let Some(c) = normal.pop_front() {
                println!("tick {}: VIP vazio, atendendo normal {}", tick, c.id);
            }
        } else {
            if let Some(c) = normal.pop_front() {
                println!("tick {}: atendendo normal {}", tick, c.id);
            } else if let Some(c) = vip.pop_front() {
                println!("tick {}: normal vazio, atendendo VIP {}", tick, c.id);
            }
        }
        serve_vip_next = !serve_vip_next;
    }

    println!("Atendimento finalizado em {} ticks", tick);
}
