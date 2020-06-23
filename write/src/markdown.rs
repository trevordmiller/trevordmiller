use pulldown_cmark::{html, Options, Parser};

pub fn to_html(markdown: &str, title: &str, description: &str) -> std::string::String {
    let css = r#"
        body {
            max-width: 80ch;
            margin: 0 auto;
            padding: 1rem 1rem 2rem 1rem;
            font-family: "Helvetica Neue", Helvetica, Arial, Verdana, sans-serif;
            color: #333333;
        }
        code {
            overflow: auto;
            display: block;
            background: #f5f5f5;
        }
        nav a, li, code {
            padding: 0.5rem;
        }
        header, code {
            font-family: "Courier New", Courier, monospace;
        }
        main {
            margin-top: 1rem;
            border-top: 1px solid #d3d3d3;
        }
        header, nav {
            display: flex;
            align-items: center;
            flex-wrap: wrap;
        }
        header::before {
            content: ">";
            margin: 0 1ch 0 0;
        }
        header::after {
            content: "";
            margin: 0 0 0 1ch;
            display: inline-block;
            background: #333333;
            width: 1ch;
            height: 1em;
            animation: cursor 800ms infinite;
        }
        @keyframes cursor {
            0% {
                opacity: 0;
            }
            50% {
                opacity: 1;
            }
            to {
                opacity: 0;
            }
        }
    "#;

    let parser = Parser::new_ext(markdown, Options::empty());
    let mut html = String::new();
    html::push_html(&mut html, parser);

    format!("
        <!DOCTYPE html>
        <html lang='en-US'>
            <head>
                <title>trevordmiller | {}</title>
                <meta name='description' content='{}'>
                <meta name='author' content='Trevor D. Miller'>
                <meta charset='utf-8'>
                <meta name='viewport' content='width=device-width, initial-scale=1'>
                <style type='text/css'>
                    {}
                </style>
                <link rel='alternate' type='application/rss+xml' title='trevordmiller RSS feed' href='/rss.xml'>
            </head>
            <body>
                <header>trevordmiller</header>
                <nav>
                    <a href='/'>Articles</a>
                    <a href='/about/'>About</a>
                    <a href='/resume/'>Resume</a>
                    <a href='/projects/'>Projects</a>
                    <a href='/rss.xml'>RSS</a>
                </nav>
                <main>
                    {}
                </main>
            </body>
        </html>
    ",
        &title,
        &description,
        &css,
        &html
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_html_documents() {
        let markdown_file_contents = "- Some list item.";
        let title = "Some title";
        let description = "Some description.";

        let html_document = to_html(&markdown_file_contents, &title, &description);

        assert_eq!(
            html_document.contains("<title>trevordmiller | Some title</title>"),
            true
        );
        assert_eq!(
            html_document.contains("<meta name='description' content='Some description.'>"),
            true
        );
        assert_eq!(
            html_document.contains("font-family: \"Courier New\", Courier, monospace;"),
            true
        );
        assert_eq!(html_document.contains("<li>Some list item.</li>"), true);
    }
}
