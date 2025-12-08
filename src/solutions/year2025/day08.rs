use std::time::{Duration, Instant};
use std::collections::HashSet;
use std::cmp::Ordering;

pub struct Day08;

impl super::super::Solution for Day08 {
    fn solve(&self, input: &str) -> (String, String, Duration, Duration) {
        let start = Instant::now();
        let p1 = self.part1(input);
        let t1 = start.elapsed();

        let start = Instant::now();
        let p2 = self.part2(input);
        let t2 = start.elapsed();

        (p1, p2, t1, t2)
    }
}

struct Edge {
    dist: f64,
    i: usize,
    j: usize,
}

fn parse_points(input: &str) -> Vec<[f64; 3]> {
    // Each junction box is represented as a vector p_i = [x_i, y_i, z_i]
    input.lines()
        .map(|line| {
            let coords: Vec<f64> = line.split(',').map(|v| v.parse().unwrap()).collect();
            [coords[0], coords[1], coords[2]]
        })
        .collect()
}

fn compute_edges(points: &[[f64; 3]]) -> Vec<Edge> {
    let n = points.len();
    // Compute all pairwise distances: d_ij = sqrt{||p_i - p_j||^2}
    let mut edges: Vec<Edge> = Vec::new(); // (d_ij^2, i, j)
    for i in 0..n {
        for j in i+1..n {
            // Calculate Euclidean distance between points i and j
            let dx = points[i][0] - points[j][0];
            let dy = points[i][1] - points[j][1];
            let dz = points[i][2] - points[j][2];
            let dist = (dx*dx + dy*dy + dz*dz).sqrt();
            edges.push(Edge {dist, i, j});
        }
    }

    edges.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Equal));
    edges
}

// Union Find (Disjoint Set Union) data structure with Kruskal's algorithm
// Used to track connected components in a graph
struct UnionFind {
    parent: Vec<usize>, // parent[i] = representative of the set containing i
    size: Vec<usize>,   // size[i] = size of the set where i is representative
}

impl UnionFind {
    // Create a new UnionFind with n elements, each in its own set
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    // Find the representative (root) of the set containint x, with path compression
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            // Path compression: make parent point directly to root
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // Union the sets containing x and y
    // Returns true if the sets were merged, false they were already in the same set
    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x); // Find root of x's set
        let py = self.find(y); // Find root of y's set
        if px == py { return false; } // Already in the same set

        // Union by size: attach smaller tree under larger tree
        if self.size[px] < self.size[py] {
            self.parent[px] = py;           // Make py parent of px
            self.size[py] += self.size[px]; // Update size of py's set
        } else {
            self.parent[py] = px;           // Make px parent of py
            self.size[px] += self.size[py]; // Update size of px's set
        }
        true
    }
}

impl Day08 {
    pub fn part1(&self, input: &str) -> String {
        let points = parse_points(input);
        let edges = compute_edges(&points);
        let n = points.len();
        let mut uf = UnionFind::new(n);

        // Determine how many edges to process:
        // For the test case (20 points), process 10 shortest edges
        // For the real puzzle, process 1000 shortest edges
        let limit = if n == 20 { 10 } else { edges.len().min(1000) };

        // Process the shortest edges_to_process edges
        // For each edge (dist, i, j) try to connect i and j
        // Even if they're alreadz connected (union returns false), it still counts as a connection
        // Nothing happens when circuits are already connected
        for edge in edges.iter().take(limit) {
            uf.union(edge.i, edge.j);
        }
        
        // After processing edges, identify all distinct circuits (connected components)
        let mut sizes = Vec::new();    // Store sizes of each circuit
        let mut seen = HashSet::new(); // Track which roots have been counted

        for i in 0..n {
            let root = uf.find(i);         // Find which circuit this junction box belongs to
            if seen.insert(root) {         // If this is the first time seeing this root
                sizes.push(uf.size[root]); // Add the size of this circuit
            }
        }

        // Sort circuit sizes in descending order
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        
        // Return the product of the three largest circuits
        (sizes[0] * sizes[1] * sizes[2]).to_string()
    }

    pub fn part2(&self, input: &str) -> String {
        let points = parse_points(input);
        let edges = compute_edges(&points);
        let n = points.len();
        let mut uf = UnionFind::new(n);

        let mut components = n;

        for edge in edges {
            // Try to union the components containint the two endpoints of this edge.
            if uf.union(edge.i, edge.j) {

                // A successful union means two components just merged into one.
                components -= 1;

                // When only one component remains, the entire graph is fully connected.
                // This edge would be the final merge needed
                if components == 1 {
                    // final merge occurred here
                    let x1 = points[edge.i][0] as usize;
                    let x2 = points[edge.j][0] as usize;
                    return (x1 * x2).to_string();
                }
            }
        }
        unreachable!("Uh oh, contact admin if you see this...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(Day08.part1(TEST_INPUT), "40");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day08.part2(TEST_INPUT), "25272");
    }
}

