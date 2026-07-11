use petgraph::{dot::Dot, graph::NodeIndex, visit::EdgeRef, Graph};

use crate::utils::read;
type G<'a> = Graph<&'a str, i32, petgraph::prelude::Undirected>;

fn merge_nodes<'a>(g: &'a mut G, node1: NodeIndex, node2: NodeIndex) {
    // add all edges between node1 and other nodes
    g.remove_edge(g.find_edge(node1, node2).unwrap());

    for (node_index, weight) in g
        .edges(node1)
        .map(|e| {
            assert_eq!(e.source(), node1);
            assert!(e.target() != node2);
            (e.target(), *e.weight())
        }) // Check if target is other node
        .collect::<Vec<_>>()
    {
        if let Some(old_edge) = g.find_edge(node2, node_index) {
            g.update_edge(node2, node_index, g[old_edge] + weight);
        } else {
            g.add_edge(node2, node_index, weight);
        }
    }

    g.remove_node(node1);
}

// std::fs::write(
//             format!("src/y2023/d25/dbg/{i}.dot"),
//             format!("{:?}", Dot::new(&g)),
//         )
//         .unwrap();

// fn add_or_get<'a>(graph: &'a mut G<'a>, name: &'a str) -> NodeIndex {
//     if let Some(idx) = graph.raw_nodes().iter().position(|x| x.weight == name) {
//         NodeIndex::new(idx)
//     } else {
//         graph.add_node(name)
//     }
// }
fn reduce_to_cut<'a>(g: &mut G, j: usize) {
    let mut current = g.node_indices().next().unwrap();
    let mut i = 0;
    std::fs::write(
        format!("src/y2023/d25/dbg/{j}.{i}.dot"),
        format!("{:?}", Dot::new(&g.clone())),
    )
    .unwrap();

    while g.node_count() > 2 {
        i += 1;
        let next = g
            .edges(current)
            .max_by_key(|e| e.weight())
            .map(|e| {
                if e.target() == current {
                    e.source()
                } else {
                    e.target()
                }
            })
            .unwrap();

        merge_nodes(g, current, next);
        current = next;
        std::fs::write(
            format!("src/y2023/d25/dbg/{j}.{i}.dot"),
            format!("{:?}", Dot::new(&g.clone())),
        )
        .unwrap();
    }
}

pub fn answer() -> i64 {
    let mut graph: Graph<&str, i32, petgraph::prelude::Undirected> = Graph::new_undirected();

    let file = read("src/y2023/d25/input");

    for line in file.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let left_n = graph
            .raw_nodes()
            .iter()
            .position(|x| x.weight == name)
            .map_or_else(|| graph.add_node(name), NodeIndex::new);

        for to in rest.split(' ') {
            let right_n = graph
                .raw_nodes()
                .iter()
                .position(|x| x.weight == to)
                .map_or_else(|| graph.add_node(to), NodeIndex::new);

            graph.add_edge(left_n, right_n, 1);
        }
    }

    let mut i = 0;

    std::fs::write(
        format!("src/y2023/d25/dbg/{i}.dot"),
        format!("{:?}", Dot::new(&graph)),
    )
    .unwrap();

    while graph.node_count() > 2 {
        let mut cut = graph.clone();
        reduce_to_cut(&mut cut, i);
        i += 1;

        let l = NodeIndex::new(
            graph
                .raw_nodes()
                .iter()
                .position(|x| x.weight == cut[NodeIndex::new(0)])
                .unwrap(),
        );

        let r = NodeIndex::new(
            graph
                .raw_nodes()
                .iter()
                .position(|x| x.weight == cut[NodeIndex::new(1)])
                .unwrap(),
        );

        merge_nodes(&mut graph, l, r);

        std::fs::write(
            format!("src/y2023/d25/dbg/{i}.dot"),
            format!("{:?}", Dot::new(&graph)),
        )
        .unwrap();
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_compute_solution() {
        assert_eq!(answer(), 0);
    }
}
