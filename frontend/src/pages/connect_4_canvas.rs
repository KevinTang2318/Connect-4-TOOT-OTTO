use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Properties;
use yew::Callback;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, HtmlElement
};

use super::connect_4_side::Difficulty;

pub struct Connect4Canvas {
    props: GameProperty,
    canvas_id: String,
    canvas: NodeRef,
    board: Vec<Vec<i64>>,
    on_click_cb: Callback<MouseEvent>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameProperty {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    // pub game_end_callback: Callback<i64>,
}

pub enum Msg {
    Click(MouseEvent),
    // Render
}

impl Connect4Canvas {

    // Place plate in the corresponding column
    fn place_plate(&mut self, col: usize) {

        let mut row : usize = 0;
        let mut row_found : bool = false;

        if self.board[row][col] != 0 {
            // The column is full, cannot add more plates
            // TODO: add corresponding reactions
            gloo::console::log!("66666");
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

        gloo::console::log!(&format!("row : {} ; col : {}", row, col));

        // Place the plate in the board by placing a 1 in the corresponding array location
        self.board[row][col] = 1;

        self.draw_plate((75 * col + 100) as f64 , (75 * row + 50) as f64, "#ff4136", "black"); // place it at the first line
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

    // Determins which column is the current plate placed
    fn in_col(&self, coord: f64, x: f64, radius: f64) -> bool {
        return (coord - x) * (coord - x) <= radius * radius;
    }

    fn process_mouse_click(&self, e : MouseEvent) {

    }

    // fn animate()
}

impl Component for Connect4Canvas {
    type Message = Msg;
    type Properties = GameProperty;

    fn create(ctx: &Context<Self>) -> Self {
        let canvas_id = ctx.props().canvas_id.clone().unwrap();

        let mut board: Vec<Vec<i64>> = vec![vec![0; 7]; 6];

        // The callback function for mouse click
        // Used to send a mesage and let the component to get the click position
        let on_click_cb = ctx.link().callback(|e: MouseEvent| Msg::Click(e));

        Self {
            props: ctx.props().clone(),
            canvas_id,
            canvas: NodeRef::default(),
            board,
            on_click_cb,
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