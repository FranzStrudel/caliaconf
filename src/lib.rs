use seed::prelude::*;
use std::convert::TryInto;

pub use route::Route;

mod entity;
mod loading;
mod logger;
mod page;
mod request;
mod route;

// ------ ------
//     Model
// ------ ------

enum Model {
    Redirect,
    NotFound,
    Home(page::home::Model),
    Settings(page::settings::Model),
}

impl Default for Model {
    fn default() -> Self {
        Model::Redirect
    }
}

// ------ ------
// Before Mount
// ------ ------

fn before_mount(_: Url) -> BeforeMount {
    // Since we have the "loading..." text in the app section of index.html,
    // we use MountType::Takover which will overwrite it with the seed generated html
    BeforeMount::new().mount_type(MountType::Takeover)
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(url: Url, orders: &mut impl Orders<Msg, GMsg>) -> AfterMount<Model> {
    orders.send_msg(Msg::RouteChanged(url.try_into().ok()));

    let model = Model::Redirect;
    AfterMount::new(model).url_handling(UrlHandling::None)
}

// ------ ------
//     Sink
// ------ ------

pub enum GMsg {
    RoutePushed(Route),
}

fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    if let GMsg::RoutePushed(route) = g_msg {
        orders.send_msg(Msg::RouteChanged(Some(route.clone())));
    }
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::enum_variant_names)]
enum Msg {
    RouteChanged(Option<Route>),
    HomeMsg(page::home::Msg),
    SettingsMsg(page::settings::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        }
        Msg::HomeMsg(module_msg) => {
            if let Model::Home(module_model) = model {
                page::home::update(module_msg, module_model, &mut orders.proxy(Msg::HomeMsg));
            }
        }
        Msg::SettingsMsg(module_msg) => {
            if let Model::Settings(module_model) = model {
                page::settings::update(
                    module_msg,
                    module_model,
                    &mut orders.proxy(Msg::SettingsMsg),
                );
            }
        }
    }
}

fn change_model_by_route(
    route: Option<Route>,
    model: &mut Model,
    orders: &mut impl Orders<Msg, GMsg>,
) {
    match route {
        None => *model = Model::NotFound,
        Some(route) => match route {
            Route::Root => route::go_to(Route::Home, orders),
            Route::Settings => {
                *model = Model::Settings(page::settings::init(&mut orders.proxy(Msg::SettingsMsg)));
            }
            Route::Home => {
                *model = Model::Home(page::home::init(&mut orders.proxy(Msg::HomeMsg)));
            }
        },
    };
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    use page::Page;
    match model {
        Model::Redirect => Page::Other.view(page::blank::view()),
        Model::NotFound => Page::Other.view(page::not_found::view()),
        Model::Settings(model) => Page::Settings
            .view(page::settings::view::<page::settings::Model>(model))
            .map_msg(Msg::SettingsMsg),
        Model::Home(model) => Page::Home
            .view(page::home::view::<page::home::Model>(model))
            .map_msg(Msg::HomeMsg),
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(|url| Some(Msg::RouteChanged(url.try_into().ok())))
        .sink(sink)
        .build_and_start();
}
