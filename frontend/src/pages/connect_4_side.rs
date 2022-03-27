use yew::prelude::*;
use yew::virtual_dom::VNode;


pub enum Msg {
    
}

pub struct Connect4Side {
    
}



impl Component for Connect4Side {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Connect4Side{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        return html! {
            <div>
            <form>
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Connect 4"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                <p>{"CONNECT4"}
                </p>
            </div>
            </form>
            </div>
        }
    }
}

