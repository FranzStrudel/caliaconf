use std::{convert::TryFrom, fmt};

use seed::prelude::*;

use crate::GMsg;

pub fn go_to<Ms: 'static>(route: Route, orders: &mut impl Orders<Ms, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}

// ------ Route ------

#[derive(Clone, Debug)]
pub enum Route {
    Home,
    Root,
    Settings,
}

impl Route {
    pub fn path(&self) -> Vec<&str> {
        use Route::*;
        match self {
            Home | Root => vec![],
            Settings => vec!["settings"],
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl From<Route> for seed::Url {
    fn from(route: Route) -> Self {
        route.path().into()
    }
}

impl TryFrom<seed::Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Some(Route::Home),
            Some("settings") => Some(Route::Settings),
            _ => None,
        }
        .ok_or(())
    }
}
