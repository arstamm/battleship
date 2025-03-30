use std::collections::HashMap;

use crate::gameplay::info::Player;
use crate::gameplay::constants::{CELL_SIZE, GRID_SIZE, SHIP_SIZES};
use crate::gameplay::gameplay::{CellState, GameState};
use crate::gameplay::ships::{place_ship, hover_ship};
use crate::gameplay::info::Banner;
use crate::gameplay::constants;
use crate::gameplay::constants::{X_DELTA, Y_DELTA, X_DELTA_ENEMY};
use crate::gameplay::gameplay::BattleshipGame;

pub fn toggle_direction(player: &mut Player) {
    player.current_direction = (player.current_direction + 1) % 4; // Cycle through 0-3
}

pub fn place_ships(x: f32, y: f32, p: &mut Player) {
    let col = ((x - p.x_pos) / CELL_SIZE) as usize;
    let row = ((y - p.y_pos) / CELL_SIZE) as usize;

    // println!("Click: x={}, y={}", x, y);
    // println!("Grid Origin: x={}, y={}", p.x_pos, p.y_pos);
    // println!("Calculated: col={}, row={}", col, row);

        // Reset Image
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if p.grid[r][c] == CellState::Hover {
                    p.grid[r][c] = CellState::Empty;
                }
            }
        }

    if row < GRID_SIZE && col < GRID_SIZE && x - p.x_pos >= 0.0 && y - p.y_pos >= 0.0 {
        if p.placing_ships {
            if p.current_ship_index < SHIP_SIZES.len() {
                if place_ship(&mut p.grid, row, col, p.current_direction, SHIP_SIZES[p.current_ship_index], CellState::Ship, &mut p.sprite_map) {
                    p.current_ship_index += 1;
                    if p.current_ship_index >= SHIP_SIZES.len() {
                        p.placing_ships = false;
                    }
                }
            }
        }
    }
}

pub fn hover_ships(x: f32, y: f32, p: &mut Player) {
    let col = ((x - p.x_pos) / CELL_SIZE) as usize;
    let row = ((y - p.y_pos) / CELL_SIZE) as usize;

    // println!("Click: x={}, y={}", x, y);
    // println!("Grid Origin: x={}, y={}", p.x_pos, p.y_pos);
    // println!("Calculated: col={}, row={}", col, row);
    if p.placing_ships {

        // Reset Image
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if p.grid[r][c] == CellState::Hover {
                    p.grid[r][c] = CellState::Empty;
                }
            }
        }

        // Create Hover Effect
        if row < GRID_SIZE && col < GRID_SIZE && x - p.x_pos >= 0.0 && y - p.y_pos >= 0.0 {
            if p.current_ship_index < SHIP_SIZES.len() {
                hover_ship(&mut p.grid, row, col, p.current_direction, SHIP_SIZES[p.current_ship_index], CellState::Hover);
            }
        }
    }
}

pub fn check_guess(x: f32, y: f32, p: &mut Player, button: &mut Banner) {
    let col = ((x - p.x_pos) / CELL_SIZE) as usize;
    let row = ((y - p.y_pos) / CELL_SIZE) as usize;

    if row < GRID_SIZE && col < GRID_SIZE && x - p.x_pos >= 0.0 && y - p.y_pos >= 0.0 {
        if p.num_turns > 0 {
            if p.grid[row][col] == CellState::Ship {
                p.grid[row][col] = CellState::Hit;
                p.hits += 1;
                p.num_turns -= 1;
            } else if p.grid[row][col] == CellState::Empty {
                p.grid[row][col] = CellState::Miss;
                p.num_turns -= 1;
                
            }
            let num_str = p.num_turns.to_string();
            button.label = format!("TRIES\nLEFT\n- {num_str} -");
            if p.num_turns <= 0 {
                p.end_flag = true;
            }
        }
    }
}

pub fn click_action(x: f32, y: f32, battleship_game: &mut BattleshipGame) {
    if x >= battleship_game.button.x && y >= battleship_game.button.y && x <= battleship_game.button.x + battleship_game.button.w && y <= battleship_game.button.y + battleship_game.button.h {
        println!("The button was pressed!");

        if battleship_game.game_state == GameState::Begin {
            println!("Begin Game!");
            battleship_game.game_state = GameState::ButtonPressedBegin;
        }

        if battleship_game.game_state == GameState::PlayerPlaceShips {
            if battleship_game.player.placing_ships == false {
                println!("Player Done Placing Ships");
                battleship_game.game_state = GameState::ButtonPressedPlayerPlaceShips;
            } 
        }

        if battleship_game.game_state == GameState::EnemyPlaceShips {
            if battleship_game.enemy.placing_ships == false {
                println!("Enemy Done Placing Ships");
                battleship_game.game_state = GameState::ButtonPressedEnemyPlaceShips;
            } 
        }

        if battleship_game.game_state == GameState::Win {
            println!("Playing again");
            battleship_game.game_state = GameState::SignalBegin;
        }
        // if player.placing_ships == false {
        //     println!("(Ships Hidden)");
        //     player.ship_color = player.grid_cell_color;
        //     player.turn = true;

        // }
    }
}

pub fn check_state(battleship_game: &mut BattleshipGame) {

    if battleship_game.game_state == GameState::ButtonPressedBegin {
        println!("placing ships for player");
        battleship_game.player.placing_ships = true;
        battleship_game.button.label = "CLICK\nWHEN\nDONE".to_string();
        battleship_game.text_display_box.label = "Player Place Ships (hit SPACEBAR to rotate)".to_string();
        battleship_game.game_state = GameState::PlayerPlaceShips;
    }

    if battleship_game.game_state == GameState::ButtonPressedPlayerPlaceShips {
        battleship_game.player.ship_color = battleship_game.player.grid_cell_color;
        println!("placing ships for enemy");
        battleship_game.enemy.placing_ships = true;
        battleship_game.button.label = "CLICK\nWHEN\nDONE".to_string();
        battleship_game.text_display_box.label = "Enemy Place Ships (hit SPACEBAR to rotate)".to_string();
        battleship_game.game_state = GameState::EnemyPlaceShips;
    }

    if battleship_game.game_state == GameState::ButtonPressedEnemyPlaceShips {
        battleship_game.enemy.ship_color = battleship_game.enemy.grid_cell_color;
        println!("Begin turns");
        battleship_game.enemy.turn = true;
        battleship_game.game_state = GameState::Turns
    }

    if battleship_game.player.hits >= 17 {
        battleship_game.player.hits = 0;
        battleship_game.enemy.hits = 0;
        battleship_game.enemy.ship_color = constants::SHIP_COLOR;
        battleship_game.player.grid_cell_color = constants::HOVER_COLOR;
        battleship_game.text_display_box.label = "***** ENEMY WINS !!!!! *****".to_string();
        battleship_game.game_state = GameState::SignalWin;
    }

    if battleship_game.enemy.hits >= 17 {
        battleship_game.player.hits = 0;
        battleship_game.enemy.hits = 0;
        battleship_game.player.ship_color = constants::SHIP_COLOR;
        battleship_game.enemy.grid_cell_color = constants::HOVER_COLOR;
        battleship_game.text_display_box.label = "***** PLAYER WINS !!!!! *****".to_string();
        battleship_game.game_state = GameState::SignalWin;
    }

    if battleship_game.game_state == GameState::SignalWin {
        battleship_game.button.label = "PLAY\nAGAIN?".to_string();
        battleship_game.player.num_turns = 0;
        battleship_game.enemy.num_turns = 0;
        battleship_game.game_state = GameState::Win;
    }

    if battleship_game.game_state == GameState::SignalBegin {

        battleship_game.player = Player {
            x_pos: X_DELTA,
            y_pos: Y_DELTA,
            grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            sprite_map: HashMap::new(),
            turn: false,
            placing_ships: false,
            start_flag: false,
            end_flag: false,           
            current_ship_index: 0,
            current_direction: 1,
            hits: 0,
            num_turns: 0,
            ship_color: constants::SHIP_COLOR,
            grid_cell_color: constants::GRID_CELL_COLOR
        };

        battleship_game.enemy = Player {
            x_pos: X_DELTA_ENEMY,
            y_pos: Y_DELTA,
            grid: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            sprite_map: HashMap::new(),
            turn: false,
            placing_ships: false,
            start_flag: false,
            end_flag: false,
            current_ship_index: 0,
            current_direction: 1,
            hits: 0,
            num_turns: 0,
            ship_color: constants::SHIP_COLOR,
            grid_cell_color: constants::GRID_CELL_COLOR
        };

        battleship_game.text_display_box.label = "WELCOME TO BATTLESHIP".to_string();
        battleship_game.button.label = "CLICK\nTO\nBEGIN".to_string();

        battleship_game.game_state = GameState::Begin;
    }

}

pub fn check_turn(p: &mut Player, enemy: &mut Player, msg_box: &mut Banner, msg: &str, button: &mut Banner) {
    if p.turn && p.start_flag && p.end_flag {
        p.turn = false;
        p.start_flag = false;
        enemy.turn = true;
    }
    if p.turn && !p.start_flag {
        println!("I only ran once");
        msg_box.label = msg.to_string();
        p.num_turns = 5;
        p.start_flag = true;
        p.end_flag = false;
        let num_str = p.num_turns.to_string();
        button.label = format!("TRIES\nLEFT\n- {num_str} -");
    }
    
}