use yew::prelude::*;
use yew::virtual_dom::VNode;
//use web_sys::InputEvent;
//use web_sys::DataTransfer;
//use yew_stdweb::events::ChangeData;
//use web_sys::Event;
//use yew::virtual_dom::ListenerKind;
use yew::{html, Component, Context, Html, NodeRef};
use web_sys::HtmlInputElement;
use super::connect_4_canvas::Connect4Canvas;


pub enum Msg {
    InsertName(String),
    ChooseDifficulty(String),
    StartGame,
    EndGame,
}


pub struct Connect4Side {
    player_name: String,
    disabled: bool,
    game_running: bool,
    state: String,
    difficulty: Difficulty,
    my_input: NodeRef,
    name_input: NodeRef,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Component for Connect4Side {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Connect4Side{
            player_name: "".to_string(),
            disabled: false,
            game_running: false,
            state: "none".to_string(),
            difficulty: Difficulty::Easy,
            my_input: NodeRef::default(),
            name_input: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InsertName(name) => {
                self.player_name = name;
        
            },
            Msg::ChooseDifficulty(difficulty) => {
                // self.difficulty = difficulty;
                if difficulty == "Hard" {
                    self.difficulty = Difficulty::Hard;
                }
                else if difficulty == "Medium" {
                    self.difficulty = Difficulty::Medium;
                }
                else {
                    self.difficulty = Difficulty::Easy;
                }
            },
            Msg::StartGame => {
                self.game_running = true;
                self.disabled = true;
                self.state = "block".to_string();
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
        let my_input_ref = self.my_input.clone();
        let name_input_ref = self.name_input.clone();


        let onchange = link.batch_callback(move |_| {
            let input = my_input_ref.cast::<HtmlInputElement>();
            
            input.map(|input| Msg::ChooseDifficulty(input.value()))
        });

        let oninput = link.batch_callback(move |_| {
            let input = name_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InsertName(input.value()))
        });


        return html! {
            <>
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
            <div class="col-md-offset-3 col-md-8">
                <div class="col-md-offset-3 col-md-8">

                    <input ref={self.name_input.clone()}{oninput}
                        disabled={self.disabled}
                        id="player_name"
                        type="text"
                        placeholder="Your Name"/>

                    <select ref={self.my_input.clone()}{onchange} 
                        disabled={self.disabled}>
                        <option value="Hard">{"Hard"}</option>
                        <option value="Medium">{"Medium"}</option>
                        <option value="Easy">{"Easy"}</option>
                    </select>

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
                <h4>{format!("New Game: {} Vs Computer", self.player_name)}</h4>
                <small>{format!("(Disc Colors: {} - ", self.player_name)} <b>{"Red"}</b> {"   and    Computer - "} <b>{"Yellow)"}</b></small>
                <br/>
                <Connect4Canvas  
                    canvas_id = "connect_computer" 
                    player1 = {self.player_name.clone()}
                    player2 = "Computer" 
                    difficulty = {self.difficulty}
                    game_done_cbk={link.callback(|_| Msg::EndGame)}/>
            </div>
            </>
        }
    }
}

