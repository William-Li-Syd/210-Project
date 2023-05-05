use std::collections::{HashMap, HashSet,VecDeque};
use std::fs;
use std::path::PathBuf;

// Define the main function, which creates a movie graph from a file
// and finds the shortest path between two actors.
fn main(){
    let graph = MovieGraph::from_file("movie.csv");
    let path = graph.shortest_path("Michael Craig", "James Farentino");
    println!("Shortest path: {:?}", path);
}

// Define the MovieGraph struct and its methods.
struct MovieGraph {
    adj_list: HashMap<String, HashSet<String>>,
    
}

impl MovieGraph {
     // Define a new method to create an empty movie graph.
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }
     // Define a from_file method to create a movie graph from a CSV file.
    fn from_file(fname: &str) -> Self {
        let mut graph = Self::new();

        let mut path = PathBuf::new();
        path.push(fname);
        
        // Read the contents of the file into a string.

        let contents = fs::read_to_string(fname).unwrap();

        // Iterate over each line of the file and add edges between actors.

        for line in contents.lines() {
            let fields: Vec<&str> = line.split(',').collect();
            let title = fields[0];
            let actors = fields[1].split(';').map(|a| a.trim()).collect::<Vec<&str>>();

            for i in 0..actors.len() {
                for j in (i+1)..actors.len() {
                    graph.add_edge(actors[i].to_string(), actors[j].to_string(), title.to_string());
                }
            }
        }

        graph
    }

     // Define an add_edge method to add an edge between two actors.
    fn add_edge(&mut self, u: String, v: String, _title: String) {
        self.adj_list.entry(u.clone()).or_insert(HashSet::new()).insert(v.clone());
        self.adj_list.entry(v.clone()).or_insert(HashSet::new()).insert(u.clone());
    }
    // Define an n_v method to get the number of vertices in the graph.
    fn n_v(&self) -> usize {
        self.adj_list.len()
    }
     // Define an n_e method to get the number of edges in the graph.
    fn n_e(&self) -> usize {
        let mut num_edges = 0;

        for (_, neighbors) in &self.adj_list {
            num_edges += neighbors.len();
        }

        num_edges / 2
    }
    // Define a neighbors method to get the neighbors of a vertex.
    fn neighbors(&self, v: &str) -> Option<&HashSet<String>> {
        self.adj_list.get(v)
    }
     // Define a degree method to get the degree of a vertex.
    fn degree(&self, v: &str) -> Option<usize> {
        self.adj_list.get(v).map(|neighbors| neighbors.len())
    }
    // Define a has_vertex method to check if a vertex is in the graph.
    fn has_vertex(&self, v: &str) -> bool {
        self.adj_list.contains_key(v)
    }
    // Define a has_edge method to check if an edge is in the graph.
    fn has_edge(&self, u: &str, v: &str) -> bool {
        if let Some(neighbors) = self.adj_list.get(u) {
            neighbors.contains(v)
        } else {
            false
        }
    }
    // If the actor is in the adjacency list, return a vector of movies they acted in
    fn find_movies_by_actor(&self, actor: &str) -> Vec<&str> {
        if let Some(neighbors) = self.adj_list.get(actor) {
            neighbors.iter().map(|movie| movie.as_str()).collect()
        } else { // Otherwise, return an empty vector
            vec![]
        }
    }
    // Filter the adjacency list to find actors who acted in the given movie
    fn find_actors_by_movie(&self, movie: &str) -> Vec<&str> {
        self.adj_list
            .iter()
            .filter_map(|(actor, movies)| {
                if movies.contains(movie) {// If the actor acted in the movie, return their name
                    Some(actor.as_str())
                } else { // Otherwise, return None
                    None
                }
            })
            .collect()
    }
    fn shortest_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        // Create a queue, visited set, and previous node map for BFS traversal
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut prev = HashMap::new();

        // Add the starting node to the queue and visited set
        queue.push_back((start.to_string(), vec![start.to_string()]));
        visited.insert(start.to_string());

        // Continue traversing the graph until there are no more nodes in the queue
        while let Some((u, path)) = queue.pop_front() {
            // If we have reached the end node, return the shortest path to it
            if u == end {
                return Some(path);
            }
            // Otherwise, add all unvisited neighbors of the current node to the queue
            for v in self.adj_list.get(&u).unwrap_or(&HashSet::new()) {
                if !visited.contains(v) {
                    visited.insert(v.clone());
                    let mut new_path = path.clone();
                    new_path.push(v.clone());
                    prev.insert(v.clone(), u.clone());
                    queue.push_back((v.clone(), new_path));
                }
            }
        }
        // If we did not find a path from start to end, return None
        None
    }
}

// Continuously prompt the user to enter an actor's name, and return the movies they acted in
fn query_movies(graph: &MovieGraph) {
    loop {
        println!("Enter an actor's name or 'Q' to quit:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let actor = input.trim();

        if actor == "Q" {
            break;
        }

        let movies = graph.find_movies_by_actor(actor);
        if movies.is_empty() {
            println!("{} did not act in any movies in our database.", actor);
        } else {
            println!("{} acted in the following movies:", actor);
            for movie in movies {
                println!("- {}", movie);
            }
        }
    }
}

fn query_actors(graph: &MovieGraph) {
    // Continuously prompt the user to enter a movie title, and return the actors who starred in it
    loop {
        println!("Enter a movie title or 'Q' to quit:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let movie = input.trim();

        if movie == "Q" {
            break;
        }

        let actors = graph.find_actors_by_movie(movie);
        if actors.is_empty() {
            println!("No actors were found for the movie: {}", movie);
        } else {
            println!("The following actors starred in {}:", movie);
            for actor in actors {
                println!("- {}", actor);
            }
        }
    }
}