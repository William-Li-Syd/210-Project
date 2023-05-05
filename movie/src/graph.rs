


#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
    
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    // fn add_node(&mut self, node: String) {
    //     self.nodes.push(node);
    // }

    fn add_edge(&mut self, node1: String, node2: String) {
        self.edges.push((node1, node2));
    }

    fn get_adjacency_list(&self) -> HashMap<String, Vec<String>> {
        let mut adjacency_list = HashMap::new();
        for (node1, node2) in &self.edges {
            adjacency_list
                .entry(node1.clone())
                .or_insert_with(Vec::new)
                .push(node2.clone());
            adjacency_list
                .entry(node2.clone())
                .or_insert_with(Vec::new)
                .push(node1.clone());
        }
        adjacency_list
    }
}


fn read_file() {
    let mut graph = Graph::new();
    let adjacency_list = graph.get_adjacency_list();

    let data = std::fs::read_to_string("movie.csv").unwrap()
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let title = fields[0].to_string();
            let actors: Vec<String> = fields[1..].iter().map(|a| a.to_string()).collect();
            vec![title, actors.join(";")]
        })
        .collect::<Vec<_>>();

    for edge in &data {
        let title = &edge[0];
        let actors = &edge[1];

        // Add the movie as an edge to the graph
        graph.add_edge(title.to_string(), actors.to_string());

        // Add the actors as nodes to the graph
        let actor_vec: Vec<_> = actors.split(";").collect();
        for i in 0..actor_vec.len() {
            for j in (i + 1)..actor_vec.len() {
                graph.add_edge(actor_vec[i].to_string(), actor_vec[j].to_string());
            }
        }
        // println!("{:?}", actor_vec);
    }

    // println!("{:?}", graph.edges);
}

// fn bfs(graph: &Graph, start: &str, end: &str) -> Option<Vec<String>> {
//     let mut queue = VecDeque::new();
//     let mut visited = HashMap::new();
//     let mut parent = HashMap::new();

//     queue.push_back(start);
//     visited.insert(start, true);
//     parent.insert(start, None);