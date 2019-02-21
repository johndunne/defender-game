use amethyst::assets::Loader;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera,
    Event,
    KeyboardInput,
    Projection,
    VirtualKeyCode,
    WindowEvent,
};
use amethyst::ui::{
    Anchor,
    TtfFormat,
    UiText,
    UiTransform
};
use rand::prelude::*;

pub mod config;
use config::{
    consts::{
        FRAC_WIN_HEIGHT_2,
        FRAC_WIN_WIDTH_2,
        WIN_HEIGHT,
        WIN_WIDTH,
    },
    BulletConfig,
    EnemyConfig,
    GameConfig,
    PlayerConfig,
};

mod entity;
use entity::{
    Bullet,
    BulletResource,
    Enemy,
    EnemyResource,
    Player,
    ScoreText
};

mod render;
use render::{
    create_mesh,
    create_material,
    generate_rectangle_vertices,
    generate_triangle_vertices,
};

pub mod systems;

pub struct Defender;

impl SimpleState for Defender {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Initialize entities that exist at the beginning.
        initialize_camera(world);
        initialize_enemies(world);
        initialize_player(world);
        // Initialize resources
        initialize_bullet(world);
        initialize_score(world);
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => Trans::Quit,
                        WindowEvent::CloseRequested => Trans::Quit,
                        _ => Trans::None,
                    }
                },
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}

fn initialize_bullet(world: &mut World) {
    let (dimensions, color) = {
        let config = &world.read_resource::<BulletConfig>();
        (config.dimensions, config.color)
    };

    let bullet_mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, dimensions[0], dimensions[1])
    );

    let bullet_material = create_material(world, color);
    let bullet_resource = BulletResource {
        material: bullet_material,
        mesh: bullet_mesh
    };

    // Register bullet entity & add resource so we can use it later.
    world.register::<Bullet>();
    world.add_resource(bullet_resource.clone());
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -FRAC_WIN_WIDTH_2,
            FRAC_WIN_WIDTH_2,
            -FRAC_WIN_HEIGHT_2,
            FRAC_WIN_HEIGHT_2,
        )))
        .with(transform)
        .build();
}

fn initialize_enemies(world: &mut World) {
    let mut rng = rand::thread_rng();

    let dimensions = {
        let config = &world.read_resource::<EnemyConfig>();
        config.dimensions
    };

    let num_enemies = {
        let config = &world.read_resource::<GameConfig>();
        config.enemy_count
    };

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, dimensions[0], dimensions[1])
    );

    let material = create_material(world, [1.0, 0.0, 0.0, 1.0]);
    // let resource = EnemyResource { material, mesh };

    world.register::<Enemy>();
    for _ in 0..num_enemies {
        let mut transform = Transform::default();
        let x = (rng.gen::<f32>() * WIN_WIDTH - FRAC_WIN_WIDTH_2)
            .min(FRAC_WIN_WIDTH_2)
            .max(-FRAC_WIN_WIDTH_2);

        let y: f32 = (rng.gen::<f32>() * WIN_HEIGHT - FRAC_WIN_HEIGHT_2)
            .min(FRAC_WIN_HEIGHT_2)
            .max(-FRAC_WIN_HEIGHT_2);

        transform.set_xyz(x, y, 0.0);

        world.create_entity()
            .with(mesh.clone())
            .with(material.clone())
            .with(Enemy::default())
            .with(transform)
            .build();
    }
}

fn initialize_player(world: &mut World) {
    let mut player_transform = Transform::default();
    player_transform.set_xyz(0.0, 0.0, 0.0);

    let (dimensions, color) = {
        let config = &world.read_resource::<PlayerConfig>();
        (config.dimensions, config.color)
    };

    let player_mesh = create_mesh(
        world,
        generate_triangle_vertices(0.0, 0.0, dimensions[0], dimensions[1])
    );

    let player_material = create_material(world, color);

    // Create player triangle
    world.create_entity()
        .with(player_mesh)
        .with(player_material)
        .with(Player {
            direction: 0.0,
            weapon_cooldown: 0.0
        })
        .with(player_transform)
        .build();
}

fn initialize_score(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "resources/fonts/PxPlus_IBM_VGA8.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let transform = UiTransform::new(
        "Score".to_string(),
        Anchor::TopLeft,
        // x, y, z
        85.0, -20.0, 1.0,
        // width, height
        400.0, 40.0,
        // Tab order
        0
    );

    let text = world.create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "Score: 00000".to_string(),
            [1., 1., 1., 1.],
            25.,
        )).build();

    world.add_resource(ScoreText { text } );
}