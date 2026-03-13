use std::collections::VecDeque;

pub fn find_all_components(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut visited = vec![false; n]; 
    let mut components = Vec::new();

    for i in 0..n {
        if !visited[i] {
            let component = bfs_traverse(graph, i, &mut visited);
            components.push(component);
        }
    }

    components.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    components
}

fn bfs_traverse(graph: &Vec<Vec<usize>>, start: usize, visited: &mut Vec<bool>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut component = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        component.push(current);

        if let Some(neighbours) = graph.get(current) {
            for &neighbour in neighbours {
                if !visited[neighbour] {
                    visited[neighbour] = true;
                    queue.push_back(neighbour);
                }
            }
        }
    }
    component
}