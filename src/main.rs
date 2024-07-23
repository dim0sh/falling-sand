mod app;

fn main() {
    nannou::app(app::model)
        .update(app::update)
        .run();
}
