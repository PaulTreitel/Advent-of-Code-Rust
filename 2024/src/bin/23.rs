use std::collections::HashSet;

use advent_of_code_2024::utils::graph_algos::{GWNodeIdx, GraphWrapper};
use petgraph::Undirected;

advent_of_code_2024::solution!(23);

type Network<'a> = GraphWrapper<&'a str, (), Undirected>;
type Triple<'a> = (&'a str, &'a str, &'a str);

pub fn part_one(input: &str) -> Option<u64> {
    let network = parse_input(input);
    let triples = get_triples(&network);
    let t_triples: Vec<_> = triples.iter()
        .filter(|x| has_t_computer(*x))
        .collect();
    Some(t_triples.len() as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let network = parse_input(input);
    let triples = get_triples(&network);
    let starter_triples: Vec<_> = triples.iter()
        .filter(|x| has_t_computer(*x))
        .collect();
    let mut largest_subgraph = HashSet::new();
    for starter in starter_triples {
        let starter_nodes = get_triple_nodes(&network, starter);
        let mut already_found = true;
        for &s_node in &starter_nodes {
            if !largest_subgraph.contains(&s_node) {
                already_found = false;
            }
        }
        if already_found {
            continue;
        }
        let subgraph = get_connected_subgraph(
            &network,
            starter,
            &triples
        );
        if subgraph.len() > largest_subgraph.len() {
            largest_subgraph = subgraph;
        }
    }
    let mut largest_subgraph: Vec<_> = largest_subgraph.iter()
        .map(|x| *network.node_weight(*x).unwrap())
        .collect();
    largest_subgraph.sort();
    Some(largest_subgraph.join(","))
}

fn parse_input(input: &str) -> Network {
    let mut connections = vec![];
    for l in input.lines() {
        let mut l = l.split('-');
        connections.push((
            l.next().unwrap(),
            l.next().unwrap(),
            ()
        ));
    }
    let mut nodes = HashSet::new();
    for c in &connections {
        nodes.insert(c.0);
        nodes.insert(c.1);
    }
    let nodes = nodes.iter().copied().collect();
    GraphWrapper::from_nodes_edges(nodes, connections)
}

fn get_connected_subgraph<'a>(
    network: &'a Network,
    base_triple: &Triple<'a>,
    all_triples: &HashSet<Triple<'a>>
) -> HashSet<GWNodeIdx> {
    let mut subgraph = HashSet::new();
    subgraph.extend(get_triple_nodes(network, base_triple));
    for triple in all_triples {
        let nodes = get_triple_nodes(network, triple);
        let mut can_join = true;
        for &subgraph_node in &subgraph {
            if subgraph_node == nodes[0] || subgraph_node == nodes[1] || subgraph_node == nodes[2] {
                continue;
            }
            if !(network.contains_edge(subgraph_node, nodes[0])
                && network.contains_edge(subgraph_node, nodes[1])
                && network.contains_edge(subgraph_node, nodes[2])
            ) {
                can_join = false;
                break;
            }
        }
        if can_join {
            subgraph.extend(nodes);
        }
    }
    subgraph
}

fn get_triple_nodes(network: &Network, triple: &(&str, &str, &str)) -> Vec<GWNodeIdx> {
    vec![
        *network.first_node_from_val(&triple.0).unwrap(),
        *network.first_node_from_val(&triple.1).unwrap(),
        *network.first_node_from_val(&triple.2).unwrap(),
    ]
}

fn get_triples<'a>(network: &'a Network) -> HashSet<Triple<'a>> {
    let mut triples = HashSet::new();
    for node in network.node_indices() {
        let neighbors: Vec<_> = network.neighbors(node).collect();
        for n in &neighbors {
            for second_n in network.neighbors(*n) {
                if neighbors.contains(&second_n) {
                    let mut triple = vec![
                        network.node_weight(node).copied().unwrap(),
                        network.node_weight(*n).copied().unwrap(),
                        network.node_weight(second_n).copied().unwrap(),
                    ];
                    triple.sort();
                    let triple = (triple[0], triple[1], triple[2]);
                    triples.insert(triple);
                }
            }
        }
    }
    triples
}

fn has_t_computer(triple: &(&str, &str, &str)) -> bool {
    triple.0.chars().next().unwrap() == 't'
    || triple.1.chars().next().unwrap() == 't'
    || triple.2.chars().next().unwrap() == 't'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2024::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
