mod random_graph_generator;
use crate::random_graph_generator::*;
mod connected_components;
use crate::connected_components::*;

fn main() {
    let graph = er_graph_adj(10,0.2);

    for (v, neighbors) in graph.iter().enumerate() {
        println!("Vertex {}: {:?}", v, neighbors);
    }

    let components = find_all_components(&graph);

    for (v, neighbors) in components.iter().enumerate() {
        println!("Vertex {}: {:?}", v, neighbors);
    }
}
