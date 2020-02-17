use crate::{entity::ErrorMessage, Route};
use seed::{prelude::*, *};
use std::borrow::Cow;

pub mod blank;
pub mod home;
pub mod not_found;
pub mod settings;

pub fn scroll_to_top() {
    seed::window().scroll_to_with_scroll_to_options(
        web_sys::ScrollToOptions::new()
            .top(0.)
            .left(0.)
            .behavior(web_sys::ScrollBehavior::Smooth),
    )
}

pub fn view_errors<Ms: Clone>(dismiss_errors: Ms, errors: &[ErrorMessage]) -> Node<Ms> {
    if errors.is_empty() {
        empty![]
    } else {
        div![
            class!["error-messages"],
            style! {
                "position" => "fixed",
                "top" => 0,
                "background" => "rgb(250, 250, 250)",
                "padding" => "20px",
                "border" => "1px solid",
                "z-index" => 9999,
            },
            errors.iter().map(|error| p![error]),
            button![simple_ev(Ev::Click, dismiss_errors), "Ok"]
        ]
    }
}

// ------ ViewPage ------

#[allow(clippy::module_name_repetitions)]
pub struct ViewPage<'a, Ms: 'static> {
    title_prefix: Cow<'a, str>,
    content: Node<Ms>,
}

impl<'a, Ms> ViewPage<'a, Ms> {
    pub fn new(title_prefix: impl Into<Cow<'a, str>>, content: Node<Ms>) -> Self {
        Self {
            title_prefix: title_prefix.into(),
            content,
        }
    }
    pub fn title(&self) -> String {
        format!("{} - Caliaconf", self.title_prefix)
    }
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_content(self) -> Node<Ms> {
        self.content
    }
}

// ------ Page ------

pub enum Page {
    Other,
    Home,
    Settings,
}

#[allow(clippy::unused_self)]
impl Page {
    fn is_active(&self, route: &Route) -> bool {
        match (self, route) {
            (Page::Home, Route::Home) | (Page::Settings, Route::Settings) => true,
            _ => false,
        }
    }

    // ------ view methods ------

    pub fn view<Ms>(&self, view_page: ViewPage<Ms>) -> Vec<Node<Ms>> {
        seed::document().set_title(&view_page.title());

        self.view_header()
            .into_iter()
            .chain(vec![view_page.into_content(), self.view_footer()])
            .collect()
    }

    // ====== PRIVATE ======

    fn view_header<Ms>(&self) -> Vec<Node<Ms>> {
        vec![
            nav![
                class!["navbar"],
                div![
                    class!["navbar-brand"],
                    a![
                        class!["navbar-item"],
                        attrs! {At::Href => Route::Home.to_string()},
                        img![
                            attrs! {At::Src => "https://www.caliatys.com/wp-content/uploads/2018/04/rectangleLogo-1.png"}
                        ],
                    ],
                    a![
                        attrs! {At::Custom("role".into()) => "button", At::Custom("aria-label".into()) => "menu", At::Custom("aria-expanded".into()) => "false", At::Custom("data-target".into()) => "navbar"},
                        span![attrs! {At::Custom("aria-hidden".into()) => "true"}],
                        span![attrs! {At::Custom("aria-hidden".into()) => "true"}],
                        span![attrs! {At::Custom("aria-hidden".into()) => "true"}],
                    ],
                ],
                div![
                    id!("navbar"),
                    class!["navbar-menu"],
                    div![
                        class!["navbar-start"],
                        self.view_navbar_link(&Route::Home, "Home"),
                        self.view_navbar_link(&Route::Settings, "Settings"),
                    ],
                ],
            ],
            div![
                class!["container"],
                h1![class!["title"], "CaliaConf"],
                p![class!["subtitle"], "Who will be the next"],
            ],
        ]
    }

    fn view_footer<Ms>(&self) -> Node<Ms> {
        footer![
            class!["footer"],
            div![
                class!["content has-text-centered"],
                p![
                    strong!["CaliaConf "],
                    "let's bring Rust to the front-end with ",
                    a![
                        attrs! {At::Href => "https://seed-rs.org/"},
                        "Seed"
                    ],
                    " (and ",
                    a![
                        attrs! { At::Href => "https://bulma.io/" },
                        "Bulma",
                    ],
                    ") and ",
                    a![
                        attrs! {At::Href => "https://darklang.com/"},
                        "Dark"
                    ],
                     " at the back-end to pick the next speaker"
                ]
            ]
        ]
    }

    // ------ view_header helpers ------

    fn view_navbar_link<Ms>(&self, route: &Route, link_content: impl UpdateEl<El<Ms>>) -> Node<Ms> {
        a![
            class![
              "navbar-item",
              "active" => self.is_active(route),
            ],
            attrs! {At::Href => route.to_string()},
            link_content
        ]
    }
}
