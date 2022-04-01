use yew::prelude::*;
use yew::virtual_dom::VNode;
//use web_sys::InputEvent;
//use web_sys::DataTransfer;
//use yew_stdweb::events::ChangeData;
//use web_sys::Event;
//use yew::virtual_dom::ListenerKind;
use yew::{html, Component, Context, Html, NodeRef};
use web_sys::HtmlInputElement;



pub enum Msg {
    InsertName(String),
    ChooseDifficulty(String),
    StartGame,
    EndGame,
    ChooseLetter(String),
}


pub struct TootOttoSide {
    player_name: String,
    difficulty: String,
    letter: String,
    disabled: bool,
    game_running: bool,
    state: String,
    
    my_input: NodeRef,
    name_input: NodeRef,
    letter_input: NodeRef,
}


impl Component for TootOttoSide {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TootOttoSide{
            player_name: "".to_string(),
            difficulty: "Easy".to_string(),
            letter: "".to_string(),
            disabled: false,
            game_running: false,
            state: "none".to_string(),
        
            my_input: NodeRef::default(),
            name_input: NodeRef::default(),
            letter_input: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InsertName(name) => {
                self.player_name = name;
        
            },
            Msg::ChooseDifficulty(difficulty) => {
                self.difficulty = difficulty;
            },
            Msg::StartGame => {
                self.game_running = true;
                self.disabled = true;
                self.state = "block".to_string();
            },
            Msg::EndGame => {
                self.game_running = false;
                self.disabled = false;
                self.state = "none".to_string();
            },
            Msg::ChooseLetter(letter) => {
                self.letter = letter;
            },
        }    
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let my_input_ref = self.my_input.clone();
        let name_input_ref = self.name_input.clone();
        let letter_input_ref = self.letter_input.clone();

        let onchange = link.batch_callback(move |_| {
            let input = my_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::ChooseDifficulty(input.value()))
        });

        let oninput = link.batch_callback(move |_| {
            let input = name_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::InsertName(input.value()))
        });

        let onselect = link.batch_callback(move |_| {
            let input = letter_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::ChooseLetter(input.value()))
        });
        let ontoggle = onselect.clone();
        
        return html! {
            <>
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
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
                //
                //Just to see difficulty is updating
                <h4>{format!("Difficulty: {}", self.difficulty)}</h4>
                //
                <small>{format!("(Winning Combination: {} - ", self.player_name)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
                {" Select a Disc Type:  "}
                <input ref={self.letter_input.clone()}{onselect} type="radio" id="T" value="T" checked={self.letter=="T"}/>
                <label for="T">{"T"}</label>
                <input ref={self.letter_input.clone()}{ontoggle} type="radio" id="O" value="O" checked={self.letter=="O"}/>
                <label for="O">{"O"}</label>
                <br/>
                <br/>
                /*<CanvasModel  
                    canvas_id = "connect_computer" 
                    player1 = self.player_name.clone()
                    player2 = "Computer" 
                    difficulty = self.difficulty
                    letter=self.letter.clone()
                    game_done_cbk={link.callback(|_| Msg::EndGame)}/>*/
            </div>
            </>
        }
    }
}