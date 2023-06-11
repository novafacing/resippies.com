use markup::{define, new, Render};

use crate::traits::RenderData;

define! {
    Layout<'a, R, Head: Render, Main: Render>(
        data: &'a R,
        head: Head,
        main: Main,
    ) where R: RenderData {
        @markup::doctype()
        html {
            head {
                @head
                @Scripts {}
                @Style {}
                meta[charset = "utf-8"];
                title { "resippies.com" }
            }
            body {
                div
                .flex {
                    @Sidebar { data }
                    main {
                        @main
                    }
                }

            }
        }
    }
    Sidebar<'a, R>(
        data: &'a R,
    ) where R: RenderData {
        aside
        ."h-screen"
        .sticky
        ."top-0"
        ."bg-background-light-100"
        ."text-foreground-light-500"
        ["aria-label" = "sidenav"] {
            div
            .flex
            ."overflow-y-auto"
            ."py-2"
            ."px-3"
            ."h-full"
            ."border-r"
            ."border-foreground-light-400" {
                ul
                ."font-normal"
                ."divide-y"
                ."divide-foreground-light-100" {
                    @if let Some(user) = data.user() {
                        li {
                            a
                            .flex
                            ."items-center"
                            ."p-3"
                            ."font-normal"
                            ."hover:border-b"
                            ."hover:border-foreground-light-500"
                            .group
                            [href=format!("/user/{}", user.id)] {
                                @user.username
                            }
                        }
                        li {
                            a
                            .flex
                            ."items-center"
                            ."p-3"
                            ."font-normal"
                            ."hover:border-b"
                            ."hover:border-foreground-light-500"
                            .group
                            [href="/logout"] {
                                "Log Out"
                            }
                        }

                    } else {
                        li {
                            a
                            .flex
                            ."items-center"
                            ."p-3"
                            ."font-normal"
                            ."hover:border-b"
                            ."hover:border-foreground-light-500"
                            .group
                            [href="/login"] {
                                "Login"
                            }
                        }
                        li {
                            a
                            .flex
                            ."items-center"
                            ."p-3"
                            ."font-normal"
                            ."hover:border-t"
                            ."hover:border-foreground-light-500"
                            .group
                            [href="/register"] {
                                "Register"
                            }
                        }
                    }
                }

            }
        }
    }
}

#[cfg(debug_assertions)]
define! {
    Scripts {
        script[src = "/static/_hyperscript.js"] {}
        script[src = "/static/htmx.js"] {}
    }
}

#[cfg(debug_assertions)]
define! {
    Style {
        link[rel = "stylesheet", href = "/static/style.css"] {}
    }
}

#[cfg(not(debug_assertions))]
define! {
    Scripts {
        script[src = "/static/_hyperscript.min.js"] {}
        script[src = "/static/htmx.min.js"] {}
    }

}

#[cfg(not(debug_assertions))]
define! {
    Style {
        link[rel = "stylesheet", href = "/static/style.min.css"] {}
    }
}
