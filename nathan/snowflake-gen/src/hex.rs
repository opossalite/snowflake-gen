pub enum HexWrapper {
    Affinity(usize),
    Hex(Box<Hex>),
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
}
impl CoreHex {
    pub fn new() -> Self {
        let hex = Hex {
            top: HexWrapper::Affinity(100),
            topl: HexWrapper::Affinity(100),
            topr: HexWrapper::Affinity(100),
            botl: HexWrapper::Affinity(100),
            botr: HexWrapper::Affinity(100),
            bot: HexWrapper::Affinity(100),
        };
        CoreHex {
            hex,
            leaves: vec![ //all six directions are open by default
                vec![Direction::Top],
                vec![Direction::TopLeft],
                vec![Direction::TopRight],
                vec![Direction::BottomLeft],
                vec![Direction::BottomRight],
                vec![Direction::Bottom],
            ],
        }
    }
}

pub struct Hex {
    top: HexWrapper,
    topl: HexWrapper,
    topr: HexWrapper,
    botl: HexWrapper,
    botr: HexWrapper,
    bot: HexWrapper,
}
impl Hex {
    /// Insert a new hexagon off of this hexagon, considering the path and the total number of hexagons
    pub fn insert(&mut self, direction: Direction, path: Vec<Direction>, total: usize) {

    }
}





