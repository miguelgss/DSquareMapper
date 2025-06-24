use std::io::{Write, stdin, stdout};

use crate::dsquaremapper;

pub fn start_map_terminal<const X: usize, const Y: usize>(map: &mut dsquaremapper::MapData<X, Y>) {
    loop {
        println!("{}", map.format_map());
        let mut s: String = String::new();
        print!("Pass: (X, Y, Type of floor): ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("String error");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        let exit = s.lines().filter(|ss| ss.contains("exit")).next();
        if exit.is_some() {
            break;
        }

        let result: Vec<&str> = s.split_whitespace().collect();
        if !is_input_valid(&result) || result.len() != 3 {
            println!("Please type only 3 arguments: X (number), Y (number) and the Type of floor.");
            continue;
        }

        let x = get_value_as_usize(&result, 0);
        let y = get_value_as_usize(&result, 1);
        let c: char = result[2].parse::<char>().unwrap();

        map.update_cell(x, y, dsquaremapper::ETypeFloor::from_char(c));
        println!("You typed: {}; Last updated coordinates: {}/{}", s, x, y);

        let _ = map.update_map_file();
    }
}

fn get_value_as_usize(a: &Vec<&str>, index: usize) -> usize {
    a[index].parse::<usize>().unwrap()
}

fn is_input_valid(a: &Vec<&str>) -> bool {
    let r1 = a[0].parse::<usize>();
    let r2 = a[1].parse::<usize>();
    let r3 = a[2].parse::<char>();

    if r1.is_err() || r2.is_err() || r3.is_err() {
        return false;
    }

    true
}