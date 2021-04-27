use yew::prelude::*;

#[derive(Properties, Clone, Debug)]
pub struct LayoutComponentProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct LayoutComponent {
    props: LayoutComponentProps,
}

pub enum LayoutComponentMessage {}

impl Component for LayoutComponent {
    type Message = LayoutComponentMessage;
    type Properties = LayoutComponentProps;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            <main>
                { self.props.children.clone() }
            </main>
        }
    }
}
