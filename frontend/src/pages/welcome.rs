use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;
//pub mod main_side;
use  super::welcome_side::MainSide;
use crate::AppRoute;


pub enum Msg {
    
}

pub struct Main {
   
}




impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Main{}
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        
        html! {
            <>
            <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav">
                    <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">{"Close Menu"}</a>
                    <div class="w3-container">
                        <h3 class="w3-padding-64"><b>{"Play"}<br/> {"Connect4 / TOOT-OTTO"}</b></h3>
                    </div>
                    <Link<AppRoute> to={AppRoute::HowToC4}>{ "How to Play Connect4" }</Link<AppRoute>>
                    <Link<AppRoute> to={AppRoute::Connect4}>{ "Play Connect4 With Computer" }</Link<AppRoute>>
                
                </nav>
                <header class="w3-container w3-top w3-hide-large w3-red w3-xlarge w3-padding">
                <a href="javascript:void(0)" class="w3-btn w3-red w3-border w3-border-white w3-margin-right">{"\u{2630}"}</a>
                <span>{"Connect 4 with MEAN"}</span>
                </header>
                <div class="w3-overlay w3-hide-large" style="cursor:pointer" title="close side menu" id="myOverlay"></div>
                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                
            {
                html !{
                    <MainSide/>
                }
            }
            </div>

            </>
        }
    }
}

