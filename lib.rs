fn generate_pairs2(n: usize, k: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();

    for i in 1..=n {
        for j in (i + 1)..=std::cmp::min(i + k - 1, n) {
            pairs.push((i, j));
        }
    }

    pairs
}

fn generate_pairs(n: isize) -> Vec<(isize, isize)> {
    let mut pairs = Vec::new();
    for i in 1..=n {
        let min = n;
        let ma = i + 1;
        for j in ma..=min {
            pairs.push((i, j));
        }
    }
    pairs
}
