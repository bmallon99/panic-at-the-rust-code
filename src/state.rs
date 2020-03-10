use amethyst::{
    assets::{AssetStorage, Loader},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

// use bincode::{deserialize, serialize};
use std::net::UdpSocket;
use std::time::Duration;

use log::info;

/*** Current State resources ***/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
    Menu,
    Gameplay,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Menu
    }
}

pub struct Game {
    pub current_state: CurrentState,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            current_state: CurrentState::default(),
        }
    }
}

/*** Component Constants ***/
pub const CRAB_HEIGHT: f32 = 60.0;
pub const CRAB_WIDTH: f32 = 117.0;
pub const CRAB_VELOCITY_Y: f64 = 2.5;

pub const PLATFORM_HEIGHT: f32 = 40.0;
pub const PLATFORM_WIDTH: f32 = 262.0;

pub const ARENA_HEIGHT: f32 = 600.0;
pub const ARENA_WIDTH: f32 = 800.0;

/*** Game States ***/
#[derive(Default)]
pub struct Crabby {
    crab_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<SpriteRender>,
}

#[derive(Default)]
pub struct MenuState {
    text_handler: Option<Entity>,
}

pub struct LoseState;

pub struct MultiplayerState {
    crab_spawn_timer: Option<f32>,
    crab: Option<Entity>,
    krab: Option<Entity>,
    socket: Option<UdpSocket>,
}

impl MultiplayerState {
    fn new(s: UdpSocket) -> Self {
        MultiplayerState {
            crab_spawn_timer: None,
            crab: None,
            krab: None,
            socket: Some(s),
        }
    }
}

/*** Components ***/
/// C R A B
pub struct Crab {
    pub velocity: f64,
    pub jump_start_time: f64,
    pub width: f32,
    pub height: f32,
    pub x_position: f32,
}

impl Crab {
    fn new(time: f64) -> Crab {
        Crab {
            velocity: CRAB_VELOCITY_Y,
            jump_start_time: time,
            width: CRAB_WIDTH,
            height: CRAB_HEIGHT,
            x_position: ARENA_WIDTH * 0.5,
        }
    }
}

impl Component for Crab {
    type Storage = DenseVecStorage<Self>;
}

pub struct Krab {
    pub velocity: f64,
    pub jump_start_time: f64,
    pub width: f32,
    pub height: f32,
    pub old_x_position: f32,
    pub new_x_position: f32,
}

impl Krab {
    fn new(time: f64) -> Krab {
        Krab {
            velocity: CRAB_VELOCITY_Y,
            jump_start_time: time,
            width: CRAB_WIDTH,
            height: CRAB_HEIGHT,
            old_x_position: ARENA_WIDTH * 0.5,
            new_x_position: ARENA_WIDTH * 0.5,
        }
    }
}

impl Component for Krab {
    type Storage = DenseVecStorage<Self>;
}

/// Platform
pub struct Platform {
    pub width: f32,
    pub height: f32,
}

impl Platform {
    fn new() -> Platform {
        Platform {
            width: PLATFORM_WIDTH,
            height: PLATFORM_HEIGHT,
        }
    }
}

impl Component for Platform {
    type Storage = DenseVecStorage<Self>;
}

/*** CRABBY STATE ***/

impl SimpleState for Crabby {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.crab_spawn_timer.replace(2.0);

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        self.sprite_sheet_handle
            .replace(load_sprite(world, "Ferris"));
        //init_sprites(world, &sprites, &dimensions);

        world.register::<Crab>();

        // Load platform sprite
        let platform_sprite = load_sprite(world, "platform_blue");

        world.register::<Platform>();
        init_platform(world, platform_sprite.clone(), ARENA_WIDTH / 2.0, 0.0);
        init_platform(
            world,
            platform_sprite.clone(),
            ARENA_WIDTH / 4.0,
            ARENA_HEIGHT / 4.0,
        );
        init_platform(
            world,
            platform_sprite.clone(),
            ARENA_WIDTH / 2.0,
            ARENA_HEIGHT / 2.0,
        );
        init_platform(
            world,
            platform_sprite,
            3.0 * ARENA_WIDTH / 4.0,
            3.0 * ARENA_HEIGHT / 4.0,
        );
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        // Keep going
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.crab_spawn_timer.take() {
            // If the timer isn't expired yet, subtract the time that passed since the last update.
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                // When timer expire, spawn the crab
                init_crab(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                // If timer is not expired yet, put it back onto the state.
                self.crab_spawn_timer.replace(timer);
            }
        }

        if data.world.write_resource::<Game>().current_state == CurrentState::Menu {
            return Trans::Push(Box::new(LoseState));
        }

        Trans::None
    }
}

/**** Multiplayer oh god ****/

impl SimpleState for MultiplayerState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.crab_spawn_timer.replace(0.0);

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        /*self.crab_sprite_sheet_handle
            .replace(load_sprite(world, "Ferris"));

        self.krab_sprite_sheet_handle
            .replace(load_sprite(world, "Ferris_blue"));*/
        let crab_sheet = load_sprite(world, "Ferris");
        self.crab.replace(init_crab(world, crab_sheet));
        let krab_sheet = load_sprite(world, "Ferris_blue");
        self.krab.replace(init_krab(world, krab_sheet));

        world.register::<Crab>();
        world.register::<Krab>();

        // Load platform sprite
        let platform_sprite = load_sprite(world, "platform_blue");

        world.register::<Platform>();
        init_platform(world, platform_sprite.clone(), ARENA_WIDTH / 2.0, 0.0);
        init_platform(
            world,
            platform_sprite.clone(),
            ARENA_WIDTH / 4.0,
            ARENA_HEIGHT / 4.0,
        );
        init_platform(
            world,
            platform_sprite.clone(),
            ARENA_WIDTH / 2.0,
            ARENA_HEIGHT / 2.0,
        );
        init_platform(
            world,
            platform_sprite,
            3.0 * ARENA_WIDTH / 4.0,
            3.0 * ARENA_HEIGHT / 4.0,
        );
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        // Keep going
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        /*
        if let Some(mut timer) = self.crab_spawn_timer.take() {
            // If the timer isn't expired yet, subtract the time that passed since the last update.
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                // When timer expire, spawn the crab
                init_crab(data.world, self.crab_sprite_sheet_handle.clone().unwrap());
                init_krab(data.world, self.krab_sprite_sheet_handle.clone().unwrap());
            } else {
                // If timer is not expired yet, put it back onto the state.
                self.crab_spawn_timer.replace(timer);
            }
        }*/
        if data.world.write_resource::<Game>().current_state == CurrentState::Menu {
            return Trans::Push(Box::new(LoseState));
        }

        let sock = self.socket.as_ref().unwrap();
        let crab_storage = data.world.read_storage::<Crab>();
        let crab_entity = crab_storage.get(self.crab.unwrap()).unwrap();
        let message = bincode::serialize(&crab_entity.x_position).unwrap();
        match sock.send_to(&message, "192.168.0.127:34255") {
            Err(e) => println!("Network error {}", e),
            _ => {}
        }

        let mut buf = [0; 16];
        let mut storage = data.world.write_storage::<Krab>();
        let mut krab_entity = storage.get_mut(self.krab.unwrap()).unwrap();
        sock.recv_from(&mut buf).expect("Didn't receive data");
        let received: f32 = bincode::deserialize(&buf).unwrap();
        krab_entity.old_x_position = krab_entity.new_x_position;
        krab_entity.new_x_position = received;

        Trans::None
    }
}

/*** MENU STATE ***/

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        let text = load_sprite(world, "text");

        self.text_handler.replace(init_text(world, text));
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if is_key_down(&event, VirtualKeyCode::Key1) {
                if let Some(entity) = self.text_handler {
                    match data.world.delete_entity(entity) {
                        Err(e) => println!("what? {}", e),
                        _ => {}
                    }
                }
                data.world.write_resource::<Game>().current_state = CurrentState::Gameplay;
                return Trans::Push(Box::new(Crabby::default()));
            }
            if is_key_down(&event, VirtualKeyCode::Key2) {
                if let Some(entity) = self.text_handler {
                    match data.world.delete_entity(entity) {
                        Err(e) => println!("what? {}", e),
                        _ => {}
                    }
                }
                data.world.write_resource::<Game>().current_state = CurrentState::Gameplay;
                if let Ok(socket) = UdpSocket::bind("192.168.0.149:34254") {
                    match socket.set_read_timeout(Some(Duration::new(1, 0))) {
                        Err(e) => println!("what {}", e),
                        _ => {}
                    }
                    return Trans::Push(Box::new(MultiplayerState::new(socket)));
                } else {
                    info!("Connection refuse");
                }
            }
        }

        // Keep going
        Trans::None
    }
}

/*** LOSE STATE ***/

impl SimpleState for LoseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        let text = load_sprite(world, "lose");

        init_text(world, text);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        // Keep going
        Trans::None
    }
}

/**** INITIALIZTION FUNCTIONS  ****/

/// Initialise text at the top middle
fn init_text(world: &mut World, sprite: SpriteRender) -> Entity {
    let mut transform = Transform::default();

    // Correctly position the text.
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.90, 0.0);

    // Create a text entity.
    world.create_entity().with(sprite).with(transform).build()
}

/// Initialise the crab in the middle on the ground
fn init_crab(world: &mut World, sprite: SpriteRender) -> Entity {
    let mut transform = Transform::default();

    // Correctly position the crab.
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, CRAB_HEIGHT, 0.0);
    let time = Time::default();
    let crab = Crab::new(time.absolute_real_time_seconds());

    // Create a crab entity.
    world
        .create_entity()
        .with(sprite)
        .with(crab)
        .with(transform)
        .build()
}

/// Initialise the krab in the middle on the ground
fn init_krab(world: &mut World, sprite: SpriteRender) -> Entity {
    let mut transform = Transform::default();

    // Correctly position the crab.
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, CRAB_HEIGHT, 0.0);
    let time = Time::default();
    let krab = Krab::new(time.absolute_real_time_seconds());

    // Create a crab entity.
    world
        .create_entity()
        .with(sprite)
        .with(krab)
        .with(transform)
        .build()
}

/// Initialise the platform at specified position
fn init_platform(world: &mut World, sprite: SpriteRender, x: f32, y: f32) {
    let mut transform = Transform::default();

    // Correctly position the platform.
    transform.set_translation_xyz(x, y, 0.0);

    // Create a platform entity.
    world
        .create_entity()
        .with(sprite)
        .with(Platform::new())
        .with(transform)
        .build();
}

/// Initialize camera
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

/// Load specified sprite as SpriteRender
fn load_sprite(world: &mut World, sprite: &str) -> SpriteRender {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let mut sprite_name = sprite.clone().to_string();
        sprite_name.push_str(".png");
        let mut sprite_path = "sprites/".to_string();
        sprite_path.push_str(&sprite_name);
        loader.load(sprite_path, ImageFormat::default(), (), &texture_storage)
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        let mut sprite_name = sprite.clone().to_string();
        sprite_name.push_str(".ron");
        let mut sprite_path = "sprites/".to_string();
        sprite_path.push_str(&sprite_name);
        loader.load(
            sprite_path,
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0,
    }
}
