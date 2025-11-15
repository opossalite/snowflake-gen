pub enum HexWrapper {
    Affinity(usize),
    Hex(Box<Hex>),
    Blocked,
}

pub enum Direction {
    Top,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Bottom,
}

pub struct CoreHex {
    hex: Hex,
    leaves: Vec<Vec<Direction>>,
    size: usize,
}
impl CoreHex {
    pub fn new() -> Self {
        let hex = Hex {
            top: HexWrapper::Affinity(100),
            top_left: HexWrapper::Affinity(100),
            top_right: HexWrapper::Affinity(100),
            bottom_left: HexWrapper::Affinity(100),
            bottom_right: HexWrapper::Affinity(100),
            bottom: HexWrapper::Affinity(100),
        };
        CoreHex {
            hex,
            //all six directions are open by default, last direction is unused for path
            // but directions are not included, so empty at first
            leaves: vec![
                //vec![Direction::Top],
                //vec![Direction::TopLeft],
                //vec![Direction::TopRight],
                //vec![Direction::BottomLeft],
                //vec![Direction::BottomRight],
                //vec![Direction::Bottom],
            ],
            size: 0,
        }
    }

    /// Follows the path, returning the Hex at the end location
    pub fn follow_path(&mut self, path: Vec<Direction>) -> Option<&mut Hex> {
        let mut hex = &mut self.hex;
        for direction in path.iter() {
            match hex.retrieve(direction) {
                HexWrapper::Affinity(_) => return None,
                HexWrapper::Hex(inner) => hex = inner,
                HexWrapper::Blocked => return None,
            }
        }
        return Some(hex);
    }

    /// Insert a new hexagon
    pub fn insert(&mut self, path: Vec<Direction>, direction: Direction) -> Option<()> {
        let end_hex = self.follow_path(path)?;
        match end_hex.retrieve(&direction) {
            HexWrapper::Affinity(source_affinity) => {
                //TODO, need to insert new hex and generate its new affinities,
                //maybe considering the directionality towards root too and distance,
                //and also find out which sides are blocked
            },
            HexWrapper::Hex(_) => return None,
            HexWrapper::Blocked => return None,
        }
        return None;
    }
}

pub struct Hex {
    top: HexWrapper,
    top_left: HexWrapper,
    top_right: HexWrapper,
    bottom_left: HexWrapper,
    bottom_right: HexWrapper,
    bottom: HexWrapper,
}
impl Hex {
    pub fn retrieve(&mut self, direction: &Direction) -> &mut HexWrapper {
        match direction {
            Direction::Top => &mut self.top,
            Direction::TopLeft => &mut self.top_left,
            Direction::TopRight => &mut self.top_right,
            Direction::BottomLeft => &mut self.bottom_left,
            Direction::BottomRight => &mut self.bottom_right,
            Direction::Bottom => &mut self.bottom,
        }
    }



    ///// Insert a new hexagon off of this hexagon, considering the path and the total number of hexagons
    //pub fn insert(&mut self, direction: Direction, path: Vec<Direction>, total: usize) {
    //    let end_hex = 
    //}
}





