#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::router::Router;
use yew_router::switch::Permissive;

use crate::components::*;
use crate::pages::home_page::HomePage;
use crate::pages::login_page::LoginPage;
use crate::pages::not_found_page::NotFoundPage;
use crate::pages::register_page::RegisterPage;
use crate::routes::{AppRoute, PublicUrlSwitch};

mod components;
mod pages;
mod routes;

struct GuardianApp;

impl GuardianApp {
    fn _switch_route(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Login => {
                html! { <LoginPage />}
            }
            AppRoute::Register => {
                html! { <RegisterPage />}
            }
            AppRoute::Home => {
                html! {<HomePage/>}
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <NotFoundPage route=route /> }
            }
        }
    }
}

impl Component for GuardianApp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <layout::LayoutComponent>
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Login => html!{<LoginPage />},
                                AppRoute::Register => html!{<RegisterPage />},
                                AppRoute::Home => html!{<HomePage/>},
                                AppRoute::PageNotFound(Permissive(route)) => html! { <NotFoundPage route=route /> },
                            }
                        })
                    />
                </layout::LayoutComponent>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<GuardianApp>::new().mount_to_body();
    // yew::start_app::<GuardianApp>();
}

/*
<AppRouter
                        render=AppRouter::render(Self::switch_route)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        })
                    />
 */
