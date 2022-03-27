use yew::prelude::*;
use yew::virtual_dom::VNode;


pub enum Msg {
    
}

pub struct MainSide {
    
}



impl Component for MainSide {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainSide{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        return html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Welcome"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                <p>
                    {"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
                </p>
                <ul>
                    <li>{"Connect 4"}</li>
                    <li>{"TOOT-OTTO"}</li>
                </ul>
                <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
            </div>
        }
    }
}




