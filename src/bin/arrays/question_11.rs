// Implemente a busca binária manualmente em um vetor ordenado.

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut lo: isize = 0;
    let mut hi: isize = arr.len() as isize - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let val = arr[mid as usize];
        if val == target {
            return Some(mid as usize);
        } else if val < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}

fn main() {
    let sorted = vec![-5, 0, 1, 3, 7, 9, 12];
    let targets = vec![3, 4, -5, 12];
    for t in targets {
        match binary_search(&sorted, t) {
            Some(i) => println!("{} encontrado no índice {}", t, i),
            None => println!("{} não encontrado", t),
        }
    }
}
