use yew::prelude::*;
use yew::virtual_dom::VNode;


pub enum Msg {
    
}

pub struct GameHistorySide {
    
}


impl Component for GameHistorySide {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        GameHistorySide{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            <div id="game-stream">
            <table>
                <tr>
                    <th>{"Game-ID"}</th>
                    <th>{"Game Type"}</th>
                    <th>{"Player1"}</th>
                    <th>{"Player2"}</th>
                    <th>{"Winner"}</th>
                    <th>{"When Played"}</th>
                </tr>
                //{ self.view_data() }
            </table>
            </div>
            </div>
        }
    }
}        