use seed::{prelude::*, *};


const GAMEOFLIFE: &str = "game-of-life-rust";

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url) -> Option<Model> {
    Some (Model::new())
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn default(self) -> Url {
        self.base_url().add_path_part(GAMEOFLIFE)
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    counter: u32,
    stop: bool
}

impl Model {
    fn new() -> Model {
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
            stop: false
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
pub enum Msg {
    //Increment,
    Start,
    Tick(RenderInfo),
    Stop
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg 
        {
            //Msg::Increment => {model.tick(); model.counter += 1;}
        
            Msg::Start => 
            {
                model.stop = false;
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

pub fn view(model: &Model) -> Node<Msg> {


    div![
        "Ticks: ",
        C!["counter"],
        attrs!{ At::Id => "counter"}, model.counter,
        div![button!["Start", ev(Ev::Click, |_| Msg::Start)],
        button!["Stop", ev(Ev::Click, |_| Msg::Stop)]
        ],

        div![attrs!{At::Class => "gameclass"}, model.get_string()],
        
        ]
}