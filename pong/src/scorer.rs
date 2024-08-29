use bevy::prelude::*;

use crate::{ball::Ball, Position, Velocity};

enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
struct Scored(Scorer);

#[derive(Resource, Default)]
struct Score {
    player: u32,
    ai: u32,
}

#[derive(Component)]
struct PlayerScoreboard;

#[derive(Component)]
struct AiScoreboard;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_event::<Scored>();
        app.add_systems(Startup, Self::spawn_scoreboard);
        app.add_systems(
            Update,
            (
                Self::detect_scoring,
                Self::reset_ball.after(Self::detect_scoring),
                (Self::update_score, Self::update_scoreboard)
                    .after(Self::detect_scoring)
                    .chain(),
            ),
        );
    }
}

impl ScorePlugin {
    fn spawn_scoreboard(mut commands: Commands) {
        commands.spawn((
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 72.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            }),
            PlayerScoreboard,
        ));

        commands.spawn((
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 72.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..Default::default()
            }),
            AiScoreboard,
        ));
    }

    fn update_scoreboard(
        mut player_score: Query<&mut Text, (With<PlayerScoreboard>, Without<AiScoreboard>)>,
        mut ai_score: Query<&mut Text, (With<AiScoreboard>, Without<PlayerScoreboard>)>,
        score: Res<Score>,
    ) {
        if score.is_changed() {
            if let Ok(mut player_score) = player_score.get_single_mut() {
                player_score.sections[0].value = score.player.to_string();
            }

            if let Ok(mut ai_score) = ai_score.get_single_mut() {
                ai_score.sections[0].value = score.ai.to_string();
            }
        }
    }

    fn detect_scoring(
        ball: Query<&Position, With<Ball>>,
        window: Query<&Window>,
        mut events: EventWriter<Scored>,
    ) {
        if let (Ok(window), Ok(ball)) = (window.get_single(), ball.get_single()) {
            let window_width = window.resolution.width();
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::Player));
            }
        }
    }

    fn reset_ball(
        mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
        mut events: EventReader<Scored>,
    ) {
        for event in events.read() {
            if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
                position.0 = Vec2::ZERO;
                match event.0 {
                    Scorer::Ai => velocity.0 = Vec2::new(-1., 2.),
                    Scorer::Player => velocity.0 = Vec2::new(1., 2.),
                }
            }
        }
    }

    fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
        for event in events.read() {
            match event.0 {
                Scorer::Ai => score.ai += 1,
                Scorer::Player => score.player += 1,
            }

            debug!("Score: player {} - ai {}", score.player, score.ai);
        }
    }
}
