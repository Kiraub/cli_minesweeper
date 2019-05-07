use std::{
    fmt::{
        Display,
        Formatter,
    },
    collections::{
        HashMap,
        HashSet
    }
};
use rand::Rng;
use crate::{
    part_traits::{
        board_traits::{
            BoardT
        },
        field_traits::{
            FieldT
        },
        point_traits::{
            PointT,
            PointHandler
        }
    },
    Message,
    FmtResult,
    Number
};
use super::{
    point::Coord,
    field::Field
};

pub struct MineBoard {
    field_map: HashMap<Coord,Field>
}

impl MineBoard {
    pub fn init(&mut self, width: Number, height: Number, bombs: Number) -> Result<(), Message> {
        let field_size : usize = width * height;
        let mut rng = rand::thread_rng();
        let mut bomb_indices : HashSet<Number> = HashSet::with_capacity(bombs);
        let mut bomb_c = 0;
        while bomb_c < bombs {
            let rand_usize : usize = rng.gen();
            if !bomb_indices.contains(&(rand_usize % field_size)) {
                bomb_c += 1;
                bomb_indices.insert(rand_usize % field_size);
            }
        }
        for width_c in 0..width {
            for height_c in 0..height {
                let coord = Coord::from((width_c,height_c));
                if bomb_indices.contains(&(width_c + width*height_c)) {
                    self.field_map.insert(coord, Field::bomb());
                } else {
                    self.field_map.insert(coord, Field::new());
                }
            }
        }
        for index in bomb_indices {
            let coord = self.get_point(index);
            self.incr_neighbourhood(&coord);
        }
        Ok(())
    }

    pub fn pick(&mut self, args: &Vec<String>) -> (Message,bool) {
        match MineBoard::parse_coord(args) {
            Ok(coord) => match self.reveal(&coord) {
                Some(msg) => (msg, false),
                None => ("Bomb picked.".to_string(), true)
            },
            Err(msg) => (msg.to_string(), false)
        }
    }

    pub fn mark(&mut self, args: &Vec<String>) -> Message {
        match MineBoard::parse_coord(args) {
            Ok(coord) => {
                if let Some(m_field) = self.get_mut(&coord) {
                    m_field.set_marked(!m_field.get_marked());
                    "Toggled mark on field.".to_string()
                } else {
                    "No such field.".to_string()
                }
            },
            Err(msg) => msg.to_string()
        }
    }

    fn get(&self, pos: &Coord) -> Option<&Field> {
        self.field_map.get(pos)
    }

    fn get_mut(&mut self, pos: &Coord) -> Option<&mut Field> {
        self.field_map.get_mut(pos)
    }

    fn set(&mut self, pos: &Coord, field: Field) -> bool {
        if let Some(s_field) = self.field_map.get_mut(pos) {
            s_field.set_value(field.get_value());
            s_field.set_visible(field.get_visible());
            true
        } else {
            false
        }
    }

    fn incr_neighbourhood(&mut self, pos: &Coord) {
        let x_max = self.get_x_max();
        let y_max = self.get_y_max();
        let pos_x = pos.get_x();
        let pos_y = pos.get_y();
        let left = match pos_x {0 => 0, _ => pos_x-1};
        let right = usize::min(x_max, pos_x+1);
        let up = match pos_y {0 => 0, _ => pos_y-1};
        let down = usize::min(y_max,pos_y+1);
        for x in left..=right {
            for y in up..=down {
                let distance : usize = usize::from(pos_x!=x) + usize::from(pos_y!=y);
                if distance > 0 {
                    let neighbour = Coord::create(x,y);
                    self.get_mut(&neighbour).unwrap().incr();
                }
            }
        }
    }

    fn reveal(&mut self, pos: &Coord) -> Option<Message> {
        match self.get_field(pos) {
            Ok(field) => {
                let bomb = field.is_bomb();
                self.reveal_neighbourhood(pos);
                if !bomb {
                    Some("Field picked.".to_string())
                } else {
                    None
                }
            },
            Err(msg) => Some(msg)
        }
    }

    fn reveal_neighbourhood(&mut self, pos: &Coord) {
        let x_max = self.get_x_max();
        let y_max = self.get_y_max();
        let pos_x = pos.get_x();
        let pos_y = pos.get_y();
        let left = match pos_x {0 => 0, _ => pos_x-1};
        let right = usize::min(x_max, pos_x+1);
        let up = match pos_y {0 => 0, _ => pos_y-1};
        let down = usize::min(y_max,pos_y+1);
        let empty_center = self.get(pos).unwrap().is_empty();
        for x in left..=right {
            for y in up..=down {
                let distance : usize = usize::from(pos_x!=x) + usize::from(pos_y!=y);
                let coord = Coord::create(x,y);
                let neighbour : &mut Field = self.get_mut(&coord).unwrap();
                if distance == 0 {
                    neighbour.set_visible(true);
                } else if empty_center {
                    if !neighbour.get_visible() {
                        neighbour.set_visible(true);
                        self.reveal_neighbourhood(&coord);
                    }
                }
            }
        }
    }

    fn parse_coord(args: &Vec<String>) -> Result<Coord,&'static str> {
        if args.len() < 2 {
            Err("Not enough arguments provided")
        } else {
            if let Ok(col) = usize::from_str_radix(&args[0],10) {
                if let Ok(row) = usize::from_str_radix(&args[1],10) {
                    Ok(Coord::create(col-1,row-1))
                } else {
                    Err("Problem parsing arguments.")
                }
            } else {
                Err("Problem parsing arguments.")
            }
        }
    }
}

impl Display for MineBoard {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use std::cmp::Ordering;
        let mut max_width = 0;
        let mut list : Vec<(Coord, Field)> = Vec::new();
        let mut out : String = String::new();
        for key in self.field_map.keys() {
            list.push((*key,self.field_map.get(key).unwrap().clone()));
            if key.get_x() > max_width {
                max_width = key.get_x();
            }
        }
        list.sort_by(|some,other| -> Ordering {
            match (*some).0.get_y().cmp(&(*other).0.get_y()) {
                Ordering::Equal => (*some).0.get_x().cmp(&(*other).0.get_x()),
                any_order => any_order
            }
        });
        let mut list_iter = list.iter();
        while let Some(elem) = list_iter.next() {
            out.push_str(&format!(" {}", elem.1));
            if elem.0.get_x() == max_width {
                if (elem.0.get_y()+1) % 5 == 0 {
                    out.push_str(&"--");
                }
                out.push('\n');
            }
        }
        for column in 1..=max_width+1 {
            if (column) % 5 == 0 {
                out.push_str(&" |");
            } else {
                out.push_str(&"  ");
            }
        }
        out.push('\n');
        write!(f, "{}", out)
    }
}

impl BoardT<Field,Number> for MineBoard {
    type PositionThing = Coord;
    type MessageThing = Message;

    fn new() -> Self {
        MineBoard{
            field_map: HashMap::new()
        }
    }

    fn get_field(&self, pos: &Self::PositionThing) -> Result<&Field, Self::MessageThing> {
        if let Some(field) = self.get(&pos) {
            Ok(field)
        } else {
            Err("No such field".to_string())
        }
    }

    fn set_field(&mut self, pos: &Self::PositionThing, field: Field) -> Result<(), Self::MessageThing> {
        if self.set(pos, field) {
            Ok(())
        } else {
            Err("No such field".to_string())
        }
    }
}

impl PointHandler<Number> for MineBoard {
    type PointThing = Coord;

    fn get_x_min(&self) -> Number {
        let mut min_value = usize::max_value();
        for key in self.field_map.keys() {
            if key.get_x() < min_value {
                min_value = key.get_x();
            }
        }
        min_value
    }

    fn get_x_max(&self) -> Number {
        let mut max_value = usize::min_value();
        for key in self.field_map.keys() {
            if key.get_x() > max_value {
                max_value = key.get_x();
            }
        }
        max_value
    }

    fn get_y_min(&self) -> Number {
        let mut min_value = usize::max_value();
        for key in self.field_map.keys() {
            if key.get_y() < min_value {
                min_value = key.get_y();
            }
        }
        min_value
    }

    fn get_y_max(&self) -> Number {
        let mut max_value = usize::min_value();
        for key in self.field_map.keys() {
            if key.get_y() > max_value {
                max_value = key.get_y();
            }
        }
        max_value
    }

    fn get_index(&self, pos: Self::PointThing) -> Number {
        let width = self.get_x_max();
        width * pos.get_y() + pos.get_x()
    }

    fn get_point(&self, idx: Number) -> Self::PointThing {
        let width = self.get_x_max() + 1;
        let row = idx / width;
        let col = idx % width;
        Coord::create(col, row)
    }
}