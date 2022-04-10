use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;

//pub mod router;
use crate::AppRoute;
use crate::AppRouter;


pub enum Msg {
    
}

pub struct SideBar {
    
}




impl Component for SideBar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SideBar{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
        
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        html!{
                <>
                <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav">
                        <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">{"Close Menu"}</a>
                        <div class="w3-container">
                            <h3 class="w3-padding-64"><b>{"Play"}<br/> {"Connect4 / TOOT-OTTO"}</b></h3>
                        </div>

                        <Link<AppRoute> to={AppRoute::HowToC4Side}>{ "How to Play Connect4" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Connect4Side}>{ "Play Connect4 With Computer" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Connect4HumanSide}>{ "Play Connect4 with another Human" }</Link<AppRoute>>
                        <br/>
                        <Link<AppRoute> to={AppRoute::HowToTootSide}>{ "How to Play TOOT-OTTO" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::TootOttoSide}>{ "Play TOOT-OTTO With Computer" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::TootOttoHumanSide}>{ "Play TOOT-OTTO With another Human" }</Link<AppRoute>>
                        <br/>
                        <Link<AppRoute> to={AppRoute::GameHistorySide}>{ "View Game History" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::ScoreBoardSide}>{ "Score Board" }</Link<AppRoute>>
 
                    </nav>
                    <header class="w3-container w3-top w3-hide-large w3-red w3-xlarge w3-padding">
                    <a href="javascript:void(0)" class="w3-btn w3-red w3-border w3-border-white w3-margin-right">{"\u{2630}"}</a>
                    <span>{"Connect 4 with MEAN"}</span>
                    </header>
                    <div class="w3-overlay w3-hide-large" style="cursor:pointer" title="close side menu" id="myOverlay"></div>
                    </>                 
                
        }    
        
    }

}