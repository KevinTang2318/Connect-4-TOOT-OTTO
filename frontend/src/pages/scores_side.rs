use yew::prelude::*;
use yew::virtual_dom::VNode;


pub enum Msg {
    
}

pub struct ScoresSide {
    
}


impl Component for ScoresSide {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ScoresSide{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            <div><h4>{"Games Won by Computer"}</h4></div>
                <table>
                    <tr>
                        <th>{"Total Games Played"}</th>
                        <th>{"Games Against Computer"}</th>
                        <th>{"Games Computer Won"}</th>
                    </tr>
                    //{ self.view_total_games() }
                </table>
            <br/>
            <div><h4>{"Details of Games Won by Computer"}</h4></div>
                <div id="game-stream">
                <table>
                    <tr>
                        <th>{"Sl. No."}</th>
                        <th>{"Game Type"}</th>
                        <th>{"Winner"}</th>
                        <th>{"Played Against"}</th>
                        <th>{"When Played"}</th>
                    </tr>
                    //{ self.view_computer_wins() }
                 </table>
            </div>
            <br/>
            <div><h4>{"Details of Games Won by All Players"}</h4></div>
            <div id="game-stream">
                <table>
                    <tr>
                        <th>{"Sl. No."}</th>
                        <th>{"Winner or Draw"}</th>
                        <th>{"No. of Wins"}</th>
                    </tr>
                    //{ self.view_total_wins() }
                </table>
            </div>
            </div>
        }
    }
}        