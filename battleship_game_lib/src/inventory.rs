

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
    pub ship: Ship
}

#[derive(Debug)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Ship {
    pub origin: GridPoint,
    pub orientation: Orientation,
    size: i32,
    points: i32,
    pub name: String,
    pub ship_type: ShipType
}

impl Ship {
    pub fn build(ship_type: ShipType, origin: GridPoint, orientation: Orientation) -> Ship {
        match ship_type {
            ShipType::AircraftCarrier => Ship { ship_type, origin, orientation, size: 5, points: 2, name: String::from("Aircraft Carrier")},
            ShipType::Battleship => Ship { ship_type, origin, orientation, size: 4, points: 4, name: String::from("Battleship")},
            ShipType::Submarine => Ship { ship_type, origin, orientation, size: 3, points: 6, name: String::from("Submarine")},
            ShipType::Destroyer => Ship { ship_type, origin, orientation, size: 2, points: 8, name: String::from("Destroyer")},
            ShipType::PatrolBoat => Ship { ship_type, origin, orientation, size: 1, points: 10, name: String::from("Patrol Boat")},
        }
    }

    pub fn getPoint(&self) -> i32 {
        self.points
    }

    pub fn getSize(&self) -> i32 {
        self.size
    }
}



#[derive(Debug)]
pub struct Grid {
    pub layout: [[Square; 10]; 10],
    offset: i32
}


