mod deluxe;
use crate::deluxe::{Color, GameManager, GameObject, FPS_2};

const WIDTH: i16 = 80;
const HEIGHT: i16 = 20;
const DEFAULT_FORGROUND_COLOR: Color = Color::Rgb {
    r: 160,
    g: 220,
    b: 158,
};
const DEFAULT_BACKGROUND_COLOR: Color = Color::Rgb { r: 0, g: 0, b: 0 };

fn main() {
    let mut game_manager = GameManager::new("Hotel Deluxe 2".to_string(), WIDTH, HEIGHT);

    let title = GameObject {
        x: 0,
        y: 0,
        z: 0,
        id: 0,
        solid: true,
        driveable: false,
        name: "title".to_string(),
        color: DEFAULT_FORGROUND_COLOR,
        background_color: DEFAULT_BACKGROUND_COLOR,
        sigil_index: 0,
        sigils: vec!["  Hotel Deluxe 2  ".to_string()],
        visible: true,
    };

    let nick = GameObject {
        x: 1,
        y: 1,
        z: 0,
        id: 1,
        solid: true,
        driveable: true,
        name: "nick".to_string(),
        color: Color::Rgb {
            r: 255,
            g: 0,
            b: 100,
        },
        background_color: DEFAULT_BACKGROUND_COLOR,
        sigil_index: 0,
        sigils: vec![
            "^^^\n^^^".to_string(),
            "vvv\nvvv".to_string(),
            "<<<\n<<<".to_string(),
            ">>>\n>>>".to_string(),
        ],
        visible: true,
    };

    let chicken = GameObject {
        x: 10,
        y: 10,
        z: 0,
        id: 2,
        solid: true,
        driveable: false,
        name: "chicken".to_string(),
        color: Color::Rgb { r: 255, g: 0, b: 0 },
        background_color: Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
        sigil_index: 0,
        sigils: vec!['>'.to_string()],
        visible: true,
    };

    let game_objects = vec![title, nick, chicken];

    game_manager.game_objects = game_objects;
    game_manager.frame_rate = FPS_2;
    game_manager.run();
    let mut count = 0;
    while game_manager.is_running {
        count += 1;
        game_manager.game_objects[1].sigils[0] = format!(">{}<", count);

        game_manager.render();
        // deluxe::process_input(&mut game_manager);
        // deluxe::update(&mut game_manager);
        // deluxe::sleep(Duration::from_millis(50));
    }
    game_manager.stop();
    println!("Thanks for playing!");
}
