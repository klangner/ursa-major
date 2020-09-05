mod components;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        math::{Point3, Vector3},
        Transform, TransformBundle,
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        camera::Camera,
        debug_drawing::DebugLinesComponent,
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        types::DefaultBackend,
        RenderDebugLines, RenderFlat2D, RenderToWindow, RenderingBundle, Texture,
        palette::Srgba
    },
    tiles::{MortonEncoder, RenderTiles2D, Tile, TileMap},
    utils::application_root_dir,
    window::ScreenDimensions,
    winit,
};
use mapgen::dungeon::{
    MapBuilder,
    map::{Map, Point, TileType},
    cellular_automata::CellularAutomataGen,
    starting_point::{AreaStartingPosition, XStart, YStart},
    cull_unreachable::CullUnreachable,
    distant_exit::DistantExit,
};
use components::*;


#[derive(Default, Clone)]
struct MapTile;
impl Tile for MapTile {
    fn sprite(&self, tile: Point3<u32>, world: &World) -> Option<usize> {
        let map = world.read_resource::<Map>();
        let renderables = world.read_storage::<Renderable>();
        let positions = world.read_storage::<Position>();
        let point = Point::new(tile.x as usize, tile.y as usize);

        // Check if there is renderable entity at the tile
        for (r, p) in (&renderables, &positions).join() {
            if p.x == point.x && p.y == point.y {
                return Some(r.glyph);
            }
        }

        if map.exit_point == Some(point) {
            Some(12)
        } else if map.at(point.x, point.y) == TileType::Wall {
            Some(60)
        } else {
            Some(19)
        }
    }

    fn tint(&self, p: Point3<u32>, world: &World) -> Srgba {
        let map = world.read_resource::<Map>();
        let pos = Some(Point::new(p.x as usize, p.y as usize));
        if map.starting_point == pos || map.exit_point == pos {
            Srgba::new(1.0, 1.0, 0.0, 1.0)
        } else {
            Srgba::new(1.0, 1.0, 1.0, 1.0)
        }
    }
}

fn load_tiles_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn init_camera(world: &mut World, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(camera)
        .named("camera")
        .build()
}

fn generate_map() -> Map {
    MapBuilder::new(Box::new(CellularAutomataGen::new(80, 50)))
        .with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER))
        .with(CullUnreachable::new())
        .with(DistantExit::new())
        .build_map()
}

fn init_player(world: &mut World, pos: Position) -> Entity {
    world
        .create_entity()
        .with(pos)
        .with(Player {})
        .with(Renderable::new(160))
        .named("camera")
        .build()
}

struct PlayState;
impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Position>();
        world.register::<Player>();
        world.register::<Renderable>();

        // Create map
        let map = generate_map();

        let map_sprite_sheet_handle =
            load_tiles_sprite_sheet(world, "texture/basic.png", "texture/basic.ron");

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _camera = init_camera(
            world,
            Transform::from(Vector3::new(0.0, 0.0, 1.0)),
            Camera::standard_2d(width, height),
        );

        // create a test debug lines entity
        let _ = world
            .create_entity()
            .with(DebugLinesComponent::with_capacity(1))
            .build();

        let tile_map = TileMap::<MapTile, MortonEncoder>::new(
            Vector3::new(80, 50, 1),
            Vector3::new(52, 52, 1),
            Some(map_sprite_sheet_handle),
        );

        let _map_entity = world
            .create_entity()
            .with(tile_map)
            .with(Transform::default())
            .build();

        let player_pos = map.starting_point.unwrap_or(Point::new(0, 0));
        let _player = init_player(world, Position::new(player_pos.x, player_pos.y));
        world.insert(map);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("roguelike", log::LevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;
    let assets_directory = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file("config/input.ron")?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<MapTile, MortonEncoder>::default()),
        )?;

    let mut game = Application::build(assets_directory, PlayState)?.build(game_data)?;
    game.run();
    Ok(())
}