// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]


extern crate rand;

use rand::prelude::*;

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {

    //orders.after_next_render(Msg::Increment);
    Model::new(48, 48, false)
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    counter: u32,
    stop: bool,
    defaultsize: (u32, u32),
    newsize: (u32, u32)
}

impl Model {
    fn new(width: u32, height: u32, random: bool) -> Model {
        let mut firstalive = 2;
        let mut secondalive = 7;

        match random {
            true => {
                let mut rng = thread_rng();
                firstalive = rng.gen_range(1..10);
                secondalive = rng.gen_range(1..10);
            },
            false => {}
        }

        let cells = (0..width * height)
        .map(|i| {
            if i % firstalive == 0 || i % secondalive == 0 {
                Cell::Alive
            } else {Cell::Dead}
        })
        .collect();

        Model {
            width, 
            height, 
            cells,
            counter: 0,
            stop: true,
            defaultsize: (48, 48),
            newsize: (width, height)
        }
    }
    
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.width - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
            
        }
                count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
            let idx = self.get_index(row, col);
            let cell = self.cells[idx];
            let live_neighbors = self.live_neighbor_count(row, col);

            let next_cell = match (cell, live_neighbors) {
                // Rule 1: Any live cell with fewer than two live neighbours
                // dies, as if caused by underpopulation.
                (Cell::Alive, x) if x < 2 => Cell::Dead,
                // Rule 2: Any live cell with two or three live neighbours
                // lives on to the next generation.
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                // Rule 3: Any live cell with more than three live
                // neighbours dies, as if by overpopulation.
                (Cell::Alive, x) if x > 3 => Cell::Dead,
                // Rule 4: Any dead cell with exactly three live neighbours
                // becomes a live cell, as if by reproduction.
                (Cell::Dead, 3) => Cell::Alive,
                // All other cells remain in the same state.
                (otherwise, _) => otherwise,
            };
            
            next[idx] = next_cell;
        }
    }
    self.cells = next;
    }

    fn get_string(&self) -> String {

            let mut s: String = String::from("");
            let br = String::from("\n");

            for line in self.cells.as_slice().chunks(self.width as usize) {
                for &cell in line {
                    
                    let symbol = if cell == Cell::Dead { '🌿' } else { '🐦' };
                    s.push(symbol);
                }

                s.push_str(&br);
                
            }

            s
            
        }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    //Increment,
    Start,
    Tick(RenderInfo),
    Stop,
    Shuffle,
    Reset,
    Resize,
    SetX(String),
    SetY(String),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg 
        {
            //Msg::Increment => {model.tick(); model.counter += 1;}
        
            Msg::Start => 
            {
                match model.stop {
                    true => model.stop = false,
                    false => return
                }
                orders.after_next_render(Msg::Tick);
            }

            Msg::Tick(render_info) => {
                let delta = render_info.timestamp_delta.unwrap_or_default();
                
                match model.stop {
                    false => {orders.after_next_render(Msg::Tick);}
                    true => {return}
                }
                
                if delta > 0. {
                    model.tick();
                    model.counter += 1;
                }

            }

            Msg::Stop => {
                model.stop = true;
            }

            Msg::Shuffle => {
                model.stop = true;
                *model = Model::new(model.width, model.height, true);
                
            }

            Msg::Reset => {
                model.stop = true;
                *model = Model::new(model.defaultsize.0, model.defaultsize.1, false);
            }

            Msg::Resize => {
                model.stop = true;
                let mut size = (model.newsize.0, model.newsize.1);
                if size == (0,0) {size.0 = model.defaultsize.0; size.1 = model.defaultsize.1;};
                *model = Model::new(size.0, size.1, false);
            }

            Msg::SetX(x) => {
                let check = x.parse::<u32>();
                match check {
                    Ok(x) => {model.newsize.0 = x; }
                    _ => {}
                }
            }

            Msg::SetY(y) => {
                let check = y.parse::<u32>();
                match check {
                    Ok(y) => {model.newsize.1 = y; }
                    _ => {}
            }
        }

        } 
        
}
    
    


// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    
    div![
        div![
        C!["counter"],
        attrs!{ At::Id => "counter"},
        "Ticks: ",
        model.counter],
        

        div![
        C!["about"],
        a![attrs!{At::Href => "https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life", At::Target => "_blank"},
        "About"]], 


        div![
        C!["buttons"], 
        button!["Start", ev(Ev::Click, |_| Msg::Start)],
        button!["Stop", ev(Ev::Click, |_| Msg::Stop)],
        button!["Shuffle", ev(Ev::Click, |_| Msg::Shuffle)],
        button!["Reset", ev(Ev::Click, |_| Msg::Reset)],
        button!["Resize", ev(Ev::Click, move |_| Msg::Resize)],

        input![C!["input"], 
        style!{
            
            St::BoxShadow => "none",
            St::BackgroundColor => "transparent",
            St::Height => rem(2),
            St::Border => "none",
            St::BorderBottom => format!("{} {} {}", "solid", "#3273dc", px(2)),
            St::MaxWidth => percent(55),
        },
        attrs!{
            //At::Value => "48",
            At::Type => "number",
            At::Min => "1",
            At::Max => "256"
        },
        input_ev(Ev::Input, move |x| Msg::SetX(x)),
        
    ],

    input![C!["input"], 
    style!{
        
        St::BoxShadow => "none",
        St::BackgroundColor => "transparent",
        St::Height => rem(2),
        St::Border => "none",
        St::BorderBottom => format!("{} {} {}", "solid", "#3273dc", px(2)),
        St::MaxWidth => percent(55),
    },
    attrs!{
        //At::Value => "48",
        At::Type => "number",
        At::Min => "1",
        At::Max => "256"
    },
    input_ev(Ev::Input, move |y| Msg::SetY(y)),
    
],
    " ",
    "X: ", model.newsize.0,
    " ",
    "Y: ", model.newsize.1,

        ],

        div![
        attrs!{At::Class => "gameclass"},
        model.get_string()],
        
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
