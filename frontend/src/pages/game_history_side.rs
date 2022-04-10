use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use yew::prelude::*;
use yew::virtual_dom::VNode;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use stdweb::web::Date;
use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Game {
    pub GameDate: i64,
    pub gameType: String,
    pub gameNumber: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
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



pub enum Msg {
    GetOK(Vec<Game>),
    GetFailed(String),
}

pub struct GameHistorySide {
    data: Option<Vec<Game>>,
}

impl GameHistorySide {
    fn view_data(&self) -> Html {

        if let Some(ref games) = self.data {
            gloo::console::log!(&format!("Size: {}", games.len()));
            html!{
                { games.iter().enumerate().map(|(i, game)| {
                        // Create a NaiveDateTime from the timestamp
                        let naive = NaiveDateTime::from_timestamp(game.GameDate / 1000, ((game.GameDate % 1000) as u32) * 1000000 );
                        
                        // Create a normal DateTime from the NaiveDateTime
                        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                        
                        // Format the datetime how you want
                        let newdate = datetime.format("%Y-%m-%d %H:%M:%S UTC");

                        html! {
                            <tr>
                            <td>{i + 1}</td>
                            <td>{game.gameType.as_str()}</td>
                            <td>{game.Player1Name.as_str()}</td>
                            <td>{game.Player2Name.as_str()}</td>
                            <td>{game.WinnerName.as_str()}</td>
                            <td>{newdate}</td>
                            </tr>
                        }
                    }).collect::<Html>() }
            }
        }
        else {
            html! {
                <tr><td colspan="6">{"Loading..."}</td></tr>
            }
        }
    }
}

pub async fn get_game_data() -> Result<Vec<Game>, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("/games/game_data", &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let game_data: Vec<Game> = json.into_serde().unwrap();

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(game_data)
}

impl Component for GameHistorySide {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_future(async {
            match get_game_data().await {
                Ok(game_data) => {
                    Msg::GetOK(game_data)
                },
                Err(err) => {
                    Msg::GetFailed(err.to_string())
                }
            }
        });

        GameHistorySide{
            data: None,
        }

    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // true
        match msg {
            Msg::GetOK(game_data) => {
                gloo::console::log!("Get data success!");

                for game in &game_data {
                    gloo::console::log!(&format!("Date: {}", game.GameDate));
                }

                self.data = Some(game_data);
                true
            },
            Msg::GetFailed(err) => {
                gloo::console::log!(&format!("{:?}", &err));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        gloo::console::log!("77777777777777");

        let link = ctx.link();
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            <div id="game-stream">
            <table border=1>
                <tr>
                    <th>{"Game-ID"}</th>
                    <th>{"Game Type"}</th>
                    <th>{"Player1"}</th>
                    <th>{"Player2"}</th>
                    <th>{"Winner"}</th>
                    <th>{"When Played"}</th>
                </tr>
                { self.view_data() }
            </table>
            </div>
            </div>
        }
    }
}        