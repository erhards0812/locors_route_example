#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

pub async fn hello(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    crate::views::home::home(v)
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(hello))
}
