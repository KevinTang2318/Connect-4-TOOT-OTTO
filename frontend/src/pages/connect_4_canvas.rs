use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Properties;
use yew::Callback;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, HtmlElement
};

use std::{thread, time};

use super::connect_4_side::Difficulty;

pub struct Connect4Canvas {
    props: GameProperty,
    canvas_id: String,
    canvas: NodeRef,
    board: Vec<Vec<i64>>,
    on_click_cb: Callback<MouseEvent>,
    animation_cb: Closure<dyn FnMut()>,
    plate_position: PlatePosition,
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameProperty {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    // pub game_end_callback: Callback<i64>,
}

// This struct is used to store the plate destination and current location
// Used for creating animation
#[derive(Clone, PartialEq)]
pub struct PlatePosition {
    pub row: usize,
    pub col: usize,
    pub current_pos: usize,
}

pub enum Msg {
    Click(MouseEvent),
    AnimateMsg, // row, col, current_pos, 
    // Render
}

impl Connect4Canvas {

    // Determins which column is the current plate placed
    fn in_col(&self, coord: f64, x: f64, radius: f64) -> bool {
        return (coord - x) * (coord - x) <= radius * radius;
    }

    // Place plate in the corresponding column
    fn place_plate(&mut self, col: usize) {

        let mut row : usize = 0;
        let mut row_found : bool = false;

        if self.board[row][col] != 0 {
            // The column is full, cannot add more plates
            // TODO: add corresponding reactions
            gloo::console::log!("Column full! Connot add more plates!");
            return;
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

        // Place the plate in the board by placing a 1 in the corresponding array location
        self.board[row][col] = 1;

        self.draw_plate_animate(row, col, 0);
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
    fn draw_plate_animate(&mut self, row: usize, col: usize, current_pos: usize) {
        let mut plate_color = "#ff4136";

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

            window()
                .unwrap()
                .request_animation_frame(&self.animation_cb.as_ref().unchecked_ref())
                .unwrap();
        }
        else {
            self.board[row][col] = 1;
            self.clear_board();
            self.draw_game();
            self.draw_board();
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


        // let animation_cb = Closure::wrap(Box::new(move |row: usize, col: usize, current_pos: usize| {
        //     l.send_message(Msg::AnimateMsg((row, col, current_pos)))
        // }) as Box<dyn FnMut(usize, usize, usize)>);

        let animation_cb = Closure::wrap(Box::new(move || l.send_message(Msg::AnimateMsg)) as Box<dyn FnMut()>);

        Self {
            props: ctx.props().clone(),
            canvas_id,
            canvas: NodeRef::default(),
            board,
            on_click_cb,
            animation_cb,
            plate_position : PlatePosition{
                row: 0,
                col: 0,
                current_pos: 0,
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(e) => {
                if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                    let rect = target.get_bounding_client_rect();
                    let x = (e.client_x() as f64) - rect.left();
                    let y = (e.client_y() as f64) - rect.top();
                    // gloo::console::log!(&format!("Left? : {} ; Top? : {}", x, y));

                    // check which column is the click in
                    // put the plate in corresponding column
                    for col in 0..7 {
                        if self.in_col(x, (75 * col + 100) as f64, 25 as f64) {
                            self.place_plate(col);
                        }
                    }
                }
                false
            }
            Msg::AnimateMsg => {
                self.draw_plate_animate(self.plate_position.row, self.plate_position.col, self.plate_position.current_pos);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // ctx.link().send_message(Msg::Render);

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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        self.draw_board();
    }


} 