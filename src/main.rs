use engine::{Engine, GPUCamera, GPUSprite, Game};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};
struct TestGame {
    camera: GPUCamera,
}

#[async_trait::async_trait]
impl Game for TestGame {
    async fn init(&mut self, engine: &mut Engine) {
        let (tex_king, mut kingtex_kingpng) = engine
            .load_texture(
                "/Users/calebtheperson/RustProjects/somuchpain/scene2d/src/king.png",
                None,
            )
            .await
            .expect("Couldn't load 47 img");

        engine.sprites.add_sprite_group(
            &engine.gpu,
            tex_king,
            vec![
                //It's the 2 different sprites for king.png at 2 different locations
                GPUSprite {
                    screen_region: [32.0, 32.0, 64.0, 64.0],
                    sheet_region: [0.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
                },
                GPUSprite {
                    screen_region: [32.0, 128.0, 64.0, 64.0],
                    sheet_region: [16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
                },
                GPUSprite {
                    screen_region: [128.0, 32.0, 64.0, 64.0],
                    sheet_region: [0.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
                },
                GPUSprite {
                    screen_region: [128.0, 128.0, 64.0, 64.0],
                    sheet_region: [16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
                },
            ],
            self.camera,
        );
    }
    fn update(&mut self, engine: &mut Engine) {
        engine.sprites.set_camera_all(&engine.gpu, self.camera);
        engine
            .sprites
            .refresh_sprites(&engine.gpu, 0, 0..(engine.sprites.get_sprites(0).len()));
    }
}

fn main() {
    let camera = GPUCamera {
        screen_pos: [0.0, 0.0],
        // Consider using config.width and config.height instead,
        // it's up to you whether you want the window size to change what's visible in the game
        // or scale it up and down
        screen_size: [1024.0, 768.0],
    };
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    Engine::start(event_loop, window, TestGame { camera });
}
