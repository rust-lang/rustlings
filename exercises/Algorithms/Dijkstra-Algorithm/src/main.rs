use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}
#[warn(clippy::redundant_closure)]
impl Graph {
    fn from_edge_list(edge_list: &[(Node, Node, Cost)]) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));
            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

// #[derive(Clone, PartialEq, Eq)]
#[derive(Eq, PartialEq)]
struct Curr {
    cost: Cost,
    position: Node,
    path: Vec<Node>,
}
impl Curr {
    fn new(position: Node, cost: Cost, path: Vec<Node>) -> Self {
        Curr {
            cost,
            position,
            path,
        }
    }
    fn new_start(start: Node) -> Self {
        Curr {
            cost: 0,
            position: start,
            path: vec![],
        }
    }
}


impl Ord for Curr {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Curr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Curr::new_start(start));

    let mut dist: HashMap<Node, Cost> = g
        .nodes
        .iter()
        .map(|&x| if x == start { (x, 0) } else { (x, usize::MAX) })
        .collect();

    while let Some(Curr {
        cost,
        position,
        mut path,
    }) = priority_queue.pop()
    {
        if position == goal {
            path.push(position);
            return Some((path, cost));
        }

        // pq. push (cost+ currcost, start->next, path.add(start) )
        // So iterate, through all the nodes of start node which are in g
        // g[start]= Vec[node, cost]

        if let Some(connected_nodes) = &g.edges.get(&position) {
            for &(next_node, next_cost) in connected_nodes.iter() {
                if next_cost < dist[&next_node] {
                    let mut next = Curr::new(next_node, cost + next_cost, path.clone());
                    next.path.push(position);
                    priority_queue.push(next);

                    if let Some(past_cost) = dist.get_mut(&next_node) {
                        *past_cost = next_cost;
                    }
                }
            }
        }
    }

    None
}

fn main() {
    let edge_list = include!("graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1, 6) {
        println!("Shortest distance from 1 to 6 is: , {path:?} {cost}");
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1, 6);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 20);
}
