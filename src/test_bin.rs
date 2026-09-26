use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use wry::WebViewBuilder;

fn main() {
    let el = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&el).unwrap();
    let _wv = WebViewBuilder::new().build(&window).unwrap();
}
