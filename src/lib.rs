// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod page;

use seed::{prelude::*, *};

const GAMEOFLIFE: &str = "game-of-life";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    
    orders.subscribe(Msg::UrlChanged);

    
    Model {
    base_url: url.to_base_url(),
    page: Page::init(url),
    }
    

}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    page: Page,
    base_url: Url

}

// ------ ------
//    Pages
// ------ ------

enum Page {
    Home,
    GameOfLife(page::game_of_life::Model),
    NotFound
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(GAMEOFLIFE) => page::game_of_life::init(url).map_or(Self::NotFound, Self::GameOfLife),
            Some(_) => Self::NotFound,
        }
    }
}


// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn gameoflife_urls(self) -> page::game_of_life::Urls<'a> {
        page::game_of_life::Urls::new(self.base_url().add_path_part(GAMEOFLIFE))
    }
}

// ------ ------
//    Update
// ------ ------


// `Msg` describes the different events you can modify state with.
enum Msg {
    //Increment,
    UrlChanged(subs::UrlChanged),
    GameOfLifeMsg(page::game_of_life::Msg),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg 
        {
            Msg::UrlChanged(subs::UrlChanged(url)) => {
                model.page = Page::init(url);
            }

            Msg::GameOfLifeMsg(msg) => {
                if let Page::GameOfLife(model) = &mut model.page {
                    page::game_of_life::update(msg, model, &mut orders.proxy(Msg::GameOfLifeMsg));
                }
            }
        } 
        
}
    
    


// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> impl IntoNodes<Msg> {
    
    vec![
        header(&model),
        match &model.page {
            Page::Home => div![
                div!["Welcome home!"],
                button![
                    "Go to Url prefixed by base path (see `base` in `index.html`)",
                    ev(Ev::Click, |_| Url::new()
                        .set_path(&["base", "path"])
                        .go_and_load())
                ]
            ],
            Page::GameOfLife(gameoflife_model) => page::game_of_life::view(gameoflife_model).map_msg(Msg::GameOfLifeMsg),
            Page::NotFound => div!["404"],
        },
    ]
}

fn header(model: &Model) -> Node<Msg>{
    ul![
        li![a![
            attrs! { At::Href => Urls::new(&model.base_url).home() },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(&model.base_url).gameoflife_urls().default() },
            "Game of Life",
        ]],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
