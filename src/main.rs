use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app)
}

fn app(cx: Scope) -> Element {
    let routes = use_segment(&cx, || {
        Segment::default()
            .index(RcComponent(Home))
            .fixed(
                "blog",
                Route::new(RcComponent(Blog)).nested(
                    Segment::default().index(RcComponent(BlogList)).parameter(
                        ParameterRoute::new("post_id", RcComponent(BlogPost)).name("blog_post"),
                    ),
                ),
            )
            .fixed(
                "myblog",
                Route::new(RcRedirect(NtPath(String::from("/blog")))),
            )
    });

    cx.render(rsx! {
        Router {
            routes: routes.clone(),
            fallback: RcComponent(PageNotFound),
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
                        target: NtName("root_index", vec![], QNone),
                        "Home"
                    }
                }
                li {
                    Link {
                        target: NtPath(String::from("/blog")),
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
