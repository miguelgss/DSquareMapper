const USE_UI: bool = true;
const OFFSET: f32 = 20.;
const SQUARE_SIZE: f32 = OFFSET - 1.;

use macroquad::{miniquad::window::quit, prelude::*, ui::{hash, root_ui}};

mod dsquaremapper;
mod terminal_flow;

use crate::terminal_flow::start_map_terminal;

#[macroquad::main(window_conf)]
async fn main() {
    let mut md = dsquaremapper::MapData::default();
    let mut mt = dsquaremapper::MapTools::default();
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

async fn update<const X: usize, const Y: usize>(map: &mut dsquaremapper::MapData<X, Y>, map_tools: &mut dsquaremapper::MapTools) {
    let (_, _, m_x_offset, m_y_offset, _) = mouse_offset(map).await;
    let cell = map.get_cell(m_x_offset - 1, m_y_offset - 1);

    if cell.is_ok() {
        if is_mouse_button_down(MouseButton::Left) {
            map.update_cell(m_x_offset, m_y_offset, map_tools.type_floor_selected);
        } else if is_mouse_button_down(MouseButton::Right) {
            map.update_cell(m_x_offset, m_y_offset, dsquaremapper::ETypeFloor::None);
        }
    }

    match get_last_key_pressed() {
        Some(KeyCode::Key1) => map_tools.change_type_floor_selected(dsquaremapper::ETypeFloor::Floor),
        Some(KeyCode::Key2) => map_tools.change_type_floor_selected(dsquaremapper::ETypeFloor::Door),
        Some(KeyCode::Key3) => map_tools.change_type_floor_selected(dsquaremapper::ETypeFloor::Wall),
        Some(KeyCode::Key4) => map_tools.change_type_floor_selected(dsquaremapper::ETypeFloor::Trap),
        Some(KeyCode::E) => map_tools.change_type_floor_selected(dsquaremapper::ETypeFloor::None),
        _ => (),
    }

    return;
}

async fn draw<const X: usize, const Y: usize>(map: &mut dsquaremapper::MapData<X, Y>, mt: &dsquaremapper::MapTools) {
    clear_background(BLACK);
    for col in map.cells.iter_mut() {
        for cell in col {
            draw_rectangle(
                (cell.x as f32) * OFFSET,
                (cell.y as f32) * OFFSET,
                SQUARE_SIZE,
                SQUARE_SIZE,
                Color::from_hex(dsquaremapper::ETypeFloor::get_color(&cell.type_floor)),
            );
        }
    }

    debug_draw_mouse_data(map).await;
    draw_tools(map, mt).await;
    next_frame().await;
}

async fn draw_tools<const X: usize, const Y: usize>(map: &mut dsquaremapper::MapData<X, Y>, mt: &dsquaremapper::MapTools) {
    draw_text(
        &format!("Type floor: {:?}", mt.type_floor_selected),
        map.map_size_x as f32 * SQUARE_SIZE + 40.,
        20.,
        20.,
        WHITE,
    );

    root_ui().window(hash!(), vec2(map.map_size_x as f32 * SQUARE_SIZE + 40., 50.), vec2(200., 150.), |ui| {
        ui.input_text(hash!(), "Filename", &mut map.name);
        if ui.button(Vec2::new(10., 55.), "Reset") {
            map.reset();
        }

        if ui.button(Vec2::new(10.,75.), "Load") {
            todo!("Implement file loading. Don't know how.")
        }
    });
}

async fn debug_draw_mouse_data<const X: usize, const Y: usize>(map: &mut dsquaremapper::MapData<X, Y>) {
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
    map: &mut dsquaremapper::MapData<X, Y>,
) -> (f32, f32, usize, usize, Rect) {
    let (mouse_x, mouse_y) = mouse_position();
    let (m_x_offset, m_y_offset) = (
        ((mouse_x + if !map.invert_row_origin { OFFSET } else { 0. }) / OFFSET) as usize,
        ((mouse_y + if !map.invert_col_origin { OFFSET } else { 0. }) / OFFSET) as usize,
    );
    let mouse_rect = Rect::new(mouse_x, mouse_y, SQUARE_SIZE, SQUARE_SIZE);

    (mouse_x, mouse_y, m_x_offset, m_y_offset, mouse_rect)
}

async fn run_ui<const X: usize, const Y: usize>(map: &mut dsquaremapper::MapData<X, Y>, map_tools: &mut dsquaremapper::MapTools) {
    update(map, map_tools).await;
    draw(map, map_tools).await;

    if is_key_pressed(KeyCode::Escape) {
        quit();
    }
}

