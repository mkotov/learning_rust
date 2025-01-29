https://leetcode.com/problems/redundant-connection/description/

use std::collections::HashMap;
use std::collections::HashSet;

fn dfs(graph: &HashMap<i32, HashSet<i32>>, seen: &mut HashSet<i32>, source: i32, target: i32) -> bool {
    if !seen.insert(source) {
        return false
    }
    if source == target {
        return true
    }
    if let Some(neighbors) = graph.get(&source) {
        for &neighbor in neighbors {
            if dfs(graph, seen, neighbor, target) {
                return true
            }
        }
    }

    return false
}    

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            let mut seen = HashSet::new();
            if graph.contains_key(&u) && graph.contains_key(&v) && dfs(&graph, &mut seen, u, v) {
                return vec![u, v]
            }
            graph.entry(u).or_default().insert(v);
            graph.entry(v).or_default().insert(u);
        }
        unreachable!()    
    }
}
