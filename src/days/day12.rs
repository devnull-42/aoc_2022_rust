use petgraph::algo::dijkstra;
// use petgraph::dot::{Config, Dot};
use petgraph::graphmap::GraphMap;
use petgraph::*;

fn parse_terrain_map(input: &str) -> (Vec<Vec<u32>>, (usize, usize), (usize, usize)) {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut y = 0 as usize;
    let mut terrain_map: Vec<Vec<u32>> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let mut x = 0 as usize;

        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            if c == 'S' {
                start = (x, y);
                row.push('a' as u32);
            } else if c == 'E' {
                end = (x, y);
                row.push('z' as u32);
            } else {
                row.push(c as u32);
            }
            x += 1;
        }
        terrain_map.push(row);
        y += 1;
    }
    (terrain_map, start, end)
}

fn parse_terrain_map_multiple_starts(
    input: &str,
) -> (Vec<Vec<u32>>, Vec<(usize, usize)>, (usize, usize)) {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut end: (usize, usize) = (0, 0);

    let mut y = 0 as usize;
    let mut terrain_map: Vec<Vec<u32>> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let mut x = 0 as usize;

        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            if c == 'S' || c == 'a' {
                starts.push((x, y));
                row.push('a' as u32);
            } else if c == 'E' {
                end = (x, y);
                row.push('z' as u32);
            } else {
                row.push(c as u32);
            }
            x += 1;
        }
        terrain_map.push(row);
        y += 1;
    }
    (terrain_map, starts, end)
}

fn build_graph(terrain_map: Vec<Vec<u32>>) -> GraphMap<(usize, usize), i32, Directed> {
    let mut graph: GraphMap<(usize, usize), i32, Directed> =
        petgraph::graphmap::GraphMap::<_, _, Directed>::new();
    for y in 0..terrain_map.len() {
        for x in 0..terrain_map[y].len() {
            graph.add_node((x, y));
        }
    }

    for y in 0..terrain_map.len() {
        for x in 0..terrain_map[y].len() {
            let node = terrain_map.get(y).unwrap().get(x).unwrap();
            if y > 0 {
                let up = terrain_map.get(y - 1).unwrap().get(x).unwrap();
                if *up <= node + 1 {
                    graph.add_edge((x, y), (x, y - 1), 1);
                }
            }
            if y < terrain_map.len() - 1 {
                let down = terrain_map.get(y + 1).unwrap().get(x).unwrap();
                if *down <= node + 1 {
                    graph.add_edge((x, y), (x, y + 1), 1);
                }
            }
            if x > 0 {
                let left = terrain_map.get(y).unwrap().get(x - 1).unwrap();
                if *left <= node + 1 {
                    graph.add_edge((x, y), (x - 1, y), 1);
                }
            }
            if x < terrain_map.get(y).unwrap().len() - 1 {
                let right = terrain_map.get(y).unwrap().get(x + 1).unwrap();
                if *right <= node + 1 {
                    graph.add_edge((x, y), (x + 1, y), 1);
                }
            }
        }
    }

    graph
}

pub fn part1(input: &str) -> String {
    let (terrain_map, start, end) = parse_terrain_map(input);
    let graph = build_graph(terrain_map);

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let shortest_path = dijkstra(&graph, start, Some(end), |_| 1);
    if let Some(path) = shortest_path.get(&end) {
        return path.to_string();
    } else {
        return "path not found".to_string();
    }
}

pub fn part2(input: &str) -> String {
    let (terrain_map, starts, end) = parse_terrain_map_multiple_starts(input);
    let graph = build_graph(terrain_map);
    let mut steps = i32::MAX;
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    for start in starts {
        let shortest_path = dijkstra(&graph, start, Some(end), |_| 1);
        if let Some(path) = shortest_path.get(&end) {
            if path < &steps {
                steps = *path;
            }
        }
    }
    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, "29");
    }
}
