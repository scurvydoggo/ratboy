use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        }
    }
};
use bevy_rapier2d::prelude::*;

const TERMINAL_WIDTH: u32 = 80;
const TERMINAL_HEIGHT: u32 = 20;

#[derive(Component)]
struct PixelBuffer {
    handle: Handle<Image>
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            bevy_input::InputPlugin,
            bevy_asset::AssetPlugin::default(),
            bevy_render::texture::ImagePlugin::default_nearest(), // Prevent blurry sprites
            bevy_app::TerminalCtrlCHandlerPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (setup_camera, setup_scene))
        .add_systems(Update, print_terminal)
        .run();
}

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Create an image to draw on
    let image_handle = {
        let size = Extent3d {
            width: TERMINAL_WIDTH,
            height: TERMINAL_HEIGHT,
            ..default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::COPY_DST
                    | TextureUsages::COPY_SRC
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        image.resize(size);

        images.add(image)
    };

    // Create a camera that draws to the image
    commands.spawn((
        Camera2d::default(),
        Camera {
            target: RenderTarget::Image(image_handle.clone().into()),
            ..default()
        }
    ));

    // Track the image with an entity
    commands.spawn(PixelBuffer { handle: image_handle.clone() });
}

fn setup_scene(
    mut commands: Commands,
) {
    // Create the ground
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));

    // Create a falling block
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(13.0, 20.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0));
}

fn print_terminal(
    key: Res<ButtonInput<KeyCode>>,
    pixel_buffer: Single<&PixelBuffer>,
    images: Res<Assets<Image>>,
) {
    if key.just_pressed(KeyCode::Space) {
        let image = images.get(pixel_buffer.handle.id()).expect("Image for pixel buffer not found");
        for x in 0..TERMINAL_WIDTH {
            for y in 0..TERMINAL_HEIGHT {
                let pixel = image.get_color_at(x, y).expect("Pixel out of bounds");
                let rgba = pixel.to_srgba();
                println!("Pixel at ({}, {}): [{},{},{}]", x, y, rgba.red, rgba.green, rgba.blue);
            }
        }
    }
}
