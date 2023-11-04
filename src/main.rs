#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;

// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                BlogList {},
                #[route("/post/:name")]
                BlogPost { name: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogList {})]
        #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
// ANCHOR_END: router

pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::BlogList {}, "Blog" } }
            }
        }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to KevKevDev!" }
    }
}

#[inline_props]
fn Blog(cx: Scope) -> Element {
    render! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}

#[inline_props]
fn BlogList(cx: Scope) -> Element {
    render! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::BlogPost { name: "Blog post 1".into() },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost { name: "Blog post 2".into() },
                    "Read the second blog post"
                }
            }
        }
    }
}

#[inline_props]
fn BlogPost(cx: Scope, name: String) -> Element {
    render! {
        h2 { "Blog Post: {name}"}
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

se axum_desktop::*;
use dioxus_fullstack::prelude::server_fn::set_server_url;

fn main() {
    // Set the url of the server where server functions are hosted.
    set_server_url("http://127.0.0.1:6060");

    LaunchBuilder::new(App).launch();
}

