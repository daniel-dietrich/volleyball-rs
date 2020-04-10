use crate::{
    audio::initialize_audio,
    components::{
        ball::Ball,
        player::{Player, Side, PLAYER_HEIGHT, PLAYER_WIDTH},
    },
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const WINDOW_HEIGHT: f32 = 500.0;
pub const WINDOW_WIDTH: f32 = 500.0;

#[derive(Default)]
pub struct ScoreBoard {
    pub player_1: u32,
    pub player_2: u32,
}
pub struct ScoreText {
    pub player_1: Entity,
    pub player_2: Entity,
}

pub struct Volleyball;

impl SimpleState for Volleyball {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let objects_sprite_sheet_handle = load_sprite_sheet(world, "spritesheet");
        let background_sprite_sheet_handle = load_sprite_sheet(world, "background");

        initialize_camera(world);
        initialize_background(world, background_sprite_sheet_handle);
        initialize_players(world, objects_sprite_sheet_handle.clone());
        initialize_ball(world, objects_sprite_sheet_handle.clone());
        initialize_scoreboard(world);
        initialize_audio(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_background(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WINDOW_WIDTH / 2.0, WINDOW_WIDTH / 2.0, -1.0);

    let background_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(background_render)
        .with(transform)
        .build();
}

fn initialize_players(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let offset_x = PLAYER_WIDTH / 2.0;
    let offset_y = PLAYER_HEIGHT / 2.0;

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    left_transform.set_translation_xyz(offset_x, offset_y, 0.0);
    right_transform.set_translation_xyz(WINDOW_WIDTH - offset_x, offset_y, 0.0);

    let left_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    let right_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2,
    };

    world
        .create_entity()
        .with(Player::new(Side::Left))
        .with(left_sprite_render)
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(Player::new(Side::Right))
        .with(right_sprite_render)
        .with(right_transform)
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Ball::new())
        .with(sprite_render)
        .with(transform)
        .build();
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/kenvector_future.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let p1_transform = UiTransform::new(
        String::from("P1"),
        Anchor::TopMiddle,
        Anchor::Middle,
        -50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
    );

    let p2_transform = UiTransform::new(
        String::from("P2"),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            String::from("0"),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            String::from("0"),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        ))
        .build();

    world.insert(ScoreText {
        player_1: p1_score,
        player_2: p2_score,
    });
}

fn load_sprite_sheet(world: &mut World, filename: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let png_path = format!("textures/{}.png", filename);
    let ron_path = format!("textures/{}.ron", filename);

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };

    let sprite_sheet_handle = {
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        loader.load(
            ron_path,
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_storage,
        )
    };

    sprite_sheet_handle
}
