const USE_UI: bool = true;
const OFFSET: f32 = 20.;
const SQUARE_SIZE: f32 = OFFSET - 1.;

use std::fs::File;
use std::io::{Error, ErrorKind, Write, stdin, stdout};

use macroquad::ui::{hash, root_ui};
use macroquad::{miniquad::window::quit, prelude::*};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
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
            _ => ETypeFloor::None,
        }
    }

    fn get_color(&self) -> u32 {
        match self {
            ETypeFloor::None => 0x9DB5B2,  // Ash-grey
            ETypeFloor::Door => 0x19535F,  // Midnight Green
            ETypeFloor::Floor => 0x0B7A75, // Skobeloff (Cian-ish green)
            ETypeFloor::Trap => 0x7B2D26,  // Falu red
            ETypeFloor::Wall => 0xD7C9AA,  // Dun ()
        }
    }
}

impl Into<char> for ETypeFloor {
    fn into(self) -> char {
        self as u8 as char
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Cell {
    x: usize,
    y: usize,
    type_floor: ETypeFloor,
}

#[allow(dead_code)]
impl Cell {
    fn _new(x: usize, y: usize, type_floor: ETypeFloor) -> Self {
        Self { x, y, type_floor }
    }

    fn update(&mut self, type_floor: ETypeFloor) {
        self.type_floor = type_floor;
    }

    fn get_data(self) -> (usize, usize, ETypeFloor) {
        (self.x, self.y, self.type_floor)
    }
}

#[allow(dead_code)]
struct MapTools {
    type_floor_selected: ETypeFloor,
}

impl MapTools {
    fn change_type_floor_selected(&mut self, t: ETypeFloor) {
        self.type_floor_selected = t;
    }
}

impl Default for MapTools {
    fn default() -> Self {
        MapTools {
            type_floor_selected: ETypeFloor::Floor,
        }
    }
}

#[derive(Debug)]
struct MapData<const X: usize, const Y: usize> {
    name: String,
    cells: [[Cell; X]; Y],
    map_size_x: usize,
    map_size_y: usize,
    invert_row_origin: bool,
    invert_col_origin: bool,
}

#[allow(dead_code)]
impl<const X: usize, const Y: usize> MapData<X, Y> {
    fn new(
        name: String,
        cells: [[Cell; X]; Y],
        map_size_x: usize,
        map_size_y: usize,
        invert_row_origin: bool,
        invert_col_origin: bool,
    ) -> Self {
        Self {
            name,
            cells,
            map_size_x,
            map_size_y,
            invert_row_origin,
            invert_col_origin,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> std::io::Result<&Cell> {
        if let Some(row) = self.cells.get(y) {
            if let Some(cell) = row.get(x) {
                return Ok(cell);
            }
        }

        Err(Error::new(
            ErrorKind::NotFound,
            "Cell not found at given coordinates",
        ))
    }

    fn update_cell(&mut self, x: usize, y: usize, t: ETypeFloor) {
        let x = if self.invert_row_origin {
            self.map_size_x - x
        } else {
            x - 1
        };
        let y = if self.invert_col_origin {
            self.map_size_y - y
        } else {
            y - 1
        };

        self.cells[y][x].update(t);
        let _ = self.update_map_file();
    }

    fn update_name(&mut self, name: String) {
        self.name = name;
    }

    fn format_map(&self) -> String {
        let mut count: i8 = if self.invert_row_origin { 20 } else { 1 };
        let mut formmated_map: String = "".to_owned();

        for line in self.cells.iter() {
            formmated_map.push_str(&(format!("{:002} - ", count)));
            for cell in line.iter() {
                formmated_map.push_str(&(format!("[{}]", Into::<char>::into(cell.type_floor))))
            }
            count = if self.invert_row_origin {
                count - 1
            } else {
                count + 1
            };
            formmated_map.push_str("\n");
        }

        formmated_map
    }

    fn update_map_file(&self) -> std::io::Result<()> {
        let mut file = File::create(format!("{}.txt", self.name))?;
        file.write_all(self.format_map().as_bytes())?;

        Ok(())
    }
}

impl Default for MapData<20, 20> {
    fn default() -> Self {
        let cells = std::array::from_fn(|y| {
            std::array::from_fn(|x| Cell {
                x,
                y,
                type_floor: ETypeFloor::from_char(' '),
            })
        });

        MapData::<20, 20> {
            name: "Map".to_string(),
            cells,
            map_size_x: 20,
            map_size_y: 20,
            invert_row_origin: false,
            invert_col_origin: false,
        }
    }
}

// tests
#[test]
fn update_map_cell_success() {
    let mut md = MapData::default();

    md.update_cell(1, 1, ETypeFloor::Floor);
    md.update_cell(5, 2, ETypeFloor::Door);
    md.update_cell(3, 2, ETypeFloor::Wall);
    md.update_cell(10, 5, ETypeFloor::Trap);

    assert_eq!(md.cells[0][0].type_floor, ETypeFloor::Floor);
    assert_eq!(md.cells[1][4].type_floor, ETypeFloor::Door);
    assert_eq!(md.cells[1][2].type_floor, ETypeFloor::Wall);
    assert_eq!(md.cells[4][9].type_floor, ETypeFloor::Trap);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut md = MapData::default();
    let mut mt = MapTools::default();
    if USE_UI {
        loop {
            run_ui(&mut md, &mut mt).await;
        }
    } else {
        start_map_terminal(&mut md);
    }
}

// UI FUNCTIONS

fn window_conf() -> Conf {
    Conf {
        window_title: "DSquareMapper".to_owned(),
        fullscreen: false,
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

async fn update<const X: usize, const Y: usize>(map: &mut MapData<X, Y>, map_tools: &mut MapTools) {
    let (_, _, m_x_offset, m_y_offset, _) = mouse_offset(map).await;
    let cell = map.get_cell(m_x_offset - 1, m_y_offset - 1);

    if cell.is_ok() {
        if is_mouse_button_down(MouseButton::Left) {
            map.update_cell(m_x_offset, m_y_offset, map_tools.type_floor_selected);
        } else if is_mouse_button_down(MouseButton::Right) {
            map.update_cell(m_x_offset, m_y_offset, ETypeFloor::None);
        }
    }

    match get_last_key_pressed() {
        Some(KeyCode::Key1) => map_tools.change_type_floor_selected(ETypeFloor::Floor),
        Some(KeyCode::Key2) => map_tools.change_type_floor_selected(ETypeFloor::Door),
        Some(KeyCode::Key3) => map_tools.change_type_floor_selected(ETypeFloor::Wall),
        Some(KeyCode::Key4) => map_tools.change_type_floor_selected(ETypeFloor::Trap),
        Some(KeyCode::E) => map_tools.change_type_floor_selected(ETypeFloor::None),
        _ => (),
    }

    return;
}

#[allow(dead_code)]
fn aabb_check_collision(item1: Rect, item2: Rect) -> bool {
    item1.x < item2.x + item2.w
        && item1.x + item1.w > item2.x
        && item1.y < item2.y + item2.h
        && item1.y + item1.h > item2.y
}

async fn draw<const X: usize, const Y: usize>(map: &mut MapData<X, Y>, mt: &MapTools) {
    clear_background(BLACK);
    for col in map.cells.iter_mut() {
        for cell in col {
            draw_rectangle(
                (cell.x as f32) * OFFSET,
                (cell.y as f32) * OFFSET,
                SQUARE_SIZE,
                SQUARE_SIZE,
                Color::from_hex(ETypeFloor::get_color(&cell.type_floor)),
            );
        }
    }

    debug_draw_mouse_data(map).await;
    draw_tools(map, mt).await;
    next_frame().await;
}

async fn draw_tools<const X: usize, const Y: usize>(map: &mut MapData<X, Y>, mt: &MapTools) {
    draw_text(
        &format!("Type floor: {:?}", mt.type_floor_selected),
        map.map_size_x as f32 * SQUARE_SIZE + 40.,
        20.,
        20.,
        WHITE,
    );

    root_ui().window(hash!(), vec2(map.map_size_x as f32 * SQUARE_SIZE + 40., 50.), vec2(200., 150.), |ui| {
        ui.input_text(hash!(), "Filename", &mut map.name);
    });
}

async fn debug_draw_mouse_data<const X: usize, const Y: usize>(map: &mut MapData<X, Y>) {
    let (mouse_x, mouse_y, m_x_offset, m_y_offset, _) = mouse_offset(map).await;
    draw_text(
        &format!("{:.2}/{:.2}", m_x_offset, m_y_offset),
        10.,
        map.map_size_y as f32 * SQUARE_SIZE + 40.,
        20.,
        WHITE,
    );
    draw_text(
        &format!("{:.2}/{:.2}", mouse_x, mouse_y),
        10.,
        map.map_size_y as f32 * SQUARE_SIZE + 52.,
        20.,
        WHITE,
    );
}

async fn mouse_offset<const X: usize, const Y: usize>(
    map: &mut MapData<X, Y>,
) -> (f32, f32, usize, usize, Rect) {
    let (mouse_x, mouse_y) = mouse_position();
    let (m_x_offset, m_y_offset) = (
        ((mouse_x + if !map.invert_row_origin { OFFSET } else { 0. }) / OFFSET) as usize,
        ((mouse_y + if !map.invert_col_origin { OFFSET } else { 0. }) / OFFSET) as usize,
    );
    let mouse_rect = Rect::new(mouse_x, mouse_y, SQUARE_SIZE, SQUARE_SIZE);

    (mouse_x, mouse_y, m_x_offset, m_y_offset, mouse_rect)
}

async fn run_ui<const X: usize, const Y: usize>(map: &mut MapData<X, Y>, map_tools: &mut MapTools) {
    update(map, map_tools).await;
    draw(map, map_tools).await;

    if is_key_pressed(KeyCode::Escape) {
        quit();
    }
}

// TERMINAL FUNCTIONS
fn start_map_terminal<const X: usize, const Y: usize>(map: &mut MapData<X, Y>) {
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

        map.update_cell(x, y, ETypeFloor::from_char(c));
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
