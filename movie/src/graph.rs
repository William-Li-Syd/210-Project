use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use rand::{seq::SliceRandom, thread_rng};

// Define the MovieGraph struct and its methods.
pub struct MovieGraph {
    adj_list: HashMap<String, HashSet<String>>,
}

impl MovieGraph {
    // Define a new method to create an empty movie graph.
    pub fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    // Define a method to get a reference to the adj_list HashMap.
    pub fn get_adj_list(&self) -> &HashMap<String, HashSet<String>> {
        &self.adj_list
    }

    // Define a from_file method to create a movie graph from a CSV file.
    pub fn from_file(fname: &str) -> Result<Self, std::io::Error> {
        let mut graph = Self::new();

        let contents = fs::read_to_string(fname)?;

        // Iterate over each line of the file and add edges between actors.
        for line in contents.lines() {
            let fields: Vec<&str> = line.split(',').collect();
            let actors = fields[1]
                .split(';')
                .map(|a| a.trim())
                .collect::<Vec<&str>>();

            for i in 0..actors.len() {
                for j in (i + 1)..actors.len() {
                    graph.add_edge(
                        actors[i].to_string(),
                        actors[j].to_string(),
                    );
                }
            }
        }
        Ok(graph)
    }

    // Define an add_edge method to add an edge between two actors.
    pub fn add_edge(&mut self, u: String, v: String) {
        self.adj_list
            .entry(u.clone())
            .or_insert(HashSet::new())
            .insert(v.clone());
        self.adj_list
            .entry(v.clone())
            .or_insert(HashSet::new())
            .insert(u.clone());
    }

    // Define a method to get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.adj_list.len()
    }

    // Define a method to get the number of edges (movies) in the graph.
    pub fn num_edges(&self) -> usize {
        let mut count = 0;
        for set in self.adj_list.values() {
            count += set.len();
        }
        count / 2 // divide by 2 since each edge is counted twice
    }
    // Define a bfs method to find the shortest path between two actors.
    pub fn bfs(&self, start: &str, end: &str) -> Option<Vec<String>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut prev = HashMap::new();
        queue.push_back(start.to_string());
        visited.insert(start.to_string());
    
        while let Some(node) = queue.pop_front() {
            if node == end {
                let mut path = Vec::new();
                let mut curr = &end.to_string();
    
                while curr != &start.to_string() {
                    path.push(curr.clone());
                    curr = prev.get(curr).unwrap();
                }
                path.push(start.to_string());
    
                path.reverse();
                return Some(path);
            }
    
            for neighbor in self.adj_list.get(&node).unwrap() {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.to_string());
                    prev.insert(neighbor.to_string(), node.clone());
                    queue.push_back(neighbor.to_string());
                }
            }
        }
    
        None
    }
    pub fn calculate_average_shortest_path(&self, n: usize) -> f64 {
        let actors: Vec<&str> = self.get_adj_list().keys().map(|s| s.as_str()).collect();
        let mut rng = thread_rng();
        let mut total_paths_length = 0;
        let mut successful_paths = 0;

        for _ in 0..n {
            let start = *actors.choose(&mut rng).unwrap();
            let end = *actors.choose(&mut rng).unwrap();

            if let Some(path) = self.bfs(start, end) {
                total_paths_length += path.len() - 1; // Subtract 1 to exclude the starting actor
                successful_paths += 1;
            }
        }

        if successful_paths == 0 {
            0.0
        } else {
            total_paths_length as f64 / successful_paths as f64
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a sample MovieGraph for testing.
    fn create_test_graph() -> MovieGraph {
        let mut graph = MovieGraph::new();

        graph.add_edge("Tom Hanks".to_string(), "Meg Ryan".to_string());
        graph.add_edge("Meg Ryan".to_string(), "Billy Crystal".to_string());
        graph.add_edge("Billy Crystal".to_string(), "Lisa Kudrow".to_string());
        graph.add_edge("Tom Hanks".to_string(), "Kevin Bacon".to_string());

        graph
    }


    // Test the creation of a MovieGraph and ensure vertices and edges are added correctly.
    #[test]
    fn test_movie_graph_creation() {
        let graph = create_test_graph();
        let adj_list = graph.get_adj_list();

        assert_eq!(graph.num_vertices(), 5);
        assert_eq!(graph.num_edges(), 4);

        // Check that the adjacency list contains the expected connections.
        assert!(adj_list.get("Tom Hanks").unwrap().contains("Meg Ryan"));
        assert!(adj_list.get("Tom Hanks").unwrap().contains("Kevin Bacon"));
        // ...
    }

    // Test that the BFS algorithm finds the correct shortest path when a path exists.
    #[test]
    fn test_bfs_path_exists() {
        let graph = create_test_graph();

        let path = graph.bfs("Tom Hanks", "Lisa Kudrow").unwrap();
        assert_eq!(path, vec!["Tom Hanks", "Meg Ryan", "Billy Crystal", "Lisa Kudrow"]);
    }

    // Test that the BFS algorithm returns None when no path exists between two actors. // suppose to fail 
    #[test]
    fn test_bfs_path_not_exists() {
        let graph = create_test_graph();

        let path = graph.bfs("Leonardo DiCaprio", "Meryl Streep");
        assert!(path.is_none());
    }

    // Test that the BFS algorithm returns a path containing only the start actor when the start and end actors are the same.
    #[test]
    fn test_bfs_same_actor() {
        let graph = create_test_graph();

        let path = graph.bfs("Tom Hanks", "Tom Hanks").unwrap();
        assert_eq!(path, vec!["Tom Hanks"]);
    }

    // Test adding an edge between two new actors to the MovieGraph.
    #[test]
    fn test_add_edge_new() {
        let mut graph = MovieGraph::new();

        graph.add_edge("Tom Hanks".to_string(), "Meg Ryan".to_string());

        let adj_list = graph.get_adj_list();
        assert!(adj_list.get("Tom Hanks").unwrap().contains("Meg Ryan"));
        assert!(adj_list.get("Meg Ryan").unwrap().contains("Tom Hanks"));
    }

    // Test adding an edge between an existing actor and a new actor in the MovieGraph.
    #[test]
    fn test_add_edge_existing() {
        let mut graph = create_test_graph();

        graph.add_edge("Meg Ryan".to_string(), "Kevin Bacon".to_string());

        let adj_list = graph.get_adj_list();
        assert!(adj_list.get("Meg Ryan").unwrap().contains("Kevin Bacon"));
        assert!(adj_list.get("Kevin Bacon").unwrap().contains("Meg Ryan"));
    }
}