use yew::prelude::*;

pub struct Counter {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
    RemoveOne,
}

impl Component for Counter {

    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::RemoveOne => {
                if self.value <= 0 {
                    return false;
                }
                self.value -= 1;
                return true;
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|e| Msg::RemoveOne)>{ "-1" }</button>
                <button onclick=self.link.callback(|e| Msg::AddOne)>{ "+1" }</button>
                <main>{ self.value }</main>
            </div>
        }
    }
}
