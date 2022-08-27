use super::game::Level;
use super::game_spawn::SpawnInstruction;
use bevy::prelude::*;
use shapeshifter_level_maker::util::SpawnLevel;

pub struct GameLevels {
    pub simplicity: Vec<SpawnLevel>,
    pub convexity: Vec<SpawnLevel>,
    pub perplexity: Vec<SpawnLevel>,
    pub complexity: Vec<SpawnLevel>,
}

impl GameLevels {
    pub fn get(&self, level: &Level) -> SpawnLevel {
        match level {
            Level::Simplicity(idx) => self.simplicity[*idx].clone(),
            Level::Convexity(idx) => self.convexity[*idx].clone(),
            Level::Perplexity(idx) => self.perplexity[*idx].clone(),
            Level::Complexity(idx) => self.complexity[*idx].clone(),
        }
    }
}

pub fn send_tutorial_text(
    simplicity_level: usize,
    spawn_instruction_event_writer: &mut EventWriter<SpawnInstruction>,
) {
    let text = match simplicity_level {
        0 => "The goal is the fit the polygon inside of the target area",
        1 => "Rotate the polygon using either the right mouse button or the scroll wheel",
        2 => "Perform a cut by holding either the Ctrl key or the C key, and then using the mouse",
        3 => "The number of remaining cuts for the level is shown in the top left corner",
        4 => "There is a \"restart level\" option in the pause menu accessible via the escape or M key",
        5 => "Your are on your own now! Good luck!",
        _ => "",
    };

    if text != "".to_owned() {
        spawn_instruction_event_writer.send(SpawnInstruction {
            text: text.to_string(),
        });
    }
}

// 004_simplicity_square_cut

impl Default for GameLevels {
    fn default() -> Self {
        let simplicity = vec![
            //
            // 0
            SpawnLevel::new3("002_simplicity_square", "002_simplicity_square", 0),
            //
            // 1
            SpawnLevel::new3("002_simplicity_square", "003_simplicity_square_oblique", 3),
            //
            // 2
            SpawnLevel::new4("002_simplicity_square", "004_simplicity_square_cut", 4, 1.2),
            //
            // 3
            SpawnLevel::new4("002_simplicity_square", "tree1", 3, 1.25),
            //
            // 4
            SpawnLevel::new4(
                "002_simplicity_square",
                "004_simplicity_square_parallel",
                4,
                1.15,
            ),
            //
            // 5
            SpawnLevel::new4("002_simplicity_square", "octogone", 3, 1.5),
        ];
        let convexity = vec![
            //
            // 6
            SpawnLevel::new4("eggplant", "tree1", 3, 1.3),
            //
            // 7
            SpawnLevel::new4("crab1", "whale1", 3, 1.3),
            //
            // 8
            SpawnLevel::new4("seal1", "pear", 3, 1.3),
        ];
        let perplexity = Vec::new();
        let complexity = Vec::new();

        GameLevels {
            simplicity,
            convexity,
            perplexity,
            complexity,
        }
    }
}
