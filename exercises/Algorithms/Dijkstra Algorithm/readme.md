## Dijsktra Algorithm in Rust 

The `src` directory consist of 2 files: `main.rs` and `graph.in`. The `graph.in` file consist of a `vec![]` which is a `vector` of data as: `(source, destination, cost)`. This graph is comprised of `6` nodes and their `cost`. 

The algorithm returns the shortest path between 2 nodes. The function `shortest_path(g: &Graph, start: Node, goal: Node)` takes start and end nodes and returns the total cost to reach the goal. 
