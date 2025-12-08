use advent_of_code_2025::read_lines;

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        JunctionBox { x, y, z }
    }

    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return None;
        }
        let x = parts[0].trim().parse().ok()?;
        let y = parts[1].trim().parse().ok()?;
        let z = parts[2].trim().parse().ok()?;
        Some(JunctionBox::new(x, y, z))
    }

    fn distance_sq(&self, other: &JunctionBox) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    dist_sq: i64,
    a: usize,
    b: usize,
}

// disjoint set union
struct DSU {
    parent: Vec<usize>,
    num_components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            num_components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        self.parent[root_a] = root_b;
        self.num_components -= 1;
        true
    }
}

fn solve(boxes: Vec<JunctionBox>) {
    let n = boxes.len();
    let mut edges = Vec::new();

    for a in 0..n {
        for b in (a + 1)..n {
            let dist_sq = boxes[a].distance_sq(&boxes[b]);
            edges.push(Edge {
                dist_sq: dist_sq,
                a,
                b,
            });
        }
    }

    edges.sort_unstable_by(|a, b| a.dist_sq.cmp(&b.dist_sq));

    let mut dsu = DSU::new(n);

    for edge in edges {
        if dsu.union(edge.a, edge.b) {
            if dsu.num_components == 1 {
                let p1 = boxes[edge.a];
                let p2 = boxes[edge.b];

                println!("Final connection made between:");
                println!("Point 1: {:?}", p1);
                println!("Point 2: {:?}", p2);
    
                let result = p1.x * p2.x;
                println!("Answer (Product of X coords): {}", result);
                return;
            }

        }
    }
}

fn main() {
    let lines = read_lines("input/day8.txt").expect("Expected input");
    let boxes = lines
        .iter()
        .map(|line| JunctionBox::from_str(line))
        .collect::<Option<Vec<_>>>()
        .expect("Invalid input");

    solve(boxes);
}
