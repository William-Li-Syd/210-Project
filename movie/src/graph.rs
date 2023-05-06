
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;

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

        let mut path = PathBuf::new();
        path.push(fname);

        // Read the contents of the file into a string.
        let contents = fs::read_to_string(fname)?;

        // Iterate over each line of the file and add edges between actors.
        for line in contents.lines() {
            let fields: Vec<&str> = line.split(',').collect();
            let title = fields[0];
            let actors = fields[1]
                .split(';')
                .map(|a| a.trim())
                .collect::<Vec<&str>>();

            for i in 0..actors.len() {
                for j in (i + 1)..actors.len() {
                    graph.add_edge(
                        actors[i].to_string(),
                        actors[j].to_string(),
                        title.to_string(),
                    );
                }
            }
        }
        Ok(graph)
    }

    // Define an add_edge method to add an edge between two actors.
    pub fn add_edge(&mut self, u: String, v: String, _title: String) {
        self.adj_list
            .entry(u.clone())
            .or_insert(HashSet::new())
            .insert(v.clone());
        self.adj_list
            .entry(v.clone())
            .or_insert(HashSet::new())
            .insert(u.clone());
    }
    
    pub fn num_vertices(&self) -> usize {
        self.adj_list.len()
    }

    // function to count the number of edges (movies) in the graph
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_graph() {
        // Create a new MovieGraph.
        let mut graph = MovieGraph::new();

        // Add some edges to the graph.
        graph.add_edge("Tom Hanks".to_string(), "Meg Ryan".to_string(), "You've Got Mail".to_string());
        graph.add_edge("Meg Ryan".to_string(), "Billy Crystal".to_string(), "When Harry Met Sally".to_string());
        graph.add_edge("Billy Crystal".to_string(), "Lisa Kudrow".to_string(), "Analyze That".to_string());
        graph.add_edge("Tom Hanks".to_string(), "Kevin Bacon".to_string(), "Apollo 13".to_string());

        // Check that the graph contains the expected edges.
        let adj_list = graph.get_adj_list();
        assert!(adj_list.get("Tom Hanks").unwrap().contains("Meg Ryan"));
        assert!(adj_list.get("Meg Ryan").unwrap().contains("Tom Hanks"));
        assert!(adj_list.get("Meg Ryan").unwrap().contains("Billy Crystal"));
        assert!(adj_list.get("Billy Crystal").unwrap().contains("Meg Ryan"));
        assert!(adj_list.get("Billy Crystal").unwrap().contains("Lisa Kudrow"));
        assert!(adj_list.get("Lisa Kudrow").unwrap().contains("Billy Crystal"));
        assert!(adj_list.get("Tom Hanks").unwrap().contains("Kevin Bacon"));
        assert!(adj_list.get("Kevin Bacon").unwrap().contains("Tom Hanks"));

        // Find the shortest path between two actors.
        let path = graph.bfs("Tom Hanks", "Lisa Kudrow").unwrap();
        assert_eq!(path, vec!["Tom Hanks", "Meg Ryan", "Billy Crystal", "Lisa Kudrow"]);
    }
}