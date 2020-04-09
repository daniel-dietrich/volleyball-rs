use crate::{
    audio::initialize_audio,
    components::{
        ball::Ball,
        player::{Player, Side, PLAYER_HEIGHT, PLAYER_WIDTH},
    },
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, transform::Transform},
    ecs::Entity,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

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
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialize_camera(world);
        initialize_ball(world, sprite_sheet_handle.clone());
        initialize_players(world, sprite_sheet_handle);
        initialize_scoreboard(world);
        initialize_audio(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_players(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let offset_x = PLAYER_WIDTH / 2.0;
    let offset_y = PLAYER_HEIGHT / 2.0;

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    left_transform.set_translation_xyz(offset_x, offset_y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - offset_x, offset_y, 0.0);

    left_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
    right_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));

    let left_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    let right_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
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
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    transform.set_scale(amethyst::core::math::Vector3::new(2.0, 2.0, 1.0));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2,
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
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let p1_transform = UiTransform::new(
        String::from("P1"),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
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

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_handle = {
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        loader.load(
            "textures/spritesheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_storage,
        )
    };

    sprite_sheet_handle
}
