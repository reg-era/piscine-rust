use road_intersection::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut delta_timer = DeltaTimer::new();
    let mut traffic_light = TrafficLight::new();
    let mut vehicles = Vehicles::new();
    'running: loop {
        let delta = delta_timer.delta();
        // traffic_light.light_rg(delta);
        traffic_light.controle_lights(delta);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let vehicles_id =
                        vehicles.spawn(Rect::new(350, 0, TILE_SIZE, TILE_SIZE), Side::North);
                    if let Some(id) = vehicles_id {
                        traffic_light.add_to_stack(Side::North, id);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let vehicles_id =
                        vehicles.spawn(Rect::new(400, 550, TILE_SIZE, TILE_SIZE), Side::South);
                    if let Some(id) = vehicles_id {
                        traffic_light.add_to_stack(Side::South, id);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let vehicles_id =
                        vehicles.spawn(Rect::new(0, 300, TILE_SIZE, TILE_SIZE), Side::East);
                    if let Some(id) = vehicles_id {
                        traffic_light.add_to_stack(Side::East, id);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let vehicles_id =
                        vehicles.spawn(Rect::new(750, 250, TILE_SIZE, TILE_SIZE), Side::West);
                    if let Some(id) = vehicles_id {
                        traffic_light.add_to_stack(Side::West, id);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let rng = rand::random_range(0..4);
                    let side: (Side, i32, i32) = match rng {
                        0 => (Side::West, 750, 250),
                        1 => (Side::East, 0, 300),
                        2 => (Side::North, 350, 0),
                        3 => (Side::South, 400, 550),
                        _ => (Side::South, 0, 0),
                    };
                    let vehicles_id = vehicles.spawn(
                        Rect::new(side.1, side.2, TILE_SIZE, TILE_SIZE),
                        side.0.clone(),
                    );
                    if let Some(id) = vehicles_id {
                        traffic_light.add_to_stack(side.0, id);
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        vehicles.move_cars(delta, &traffic_light.lights);

        draw_road(&mut canvas);
        traffic_light.draw(&mut canvas);
        vehicles.draw(&mut canvas, &mut traffic_light);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
