const MAP_SIZE_X: usize = 20;
const MAP_SIZE_Y: usize = 20;
const INVERT_ROW_ORIGIN: bool = true;
const INVERT_COL_ORIGIN: bool = false;

use std::io::{Write, stdin, stdout};
use std::fs::File;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "DSquareMapper".to_owned(),
        fullscreen: false,
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

struct GlobalSettings<const X: usize, const Y: usize> {
    map: [[char; X]; Y],
    squaresMap: [[Rect; X]; Y],
    mapSizeX: usize,
    mapSizeY: usize,
    invertRowOrigin: bool,
    invertColOrigin: bool
}

impl Default for GlobalSettings::<20, 20> {
    fn default() -> Self {
        GlobalSettings::<20, 20> {
            map: [[' '; 20]; 20],
            squaresMap: [[Rect::new(0.0, 0.0, 10.0, 10.0); 20]; 20],
            mapSizeX: 20,
            mapSizeY: 20,
            invertRowOrigin: true,
            invertColOrigin: false
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut global_settings = GlobalSettings::default();
    // let map: [[char; MAP_SIZE_X]; MAP_SIZE_Y] = [[' '; MAP_SIZE_X]; MAP_SIZE_Y];
    // start_map_terminal(map);
    loop {
        run(&mut global_settings).await;
    }

}

// UI FUNCTIONS

async fn update<const X:usize, const Y:usize>(gs: &mut GlobalSettings<X,Y>) {
    ()
}

async fn draw() {
    clear_background(DARKGREEN);
    draw_circle(mouse_position().0, mouse_position().1, 10.0, PURPLE);
    next_frame().await
}

async fn run<const X:usize, const Y:usize>(gs: &mut GlobalSettings<X,Y>) {
    update(gs).await;
    draw().await;
}

// TERMINAL FUNCTIONS
fn start_map_terminal<const N: usize>(mut map: [[char; N]; N]) {
    loop {
        println!("{}", format_map(map));
        let mut s: String = String::new();
        print!("Pass: (X, Y, Type of floor): ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
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

        map[x][y] = result[2].parse::<char>().unwrap();
        println!("You typed: {}; Last updated coordinates: {}/{}", s, x, y);

        let _ = update_map_file(map);
    }
}

fn update_map_file<const N: usize>(map: [[char; N]; N]) -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(format_map(map).as_bytes())?;

    Ok(())
}

fn format_map<const N: usize>(map: [[char; N]; N]) -> String{
    let mut count: i8 = if INVERT_ROW_ORIGIN {19 } else { 0 };
    let mut formmated_map: String = "".to_owned();
    for line in map.iter() {
        formmated_map.push_str(&(format!("{:002} - ", count)));
        for cell in line.iter() {
            formmated_map.push_str(&(format!("[{}]", cell)));
        }
        count = if INVERT_ROW_ORIGIN { count - 1 } else { count + 1 };
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
