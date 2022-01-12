// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]



use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {

    //orders.after_next_render(Msg::Increment);
    Model::new(false)
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
    stop: bool
}

impl Model {
    fn new(random: bool) -> Model {
        let width = 92;
        let height = 92;

        let cells = (0..width * height)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
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
                    
                    let symbol = if cell == Cell::Dead { '░' } else { '▓' };
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
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    //Increment,
    Start,
    Tick(RenderInfo),
    Stop
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
                if delta > 0. {
                    model.tick();
                    model.counter += 1;
                }
                match model.stop {
                    false => {orders.after_next_render(Msg::Tick);}
                    true => {}
                }

            }

            Msg::Stop => {
                model.stop = true;
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
        "About Conway's Game of Life"]], 


        div![
        C!["buttons"], 
        button!["Start", ev(Ev::Click, |_| Msg::Start)],
        button!["Stop", ev(Ev::Click, |_| Msg::Stop)],
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
