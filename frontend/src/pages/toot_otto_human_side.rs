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
    InsertName1(String),
    InsertName2(String),
    InputChanged,
    StartGame,
    EndGame,
}


pub struct TootOttoHumanSide {
    player1_name: String,
    player2_name: String,
    letter: String,
    disabled: bool,
    game_running: bool,
    state: String,
    //difficulty: String,
    name1_input: NodeRef,
    name2_input: NodeRef,
    letter_input: NodeRef,
}

/*#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}*/

impl Component for TootOttoHumanSide {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TootOttoHumanSide{
            player1_name: "".to_string(),
            player2_name: "".to_string(),
            letter: "T".to_string(),
            disabled: false,
            game_running: false,
            state: "none".to_string(),
            //difficulty: "Easy".to_string(),
            name1_input: NodeRef::default(),
            name2_input: NodeRef::default(),
            letter_input: NodeRef::default(),
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
        let name1_input_ref = self.name1_input.clone();
        let name2_input_ref = self.name2_input.clone();
        //let letter_input_ref = self.letter_input.clone();



        let onchange = link.batch_callback(move |_| {
            let input = name2_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InsertName2(input.value()))
        });

        let oninput = link.batch_callback(move |_| {
            let input = name1_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InsertName1(input.value()))
        });


        return html! {
            <>
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
            <div>
                <div>
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
            <br/>
            <div style={format!("display: {}", self.state)}>
                <h4>{format!("New Game: {} Vs {}", self.player1_name, self.player2_name)}</h4>
                <small>{format!("(Winning Combination: {} - ", self.player1_name)} <b>{"TOOT"}</b> {format!("   and    {} - ", self.player2_name)} <b>{"OTTO)"}</b></small>
                <br/>
                {"Select a Disc Type:  "}
                <input ref={self.letter_input.clone()} onclick = {link.callback(|_| Msg::InputChanged)} type="radio" name="letter" value="T" checked={self.letter == "T".to_string()}/>
                <label for="T">{"T"}</label>
                <input ref={self.letter_input.clone()} onclick = {link.callback(|_| Msg::InputChanged)} type="radio" name="letter" value="O" checked={self.letter == "O".to_string()}/>
                <label for="O">{"O"}</label>
                <h4>{format!("Letter is {}",self.letter)}</h4>
                <br/>
                <br/>
                /*<TootCanvasModel: 
                    canvas_id="toot_human" 
                    player1 = self.player1.value.clone(), 
                    player2=self.player2.value.clone(),
                    difficulty = Easy,
                    letter=self.letter.clone(), 
                    game_done_cbk=&self.end_game_callback/>*/
            </div>
            <br/>
            </>
        }
    }
}        