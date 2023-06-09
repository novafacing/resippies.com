use markup::{define, Render};

define! {
    Layout<Head: Render, Style: Render, Main: Render>(
        head: Head,
        style: Style,
        main: Main,
    ) {
        @markup::doctype()
        html {
            head {
                @head
                script[src = "/static/_hyperscript.min.js"] {}
                script[src = "/static/htmx.min.js"] {}
                @style
                meta[charset = "utf-8"];
            }
            body {
                @main
                h1 #title."text-3xl"."font-bold".underline."bg-sky-500"."hover:#fffd63" {
                    "Hello, world!"
                }
            }
        }

    }
}
