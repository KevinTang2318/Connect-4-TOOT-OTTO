use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, Component, Context, Html, NodeRef};
use web_sys::HtmlInputElement;
use super::connect_4_canvas::Connect4Canvas;
use super::connect_4_canvas::Difficulty;

pub enum Msg {
    InsertName1(String),
    InsertName2(String),
    StartGame,
    EndGame,
}


pub struct Connect4HumanSide {
    player1_name: String,
    player2_name: String,
    disabled: bool,
    game_running: bool,
    state: String,
    difficulty: Difficulty,
    name1_input: NodeRef,
    name2_input: NodeRef,
}

impl Component for Connect4HumanSide {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Connect4HumanSide{
            player1_name: "".to_string(),
            player2_name: "".to_string(),
            disabled: false,
            game_running: false,
            state: "none".to_string(),
            difficulty: Difficulty::Easy,
            name1_input: NodeRef::default(),
            name2_input: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InsertName1(name) => {
                self.player1_name = name;
        
            },
            Msg::InsertName2(name) => {
                self.player2_name = name;
            },
            Msg::StartGame => {
                if self.player1_name != "" && self.player2_name != "" {
                    if self.player2_name != "Computer" {
                        self.game_running = true;
                        self.disabled = true;
                        self.state = "block".to_string();
                    }
                    else {
                        gloo::console::log!("Second play's name cannot be Computer!");
                    }
                }
                else {
                    gloo::console::log!("Both user names cannot be empty!");
                }
            }
            Msg::EndGame => {
                self.game_running = false;
                self.disabled = false;
                self.state = "none".to_string();
            }
        }    
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let name1_input_ref = self.name1_input.clone();
        let name2_input_ref = self.name2_input.clone();


        let onchange = link.batch_callback(move |_| {
            let input = name2_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InsertName2(input.value()))
        });

        let oninput = link.batch_callback(move |_| {
            let input = name1_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InsertName1(input.value()))
        });


        html! {
            <>
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
            <div class="col-md-offset-3 col-md-8">
                <div class="col-md-offset-3 col-md-8">
                    <input ref={self.name1_input.clone()}{oninput}
                        disabled={self.disabled}
                        id="player_name"
                        type="text"
                        placeholder="Your Name"/>
                    <input ref={self.name2_input.clone()}{onchange}
                        disabled={self.disabled}
                        id="player_name"
                        type="text"
                        placeholder="Your Name"/>
                    <button
                        disabled={self.disabled}
                        id="startbutton"
                        onclick={link.callback(|_| Msg::StartGame)} 
                        title="Start Game">
                        {"Start Game"}
                    </button>
                </div>
            </div>
            <div style={format!("display: {}", self.state)}>
                <br/>
                <h4>{format!("New Game: {} Vs {}", self.player1_name, self.player2_name)}</h4>
                <small disabled={!self.disabled}>{format!("(Disc Colors: {} - ", self.player1_name)} <b>{"Red"}</b> {format!("   and    {} - ", self.player2_name)} <b>{"Yellow)"}</b></small>
                <br/>
                <Connect4Canvas  
                    canvas_id = "connect_human" 
                    player1 = {self.player1_name.clone()}
                    player2 = {self.player2_name.clone()}
                    difficulty = {self.difficulty}
                    game_done_cbk={link.callback(|_| Msg::EndGame)}/>
            </div>
            </>
        }   
    }
}    
