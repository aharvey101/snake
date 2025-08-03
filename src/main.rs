use bevy::prelude::*;
use rand::Rng;
use std::collections::VecDeque;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 15;
const CELL_SIZE: f32 = 30.0;
const WINDOW_WIDTH: f32 = GRID_WIDTH as f32 * CELL_SIZE;
const WINDOW_HEIGHT: f32 = GRID_HEIGHT as f32 * CELL_SIZE;

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverText;

#[derive(Resource)]
struct SnakeState {
    body: VecDeque<Vec2>,
    direction: Vec2,
    growing: bool,
}

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(SnakeState {
            body: VecDeque::from([Vec2::new(5.0, 5.0)]),
            direction: Vec2::new(1.0, 0.0),
            growing: false,
        })
        .insert_resource(GameTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_input,
                move_snake,
                check_food_collision,
                check_wall_collision,
                check_self_collision,
                spawn_food,
                update_snake_visuals,
                update_score,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (restart_game, show_game_over).run_if(in_state(GameState::GameOver)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    // Spawn score text
    commands.spawn((
        TextBundle::from_section(
            "Score: 1",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
    
    // Spawn initial snake head
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(
                (5.0 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0,
                (5.0 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0,
                0.0,
            ),
            ..default()
        },
        SnakeHead,
    ));
    
    // Spawn initial food
    spawn_food_at_random_position(&mut commands);
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_state: ResMut<SnakeState>,
) {
    let new_direction = if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        Vec2::new(0.0, 1.0)
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS) {
        Vec2::new(0.0, -1.0)
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA) {
        Vec2::new(-1.0, 0.0)
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) || keyboard_input.just_pressed(KeyCode::KeyD) {
        Vec2::new(1.0, 0.0)
    } else {
        return;
    };

    // Prevent the snake from going in the opposite direction
    if new_direction != -snake_state.direction {
        snake_state.direction = new_direction;
    }
}

fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut snake_state: ResMut<SnakeState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let head_pos = snake_state.body.front().unwrap().clone();
        let new_head_pos = head_pos + snake_state.direction;
        
        snake_state.body.push_front(new_head_pos);
        
        if !snake_state.growing {
            snake_state.body.pop_back();
        } else {
            snake_state.growing = false;
        }
    }
}

fn check_food_collision(
    mut commands: Commands,
    mut snake_state: ResMut<SnakeState>,
    food_query: Query<(Entity, &Transform), With<Food>>,
) {
    if let Some(head_pos) = snake_state.body.front() {
        for (food_entity, food_transform) in food_query.iter() {
            let food_grid_pos = world_to_grid(food_transform.translation.truncate());
            
            if *head_pos == food_grid_pos {
                commands.entity(food_entity).despawn();
                snake_state.growing = true;
                spawn_food_at_random_position(&mut commands);
                break;
            }
        }
    }
}

fn check_wall_collision(
    snake_state: Res<SnakeState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(head_pos) = snake_state.body.front() {
        if head_pos.x < 0.0 || head_pos.x >= GRID_WIDTH as f32 ||
           head_pos.y < 0.0 || head_pos.y >= GRID_HEIGHT as f32 {
            next_state.set(GameState::GameOver);
        }
    }
}

fn check_self_collision(
    snake_state: Res<SnakeState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(head_pos) = snake_state.body.front() {
        for (i, segment_pos) in snake_state.body.iter().enumerate() {
            if i > 0 && *head_pos == *segment_pos {
                next_state.set(GameState::GameOver);
                break;
            }
        }
    }
}

fn spawn_food(
    mut commands: Commands,
    food_query: Query<&Food>,
) {
    if food_query.is_empty() {
        spawn_food_at_random_position(&mut commands);
    }
}

fn spawn_food_at_random_position(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..GRID_WIDTH) as f32;
    let y = rng.gen_range(0..GRID_HEIGHT) as f32;
    
    let world_pos = grid_to_world(Vec2::new(x, y));
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(CELL_SIZE - 4.0, CELL_SIZE - 4.0)),
                ..default()
            },
            transform: Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
            ..default()
        },
        Food,
    ));
}

fn update_snake_visuals(
    mut commands: Commands,
    snake_state: Res<SnakeState>,
    snake_query: Query<Entity, Or<(With<SnakeHead>, With<SnakeSegment>)>>,
) {
    // Remove all existing snake entities
    for entity in snake_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Spawn new snake entities
    for (i, pos) in snake_state.body.iter().enumerate() {
        let world_pos = grid_to_world(*pos);
        let color = if i == 0 {
            Color::srgb(0.0, 1.0, 0.0) // Head - bright green
        } else {
            Color::srgb(0.0, 0.8, 0.0) // Body - darker green
        };
        
        if i == 0 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                    ..default()
                },
                SnakeHead,
            ));
        } else {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                    ..default()
                },
                SnakeSegment,
            ));
        }
    }
}

fn restart_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut snake_state: ResMut<SnakeState>,
    mut next_state: ResMut<NextState<GameState>>,
    all_entities: Query<Entity, Or<(With<SnakeHead>, With<SnakeSegment>, With<Food>, With<GameOverText>)>>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Clear all game entities
        for entity in all_entities.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset snake state
        snake_state.body = VecDeque::from([Vec2::new(5.0, 5.0)]);
        snake_state.direction = Vec2::new(1.0, 0.0);
        snake_state.growing = false;
        
        // Reset score display
        if let Ok(mut text) = score_text_query.get_single_mut() {
            text.sections[0].value = "Score: 1".to_string();
        }
        
        // Spawn new game entities
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    (5.0 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0,
                    (5.0 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0,
                    0.0,
                ),
                ..default()
            },
            SnakeHead,
        ));
        
        spawn_food_at_random_position(&mut commands);
        next_state.set(GameState::Playing);
    }
}

fn update_score(
    snake_state: Res<SnakeState>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(mut text) = score_text_query.get_single_mut() {
        text.sections[0].value = format!("Score: {}", snake_state.body.len());
    }
}

fn show_game_over(
    mut commands: Commands,
    game_over_query: Query<&GameOverText>,
) {
    // Only spawn game over text if it doesn't already exist
    if game_over_query.is_empty() {
        commands.spawn((
            TextBundle::from_section(
                "GAME OVER\nPress SPACE to restart",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(1.0, 0.0, 0.0),
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(WINDOW_HEIGHT / 2.0 - 40.0),
                left: Val::Px(WINDOW_WIDTH / 2.0 - 150.0),
                ..default()
            }),
            GameOverText,
        ));
    }
}

fn grid_to_world(grid_pos: Vec2) -> Vec2 {
    Vec2::new(
        (grid_pos.x - GRID_WIDTH as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0,
        (grid_pos.y - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0,
    )
}

fn world_to_grid(world_pos: Vec2) -> Vec2 {
    Vec2::new(
        (world_pos.x - CELL_SIZE / 2.0) / CELL_SIZE + GRID_WIDTH as f32 / 2.0,
        (world_pos.y - CELL_SIZE / 2.0) / CELL_SIZE + GRID_HEIGHT as f32 / 2.0,
    )
}
