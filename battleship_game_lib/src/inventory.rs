use std::fmt;

#[derive(Debug,Clone,Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug,Clone,Copy)]
pub enum ShipType {
    AircraftCarrier,
    Battleship,
    Submarine,
    Destroyer,
    PatrolBoat
}

#[derive(Debug,Clone,Copy)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for GridPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GridPoint(x = {}, y = {})", &self.x, &self.y)
    }
}

impl Default for GridPoint {
    fn default() -> GridPoint {
        GridPoint {
            x: 0,
            y: 0
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Ship<'a> {
    pub origin: Option<GridPoint>,
    pub orientation: Option<Orientation>,
    size: i32,
    points: i32,
    name: &'a str,
    ship_type: ShipType
}

impl Ship<'_> {
    pub fn build(ship_type: ShipType) -> Ship<'static> {
        match ship_type {
            ShipType::AircraftCarrier => Ship { ship_type, origin: None, orientation: None, size: 5, points: 2, name: "Aircraft Carrier"},
            ShipType::Battleship => Ship { ship_type, origin: None, orientation: None, size: 4, points: 4, name: "Battleship"},
            ShipType::Submarine => Ship { ship_type, origin: None, orientation: None, size: 3, points: 6, name: "Submarine"},
            ShipType::Destroyer => Ship { ship_type, origin: None, orientation: None, size: 2, points: 8, name: "Destroyer"},
            ShipType::PatrolBoat => Ship { ship_type, origin: None, orientation: None, size: 1, points: 10, name: "Patrol Boat"},
        }
    }

    pub fn get_point(&self) -> i32 {
        self.points
    }

    pub fn get_origin_x(&self) -> i32 {
        match &self.origin {
            Some(grid) => grid.x,
            None => 0,
        }
    }

    pub fn get_origin_y(&self) -> i32 {
        match &self.origin {
            Some(grid) => grid.y,
            None => 0,
        }
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> &ShipType {
        &self.ship_type
    }

    pub fn get_debug_mode_string(&self) -> String {
        let mut output = format!("{}  ", &self.name);
        match &self.orientation {
            Some(o) => match o {
                Orientation::Horizontal => output.push_str(&&self.get_horizontal_dms()),
                Orientation::Vertical => output.push_str(&self.get_vertical_dms()),
            },
            None => (),
        }

        output
    }

    fn get_horizontal_dms(&self) -> String {
        let mut output = String::new();

        for x in 0..self.size {
            output.push_str(format!("({},{}) ", &self.get_origin_x() + x, &self.get_origin_y()).as_str())
        }

        output
    }

    fn get_vertical_dms(&self) -> String {
        let mut output = String::new();

        for y in 0..self.size {
            output.push_str(format!("({},{}) ", &self.get_origin_x(), &self.get_origin_y() + y).as_str())
        }

        output
    }
}

impl fmt::Display for Ship<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ship(name = {}, starting point = {}, orientation = {:?}, size = {}, points = {})", &self.name, &self.origin.as_ref().unwrap(), &self.orientation.as_ref().unwrap(), &self.size, &self.points)
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Square<'a> {
    origin: GridPoint,
    ship: Option<Ship<'a>>
}

impl fmt::Display for Square<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Square (origin = {}, ship = {})", self.origin, self.get_ship_string())
    }
}

impl Default for Square<'_> {
    fn default() -> Self {
        Square::build(GridPoint::default())
    }
}

impl Square<'_> {
    pub fn build(origin: GridPoint) -> Square<'static> {
        Square { origin, ship: None }
    }

    pub fn set_gridpoint(&mut self, origin: GridPoint) {
        self.origin = origin;
    }

    pub fn get_ship(&self) -> &Option<Ship> {
        &self.ship
    }

    pub fn set_ship(&mut self,ship: Ship<'static>){
        self.ship = Some(ship);
    }

    pub fn has_ship(&self) -> bool{
        self.ship.is_some()
    }

    pub fn get_ship_string(&self) -> String {
        match &self.ship {
            Some(ship) => format!("{}",ship),
            None => format!("None"),
        }
    }
}

#[derive(Debug)]
pub struct Grid<'a> {
    layout: [[Square<'a>; 10]; 10]
}

impl Grid<'_> {
    pub fn build() -> Grid<'static> {
        Self::initialize_layout()
    }

    pub fn build_from_layout(layout: [[Square; 10]; 10]) -> Grid {
        Grid { layout }
    }

    fn initialize_layout() -> Grid<'static>{
        let mut layout: [[Square; 10]; 10] = [[Square::default(); 10]; 10];
        for (y, row) in layout.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                println!("{}", col);
            }
        }
        
        let grid: Grid = Self::build_from_layout(layout);

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gridpoint() {
        
    }
}


