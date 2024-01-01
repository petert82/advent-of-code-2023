use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::Result;

pub fn part1(input: &str) -> Result<usize> {
    Ok(1)
}

fn dijkstra(
    start: Vertex,
    adjacencies: HashMap<Vertex, Vec<(Vertex, usize)>>,
) -> HashMap<Vertex, usize> {
    let mut distances = HashMap::new();
    let mut to_visit = BinaryHeap::new();
    let mut visited = HashSet::new();

    distances.insert(start, 0);
    to_visit.push(Reverse(Visit {
        vertex: start,
        distance: 0,
    }));

    while let Some(Reverse(Visit { vertex, distance })) = to_visit.pop() {
        if !visited.insert(vertex) {
            continue;
        }

        if let Some(neighbours) = adjacencies.get(&vertex) {
            for (neighbour, cost) in neighbours {
                let new_distance = distance + cost;
                let is_closer = distances
                    .get(&neighbour)
                    .map_or(true, |prev| new_distance < *prev);

                if is_closer {
                    distances.insert(*neighbour, new_distance);
                    to_visit.push(Reverse(Visit {
                        vertex: *neighbour,
                        distance: new_distance,
                    }));
                }
            }
        }
    }

    distances
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex {
    pos: Coord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Visit {
    vertex: Vertex,
    distance: usize,
}

impl Vertex {
    pub fn new(x: usize, y: usize) -> Self {
        Self { pos: Coord(x, y) }
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pathing_works() {
        // 122
        // 111
        // 111
        let v1 = Vertex::new(0, 0);
        let v2 = Vertex::new(1, 0);
        let v3 = Vertex::new(2, 0);
        let v4 = Vertex::new(0, 1);
        let v5 = Vertex::new(1, 1);
        let v6 = Vertex::new(2, 1);
        let v7 = Vertex::new(0, 2);
        let v8 = Vertex::new(1, 2);
        let v9 = Vertex::new(2, 2);

        let mut adj = HashMap::new();
        adj.insert(v1, vec![(v2, 2), (v4, 1)]);
        adj.insert(v2, vec![(v1, 1), (v3, 2), (v5, 1)]);
        adj.insert(v3, vec![(v2, 2), (v6, 1)]);
        adj.insert(v4, vec![(v1, 1), (v5, 1), (v7, 1)]);
        adj.insert(v5, vec![(v2, 2), (v4, 1), (v6, 1), (v8, 1)]);
        adj.insert(v6, vec![(v3, 2), (v5, 1), (v9, 1)]);
        adj.insert(v7, vec![(v4, 1), (v8, 1)]);
        adj.insert(v8, vec![(v7, 1), (v5, 1), (v9, 1)]);
        adj.insert(v9, vec![(v8, 1), (v6, 1)]);

        let distances = dijkstra(v1, adj);
        assert_eq!(distances.get(&v1), Some(&0));
        assert_eq!(distances.get(&v2), Some(&2));
        assert_eq!(distances.get(&v3), Some(&4));
        assert_eq!(distances.get(&v9), Some(&4));
    }

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 102);
    }

    // #[test]
    // fn test_part2_gives_correct_answer() {
    //     let res = part2(INPUT).unwrap();
    //     assert_eq!(res, 51);
    // }
}
