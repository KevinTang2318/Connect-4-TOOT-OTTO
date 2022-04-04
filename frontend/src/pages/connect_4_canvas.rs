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


use super::connect_4_side::Difficulty;
use super::score_board::Game;

pub struct Connect4Canvas {
    props: GameProperty,
    canvas_id: String,
    canvas: NodeRef,
    board: Vec<Vec<i64>>,
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

#[derive(Clone, PartialEq, Properties)]
pub struct GameProperty {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    pub game_done_cbk: Callback<i64>,
}

// This struct is used to store the plate destination and current location
// Used for creating animation
#[derive(Clone, PartialEq)]
pub struct PlatePosition {
    pub row: usize,
    pub col: usize,
    pub current_pos: usize,
    mode: bool,
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

impl Connect4Canvas {
    //-------------------------------------------------------------- Game Algorithm -------------------------------------------------------------------- 
    // Same as the checkState function in Connect4App.js
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
                    if j + k < 7 {
                        temp_r += state[i][j + k];
                    }

                    if i + k < 6 {
                        temp_b += state[i + k][j];
                    }

                    if i + k < 6 && j + k < 7 {
                        temp_br += state[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_tr += state[i - k][j + k];
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

    // Same as the playerMove function in Connect4App.js
    fn player_move(&self) -> i64 {
        if self.current_move % 2 == 0 {
            return 1;
        }
        return -1;
    }

    fn check(&mut self) {
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
                    if j + k < 7 {
                        temp_r += self.board[i][j + k];
                    }

                    if i + k < 6 {
                        temp_b += self.board[i + k][j];
                    }

                    if i + k < 6 && j + k < 7 {
                        temp_br += self.board[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_tr += self.board[i - k][j + k];
                    }
                }
                if temp_r.abs() == 4 {
                    self.win(temp_r);
                } 
                else if temp_b.abs() == 4 {
                    self.win(temp_b);
                } 
                else if temp_br.abs() == 4 {
                    self.win(temp_br);
                } 
                else if temp_tr.abs() == 4 {
                    self.win(temp_tr);
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
        // self.reject_click = false;

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
            gameType: String::from("Connect-466666"),
            gameNumber: String::new(),
            Player1Name: self.props.player1.as_ref().unwrap().clone(),
            Player2Name: self.props.player2.as_ref().unwrap().clone(),
            WinnerName: winner.clone(),
        };
        
        self.post_cb.emit(0);

        ctx.restore();

    }

    // The fillMap function in Connect4App.js
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
    fn value(&self, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64, ai_move_value: i64) -> (i64, i64) {
        let val = self.check_state(state);

        let max_depth = match self.props.difficulty {
            Easy => 1,
            Medium => 3,
            Hard => 5
        };

        if depth >= max_depth {
            let mut ret_val = 0;

            // if win, value = +inf
            let win_val = val.0;
            let chain_val = val.1 * ai_move_value;
            ret_val = chain_val;

            // If it lead to winning, then do it
            if win_val == 4 * ai_move_value {
                // AI win, AI wants to win of course
                ret_val = 999999 - depth * depth;
            } else if win_val == 4 * ai_move_value * -1 {
                // AI lose, AI hates losing
                ret_val = 999999 * -1 - depth * depth;
            }

            return (ret_val, -1);
        }

        let win = val.0;
        // if already won, then return the value right away
        if win == 4 * ai_move_value {
            // AI win, AI wants to win of course
            return (999999 - depth * depth, -1);
        }
        if win == 4 * ai_move_value * -1 {
            // AI lose, AI hates losing
            return (999999 * -1 - depth * depth, -1);
        }

        if depth % 2 == 0 {
            return self.min_state(state, depth + 1, alpha, beta, ai_move_value);
        }
        return self.max_state(state, depth + 1, alpha, beta, ai_move_value,);
    }

    // The choose function in Connect4App.js
    fn choose(&self, choice: &Vec<usize>) -> i64 {
        let index = self.get_random_val(choice.len());
        return choice[index] as i64;
    }

    fn get_random_val(&self, val: usize) -> usize {
        let rand = js! { return Math.random(); };
        let base: f64 = stdweb::unstable::TryInto::try_into(rand).unwrap();
        let max_val = val as f64;

        return (base * max_val).floor() as usize;
    }

    // The maxState function in Connect4App.js
    fn max_state(&self, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64, ai_move_value: i64) -> (i64, i64){
        let mut v = -100000000007;
        let mut new_move: i64 = -1;
        let mut move_queue = Vec::new();

        for j in 0..7 {
            let temp_state_option = self.fill_map(state, j, ai_move_value);

            if let Some(temp_state) = temp_state_option {
                let temp_val = self.value(&temp_state, depth, alpha, beta, ai_move_value);

                if temp_val.0 > v {
                    v = temp_val.0;
                    new_move = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j);
                } 
                else if temp_val.0 == v {
                    move_queue.push(j);
                }

                // alpha-beta pruning
                if v > beta {
                    new_move = self.choose(&move_queue);
                    return (v, new_move);
                }
                alpha = std::cmp::max(alpha, v);
            }
        }
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    fn min_state(&self, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64, ai_move_value: i64) -> (i64, i64){
        let mut v = 100000000007;
        let mut new_move: i64 = -1;
        let mut move_queue = Vec::new();

        for j in 0..7 {
            let temp_state_option = self.fill_map(state, j, ai_move_value);

            if let Some(temp_state) = temp_state_option {
                let temp_val = self.value(&temp_state, depth, alpha, beta, ai_move_value);

                if temp_val.0 < v {
                    v = temp_val.0;
                    new_move = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j);
                } 
                else if temp_val.0 == v {
                    move_queue.push(j);
                }

                // alpha-beta pruning
                if v < alpha {
                    new_move = self.choose(&move_queue);
                    return (v, new_move);
                }
                beta = std::cmp::max(beta, v);
            }
        }
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    pub fn ai(&mut self, ai_move_value: i64) {
        let new_map = self.board.clone();
        let val_choice = self.max_state(&new_map, 0, -100000000007, 100000000007, ai_move_value);

        let val = val_choice.0;
        let choice = val_choice.1;

        // self.paused = false;
        let mut ret = self.place_plate(choice as usize, true);

        while ret < 0 {
            gloo::console::log!("Using random agent");
            let random_choice = self.get_random_val(7);
            ret = self.place_plate(random_choice, true);
        }
    }


    //-------------------------------------------------------------- Canvas Related Operations -------------------------------------------------------------

    // Determins which column is the current plate placed
    fn in_col(&self, coord: f64, x: f64, radius: f64) -> bool {
        return (coord - x) * (coord - x) <= radius * radius;
    }

    // Place plate in the corresponding column
    fn place_plate(&mut self, col: usize, mode: bool) -> i64 {
    
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

        self.draw_plate_animate(row, col, 0, mode);

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

    fn draw_plate(&self, x:f64, y: f64, fill_color: &str, stroke_color: &str) {
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
    }

    // Draw the plate with animation (plate slowly dropping)
    fn draw_plate_animate(&mut self, row: usize, col: usize, current_pos: usize, mode: bool) {

        let mut plate_color = "#ff4136";

        if self.player_move() >= 1 {
            plate_color = "#ff4136";
        }
        else if self.player_move() <= -1  {
            plate_color = "#ffff00";
        }

        if row * 75 > current_pos {
            // Still in progress of dropping
            self.clear_board();
            self.draw_game();
            self.draw_board();
            self.draw_plate((75 * col + 100) as f64, (current_pos + 50) as f64, plate_color, "black");

            // Update the point location for next drawing
            self.plate_position.row = row;
            self.plate_position.col = col;
            self.plate_position.current_pos = current_pos + 25;
            self.plate_position.mode = mode;

            window()
                .unwrap()
                .request_animation_frame(&self.animation_cb.as_ref().unchecked_ref())
                .unwrap();
        }
        else {
            self.board[row][col] = self.player_move();
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
                self.ai(-1);
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
                // determin the plate color
                let mut fill_color = "transparent";
                if self.board[row][col] >= 1 {
                    fill_color = "#ff4136";
                } else if self.board[row][col] <= -1 {
                    fill_color = "#ffff00";
                }

                if fill_color != "transparent" {
                    self.draw_plate((75 * col + 100) as f64, (75 * row + 50) as f64, fill_color, "black");
                }
            }
        }
    }

    //This method resets the board for new game
    fn reset(&mut self) {
        self.board = vec![vec![0; 7]; 6];
        self.current_move = 0;
        // self.paused = false;
        self.won = false;
        self.reject_click = false;
        self.plate_position.row = 0;
        self.plate_position.col = 0;
        self.plate_position.current_pos = 0;
        self.plate_position.mode = false;

        self.clear_board();
        self.draw_board();
    }
}

// #[wasm_bindgen]
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

impl Component for Connect4Canvas {
    type Message = Msg;
    type Properties = GameProperty;

    fn create(ctx: &Context<Self>) -> Self {
        let canvas_id = ctx.props().canvas_id.clone().unwrap();

        let mut board: Vec<Vec<i64>> = vec![vec![0; 7]; 6];

        // The callback function for mouse click
        // Used to send a mesage and let the component to get the click position
        let on_click_cb = ctx.link().callback(|e: MouseEvent| Msg::Click(e));
        let l = ctx.link().clone();

        let animation_cb = Closure::wrap(Box::new(move || l.send_message(Msg::AnimateMsg)) as Box<dyn FnMut()>);

        let l2 = ctx.link().clone();
        // let post_cb = Closure::wrap(Box::new(move || l2.send_message(Msg::SendPost)) as Box<dyn FnMut()>);
        let post_cb = ctx.link().callback(|e| Msg::SendPost);

        let difficulty = ctx.props().difficulty.clone();

        // match difficulty {
        //     Difficulty::Easy => {
        //         gloo::console::log!("Current difficulty: Easy");
        //     },
        //     Difficulty::Medium=> {
        //         gloo::console::log!("Current difficulty: Medium");
        //     },
        //     Difficulty::Hard => {
        //         gloo::console::log!("Current difficulty: Hard");
        //     }
        // }

        Self {
            props: ctx.props().clone(),
            canvas_id,
            canvas: NodeRef::default(),
            board,
            on_click_cb,
            animation_cb,
            post_cb,
            plate_position : PlatePosition{
                row: 0,
                col: 0,
                current_pos: 0,
                mode: false
            },
            current_move: 0,
            won: false,
            // paused: false,
            reject_click: false,
            game_result: Game {
                GameDate: Date::now() as i64,
                gameType: String::from("Connect-4"),
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

                            if self.place_plate(col, false) == 1 {
                                self.reject_click = true;
                            }

                            break;
                        }
                    }
                }
                false
            },
            Msg::AnimateMsg => {
                self.draw_plate_animate(self.plate_position.row, self.plate_position.col, self.plate_position.current_pos, self.plate_position.mode);
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

        // let difficulty = ctx.props().difficulty.clone();
        // match difficulty {
        //     Difficulty::Easy => {
        //         gloo::console::log!("Current difficulty changed to: Easy");
        //     },
        //     Difficulty::Medium=> {
        //         gloo::console::log!("Current difficulty changed to: Medium");
        //     },
        //     Difficulty::Hard => {
        //         gloo::console::log!("Current difficulty changed to: Hard");
        //     }
        // }

        true
    }
} 