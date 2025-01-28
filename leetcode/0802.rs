// https://leetcode.com/problems/find-eventual-safe-states/description/
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();

        let mut visit = vec![false; n];
        let mut inStack = vec![false; n];

        fn dfs(graph: &Vec<Vec<i32>>, visit: &mut Vec<bool>, inStack: &mut Vec<bool>, node: usize) -> bool {
            if inStack[node] {
                return true
            }
            if visit[node] {
                return false;
            }
            visit[node] = true;
            inStack[node] = true;
            for &neighbor in &graph[node] {
                if dfs(graph, visit, inStack, neighbor as usize) {
                    return true;
                }
            }
            inStack[node] = false;
            return false
        };

        let mut result: Vec<i32> = Vec::new();
        for i in 0..n {
            if !dfs(&graph, &mut visit, &mut inStack, i) {
                result.push(i as i32);
            }
        }
        result
    }
}
