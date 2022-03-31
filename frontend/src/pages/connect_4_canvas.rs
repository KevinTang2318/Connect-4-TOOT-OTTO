use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Properties;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap,
};

use super::connect_4_side::Difficulty;

pub struct CanvasModel {
    props: GameProperty,
    canvas_id: String,
    canvas: NodeRef,
    map: Vec<Vec<i64>>,
    // cb: Closure<dyn FnMut()>,
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
    // Render
}

impl CanvasModel {

    fn render(&self) {
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

        // window()
        //     .unwrap()
        //     .request_animation_frame(self.cb.as_ref().unchecked_ref())
        //     .unwrap();
    }
}

impl Component for CanvasModel {
    type Message = Msg;
    type Properties = GameProperty;

    fn create(ctx: &Context<Self>) -> Self {
        let canvas_id = ctx.props().canvas_id.clone().unwrap();

        let mut map: Vec<Vec<i64>> = vec![vec![0; 7]; 6];

        // let l = ctx.link().clone();
        // let cb = Closure::wrap(Box::new(move || l.send_message(Msg::Render)) as Box<dyn FnMut()>);


        Self {
            props: ctx.props().clone(),
            canvas_id,
            canvas: NodeRef::default(),
            map,
            // cb,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg {
        //     Msg::Render => {
        //         self.render();
        //         false
        //     }
        // }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // ctx.link().send_message(Msg::Render);

        html! {
            <div>
                <canvas id={self.canvas_id.clone()} height="480" width="640" ref={self.canvas.clone()}></canvas>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        self.render();
    }


} 