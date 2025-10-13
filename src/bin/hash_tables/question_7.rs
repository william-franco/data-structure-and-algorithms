// Realoque a tabela quando o fator de carga for maior que 0.75.

#[derive(Debug)]
struct HashTable {
    data: Vec<Vec<(usize, i32)>>,
    count: usize,
}

impl HashTable {
    fn new(size: usize) -> Self {
        Self {
            data: vec![Vec::new(); size],
            count: 0,
        }
    }

    fn hash(&self, key: usize) -> usize {
        key % self.data.len()
    }

    fn insert(&mut self, key: usize, value: i32) {
        if self.load_factor() > 0.75 {
            self.resize();
        }
        let idx = self.hash(key);
        self.data[idx].push((key, value));
        self.count += 1;
    }

    fn load_factor(&self) -> f64 {
        self.count as f64 / self.data.len() as f64
    }

    fn resize(&mut self) {
        let new_size = self.data.len() * 2;
        let mut new_data = vec![Vec::new(); new_size];
        for bucket in &self.data {
            for (k, v) in bucket {
                let new_index = k % new_size;
                new_data[new_index].push((*k, *v));
            }
        }
        self.data = new_data;
    }
}

fn main() {
    let mut t = HashTable::new(4);
    for i in 0..5 {
        t.insert(i, i as i32 * 10);
    }
    println!("Realoque: tamanho = {}, dados = {:?}", t.data.len(), t.data);
}
