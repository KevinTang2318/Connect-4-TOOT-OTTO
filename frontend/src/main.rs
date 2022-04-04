
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;

mod pages;
use pages::fullpage::{welcome::Main, connect_4::Connect4, how_to_c4::HowToC4, toot_otto::TootOtto,
                      connect_4_human::Connect4Human, toot_otto_human::TootOttoHuman, how_to_toot::HowToToot,
                      game_history::GameHistory, scores::Scores};



pub enum Msg {
    
}

pub struct AppRouter {
    
}

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Main,

    #[at("/HowToConnect4")]
    HowToC4,
    #[at("/Connect4Computer")]
    Connect4,
    #[at("/Connect4Human")]
    Connect4Human,

    #[at("/HowToToot")]
    HowToToot,
    #[at("/TootOttoComputer")]
    TootOtto,
    #[at("/TootOttoHuman")]
    TootOttoHuman,

    #[at("/GameHistory")]
    GameHistory,
    #[at("/Scores")]
    Scores,
   

}


impl Component for AppRouter {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AppRouter{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
        
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
        
    }
}



pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Main => { html! { <Main/> } },

        AppRoute::HowToC4 => { html! { <HowToC4/> } },
        AppRoute::Connect4 => { html! { <Connect4/> } },
        AppRoute::Connect4Human => { html! { <Connect4Human/> } },

        AppRoute::HowToToot => { html! { <HowToToot/> } },
        AppRoute::TootOtto => { html! { <TootOtto/> } },
        AppRoute::TootOttoHuman => { html! { <TootOttoHuman/> } },

        AppRoute::GameHistory => { html! { <GameHistory/> } },
        AppRoute::Scores => { html! { <Scores/> } },

        _ => {html! { <Main/> } },
    }
}        

fn main() {
    yew::start_app::<AppRouter>();
}



