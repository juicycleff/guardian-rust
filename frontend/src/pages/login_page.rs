use yew::prelude::*;

pub struct LoginPage;

pub enum LoginPageMessage {
    AddOne,
}

#[derive(Properties, Clone)]
pub struct LoginPageProps {}

impl Component for LoginPage {
    type Message = ();
    type Properties = LoginPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{"Login Page"}</p>
            </div>
        }
    }
}
