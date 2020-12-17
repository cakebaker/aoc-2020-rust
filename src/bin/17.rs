use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() {
    const MAX_CYCLES: usize = 6;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let mut points = parse(file_content);

    for _ in 1..=MAX_CYCLES {
        points = cycle(&points);
    }

    println!("Result of puzzle 1: {}", points.len());
}

fn cycle(points: &[Point]) -> Vec<Point> {
    let mut active_points = HashSet::new();
    let mut potential_active_points: HashSet<Point> = HashSet::new();

    for point in points {
        let count = count_neighbors(points, *point);

        if count == 2 || count == 3 {
            active_points.insert(*point);
        }

        potential_active_points.extend(HashSet::<Point>::from_iter(generate_neighbor_positions(
            *point,
        )));
    }

    for point in potential_active_points {
        if count_neighbors(points, point) == 3 {
            active_points.insert(point);
        }
    }

    Vec::from_iter(active_points)
}

fn generate_neighbor_positions(p: Point) -> Vec<Point> {
    let mut points = Vec::new();

    for x in (p.x - 1)..=(p.x + 1) {
        for y in (p.y - 1)..=(p.y + 1) {
            for z in (p.z - 1)..=(p.z + 1) {
                points.push(Point { x, y, z });
            }
        }
    }

    points
}

fn count_neighbors(points: &[Point], p: Point) -> usize {
    points
        .iter()
        .filter(|&point| is_neighbor(*point, p))
        .count()
}

fn is_neighbor(a: Point, b: Point) -> bool {
    a != b && (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1 && (a.z - b.z).abs() <= 1
}

fn parse(file_content: String) -> Vec<Point> {
    let mut points = Vec::new();

    for (y, line) in file_content.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                points.push(Point {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                });
            }
        }
    }

    points
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}
