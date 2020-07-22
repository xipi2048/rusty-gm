use {
    skulpin::{
        app::{AppBuilder, AppDrawArgs, AppError, AppHandler, AppUpdateArgs, VirtualKeyCode},
        skia_safe, CoordinateSystem, LogicalSize,
    },
    std::ffi::CString,
};

fn main() {
    let main_app = MainApp::new();
    let logical_size = LogicalSize::new(900, 600);
    let visible_range = skulpin::skia_safe::Rect {
        left: 0.0,
        right: logical_size.width as f32,
        top: 0.0,
        bottom: logical_size.height as f32,
    };
    let scale_to_fit = skulpin::skia_safe::matrix::ScaleToFit::Center;

    AppBuilder::new()
        .app_name(CString::new("Game Master").unwrap())
        .use_vulkan_debug_layer(true)
        .inner_size(logical_size)
        .coordinate_system(CoordinateSystem::VisibleRange(visible_range, scale_to_fit))
        .run(main_app);
}

struct MainApp {}
impl MainApp {
    pub fn new() -> Self {
        MainApp {}
    }
}

impl AppHandler for MainApp {
    fn update(&mut self, update_args: AppUpdateArgs) {
        
    }
    fn draw(&mut self, draw_args: AppDrawArgs) {
        let canvas = draw_args.canvas;
        canvas.clear(skia_safe::Color::from_argb(0, 0, 0, 255));
    }
    fn fatal_error(&mut self, error: &AppError) {
        println!("{}", error);
    }
}
