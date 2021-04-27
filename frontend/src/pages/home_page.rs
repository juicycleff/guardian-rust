use yew::prelude::*;

pub struct HomePage;

pub enum HomePageMessage {
    AddOne,
}

#[derive(Properties, Clone)]
pub struct HomePageProps {}

impl Component for HomePage {
    type Message = ();
    type Properties = HomePageProps;

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
                <p>{"Home Page"}</p>
            </div>
        }
    }
}
