mod graph;
use graph::MovieGraph;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let graph = MovieGraph::from_file("movie.csv").expect("Error reading movie.csv");

    // Print the number of vertices and edges in the graph.
    println!("Number of vertices: {}", graph.num_vertices());
    println!("Number of edges: {}", graph.num_edges());

    // Get a vector of all actors in the graph
    let actors: Vec<&str> = graph.get_adj_list().keys().map(|s| s.as_str()).collect();

    // Choose two random actors from the vector
    let mut rng = thread_rng();
    let start = *actors.choose(&mut rng).unwrap();
    let end = *actors.choose(&mut rng).unwrap();

    // Find the shortest path between the two actors
    let path = graph.bfs(start, end);

    // Print the shortest path between the two actors
    println!("Shortest path from {} to {}: {:?}", start, end, path);

    // Count the number of actors in the path (excluding the start and end actors)
    let count = path.unwrap().len();
    println!("The number of actors to reach the end of the path is {}", count - 2);
}