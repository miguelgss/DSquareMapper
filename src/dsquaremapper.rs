use std::fs::File;
use std::io::{Error, ErrorKind, Write};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ETypeFloor {
    None = b' ',
    Floor = b'f',
    Door = b'd',
    Trap = b't',
    Wall = b'w',
}

impl ETypeFloor {
    pub fn from_char(c: char) -> Self {
        match c {
            ' ' => ETypeFloor::None,
            'f' => ETypeFloor::Floor,
            'd' => ETypeFloor::Door,
            't' => ETypeFloor::Trap,
            'w' => ETypeFloor::Wall,
            _ => ETypeFloor::None,
        }
    }

    pub fn get_color(&self) -> u32 {
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
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub type_floor: ETypeFloor,
}

#[allow(dead_code)]
impl Cell {
    pub fn _new(x: usize, y: usize, type_floor: ETypeFloor) -> Self {
        Self { x, y, type_floor }
    }

    pub fn update(&mut self, type_floor: ETypeFloor) {
        self.type_floor = type_floor;
    }

    pub fn get_data(self) -> (usize, usize, ETypeFloor) {
        (self.x, self.y, self.type_floor)
    }
}

pub struct MapTools {
    pub type_floor_selected: ETypeFloor,
}

impl MapTools {
    pub fn change_type_floor_selected(&mut self, t: ETypeFloor) {
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
pub struct MapData<const X: usize, const Y: usize> {
    pub name: String,
    pub cells: [[Cell; X]; Y],
    pub map_size_x: usize,
    pub map_size_y: usize,
    pub invert_row_origin: bool,
    pub invert_col_origin: bool,
}

#[allow(dead_code)]
impl<const X: usize, const Y: usize> MapData<X, Y> {
    pub fn new(
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

    pub fn reset(&mut self) {
        let cells: [[Cell; X]; Y] = std::array::from_fn(|y: usize| {
            std::array::from_fn(|x: usize| Cell {
                x,
                y,
                type_floor: ETypeFloor::from_char(' '),
            })
        });
        self.cells = cells;

        let _ = self.update_map_file();
    }

    pub fn get_cell(&self, x: usize, y: usize) -> std::io::Result<&Cell> {
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

    pub fn update_cell(&mut self, x: usize, y: usize, t: ETypeFloor) {
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

    pub fn format_map(&self) -> String {
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

    pub fn update_map_file(&self) -> std::io::Result<()> {
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
