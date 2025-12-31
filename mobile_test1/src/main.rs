use dioxus::prelude::*;
use tokio::time::{sleep, Duration};
use rand::Rng;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, PartialEq)]
struct Enemy {
    x: f64,
    y: f64,
    id: usize,
}

#[derive(Clone, PartialEq)]
struct Bullet {
    x: f64,
    y: f64,
    id: usize,
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        style { {include_str!("../assets/game.css")} }
        SpaceGame {}
    }
}

#[component]
fn SpaceGame() -> Element {
    let mut player_x = use_signal(|| 50.0);
    let mut score = use_signal(|| 0);
    let mut game_over = use_signal(|| false);
    let mut enemies = use_signal(|| Vec::<Enemy>::new());
    let mut bullets = use_signal(|| Vec::<Bullet>::new());
    let mut next_enemy_id = use_signal(|| 0);
    let mut next_bullet_id = use_signal(|| 0);
    let mut restart_counter = use_signal(|| 0);

    // Game loop - spawn enemies
    use_effect(use_reactive!(|(restart_counter,)| {
        let mut next_enemy_id = next_enemy_id.clone();
        let mut enemies = enemies.clone();
        let game_over = game_over.clone();

        spawn(async move {
            loop {
                sleep(Duration::from_millis(2000)).await;

                if game_over() {
                    break;
                }

                let id = next_enemy_id();
                next_enemy_id.set(id + 1);
                enemies.write().push(Enemy {
                    x: rand::thread_rng().gen_range(10.0..90.0),
                    y: 0.0,
                    id,
                });
            }
        });
    }));

    // Game loop - move enemies
    use_effect(use_reactive!(|(restart_counter,)| {
        let mut game_over = game_over.clone();
        let mut enemies = enemies.clone();

        spawn(async move {
            loop {
                sleep(Duration::from_millis(50)).await;

                if game_over() {
                    break;
                }

                let mut should_end = false;
                enemies.write().iter_mut().for_each(|enemy| {
                    enemy.y += 0.5;
                    if enemy.y > 100.0 {
                        should_end = true;
                    }
                });

                if should_end {
                    game_over.set(true);
                    break;
                }
            }
        });
    }));

    // Game loop - move bullets and check collisions
    use_effect(use_reactive!(|(restart_counter,)| {
        let game_over = game_over.clone();
        let mut bullets = bullets.clone();
        let mut enemies = enemies.clone();
        let mut score = score.clone();

        spawn(async move {
            loop {
                sleep(Duration::from_millis(50)).await;

                if game_over() {
                    break;
                }

                let mut bullets_vec = bullets.write();
                let mut enemies_vec = enemies.write();

                // Move bullets
                bullets_vec.iter_mut().for_each(|bullet| bullet.y -= 2.0);
                bullets_vec.retain(|bullet| bullet.y > 0.0);

                // Check collisions
                let mut hit_bullets = Vec::new();
                let mut hit_enemies = Vec::new();

                for bullet in bullets_vec.iter() {
                    for enemy in enemies_vec.iter() {
                        let dx = bullet.x - enemy.x;
                        let dy = bullet.y - enemy.y;
                        if dx * dx + dy * dy < 25.0 {
                            hit_bullets.push(bullet.id);
                            hit_enemies.push(enemy.id);
                        }
                    }
                }

                let hits = hit_bullets.len();
                bullets_vec.retain(|b| !hit_bullets.contains(&b.id));
                enemies_vec.retain(|e| !hit_enemies.contains(&e.id));

                if hits > 0 {
                    score.set(score() + (hits * 10));
                }
            }
        });
    }));

    let shoot = move |_| {
        if !game_over() {
            let id = next_bullet_id();
            next_bullet_id += 1;
            bullets.write().push(Bullet {
                x: player_x(),
                y: 90.0,
                id,
            });
        }
    };

    let move_left = move |_| {
        if player_x() > 5.0 {
            player_x -= 5.0;
        }
    };

    let move_right = move |_| {
        if player_x() < 95.0 {
            player_x += 5.0;
        }
    };

    let restart_game = move |_| {
        // Increment counter first to kill old loops and trigger new effects
        restart_counter += 1;
        // Then reset all game state
        game_over.set(false);
        score.set(0);
        enemies.set(Vec::new());
        bullets.set(Vec::new());
        player_x.set(50.0);
        next_enemy_id.set(0);
        next_bullet_id.set(0);
    };

    rsx! {
        div { class: "game-container",

            div { class: "score-board",
                div { style: "display: flex; align-items: center; gap: 20px;",
                    span { "Score: {score}" }
                    button {
                        onclick: restart_game,
                        style: "background: #4a5568; border: 1px solid #fff; border-radius: 6px; color: #fff; padding: 8px 16px; cursor: pointer; font-size: 14px;",
                        "Restart Game"
                    }
                }
            }

            if game_over() {
                div { class: "game-over",
                    h1 { "GAME OVER!" }
                    p { "Final Score: {score}" }
                    button { onclick: restart_game, "Restart" }
                }
            }

            div { class: "game-area",
                // Player spaceship
                div { class: "player", style: "left: {player_x}%;", "🚀" }

                // Enemies
                for enemy in enemies.read().iter() {
                    div {
                        key: "{enemy.id}",
                        class: "enemy",
                        style: "left: {enemy.x}%; top: {enemy.y}%;",
                        "👾"
                    }
                }

                // Bullets
                for bullet in bullets.read().iter() {
                    div {
                        key: "{bullet.id}",
                        class: "bullet",
                        style: "left: {bullet.x}%; top: {bullet.y}%;",
                        "•"
                    }
                }
            }

            div { class: "controls",
                button { class: "control-btn", onclick: move_left, "◀" }
                button { class: "control-btn shoot-btn", onclick: shoot, "FIRE" }
                button { class: "control-btn", onclick: move_right, "▶" }
            }
        }
    }
}