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

    pub fn follow_path(&mut self, path: Vec<Direction>) -> Result<&mut Hex, ()> {
        let mut hex = &mut self.hex;
        for direction in path {
            match direction {
                Direction::Top => {
                    match &mut hex.top {
                        HexWrapper::Affinity(_) => return Err(()),
                        HexWrapper::Hex(inner) => hex = inner,
                    }
                },
                Direction::TopLeft => {
                    match &mut hex.topl {
                        HexWrapper::Affinity(_) => return Err(()),
                        HexWrapper::Hex(inner) => hex = inner,
                    }
                },
                Direction::TopRight => {
                    match &mut hex.topr {
                        HexWrapper::Affinity(_) => return Err(()),
                        HexWrapper::Hex(inner) => hex = inner,
                    }
                },
                Direction::BottomLeft => {
                    match &mut hex.botl {
                        HexWrapper::Affinity(_) => return Err(()),
                        HexWrapper::Hex(inner) => hex = inner,
                    }
                },
                Direction::BottomRight => {
                    match &mut hex.botr {
                        HexWrapper::Affinity(_) => return Err(()),
                        HexWrapper::Hex(inner) => hex = inner,
                    }
                },
                Direction::Bottom => {
                    match &mut hex.bot {
                        HexWrapper::Affinity(_) => return Err(()),
                        HexWrapper::Hex(inner) => hex = inner,
                    }
                },
            }
        }
        return Ok(hex);

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





