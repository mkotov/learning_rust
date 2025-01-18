// https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

type Node = (usize, usize);
type Graph = HashMap<Node, Vec<(Node, i32)>>;

fn dijkstra(graph: &Graph, start: &Node, target: &Node) -> Option<i32> {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));
    let mut visited: HashSet<Node> = HashSet::new();

    while let Some(Reverse((current_distance, current_node))) = queue.pop() {
        if visited.contains(&current_node) {
            continue;
        }
        visited.insert(current_node.clone());
        if current_node == target {
            return Some(current_distance);
        }
        if let Some(neighbors) = graph.get(&current_node) {
            for (neighbor, weight) in neighbors {
                if !visited.contains(neighbor) {
                    queue.push(Reverse((current_distance + weight, &neighbor)));
                }
            }
        }
    }
    None
}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut graph: Graph = HashMap::new();

        for i in 0..m {
            for j in 0..n {
                let mut neighbors = Vec::new();

                if j + 1 < n {
                    neighbors.push(((i, j + 1), if grid[i][j] == 1 { 0 } else { 1 }))
                }
                if j - 1 >= 0 {
                    neighbors.push(((i, j - 1), if grid[i][j] == 2 { 0 } else { 1 }))
                }
                if i + 1 < m {
                    neighbors.push(((i + 1, j), if grid[i][j] == 3 { 0 } else { 1 }))
                }
                if i - 1 >= 0 {
                    neighbors.push(((i - 1, j), if grid[i][j] == 4 { 0 } else { 1 }))
                }
                graph.insert((i, j), neighbors);
            }
        }

        if let Some(distance) = dijkstra(&graph, &(0, 0), &(m - 1, n - 1)) {
            return distance
        } else {
            unreachable!()
        }
    }
}
