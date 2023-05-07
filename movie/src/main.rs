mod graph;
use graph::MovieGraph;
use rand::{seq::SliceRandom, thread_rng};
fn main() {
    let graph = MovieGraph::from_file("movie.csv").expect("Error reading movie.csv");

    // Print the number of vertices and edges in the graph.
    println!("Number of vertices: {}", graph.num_vertices());
    println!("Number of edges: {}", graph.num_edges());

    // Get a vector of all actors in the graph.
    let actors: Vec<&str> = graph.get_adj_list().keys().map(|s| s.as_str()).collect();

    // Choose two random actors from the vector.
    let mut rng = thread_rng();
    let start = *actors.choose(&mut rng).unwrap();
    let end = *actors.choose(&mut rng).unwrap();

    // Find the shortest path between the two actors.
    let path = graph.bfs(start, end);

    match path {
        Some(p) => {
            println!("Shortest path from {} to {}:", start, end);
            for (idx, actor) in p.iter().enumerate() {
                if idx != 0 {
                    print!(" -> ");
                }
                print!("{}", actor);
            }
            println!();

            // Count the number of actors in the path (excluding the start and end actors).
            let count = p.len();
            println!("The number of actors to reach the end of the path is {}", count - 2);
        }
        None => {
            println!("No path found between {} and {}.", start, end);
        }
    }

    let avg_shortest_path = graph.calculate_average_shortest_path(100);
    println!("Average shortest path length for 100 random pairs of actors: {:.2}", avg_shortest_path);
    
}