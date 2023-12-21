
#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum ShipType {
    AircraftCarrier,
    Battleship,
    Submarine,
    Destroyer,
    PatrolBoat
}

#[derive(Debug)]
pub struct Square {
    pub origin: GridPoint,
    pub ship: Option<Ship>
}

#[derive(Debug)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Ship {
    pub origin: Option<GridPoint>,
    pub orientation: Option<Orientation>,
    size: i32,
    points: i32,
    name: String,
    ship_type: ShipType
}

impl Ship {
    pub fn build(ship_type: ShipType) -> Ship {
        match ship_type {
            ShipType::AircraftCarrier => Ship { ship_type, origin: None, orientation: None, size: 5, points: 2, name: String::from("Aircraft Carrier")},
            ShipType::Battleship => Ship { ship_type, origin: None, orientation: None, size: 4, points: 4, name: String::from("Battleship")},
            ShipType::Submarine => Ship { ship_type, origin: None, orientation: None, size: 3, points: 6, name: String::from("Submarine")},
            ShipType::Destroyer => Ship { ship_type, origin: None, orientation: None, size: 2, points: 8, name: String::from("Destroyer")},
            ShipType::PatrolBoat => Ship { ship_type, origin: None, orientation: None, size: 1, points: 10, name: String::from("Patrol Boat")},
        }
    }

    pub fn get_point(&self) -> i32 {
        self.points
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_type(&self) -> &ShipType {
        &self.ship_type
    }
}


#[derive(Debug)]
pub struct Grid {
    layout: [[Square; 10]; 10]
}

impl Grid {
    pub fn build() -> Grid {
        Self::initialize_layout()
    }

    pub fn build_from_layout(layout: [[Square; 10]; 10]) -> Grid {
        Grid { layout }
    }

    fn initialize_layout() -> Grid{
        let grid: Grid = Self::build_from_layout(layout);

        grid
    }
}


