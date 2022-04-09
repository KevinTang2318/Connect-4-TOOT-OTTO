use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use yew::prelude::*;
use yew::Properties;
use yew::Callback;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, HtmlElement, Request, RequestInit, RequestMode, Response, Blob
};
use stdweb::js;
use stdweb::unstable::TryInto;
use stdweb::web::Date;



// use super::connect_4_side::Difficulty;
use super::game_history_side::Game;

pub struct TOOT_OTTO_Canvas {
    props: GameProperty,
    canvas_id: String,
    canvas: NodeRef,
    board: Vec<Vec<i64>>,
    letter_board: Vec<Vec<char>>,
    letter: String,
    on_click_cb: Callback<MouseEvent>,
    animation_cb: Closure<dyn FnMut()>,
    post_cb: Callback<i32>,
    plate_position: PlatePosition,
    current_move: i64,
    won: bool,
    // paused: bool,
    reject_click: bool,
    game_result: Game,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameProperty {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    pub game_done_cbk: Callback<i64>,
    pub letter: String,
}

// This struct is used to store the plate destination and current location
// Used for creating animation
#[derive(Clone, PartialEq)]
pub struct PlatePosition {
    pub row: usize,
    pub col: usize,
    pub current_pos: usize,
    mode: bool,
    letter: char,
}

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

pub enum Msg {
    Click(MouseEvent),
    AnimateMsg, 
    SendPost,
    Ignore
    // Render
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

impl TOOT_OTTO_Canvas {
    //-------------------------------------------------------------- Game Algorithm -------------------------------------------------------------------- 
    //Implementation of checkState function in TootOttoComputer.js
    // TODO: sign problem
    fn check_state(&self, state: &Vec<Vec<i64>>) -> (i64, i64) {
        let mut win_val = 0;
        let mut chain_val = 0;
        let mut temp_r = 0;
        let mut temp_b = 0;
        let mut temp_br = 0;
        let mut temp_tr = 0;

        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {

                    let sign: i64 = if k == 0 || k == 3 { -1 } else { 1 };

                    if j + k < 7 {
                        temp_r += sign * state[i][j + k];
                    }

                    if i + k < 6 {
                        temp_b += sign * state[i + k][j];
                    }

                    if i + k < 6 && j + k < 7 {
                        temp_br += sign * state[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_tr += sign * state[i - k][j + k];
                    }
                }
                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;

                if temp_r.abs() == 4 {
                    win_val = temp_r;
                } 
                else if temp_b.abs() == 4 {
                    win_val = temp_b;
                } 
                else if temp_br.abs() == 4 {
                    win_val = temp_br;
                } 
                else if temp_tr.abs() == 4 {
                    win_val = temp_tr;
                }
            }
        }

        return (win_val, chain_val);
    }


    //Implementation of playerMove function in TootOttoComputer.js
    fn player_move(&self) -> i64 {
        match self.letter.as_str() {
            "T" => 1,
            "O" => -1,
            _ => 0, 
        }
    }

    //Implementation of check function in TootOttoComputer.js
    fn check(&mut self) {

        let mut temp_r1 = Vec::new();
        let mut temp_b1 = Vec::new();
        let mut temp_br1 = Vec::new();
        let mut temp_br2 = Vec::new();

        for i in 0..6 {
            for j in 0..7 {
                temp_r1 = vec!['a'; 4];
                temp_b1 = vec!['a'; 4];
                temp_br1 = vec!['a'; 4];
                temp_br2 = vec!['a'; 4];

                for k in 0..=3 {
                    if j + k < 7 {
                        temp_r1[k] = self.letter_board[i][j + k];
                    }

                    if i + k < 6 {
                        temp_b1[k] = self.letter_board[i + k][j];
                    }

                    if i + k < 6 && j + k < 7 {
                        temp_br1[k] = self.letter_board[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_br2[k] = self.letter_board[i - k][j + k];
                    }
                }


                if temp_r1[0] == 'T' && temp_r1[1] == 'O' && temp_r1[2] == 'O' && temp_r1[3] == 'T'{
                    self.win(1);
                }
                else if temp_r1[0] =='O' && temp_r1[1] == 'T' && temp_r1[2] == 'T' && temp_r1[3] == 'O'{
                    self.win(-1);
                }
                else if temp_b1[0] == 'T' && temp_b1[1] =='O' && temp_b1[2] == 'O' && temp_b1[3] == 'T'{
                    self.win(1);
                }
                else if temp_b1[0] == 'O' && temp_b1[1] == 'T' && temp_b1[2] == 'T' && temp_b1[3] == 'O'{
                    self.win(-1);
                }
                else if temp_br1[0] == 'T' && temp_br1[1] == 'O' && temp_br1[2] == 'O' && temp_br1[3] == 'T'{
                    self.win(1);
                }
                else if temp_br1[0] =='O' && temp_br1[1] == 'T' && temp_br1[2] == 'T' && temp_br1[3] == 'O'{
                    self.win(-1);
                }
                else if temp_br2[0] == 'T' && temp_br2[1] =='O' && temp_br2[2] == 'O' && temp_br2[3] == 'T' {
                    self.win(1);
                }
                else if temp_br2[0] == 'O' && temp_br2[1] == 'T' && temp_br2[2] == 'T' && temp_br2[3] == 'O'{
                    self.win(-1);
                }
            }
        }

        // check if draw
        if (self.current_move == 42) && (!self.won) {
            self.win(0);
        }
    }

    fn win(&mut self, player_value: i64) {
        // self.paused = true;
        self.won = true;
        self.reject_click = false;

        let mut msg = String::new();
        let mut winner = String::new();
        if player_value > 0 {
            winner = self.props.player1.as_ref().unwrap().clone();
            msg = format!("{} wins", self.props.player1.as_ref().unwrap());
        } else if player_value < 0 {
            winner = self.props.player2.as_ref().unwrap().clone();
            msg = format!("{} wins", self.props.player2.as_ref().unwrap());
        } else {
            winner = String::from("Draw");
            msg = "Draw".to_string();
        }

        let to_print = format!("{} - Click on game board to reset", msg);

        gloo::console::log!(&to_print);

        // Draw wining information on board
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.set_font("14pt sans-serif");
        ctx.set_fill_style(&JsValue::from("#111111"));

        ctx.begin_path();
        ctx.fill_text(&to_print, 150.0, 20.0);

        self.game_result = Game {
            GameDate: Date::now() as i64,
            gameType: String::from("TOOT-OTTO"),
            gameNumber: String::new(),
            Player1Name: self.props.player1.as_ref().unwrap().clone(),
            Player2Name: self.props.player2.as_ref().unwrap().clone(),
            WinnerName: winner.clone(),
        };
        
        self.post_cb.emit(0);

        ctx.restore();

    }

    // The fillMap function in TootOttoComputer.js
    fn fill_map(&self, state: &Vec<Vec<i64>>, column: usize, value: i64) -> Option<Vec<Vec<i64>>> {
        let mut temp_map = state.clone();
        if temp_map[0][column] != 0 || column > 6 {
            return None;
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if temp_map[i + 1][column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = 5;
        }

        temp_map[row][column] = value;
        return Some(temp_map);
    }

    // The MiniMax Algorithm with alpha beta pruning
    fn value(&self, state: &Vec<Vec<i64>>, depth: i64, alpha: &mut i64, beta: &mut i64) -> i64 {
        let val = self.check_state(state);

        let max_depth = match self.props.difficulty {
            Difficulty::Easy => 1,
            Difficulty::Medium => 3,
            Difficulty::Hard => 5
        };

        // gloo::console::log!(&format!("Max depth: {}", &max_depth));

        if depth >= max_depth {
            let mut ret_val = 0;

            // if win, value = +inf
            let win_val = val.0;
            let chain_val = val.1;
            ret_val = chain_val;

            // If it lead to winning, then do it
            if win_val == 4 {
                // AI win, AI wants to win of course
                ret_val = 999999;
            } 
            else if win_val == 4 * -1 {
                // AI lose, AI hates losing
                ret_val = 999999 * -1;
            }
            ret_val -= depth * depth;

            return (ret_val);
        }

        let win = val.0;
        // if already won, then return the value right away
        if win == 4 {
            // AI win, AI wants to win of course
            return (999999 - depth * depth);
        }
        if win == -4 {
            // AI lose, AI hates losing
            return (999999 * (-1) - depth * depth);
        }

        if depth % 2 == 0 {
            return self.min_state(state, depth + 1, alpha, beta).0;
        }
        return self.max_state(state, depth + 1, alpha, beta).0;
    }

    // The choose function in TootOttoComputer.js
    fn choose<T: Copy>(&self, choice: &Vec<T>) -> T {
        let index = self.get_random_val(choice.len());
        return choice[index];
    }

    fn get_random_val(&self, val: usize) -> usize {
        let rand = js! { return Math.random(); };
        let base: f64 = stdweb::unstable::TryInto::try_into(rand).unwrap();
        let max_val = val as f64;

        return (base * max_val).floor() as usize;
    }

    // The maxState function in TootOttoComputer.js
    fn max_state(&self, state: &Vec<Vec<i64>>, depth: i64, alpha: &mut i64, beta: &mut i64) -> (i64, (i64, char)){
        let mut v = -100000000007;
        let mut new_move: (i64, char) = (-1, '0');
        let mut move_queue = Vec::new();

        for letter in &['T', 'O'] {
            for j in 0..7 {
                let mut move_value = 0;
                if *letter == 'T' {
                    move_value = 1;
                }
                else {
                    move_value = -1;
                }

                let temp_state_option = self.fill_map(state, j, move_value);
    
                // gloo::console::log!(&format!("Max aplha: {}, Max beta: {}", alpha, beta));
    
                if let Some(temp_state) = temp_state_option {
                    let temp_val = self.value(&temp_state, depth, alpha, beta);
    
                    if temp_val > v {
                        v = temp_val;
                        new_move = (j as i64, *letter);
                        move_queue = Vec::new();
                        move_queue.push((j as i64, *letter));
                    } 
                    else if temp_val == v {
                        move_queue.push((j as i64, *letter));
                    }
    
                    // alpha-beta pruning
                    if v > *beta {
                        new_move = self.choose(&move_queue);
                        return (v, new_move);
                    }
                    *alpha = std::cmp::max(*alpha, v);
                }
            }
        }

        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    // The minState function in TootOttoComputer.js
    fn min_state(&self, state: &Vec<Vec<i64>>, depth: i64, alpha: &mut i64, beta: &mut i64) -> (i64, (i64, char)){
        let mut v = 100000000007;
        let mut new_move: (i64, char) = (-1, '0');
        let mut move_queue = Vec::new();

        for letter in &['T', 'O'] {
            for j in 0..7 {
                let mut move_value = 0;
                if *letter == 'T' {
                    move_value = 1;
                }
                else {
                    move_value = -1;
                }

                let temp_state_option = self.fill_map(state, j, move_value);
    
                // gloo::console::log!(&format!("Min alpha: {}, Min beta: {}", alpha, beta));
    
                if let Some(temp_state) = temp_state_option {
                    let temp_val = self.value(&temp_state, depth, alpha, beta);
    
                    if temp_val < v {
                        v = temp_val;
                        new_move = (j as i64, *letter);
                        move_queue = Vec::new();
                        move_queue.push((j as i64, *letter));
                    } 
                    else if temp_val == v {
                        move_queue.push((j as i64, *letter));
                    }
    
                    // alpha-beta pruning
                    if v < *alpha {
                        new_move = self.choose(&move_queue);
                        return (v, new_move);
                    }
                    *beta = std::cmp::min(*beta, v);
                }
            }
        }

        
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    pub fn ai(&mut self) {
        gloo::console::log!("AI move!");
        let mut alpha : i64 = -100000000007;
        let mut beta : i64 = 100000000007;
        let new_map = self.board.clone();
        let (val, (column, letter)) = self.max_state(&new_map, 0, &mut alpha, &mut beta);


        // self.paused = false;
        let mut ret = self.place_plate(column as usize, letter, true);

        while ret < 0 {
            gloo::console::log!("Using random agent");
            let random_choice = self.get_random_val(7);

            let letter = if self.get_random_val(2) == 0 {
                'T'
            } else {
                'O'
            };
            ret = self.place_plate(random_choice, letter, true);
        }
    }


    //-------------------------------------------------------------- Canvas Related Operations -------------------------------------------------------------

    // Determins which column is the current plate placed
    fn in_col(&self, coord: f64, x: f64, radius: f64) -> bool {
        return (coord - x) * (coord - x) <= radius * radius;
    }

    // Place plate in the corresponding column
    fn place_plate(&mut self, col: usize, letter: char, mode: bool) -> i64 {
    
        let mut row : usize = 0;
        let mut row_found : bool = false;

        if  self.won {
            return 0;
        }

        if self.board[row][col] != 0 || col > 6 {
            // The column is full, cannot add more plates
            gloo::console::log!("Column full! Connot add more plates!");
            return -1;
        }

        // Find the row to place the plate
        for i in 0..5 {
            if self.board[i+1][col] != 0 {
                row_found = true;
                row = i;
                break;
            }
        }
        if !row_found {
            row = 5;
        }

        self.draw_plate_animate(row, col, 0, mode, letter);

        // self.paused = true;
        return 1;
    }

    fn draw_board(&self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.set_global_alpha(0.05);
        ctx.set_fill_style(&JsValue::from("rgb(0,191,255)"));
        ctx.set_global_alpha(1.0);
        
        ctx.begin_path();
        for y in 0..6 {
            for x in 0..7 {
                ctx.arc(
                    (75 * x + 100) as f64,
                    (75 * y + 50) as f64,
                    25.0,
                    0.0,
                    2.0 * 3.14159265359,
                ).unwrap();
                ctx.rect(
                    (75 * x + 150) as f64,
                    (75 * y) as f64,
                    -100.0,
                    100.0,
                );
            }
        }
        ctx.fill();
    }

    fn draw_plate(&self, x:f64, y: f64, fill_color: &str, stroke_color: &str, text: &str) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.save();
        ctx.begin_path(); // This make sure that we can draw shapes of different color

        // Set plate color
        ctx.set_fill_style(&JsValue::from(fill_color));
        ctx.set_stroke_style(&JsValue::from_str(stroke_color));

        // Draw plate
        ctx.arc(x, y, 25.0, 0.0, 2.0 * 3.14159265359).unwrap();
        ctx.fill();

        self.draw_text_on_plate(x, y, text);
    }

    fn draw_text_on_plate(&self, x: f64, y: f64, text: &str) {

        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.begin_path();
        ctx.set_font("14pt sans-serif");
        ctx.set_fill_style(&JsValue::from("#111111"));
        ctx.fill_text(text, x - 7.0, y + 7.0);
    }

    // Draw the plate with animation (plate slowly dropping)
    fn draw_plate_animate(&mut self, row: usize, col: usize, current_pos: usize, mode: bool, letter: char) {

        let mut plate_color = "#99ffcc";

        if self.player_move() >= 1 {
            plate_color = "#99ffcc";
        }
        else if self.player_move() <= -1  {
            plate_color = "#ffff99";
        }

        if row * 75 > current_pos {
            // Still in progress of dropping
            self.clear_board();
            self.draw_game();
            self.draw_board();
            self.draw_plate((75 * col + 100) as f64, (current_pos + 50) as f64, plate_color, "black", &letter.to_string());

            // Update the point location for next drawing
            self.plate_position.row = row;
            self.plate_position.col = col;
            self.plate_position.current_pos = current_pos + 25;
            self.plate_position.mode = mode;
            self.plate_position.letter = letter;

            window()
                .unwrap()
                .request_animation_frame(&self.animation_cb.as_ref().unchecked_ref())
                .unwrap();
        }
        else {
            self.board[row][col] = self.player_move();
            self.letter_board[row][col] = letter;
            self.current_move += 1;
            self.clear_board();
            self.draw_game();
            self.draw_board();

            self.check();

            // This makes sure that when we win the game, we can click the board to reset
            if self.won {
                self.reject_click = false;
            }

            if mode == false && self.props.player2.as_ref().unwrap() == "Computer" {
                self.ai();
            } 
            else {
                self.reject_click = false;
            }
        }
    }

    // This method clears the entire canvas, including the blue portion
    fn clear_board(&self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    }

    // Draw the plates on the board based on what is stored in the self.board
    fn draw_game(&self) {
        for row in 0..6 {
            for col in 0..7 {

                let mut text = "";
                let mut fill_color = "transparent";

                if self.board[row][col] >= 1 && self.letter_board[row][col] == 'T' {
                    fill_color = "#99ffcc";
                    text = "T";
                } else if self.board[row][col] >= 1 && self.letter_board[row][col] == 'O' {
                    fill_color = "#99ffcc";
                    text = "O";
                } else if self.board[row][col] <= -1 && self.letter_board[row][col] == 'T' {
                    fill_color = "#ffff99";
                    text = "T";
                } else if self.board[row][col] <= -1 && self.letter_board[row][col] == 'O' {
                    fill_color = "#ffff99";
                    text = "O";
                }

                if fill_color != "transparent" {
                    self.draw_plate((75 * col + 100) as f64, (75 * row + 50) as f64, fill_color, "black", text);
                }
            }
        }
    }

    //This method resets the board for new game
    fn reset(&mut self) {
        self.board = vec![vec![0; 7]; 6];
        self.letter_board = vec![vec!['0'; 7]; 6];
        self.current_move = 0;
        // self.paused = false;
        self.won = false;
        self.reject_click = false;
        self.plate_position.row = 0;
        self.plate_position.col = 0;
        self.plate_position.current_pos = 0;
        self.plate_position.mode = false;
        self.plate_position.letter = '0';

        self.clear_board();
        self.draw_board();
    }
}

pub async fn send_post_request(game_result:Game) -> Result<(), FetchError> {

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let game_result_json = serde_json::to_string(&game_result).unwrap();

    opts.body(Some(&JsValue::from_serde(&game_result_json).unwrap()));


    let request = Request::new_with_str_and_init("/games", &opts)?;

    request
        .headers()
        .set("Content-Type", "application/json")?;


    let window = window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    Ok(())
}

//-------------------------------------------------------------- Component Related ---------------------------------------------------------

impl Component for TOOT_OTTO_Canvas {
    type Message = Msg;
    type Properties = GameProperty;

    fn create(ctx: &Context<Self>) -> Self {
        let canvas_id = ctx.props().canvas_id.clone().unwrap();
        let letter = ctx.props().letter.clone();
        let mut board: Vec<Vec<i64>> = vec![vec![0; 7]; 6];
        let mut letter_board: Vec<Vec<char>> = vec![vec!['0'; 7]; 6];

        // The callback function for mouse click
        // Used to send a mesage and let the component to get the click position
        let on_click_cb = ctx.link().callback(|e: MouseEvent| Msg::Click(e));
        let l = ctx.link().clone();

        let animation_cb = Closure::wrap(Box::new(move || l.send_message(Msg::AnimateMsg)) as Box<dyn FnMut()>);

        let l2 = ctx.link().clone();
        // let post_cb = Closure::wrap(Box::new(move || l2.send_message(Msg::SendPost)) as Box<dyn FnMut()>);
        let post_cb = ctx.link().callback(|e| Msg::SendPost);

        let difficulty = ctx.props().difficulty.clone();

        match difficulty {
            Difficulty::Easy => {
                gloo::console::log!("Current difficulty: Easy");
            },
            Difficulty::Medium=> {
                gloo::console::log!("Current difficulty: Medium");
            },
            Difficulty::Hard => {
                gloo::console::log!("Current difficulty: Hard");
            }
        }

        Self {
            props: ctx.props().clone(),
            canvas_id,
            canvas: NodeRef::default(),
            board,
            letter_board,
            letter,
            on_click_cb,
            animation_cb,
            post_cb,
            plate_position : PlatePosition{
                row: 0,
                col: 0,
                current_pos: 0,
                mode: false,
                letter: '0'
            },
            current_move: 0,
            won: false,
            // paused: false,
            reject_click: false,
            game_result: Game {
                GameDate: Date::now() as i64,
                gameType: String::from("TOOT-OTTO"),
                gameNumber: String::new(),
                Player1Name: "".to_string(),
                Player2Name: "".to_string(),
                WinnerName: "".to_string(),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(e) => {
                gloo::console::log!("User move!");
                if self.reject_click {
                    return false;
                }

                if self.won {
                    self.reset();
                    self.props.game_done_cbk.emit(0); // This realized the click on game board to reset function
                    return true;
                }

                if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                    let rect = target.get_bounding_client_rect();
                    let x = (e.client_x() as f64) - rect.left();
                    let y = (e.client_y() as f64) - rect.top();

                    // check which column is the click in
                    // put the plate in corresponding column
                    for col in 0..7 {
                        if self.in_col(x, (75 * col + 100) as f64, 25 as f64) {
                            // self.paused = false;

                            if self.place_plate(col, self.letter.chars().next().unwrap(), false) == 1 {
                                self.reject_click = true;
                            }

                            break;
                        }
                    }
                }
                false
            },
            Msg::AnimateMsg => {
                self.draw_plate_animate(self.plate_position.row, self.plate_position.col, self.plate_position.current_pos, self.plate_position.mode, self.plate_position.letter);
                false
            },
            Msg::SendPost => {

                let game_result = self.game_result.clone();

                ctx.link().send_future(async {
                    match send_post_request(game_result).await {
                        Ok(_) => {
                            gloo::console::log!("Send POST request success.");
                            Msg::Ignore
                        },
                        Err(err) => {
                            gloo::console::log!("Send POST request failed");
                            gloo::console::log!(err.to_string());
                            Msg::Ignore
                        }
                    }
                });

                false
            },
            Msg::Ignore => {false},
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <canvas 
                    id={self.canvas_id.clone()} 
                    height="480" 
                    width="640" 
                    ref={self.canvas.clone()} 
                    onclick={self.on_click_cb.clone()}>
                </canvas>
            </div>
        }
    }

    // This function makes sure that the board will be displayed when we first go to the webpage
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        self.draw_board();
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.props = ctx.props().clone();
        self.letter = self.props.letter.clone();

        let difficulty = self.props.difficulty.clone();

        match difficulty {
            Difficulty::Easy => {
                gloo::console::log!("Current difficulty changed to: Easy");
            },
            Difficulty::Medium=> {
                gloo::console::log!("Current difficulty changed to: Medium");
            },
            Difficulty::Hard => {
                gloo::console::log!("Current difficulty changed to: Hard");
            }
        }

        true
    }
} 