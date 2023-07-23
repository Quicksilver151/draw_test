use raylib::prelude::*;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);
    let font = rl.load_font_ex(&thread, "assets/Ubuntu.ttf", 32, FontLoadEx::Default(0)).unwrap();
    
    
    let mut points :Vec<Vector2> = vec![];
    let mut rect_pos:Vector2 = Vector2::default();
    
    while !rl.window_should_close() {
        let mut d:RaylibDrawHandle = rl.begin_drawing(&thread);
        
        let mouse_pos:Vector2 = d.get_mouse_position();
        let x = d.is_key_down(KeyboardKey::KEY_RIGHT) as i32 - d.is_key_down(KeyboardKey::KEY_LEFT) as i32;
        let y = d.is_key_down(KeyboardKey::KEY_DOWN) as i32 - d.is_key_down(KeyboardKey::KEY_UP) as i32;
        
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            points.append(&mut vec![d.get_mouse_position()]);
        }
        if d.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            points.append(&mut vec![d.get_mouse_position()]);
        };
        if points.len() > 500 {
            points.reverse();
            points.pop();
            points.reverse();
        }
        draw_poly_line(&mut d, &points);
        move_poly_line(&mut points);
        d.draw_text_ex(&font, "Coooool line drawer 500000.1", Vector2{x:32.0,y:32.0}, 32.0, 0.0, Color::BLACK);
        rect_pos.x += x as f32;
        rect_pos.y += y as f32;
        d.draw_rectangle_v(rect_pos, mouse_pos, Color::GREEN);
        d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 10.0, raylib::prelude::color::rcolor(100,100,100,200));
        d.draw_text("Coooool line drawer 500000.1", 12, 12, 20, Color::BLACK);
        
        d.clear_background(Color::WHITE);
    }
    
    // rl.unload_font(font)
}

fn draw_poly_line(d: &mut RaylibDrawHandle, points: &[Vector2]) {
    for (i, point) in points.iter().enumerate() {
        let prev_i:usize = (i as i32 - 1).max(0) as usize;
        
        d.draw_line(point.x as i32, point.y as i32, points[prev_i].x as i32, points[prev_i].y as i32, raylib::prelude::color::rcolor(100,100,100,100));
    }
}

fn move_poly_line(points: &mut [Vector2]) {
    let len = points.len() as f32;
    for (i, point) in points.iter_mut().enumerate() {
        let multipiler = 1.01;
        // dbg!(multipiler);
        point.y *= multipiler;
    }
}
