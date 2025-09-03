use std::time::Duration;
use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::winit::WinitPlugin;
use bevy_rapier2d::prelude::*;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};
use bevy_ratatui_camera::{RatatuiCamera, RatatuiCameraPlugin, RatatuiCameraStrategy, RatatuiCameraWidget};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Alignment};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Widget};
use tui_logger::TuiLoggerWidget;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build()
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>(),
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1. / 60.)),
        ))
        .add_plugins((
            // set up the Ratatui context and forward input events
            RatatuiPlugins::default(),
            // connect a bevy camera target to a ratatui widget
            RatatuiCameraPlugin
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (setup_camera, setup_scene))
        .add_systems(Update, print_scene)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Flags>()
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        RatatuiCamera::default(),
        RatatuiCameraStrategy::luminance_braille(),
        Camera2d,
    ));
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create the ground
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));

    // Create a falling block
    commands
        .spawn(Sprite {
            image: asset_server.load("slate-block.png"),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(13.0, 20.0))
        .insert(Restitution::coefficient(0.2))
        .insert(Transform::from_xyz(200.0, 800.0, 0.0));
}

fn print_scene(
    mut ratatui: ResMut<RatatuiContext>,
    mut camera_widget: Single<&mut RatatuiCameraWidget>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
) -> Result {
    ratatui.draw(|frame| {
        let area = debug_frame(frame, &flags, &diagnostics);

        camera_widget.render(area, frame.buffer_mut());
    })?;

    Ok(())
}

#[allow(dead_code)]
#[derive(Resource, Default)]
pub struct Flags {
    pub debug: bool,
}

#[allow(dead_code)]
pub fn debug_frame(
    frame: &mut Frame,
    flags: &Flags,
    diagnostics: &DiagnosticsStore,
) -> ratatui::layout::Rect {
    let mut block = Block::bordered()
        .bg(ratatui::style::Color::Rgb(0, 0, 0))
        .border_style(Style::default().bg(ratatui::style::Color::Black))
        .title_bottom("[q for quit]")
        .title_bottom("[d for debug]")
        .title_alignment(Alignment::Center);

    if flags.debug {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(66), Constraint::Fill(1)],
        )
        .split(frame.area());

        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            block = block.title_top(format!("[fps: {value:.0}]"));
        }

        let inner = block.inner(layout[0]);
        frame.render_widget(block, layout[0]);
        frame.render_widget(
            TuiLoggerWidget::default()
                .block(Block::bordered())
                .style(Style::default().bg(ratatui::style::Color::Reset)),
            layout[1],
        );

        inner
    } else {
        let inner = block.inner(frame.area());
        frame.render_widget(block, frame.area());

        inner
    }
}
