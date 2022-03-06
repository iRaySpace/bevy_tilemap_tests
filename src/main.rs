use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tiles.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let layer_settings = LayerSettings::new(
        MapSize(1, 1),
        ChunkSize(64, 64),
        TileSize(16.0, 16.0),
        TextureSize(96.0, 16.0),
    );

    // let center = layer_settings.get_pixel_center();

    let (mut layer_builder, layer_entity) =
        LayerBuilder::<TileBundle>::new(&mut commands, layer_settings, 0u16, 0u16);

    layer_builder
        .set_tile(
            TilePos(0, 0),
            Tile {
                texture_index: 2,
                ..Default::default()
            }
            .into(),
        )
        .unwrap();
    
        layer_builder
        .set_tile(
            TilePos(0, 1),
            Tile {
                texture_index: 0, // light green
                ..Default::default()
            }
            .into(),
        )
        .unwrap();

        layer_builder
        .set_tile(
            TilePos(1, 0),
            Tile {
                texture_index: 1, // blue
                ..Default::default()
            }
            .into(),
        )
        .unwrap();

    // TODO: tilemap builder from csv
    // layer_builder.set_all(TileBundle {
    //     tile: Tile {
    //         texture_index: 2,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    map_query.build_layer(&mut commands, layer_builder, texture_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);
    
    // info!("{:?}", center);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(GlobalTransform::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy Tilemap Tests".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(set_texture_filters_to_nearest)
        .run();
}
