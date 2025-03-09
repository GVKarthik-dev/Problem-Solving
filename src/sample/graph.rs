use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<i32, Vec<i32>>, // Each node maps to a vector of neighbors
}

impl Graph {
    // Create a new, empty graph
    fn new() -> Graph {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    // Add an edge to the graph (undirected)
    fn add_edge(&mut self, u: i32, v: i32) {
        self.adj_list.entry(u).or_insert(Vec::new()).push(v);
        self.adj_list.entry(v).or_insert(Vec::new()).push(u);
    }

    // Perform a Depth-First Search starting from a given node
    fn dfs(&self, start: i32) {
        let mut visited = HashSet::new();
        self.dfs_helper(start, &mut visited);
    }

    // Helper function to perform DFS recursively
    fn dfs_helper(&self, node: i32, visited: &mut HashSet<i32>) {
        // If the node has already been visited, return
        if visited.contains(&node) {
            return;
        }

        // Mark the current node as visited
        visited.insert(node);
        println!("Visiting node: {}", node);

        // Recursively visit all unvisited neighbors
        if let Some(neighbors) = self.adj_list.get(&node) {
            for &neighbor in neighbors {
                self.dfs_helper(neighbor, visited);
            }
        }
    }
}

pub fn main() {
    // Create a new graph
    let mut graph = Graph::new();

    // Add edges (the graph is undirected)
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);

    // Print the graph structure
    println!("Graph adjacency list: {:?}", graph.adj_list);

    // Perform DFS starting from node 1
    println!("Starting DFS from node 1:");
    graph.dfs(1);
}


use std::fmt;

// Define an enum for the unit types
#[derive(Debug, PartialEq)]
enum Unit {
    Meter,
    Centimeter,
    Kilometer,
}

// A struct to represent a measurement
struct Measurement {
    value: f64,
    unit: Unit,
}

impl Measurement {
    // Convert a value from one unit to another
    fn convert_to(&self, target_unit: &Unit) -> Measurement {
        let new_value = match (&self.unit, target_unit) {
            (Unit::Meter, Unit::Centimeter) => self.value * 100.0,
            (Unit::Meter, Unit::Kilometer) => self.value / 1000.0,
            (Unit::Centimeter, Unit::Meter) => self.value / 100.0,
            (Unit::Centimeter, Unit::Kilometer) => self.value / 100000.0,
            (Unit::Kilometer, Unit::Meter) => self.value * 1000.0,
            (Unit::Kilometer, Unit::Centimeter) => self.value * 100000.0,
            (_, _) => self.value, // If units are the same, return the value unchanged
        };

        Measurement {
            value: new_value,
            unit: target_unit.clone(),
        }
    }
}

// Implement Display for Measurement to make it easy to print
impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.value, self.unit)
    }
}

// Implement Clone for Unit so we can clone the unit for conversion
impl Clone for Unit {
    fn clone(&self) -> Self {
        match *self {
            Unit::Meter => Unit::Meter,
            Unit::Centimeter => Unit::Centimeter,
            Unit::Kilometer => Unit::Kilometer,
        }
    }
}

pub fn main_2() {
    // Create a measurement in meters
    let measurement_in_meters = Measurement {
        value: 5000.0,
        unit: Unit::Meter,
    };

    // Convert to centimeters
    let measurement_in_centimeters = measurement_in_meters.convert_to(&Unit::Centimeter);
    println!("{} is equivalent to {}", measurement_in_meters, measurement_in_centimeters);

    // Convert to kilometers
    let measurement_in_kilometers = measurement_in_meters.convert_to(&Unit::Kilometer);
    println!("{} is equivalent to {}", measurement_in_meters, measurement_in_kilometers);

    // Convert from kilometers to meters
    let measurement_in_kilometers2 = Measurement {
        value: 3.0,
        unit: Unit::Kilometer,
    };
    let measurement_in_meters2 = measurement_in_kilometers2.convert_to(&Unit::Meter);
    println!("{} is equivalent to {}", measurement_in_kilometers2, measurement_in_meters2);
}
