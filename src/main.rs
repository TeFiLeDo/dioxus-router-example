use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(App)
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let routes = use_segment(&cx, || {
        Segment::default()
            .index(Home as Component)
            .fixed(
                "blog",
                Route::new(Blog as Component).nested(
                    Segment::default().index(BlogList as Component).parameter(
                        ParameterRoute::new("post_id", BlogPost as Component).name("blog_post"),
                    ),
                ),
            )
            .fixed("myblog", "/blog")
    });

    cx.render(rsx! {
        Router {
            routes: routes.clone(),
            fallback: RcComponent(PageNotFound),
            active_class: "active"
            NavBar {}
            Outlet {}
        }
    })
}

#[allow(non_snake_case)]
fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            ul {
                li {
                    Link {
                        target: NtName("", vec![], QNone),
                        "Home"
                    }
                }
                li {
                    Link {
                        target: "/blog".into(),
                        "Blog"
                    }
                }
            }
        }
    })
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Welcome to the Dioxus Blog!" }
    })
}

#[allow(non_snake_case)]
fn Blog(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Blog" }
        Outlet {}
    })
}

#[allow(non_snake_case)]
fn BlogList(cx: Scope) -> Element {
    cx.render(rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    target: NtName("blog_post", vec![("post_id", String::from("1"))], QNone),
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    target: NtName("blog_post", vec![("post_id", String::from("2"))], QNone),
                    "Read the second blog post"
                }
            }
        }
    })
}

#[allow(non_snake_case)]
fn BlogPost(cx: Scope) -> Element {
    let route = use_route(&cx).unwrap();

    let post_id = route.parameters.get("post_id");
    let post = post_id
        .map(|id| id.to_string())
        .unwrap_or(String::from("unknown"));

    cx.render(rsx! {
        h2 { "Blog Post: {post}"}
    })
}

#[allow(non_snake_case)]
fn PageNotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
    })
}
