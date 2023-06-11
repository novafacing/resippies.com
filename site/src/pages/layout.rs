use markup::{define, Render};

define! {
    Layout<Head: Render, Main: Render>(
        dark: bool,
        head: Head,
        main: Main,
    ) {
        @markup::doctype()
        html {
            head {
                @head
                script[src = "/static/_hyperscript.min.js"] {}
                script[src = "/static/htmx.min.js"] {}
                link[rel = "stylesheet", href = "/static/style.css"] {}
                meta[charset = "utf-8"];
            }
            body .{if *dark { "dark" } else { "light" }} {
                aside
                .fixed
                ."top-0"
                ."left-0"
                ."h-full"
                ."bg-background-light-100"
                ."text-foreground-light-500"
                ["aria-label" = "sidenav"] {
                    div
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
                            li {
                                a
                                .flex
                                ."items-center"
                                ."p-3"
                                ."font-normal"
                                ."hover:border-b"
                                ."hover:border-foreground-light-500"
                                .group
                                [href="#"] {
                                    "Overview"
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
                                [href="#"] {
                                    "Pages"
                                }
                            }

                        }

                    }
                }
                @main
            }
        }

    }
}
