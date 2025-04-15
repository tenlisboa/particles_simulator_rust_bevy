use bevy::prelude::*;
use bevy::{DefaultPlugins, app::App};

#[derive(States, Debug, Eq, PartialEq, Hash, Clone, Default)]
pub enum AppStates {
    #[default]
    Run,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppStates>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppStates::Run), game::setup_game)
        .add_systems(
            Update,
            (game::apply_gravity, game::check_for_collision).run_if(in_state(AppStates::Run)),
        )
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 1., 24.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}

mod game {
    use bevy::{color::palettes::css::SILVER, math::Direction3d, prelude::*};

    #[derive(Component)]
    pub struct Particle;

    #[derive(Component)]
    pub struct Collider;

    #[derive(Component, Deref, DerefMut)]
    pub struct Velocity(Vec3);

    const PARTICLE: Sphere = Sphere::new(0.5);
    const FLOOR_HEIGHT: f32 = 1.0;

    pub fn setup_game(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let particle = meshes.add(PARTICLE.mesh().uv(32, 18));
        let particle_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            ..Default::default()
        });

        commands.spawn((
            Collider,
            Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.).subdivisions(10))),
            MeshMaterial3d(materials.add(Color::from(SILVER))),
            Transform::from_xyz(0., 0., 0.),
        ));

        commands.spawn((
            Particle,
            Mesh3d(particle),
            MeshMaterial3d(particle_material),
            Transform::from_xyz(0., 16., 0.),
            Velocity(Vec3::new(0.3, 0., 0.1)),
        ));

        commands.spawn((
            PointLight {
                shadows_enabled: true,
                intensity: 10_000_000.,
                range: 100.,
                shadow_depth_bias: 0.2,
                ..Default::default()
            },
            Transform::from_xyz(8., 16., 8.),
        ));
    }

    const GRAVITY: f32 = -9.81;
    const BOUNCE_FACTOR: f32 = 0.7;

    pub fn apply_gravity(
        mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
        time: Res<Time>,
    ) {
        for (mut particle_transform, mut particle_velocity) in &mut particles {
            particle_velocity.0.y += GRAVITY * time.delta_secs();

            particle_transform.translation += particle_velocity.0 * time.delta_secs();
        }
    }

    pub fn check_for_collision(
        mut particles: Query<(&mut Transform, &mut Velocity), With<Particle>>,
    ) {
        for (mut particle_transform, mut particle_velocity) in &mut particles {
            let floor_offset = FLOOR_HEIGHT / 2.;

            if particle_transform.translation.y > floor_offset {
                continue;
            }

            particle_transform.translation = particle_transform.translation.clone();
            particle_transform.translation.y = floor_offset;
            particle_velocity.0.y = -particle_velocity.0.y * BOUNCE_FACTOR;
        }
    }
}
