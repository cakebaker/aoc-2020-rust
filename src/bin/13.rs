use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (earliest_departure_time, bus_ids) = parse(file_content);

    let (bus_id, departure_time) = bus_ids
        .iter()
        .map(|bus_id| {
            (
                *bus_id,
                (0..)
                    .step_by(*bus_id as usize)
                    .skip_while(|x| *x < earliest_departure_time)
                    .take(1)
                    .collect::<Vec<isize>>()[0],
            )
        })
        .min_by_key(|(_, x)| *x)
        .unwrap();

    println!(
        "Result of puzzle 1: {}",
        (departure_time - earliest_departure_time) * bus_id
    );
}

fn parse(file_content: String) -> (isize, Vec<isize>) {
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
