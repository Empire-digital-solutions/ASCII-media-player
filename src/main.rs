use sdl2::pixels::Color;

fn main() {
    let sdl_context = sdl2::init().unwrap(); 
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ASCII media player, made by: Fadhel Hamadah", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
}