use std::borrow::Borrow;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct NotFoundPageProps {
    pub route: Option<String>,
}

pub enum NotFoundPageMessage {}

pub struct NotFoundPage {
    props: NotFoundPageProps,
}

impl Component for NotFoundPage {
    type Message = NotFoundPageMessage;
    type Properties = NotFoundPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        todo!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.ne(props.borrow())
    }

    fn view(&self) -> Html {
        html! {
            <section class="hero is-danger is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "Page not found" }
                        </h1>
                        <h2 class="subtitle">
                            { "This page does not seem to exist" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}
