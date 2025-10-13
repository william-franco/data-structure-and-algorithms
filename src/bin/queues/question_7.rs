// Calcule o tempo médio de espera em uma fila de clientes.

use std::collections::VecDeque;

fn main() {
    let mut seed = 123456789u64;
    let mut rand_u32 = || -> u32 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (seed >> 32) as u32
    };

    let sim_time = 1000;
    let arrival_prob = 0.2; // probabilidade de chegada por tick
    let mut q: VecDeque<u32> = VecDeque::new();
    let mut clock = 0;
    let mut service_remaining = 0;
    let mut total_wait = 0u64;
    let mut served = 0u64;

    while clock < sim_time {
        if (rand_u32() % 1000) < (arrival_prob * 1000.0) as u32 {
            q.push_back(clock);
        }

        if service_remaining == 0 {
            if let Some(arrival) = q.pop_front() {
                let wait = (clock - arrival) as u64;
                total_wait += wait;
                served += 1;
                // tempo de serviço aleatório entre 1 e 5
                service_remaining = 1 + (rand_u32() % 5) as u32;
            }
        }

        if service_remaining > 0 {
            service_remaining -= 1;
        }
        clock += 1;
    }

    let avg_wait = if served > 0 {
        total_wait as f64 / served as f64
    } else {
        0.0
    };
    println!("Servidos: {}, espera média: {:.3}", served, avg_wait);
}
