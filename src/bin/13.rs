use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (earliest_departure_time, bus_ids) = parse_for_part_one(file_content);
    let (departure_time, bus_id) = find_earliest_bus(earliest_departure_time, bus_ids);

    println!(
        "Result of puzzle 1: {}",
        (departure_time - earliest_departure_time) * bus_id
    );
}

fn find_earliest_bus(earliest_departure_time: isize, bus_ids: Vec<isize>) -> (isize, isize) {
    bus_ids
        .iter()
        .map(|bus_id| {
            (
                (0..)
                    .step_by(*bus_id as usize)
                    .skip_while(|x| *x < earliest_departure_time)
                    .take(1)
                    .collect::<Vec<isize>>()[0],
                *bus_id,
            )
        })
        .min_by_key(|(x, _)| *x)
        .unwrap()
}

fn parse_for_part_one(file_content: String) -> (isize, Vec<isize>) {
    let mut lines = file_content.lines();
    let earliest_departure_time = lines.next().unwrap().parse().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(|c| c == ',')
        .filter(|elem| *elem != "x")
        .map(|elem| elem.parse().unwrap())
        .collect();

    (earliest_departure_time, bus_ids)
}
