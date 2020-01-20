use super::paths;
use pulldown_cmark::{html, Options, Parser};
use std::fs;

// Removes any previously generated output
pub fn clean() {
    paths::remove_dir(&paths::public());
}

// Generates a static HTML bundle from markdown notes
pub fn build() {
    paths::create_dir(&paths::public());

    match fs::read_dir(&paths::notes()) {
        Ok(markdown_files) => {
            let mut markdown_links_to_routes = Vec::new();

            for markdown_file in markdown_files {
                let markdown_file = match &markdown_file {
                    Ok(markdown_file) => markdown_file.path(),
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let markdown_file_contents = match fs::read_to_string(&markdown_file) {
                    Ok(contents) => contents,
                    Err(error) => panic!("There was a problem: {:?}", error),
                };

                let route = &paths::file_stem(markdown_file);
                paths::create_dir(&paths::public().join(route));
                paths::create_file(
                    &paths::public().join(route).join("index.html"),
                    &markdown_to_html(&markdown_file_contents),
                );

                let title = match markdown_file_contents.lines().next() {
                    Some(title) => &title[2..],
                    None => panic!("Cannot find a title in {}.", &route),
                };
                markdown_links_to_routes.push(format!("- [{}](/{})", title, route).to_string());
            }

            let markdown_home = markdown_links_to_routes.join("\n");

            paths::create_file(
                &paths::public().join("index.html"),
                &markdown_to_html(&markdown_home),
            );
        }
        Err(error) => panic!("There was a problem: {:?}", error),
    };
}

// Adds a CNAME file for the host (GitHub Pages) and registrar (Hover) to use my custom domain name (trevordmiller.com)
pub fn configure() {
    paths::create_file(&paths::cname(), "trevordmiller.com");
}

// Pushes output to my GitHub Pages repo (https://github.com/trevordmiller/trevordmiller.github.io)
pub fn deploy() {}

fn markdown_to_html(markdown: &str) -> std::string::String {
    let parser = Parser::new_ext(markdown, Options::empty());
    let mut html = String::new();
    html::push_html(&mut html, parser);

    format!(
        "
        <!DOCTYPE html>
        <html lang=\"en-US\">
            <head>
                <meta charset=\"utf-8\">
                <meta name=\"author\" content=\"Trevor D. Miller\">
                <meta name=\"description\" content=\"Personal website.\">
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
                <title>trevordmiller.com</title>
            </head>
            <body>
                <header>
                    <nav>
                        <a href=\"/\">trevordmiller.com</a>
                    </nav>
                </header>
                <main>
                    <article>
                        {}
                    </article>
                </main>
                <hr />
                <footer>
                    <nav>
                        <ul>
                            <li><a href=\"https://github.com/trevordmiller\">Code</a></li>
                            <li><a href=\"https://www.linkedin.com/in/trevordmiller\">Resume</a></li>
                            <li><a href=\"https://egghead.io/instructors/trevor-miller\">Videos</a></li>
                        </ul>
                    </nav>
                </footer>
              </body>
        </html>
    ",
    &html
        )
}
