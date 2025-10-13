// Simule uma fila de atendimento bancário (chegada e atendimento).

use std::collections::VecDeque;

#[derive(Debug)]
struct Customer {
    id: usize,
    arrival: u32,
}

fn main() {
    let mut rng_seed = 0u64;
    // gerador linear simples (não-crypto) para determinismo
    let mut simple_rand = || -> u32 {
        rng_seed = rng_seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (rng_seed >> 32) as u32
    };

    let mut queue: VecDeque<Customer> = VecDeque::new();
    let mut clock: u32 = 0;
    let simulation_time = 60; // 60 unidades de tempo
    let mut next_customer_id = 1;
    let service_time = 3; // cada atendimento dura 3 unidades
    let mut service_remaining = 0;
    let mut served_count = 0;
    let mut total_wait = 0u32;
    let mut arrival_times: Vec<u32> = Vec::new();

    while clock < simulation_time {
        // chegada com ~30% chance
        if simple_rand() % 100 < 30 {
            queue.push_back(Customer {
                id: next_customer_id,
                arrival: clock,
            });
            println!("t={} chegada cliente {}", clock, next_customer_id);
            next_customer_id += 1;
        }

        if service_remaining == 0 {
            if let Some(c) = queue.pop_front() {
                // iniciar atendimento
                service_remaining = service_time;
                let wait = clock - c.arrival;
                println!(
                    "t={} iniciando atendimento cliente {} (esperou {})",
                    clock, c.id, wait
                );
                served_count += 1;
                total_wait += wait;
                arrival_times.push(wait);
            }
        }

        // avançar 1 unidade de tempo
        if service_remaining > 0 {
            service_remaining -= 1;
        }
        clock += 1;
    }

    let avg_wait = if served_count > 0 {
        total_wait as f64 / served_count as f64
    } else {
        0.0
    };
    println!(
        "Simulação finalizada. Atendidos: {}, espera média: {:.2}",
        served_count, avg_wait
    );
}
