use nannou::prelude::*;

pub struct Model {
    // Store the state of your application here
    mouse_pos: Point2,
    grid: Vec<Vec<f32>>,
    cell_size: u32,
    mouse_pressed: bool,
    width: u32,
    height: u32,
    hue: f32,
}

pub fn model(app: &App) -> Model {
    let width = 800;
    let height = 800;
    let cell_size = 10;
    app.new_window()
        .size(width, height)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .view(view)
        .build()
        .unwrap();



    // Initialise your application state here
    Model {
        mouse_pos: vec2(0.0, 0.0),
        cell_size,
        grid: vec![vec![0.0; (width / cell_size) as usize]; (height / cell_size) as usize],
        mouse_pressed: false,
        width,
        height,
        hue: 0.0,
   }

}

pub fn view(_app: &App, model: &Model, frame: Frame) {
    // Draw your application here
    let draw = _app.draw();
    draw.background().color(BLACK);

    



    model.grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| {
            if *cell > 0.0 {
                draw.rect()
                .x(i as f32 * model.cell_size as f32 - model.width as f32 / 2.0)
                .y(j as f32 * model.cell_size as f32 - model.height as f32 / 2.0)
                .w_h(model.cell_size as f32, model.cell_size as f32)
                .color(Hsv::new(*cell, 1.0, 1.0));
            }
        });
    });
draw.to_frame(_app, &frame).unwrap();
}

pub fn mouse_moved(_app: &App, model: &mut Model, pos: Vec2) {
    model.mouse_pos = pos;
}

pub fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = true;
    
}

pub fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.mouse_pressed = false;
}
pub fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut new_grid = vec![vec![0.0; (model.width / model.cell_size) as usize]; (model.height / model.cell_size) as usize];

    if model.mouse_pressed {
        place_sand(model, model.hue);
        model.hue += 0.1;
        if model.hue > 360.0 {
            model.hue = 1.0;
        }
    }

    model.grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, cell)| {
            if *cell > 0.0 {
                let lower_a = (i+1)<model.grid.len() && j>1 && model.grid[i+1][j-1] == 0.0;
                let lower_b = i>0 && j>1 && model.grid[i-1][j-1] == 0.0;

                if j>1 && model.grid[i][j-1] == 0.0 {
                    new_grid[i][j-1] = model.grid[i][j];
                } else if lower_a {
                    new_grid[i+1][j-1] = model.grid[i][j];
                } else if lower_b{
                    new_grid[i-1][j-1] = model.grid[i][j];
                } else {
                    new_grid[i][j] = model.grid[i][j];
                }
            }
        });
    });
    model.grid = new_grid;
    
}

fn place_sand(model: &mut Model, hue: f32) {
    if model.mouse_pos.x <= -(model.width as f32 / 2.0) || model.mouse_pos.x >= model.width as f32 / 2.0 || model.mouse_pos.y <= -(model.height as f32 / 2.0) || model.mouse_pos.y >= model.height as f32 / 2.0 {
        return;
    }
    let x = (model.mouse_pos.x+(model.width as f32 / 2.0)) as u32 / model.cell_size;
    let y = (model.mouse_pos.y+(model.height as f32 / 2.0)) as u32 / model.cell_size;
    model.grid[x as usize][y as usize] = hue;
}