// Faça uma simulação de impressora com fila de trabalhos.

use std::collections::VecDeque;

#[derive(Debug)]
struct Job {
    id: usize,
    pages: u32,
}

fn main() {
    let mut queue: VecDeque<Job> = VecDeque::new();
    // gerar alguns jobs
    queue.push_back(Job { id: 1, pages: 5 });
    queue.push_back(Job { id: 2, pages: 2 });
    queue.push_back(Job { id: 3, pages: 7 });

    let speed = 2; // páginas por tick
    let mut tick = 0;
    let mut current_job: Option<Job> = None;
    let mut remaining_pages = 0;

    while !queue.is_empty() || current_job.is_some() {
        if current_job.is_none() {
            current_job = queue.pop_front();
            if let Some(j) = &current_job {
                remaining_pages = j.pages;
                println!(
                    "tick {}: iniciando job {} ({} páginas)",
                    tick, j.id, j.pages
                );
            }
        }

        if let Some(j) = &current_job {
            let work = speed.min(remaining_pages as i32) as u32;
            remaining_pages = remaining_pages.saturating_sub(work);
            println!(
                "tick {}: imprimindo job {} (restam {} páginas)",
                tick, j.id, remaining_pages
            );
            if remaining_pages == 0 {
                println!("tick {}: job {} finalizado", tick, j.id);
                current_job = None;
            }
        }

        tick += 1;
    }

    println!("Todos os jobs finalizados em tick {}", tick);
}
