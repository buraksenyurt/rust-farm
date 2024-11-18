use bevy::prelude::*;

#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Resource)]
struct Timer(f32);

fn setup_level_alpha(mut query: Query<(Entity, &mut Position)>) {
    println!("setup level alpha");
    for (entity, mut position) in query.iter_mut() {
        position.x = 1.0; //rand
        position.y = 1.0;
        println!("{:?}", position);
    }
}

fn log_player(query: Query<&Position, With<Player>>) {
    println!("Player log position");
    for position in query.iter() {
        println!("{:?}", position);
    }
}

fn log_enemies(query: Query<(&Position, &Velocity), Without<Player>>) {
    println!("Log enemies");
    for (position, _) in query.iter() {
        println!("Enemy go to position {:?}", position);
    }
}

fn move_all_enemies(
    mut query: Query<(&mut Position, &Velocity), Without<Player>>,
    timer: Res<Timer>,
) {
    println!("Moving all enemies");
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x * timer.0;
        position.y += velocity.y * timer.0;
    }
}

pub fn run() {
    let mut world = World::new();

    let mut aragon = world.spawn_empty();
    aragon.insert((
        Position { x: 10.0, y: 10.0 },
        Velocity { x: 1.5, y: 0.0 },
        Player,
    ));

    let mut legolas = world.spawn_empty();
    legolas.insert((
        Position { x: 15.0, y: 10.0 },
        Velocity { x: 2.0, y: 0.0 },
        Player,
    ));

    let mut orc_warrior = world.spawn_empty();
    orc_warrior.insert((Position { x: 100.0, y: 10.0 }, Velocity { x: 0.5, y: 0.0 }));

    world.insert_resource(Timer(0.2));

    let mut schedule = Schedule::default();
    schedule.add_systems((
        (setup_level_alpha, log_player, log_enemies).chain(),
        move_all_enemies
            .after(setup_level_alpha)
            .before(log_player)
            .before(log_enemies),
    ));
    schedule.run(&mut world);
}
