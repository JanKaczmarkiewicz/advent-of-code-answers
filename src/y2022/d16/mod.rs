use std::collections::HashMap;

use crate::utils::read_lines;

pub fn answer() {
    println!("Answer to day16: {} {}", a1(), a2());
}

// Valve SW has flow rate=0; tunnels lead to valves LX, LD

fn parse_line(s: String) -> (String, usize, Vec<String>) {
    let raw = s
        .replace("Valve ", "")
        .replace(" has flow rate=", ",")
        .replace("; tunnels lead to valves ", ",")
        .replace("; tunnel leads to valve ", ",")
        .replace(" ", "");

    let mut val = raw.split(",").collect::<Vec<_>>();

    (
        val.remove(0).to_string(),
        val.remove(0).parse().unwrap(),
        val.into_iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    )
}

#[derive(Debug)]
struct TunnelWithCost {
    name: String,
    valve_stress_release: usize,
    paths: HashMap<String, usize>, // tunnel name : time cost to get there
}

// computes shortest paths for each non 0 value node + AA to each non 0 value node excluding itself
fn get_tunnels_map() -> HashMap<String, TunnelWithCost> {
    let tunnels = read_lines("src/y2022/d16/input")
        .map(parse_line)
        .collect::<Vec<_>>();

    let non_zero_tunnels = tunnels
        .iter()
        .filter(|(_, v, _)| *v > 0)
        .collect::<Vec<_>>();

    let mut tunnels_map = HashMap::new();

    let aa = tunnels.iter().find(|(n, _, _)| n == "AA").unwrap();

    for (tunnel_name, valve_stress_release, _) in non_zero_tunnels.iter().chain([&aa]) {
        let mut paths = HashMap::new();

        let mut current_depth_nodes = vec![tunnel_name.clone()];
        let mut depth: usize = 0;

        while current_depth_nodes.len() > 0 {
            for node in current_depth_nodes.iter() {
                paths.insert(node.clone(), depth);
            }

            let new_depth_nodes = current_depth_nodes
                .iter()
                .map(|node| {
                    tunnels
                        .iter()
                        .find(|(name, _, _)| name == node)
                        .map(|(_, _, child_paths)| child_paths.clone())
                        .unwrap()
                })
                .flatten()
                .filter(|child_node| !paths.contains_key(child_node))
                .collect::<Vec<_>>();

            current_depth_nodes = new_depth_nodes;
            depth += 1
        }

        let relevant_paths = HashMap::from_iter(
            non_zero_tunnels
                .iter()
                .filter(|(n, _, _)| n != tunnel_name)
                .map(|(n, _, _)| {
                    paths
                        .get_key_value(n)
                        .map(|(k, v)| (k.clone(), *v))
                        .unwrap()
                }),
        );

        tunnels_map.insert(
            tunnel_name.clone(),
            TunnelWithCost {
                name: tunnel_name.clone(),
                valve_stress_release: *valve_stress_release,
                paths: relevant_paths,
            },
        );
    }

    tunnels_map
}

// brutforce approach
fn a1() -> usize {
    let mut map = get_tunnels_map();

    let mut current_tunnel = map.remove("AA").unwrap();

    let mut time = 30;

    let mut score = 0;

    println!("AA:");
    while let Some(best_next_move) = map
        .iter()
        .map(|(tunnel_name, _)| {
            let curr_t = map.get(tunnel_name).unwrap();

            let time_when_moved = time - current_tunnel.paths.get(tunnel_name).unwrap() - 1;

            let tunnel_score = time_when_moved * curr_t.valve_stress_release;
            let system_score = map
                .iter()
                .filter(|other| other.0 != tunnel_name)
                .map(|(other, t)| {
                    (time_when_moved - curr_t.paths.get(other).unwrap()) * t.valve_stress_release
                })
                .sum::<usize>();

            println!(
                "  {tunnel_name}: {tunnel_score} {system_score} = {}",
                (system_score as f64 / tunnel_score as f64)
            );

            (
                tunnel_name.to_owned(),
                tunnel_score,
                (system_score + tunnel_score) / tunnel_score,
                time_when_moved,
            )
        })
        .max_by(|a, b| (a.2).cmp(&(b.2)))
    {
        time = best_next_move.3;
        score += best_next_move.1;
        current_tunnel = map.remove(&best_next_move.0).unwrap();
        println!("{}:", best_next_move.0);
    }

    score
    // map.values()
    //     .permutations(map.len())
    //     .map(|mut a| {
    //         a.insert(0, &aa);
    //         compute_path_score(a.iter().map(|a| *a))
    //     })
    //     .max()
    //     .unwrap()
}

// fn compute_path_score<'a>(path: impl Iterator<Item = &'a TunnelWithCost> + Clone) -> usize {
//     let mut time: i32 = 30;
//     let mut score = 0;

//     for (prev, curr) in path.clone().zip(path.skip(1)) {
//         time -= *prev.paths.get(&curr.name).unwrap() as i32 + 1; // one for turning on velve
//         if time < 0 {
//             break;
//         }
//         score += time as usize * curr.valve_stress_release;
//     }

//     score
// }

fn a2() -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_solve_first_problem() {
        // println!("{:?}", get_tunnels_map());
        assert_eq!(a1(), 6124805);
    }

    // #[test]
    // fn permutations_test() {
    //     let mut per = Permutations::new(vec![&1, &2, &3]);

    //     assert_eq!(per.next(), Some(&vec![&2, &1, &3]));
    //     assert_eq!(per.next(), Some(&vec![&2, &3, &1]));
    //     assert_eq!(per.next(), Some(&vec![&3, &2, &1]));
    //     assert_eq!(per.next(), Some(&vec![&3, &1, &2]));
    //     assert_eq!(per.next(), Some(&vec![&1, &3, &2]));
    //     assert_eq!(per.next(), Some(&vec![&1, &2, &3]));

    //     assert_eq!(per.next(), None);
    // }

    #[test]
    fn should_solve_second_problem() {
        assert_eq!(a2(), 12555527364986);
    }
}
