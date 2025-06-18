const MAP_SIZE_X: usize = 20;
const MAP_SIZE_Y: usize = 20;
const INVERT_ROW_ORIGIN: bool = true;
const INVERT_COL_ORIGIN: bool = false;

use iced::widget::button;
use std::fs::File;
use iced::widget::text;
use std::io::{Write, stdin, stdout};

#[repr(u8)]
#[derive(Copy, Clone)]
enum ETypeFloor {
    None = b' ',
    Floor = b'f',
    Door = b'd',
    Trap = b't',
    Wall = b'w',
}

impl ETypeFloor {
    fn from_char(c: char) -> Self {
        match c {
            ' ' => ETypeFloor::None,
            'f' => ETypeFloor::Floor,
            'd' => ETypeFloor::Door,
            't' => ETypeFloor::Trap,
            'w' => ETypeFloor::Wall,
            _ => ETypeFloor::None
        }
    }
}

impl Into<char> for ETypeFloor {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
    type_floor: ETypeFloor,
}

impl Cell {
    fn new(x: usize, y:usize, type_floor: ETypeFloor) -> Self {
        Self {
            x,
            y,
            type_floor
        }
    }

    fn update(&mut self, type_floor: ETypeFloor) {
        self.type_floor = type_floor;
    }
}

struct MapTools {
    type_floor_selected: ETypeFloor,
}

impl Default for MapTools {
    fn default() -> Self {
        MapTools { 
            type_floor_selected: ETypeFloor::Floor 
        }
    }
}

struct MapData<const X: usize, const Y: usize> {
    cells: [[Cell; X]; Y],
    map_size_x: usize,
    map_size_y: usize,
    invert_row_origin: bool,
    invert_col_origin: bool,
}

impl<const X:usize, const Y:usize> MapData<X,Y> {
    fn new(cells: [[Cell; X]; Y], map_size_x: usize, map_size_y: usize, invert_row_origin: bool, invert_col_origin: bool) -> Self {
        Self { cells, map_size_x, map_size_y, invert_row_origin, invert_col_origin }
    }

    fn update_cell(&mut self, x: usize, y: usize, t: ETypeFloor) {
        self.cells[x][y].update(t);
    }
}

impl Default for MapData<20, 20> {    
    fn default() -> Self {
        let cells = std::array::from_fn(|y| {
            std::array::from_fn(|x| {
                Cell {
                    x,
                    y,
                    type_floor: ETypeFloor::from_char(' '),
                }
            })
        });

        MapData::<20, 20> {
            cells,
            map_size_x: 20,
            map_size_y: 20,
            invert_row_origin: true,
            invert_col_origin: false,
        }
    }
}

fn main() {
    let md = MapData::default();
    // let a:&str = &(format!("{}", 'x'));
    // let counter = text(15);
    // let increment = button("+");
    // let decrement = button("-");
    start_map_terminal(md);
}

// UI FUNCTIONS

fn update<const X: usize, const Y: usize>(gs: &mut MapData<X, Y>) {
    ()
}

fn draw() {}

fn run<const X: usize, const Y: usize>(gs: &mut MapData<X, Y>) {}

// TERMINAL FUNCTIONS
fn start_map_terminal<const X: usize, const Y: usize>(mut map: MapData<X, Y>) {
    loop {
        println!("{}", format_map(map.cells));
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

        let x = if INVERT_ROW_ORIGIN {
            MAP_SIZE_X - 1 - get_value_as_usize(&result, 0)
        } else {
            get_value_as_usize(&result, 0)
        };

        let y = if INVERT_COL_ORIGIN {
            MAP_SIZE_Y - 1 - get_value_as_usize(&result, 1)
        } else {
            get_value_as_usize(&result, 1)
        };

        let c:char = result[2].parse::<char>().unwrap();
        map.update_cell(x, y, ETypeFloor::from_char(c));
        println!("You typed: {}; Last updated coordinates: {}/{}", s, x, y);

        let _ = update_map_file(map.cells);
    }
}

fn update_map_file<const X: usize, const Y: usize>(map: [[Cell; X]; Y]) -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(format_map(map).as_bytes())?;

    Ok(())
}

fn format_map<const X: usize, const Y: usize>(map: [[Cell; X]; Y]) -> String {
    let mut count: i8 = if INVERT_ROW_ORIGIN { 19 } else { 0 };
    let mut formmated_map: String = "".to_owned();
    for line in map.iter() {
        formmated_map.push_str(&(format!("{:002} - ", count)));
        for cell in line.iter() {
            formmated_map.push_str(&(format!("[{}]", Into::<char>::into(cell.type_floor))))
        }
        count = if INVERT_ROW_ORIGIN {
            count - 1
        } else {
            count + 1
        };
        formmated_map.push_str("\n");
    }

    formmated_map
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
