
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;

mod pages;
use pages::{welcome::Main, connect_4::Connect4, how_to_c4::HowToC4, toot_otto::TootOtto};


pub enum Msg {
    
}

pub struct AppRouter {
    
}

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Main,
    #[at("/Connect4Computer")]
    Connect4,
    #[at("/HowToConnect4")]
    HowToC4,
    #[at("/TootOttoComputer")]
    TootOtto,

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
        AppRoute::Connect4 => { html! { <Connect4/> } },
        AppRoute::HowToC4 => { html! { <HowToC4/> } },
        AppRoute::TootOtto => { html! { <TootOtto/> } },
        _ => {html! { <Main/> } },
    }
}        

fn main() {
    yew::start_app::<AppRouter>();
}



