use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app)
}

fn app(cx: Scope) -> Element {
    let routes = cx.use_hook(|_| Segment {
        index: RcComponent(Home),
        fixed: vec![
            (
                String::from("blog"),
                Route {
                    content: RcComponent(Blog),
                    sub: Some(Segment {
                        index: RcComponent(BlogList),
                        dynamic: DrParameter {
                            name: Some("blog_post"),
                            key: "post_id",
                            content: RcComponent(BlogPost),
                            sub: None,
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            (
                String::from("myblog"),
                Route {
                    content: RcRedirect(ItPath(String::from("/blog"))),
                    ..Default::default()
                },
            ),
        ],
        ..Default::default()
    });

    cx.render(rsx! {
        Router {
            routes: routes,
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
