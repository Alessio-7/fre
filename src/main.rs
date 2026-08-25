use regex::Regex;
mod tui;
mod util;
use tui::app::App;

fn main() -> color_eyre::Result<()> {
    let path: Option<String> = std::env::args().nth(1);
    ratatui::run(|terminal| App::default().run(terminal, valid_path(path)))
}

fn valid_path(path: Option<String>) -> String {
    if path.is_some() {
        let p: String = path.unwrap();
        let re: Regex = Regex::new(r"^~?\/.*$").unwrap();
        if re.is_match(&p) {
            return p;
        } else {
            panic!("the path '{}' is not valid", p);
        }
    }
    String::from("~/")
}
