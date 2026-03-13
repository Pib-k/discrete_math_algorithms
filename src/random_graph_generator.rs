use rand::{RngExt};

pub fn er_graph_adj(n: usize, p: f64) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    let mut rng = rand::rng();

    for i in 0..n {
        for j in (i + 1)..n {
            if rng.random_bool(p) {
                adj[i].push(j);
                adj[j].push(i);
            }
        }
    }
    adj
}