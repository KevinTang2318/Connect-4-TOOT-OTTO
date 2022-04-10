use yew::prelude::*;
use yew::virtual_dom::VNode;
//use web_sys::InputEvent;
//use web_sys::DataTransfer;
//use yew_stdweb::events::ChangeData;
//use web_sys::Event;
//use yew::virtual_dom::ListenerKind;
use yew::{html, Component, Context, Html, NodeRef};
use web_sys::HtmlInputElement;
use super::toot_otto_canvas::TOOT_OTTO_Canvas;
use super::toot_otto_canvas::Difficulty;



pub enum Msg {
    InsertName(String),
    ChooseDifficulty(String),
    StartGame,
    EndGame,
    InputChanged,
    //InputChangedO,
}


pub struct TootOttoSide {
    player_name: String,
    difficulty: Difficulty,
    letter: String,
    disabled: bool,
    game_running: bool,
    state: String,
    
    my_input: NodeRef,
    name_input: NodeRef,
    letter_input: NodeRef,
    //letterO_input: NodeRef,
}


impl Component for TootOttoSide {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TootOttoSide{
            player_name: "".to_string(),
            difficulty: Difficulty::Easy,
            letter: "T".to_string(),
            disabled: false,
            game_running: false,
            state: "none".to_string(),
        
            my_input: NodeRef::default(),
            name_input: NodeRef::default(),
            letter_input: NodeRef::default(),
            //letterO_input: NodeRef::default(),
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
                if self.player_name != "" {
                    self.game_running = true;
                    self.disabled = true;
                    self.state = "block".to_string();
                }
                else {
                    gloo::console::log!("User name cannot be empty!");
                }
            },
            Msg::EndGame => {
                self.game_running = false;
                self.disabled = false;
                self.state = "none".to_string();
            },
            Msg::InputChanged => {
                if let Some(letter) = self.letter_input.clone().cast::<HtmlInputElement>() {
                    if self.letter.eq(&"T".to_string()){
                        self.letter = "O".to_string();
                    }
                    else{
                        self.letter = "T".to_string();
                    }
                }
                else{

                }    
                    
            },
            
            
        }    
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let my_input_ref = self.my_input.clone();
        let name_input_ref = self.name_input.clone();
        //let letter_input_ref = self.letter_input.clone();

        let onchange = link.batch_callback(move |_| {
            let input = my_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::ChooseDifficulty(input.value()))
        });

        let oninput = link.batch_callback(move |_| {
            let input = name_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InsertName(input.value()))
        });

       // let onclick = link.callback(|_| Msg::InputChanged);
        //let ontoggle = onselect.clone();
        
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
                <small>{format!("(Winning Combination: {} - ", self.player_name)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
                {" Select a Disc Type:  "}
                <input ref={self.letter_input.clone()} onclick = {link.callback(|_| Msg::InputChanged)} type="radio" name="letter" value="T" checked={self.letter == "T".to_string()}/>
                <label for="T">{"T"}</label>
                <input ref={self.letter_input.clone()} onclick = {link.callback(|_| Msg::InputChanged)} type="radio" name="letter" value="O" checked={self.letter == "O".to_string()}/>
                <label for="O">{"O"}</label>
                <h4>{format!("Letter is {}",self.letter)}</h4>
                <br/>
                <br/>
                <TOOT_OTTO_Canvas  
                    canvas_id = "toot_computer" 
                    player1 = {self.player_name.clone()}
                    player2 = "Computer" 
                    difficulty = {self.difficulty}
                    letter = {self.letter.clone()}
                    game_done_cbk={link.callback(|_| Msg::EndGame)}/>
            </div>
            </>
        }
    }
}