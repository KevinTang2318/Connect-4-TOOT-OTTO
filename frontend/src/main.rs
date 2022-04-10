
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;

mod sidebar;
use sidebar::SideBar;
mod pages;
use crate::pages::{welcome_side::MainSide, connect_4_side::Connect4Side, how_to_c4_side::HowToC4Side, toot_otto_side::TootOttoSide,
    connect_4_human_side::Connect4HumanSide, toot_otto_human_side::TootOttoHumanSide, how_to_toot_side::HowToTootSide,
    game_history_side::GameHistorySide, scores_side::ScoresSide};


pub enum Msg {
    
}

pub struct AppRouter {
    
}

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Main,

    #[at("/HowToConnect4")]
    HowToC4Side,
    #[at("/Connect4Computer")]
    Connect4Side,
    #[at("/Connect4Human")]
    Connect4HumanSide,

    #[at("/HowToToot")]
    HowToTootSide,
    #[at("/TootOttoComputer")]
    TootOttoSide,
    #[at("/TootOttoHuman")]
    TootOttoHumanSide,

    #[at("/GameHistory")]
    GameHistorySide,
    #[at("/Scores")]
    ScoresSide,
   

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
                <>
                    <SideBar/>
                    {
                        html !{
                            <div class="w3-main" style="margin-left:390px;margin-right:40px">
                                <Switch<AppRoute> render={Switch::render(switch)} />
                            </div>
                        }
                    }    
                </>
                </main>
            </BrowserRouter> 
        }
        
    }
}



pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Main => { html! { <MainSide/> } },

        AppRoute::HowToC4Side => { html! { <HowToC4Side/> } },
        AppRoute::Connect4Side => { html! { <Connect4Side/> } },
        AppRoute::Connect4HumanSide => { html! { <Connect4HumanSide/> } },

        AppRoute::HowToTootSide => { html! { <HowToTootSide/> } },
        AppRoute::TootOttoSide => { html! { <TootOttoSide/> } },
        AppRoute::TootOttoHumanSide => { html! { <TootOttoHumanSide/> } },

        AppRoute::GameHistorySide => { html! { <GameHistorySide/> } },
        AppRoute::ScoresSide => { html! { <ScoresSide/> } },

        _ => {html! { <MainSide/> } },
    }
}        

fn main() {
    yew::start_app::<AppRouter>();
}



