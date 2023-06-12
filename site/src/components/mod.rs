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
                    main
                    ."flex-auto"
                    ."h-full"
                    ."min-h-screen"
                    ."w-full"
                    ."bg-background-light-500" {
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
        ."bg-background-light-200"
        ."text-foreground-light-500"
        ["aria-label" = "sidenav"] {
            div
            .flex
            ."flex-col"
            ."overflow-y-auto"
            ."py-2"
            ."px-3"
            ."h-full"
            ."border-r"
            ."border-foreground-light-400" {
                ul
                ."divide-y"
                ."divide-foreground-light-500" {
                    @if let Some(user) = data.user() {
                        li {
                            a
                            .flex
                            ."items-center"
                            ."p-3"
                            ."font-normal"
                            ."hover:font-semibold"
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
                            ."hover:font-semibold"
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
                            ."hover:font-semibold"
                            ."hover:border-b"
                            ."hover:border-foreground-light-500"
                            .group
                            [href="/login"] {
                                "Log In"
                            }
                        }
                        li {
                            a
                            .flex
                            ."items-center"
                            ."p-3"
                            ."font-normal"
                            ."hover:font-semibold"
                            ."hover:border-t"
                            ."hover:border-foreground-light-500"
                            .group
                            [href="/register"] {
                                "Register"
                            }
                        }
                    }
                }
                div
                .absolute
                ."bottom-0"
                ."left-0"
                ."right-0" {
                    div
                    .flex
                    ."flex-col"
                    ."items-center"
                    ."justify-center" {
                        label
                        .relative
                        ."inline-flex"
                        ."items-center"
                        ."cursor-pointer"
                        ."mb-3"
                        ."ml-3"
                        ."mr-3" {
                            input
                            #"theme-toggle"
                            ."sr-only"
                            ."peer"
                            [type = "checkbox", value = ""] {
                            }
                            div
                            ."w-10"
                            ."h-5"
                            ."peer-focus:outline-none"
                            ."rounded-full"
                            ."peer"
                            ."peer-checked:after:translate-x-5"
                            ."after:absolute"
                            ."after:border"
                            ."after:rounded-full"
                            ."after:inset-0"
                            ."after:w-5"
                            ."after:h-5"
                            ."after:transition-all"
                            ."after:bg-foreground-dark-100"
                            ."after:border-background-dark-100"
                            ."bg-foreground-dark-100"
                            ."border-background-dark-100"
                            ."peer-checked:after:border-foreground-dark-100"
                            ."peer-checked:bg-background-dark-100"
                            ."peer-checked:border-foreground-dark-100" {

                            }
                            div
                            .visible
                            ."peer-checked:invisible"
                            ."text-foreground-light-500"
                            ."fa-solid"
                            ."fa-sun"
                            {}
                            div
                            .invisible
                            ."peer-checked:visible"
                            ."text-foreground-dark-500"
                            ."fa-solid"
                            ."fa-moon"
                            {}
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
        script[src = "/static/fontawesome-free-6.4.0-web/js/all.js"] {}
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
        script[src = "/static/fontawesome-free-6.4.0-web/js/fontawesome.min.js"] {}
        script[src = "/static/fontawesome-free-6.4.0-web/js/solid.js"] {}
    }

}

#[cfg(not(debug_assertions))]
define! {
    Style {
        link[rel = "stylesheet", href = "/static/style.min.css"] {}
    }
}

pub enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}
