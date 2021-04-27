use yew::prelude::*;

pub struct RegisterPage {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum RegisterPageMessage {
    AddOne,
}

#[derive(Properties, Clone)]
pub struct RegisterPageProps {}

impl Component for RegisterPage {
    type Message = RegisterPageMessage;
    type Properties = RegisterPageProps;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RegisterPageMessage::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{"Register Page"}</p>
                <button onclick=self.link.callback(|_| RegisterPageMessage::AddOne)>{ "+hello" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
