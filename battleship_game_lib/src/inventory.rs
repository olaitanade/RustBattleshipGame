use std::fmt;

///Orientation of ship
///Horizontal, Vertical
#[derive(Copy, Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

///Ship types enum
#[derive(Debug,Clone,Copy,PartialEq, Eq, Hash)]
pub enum ShipType {
    AircraftCarrier,
    Battleship,
    Submarine,
    Destroyer,
    PatrolBoat
}

///Gridpoint representation of the 2 dimensional array
/// x > 0, x < 11 ,1-10 inclusive
/// y > 0, y < 11 ,1-10 inclusive
#[derive(Debug,Clone,Copy)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

///Display GridPoint
impl fmt::Display for GridPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GridPoint(x = {}, y = {})", &self.x, &self.y)
    }
}

///Default struct value of GridPoint
impl Default for GridPoint {
    ///GridPoint default is all zeros
    fn default() -> GridPoint {
        GridPoint {
            x: 1,
            y: 1
        }
    }
}

///Ship
#[derive(Debug,Clone,Copy)]
pub struct Ship<'a> {
    pub origin: Option<GridPoint>,
    pub orientation: Option<Orientation>,
    size: i32,
    points: i32,
    name: &'a str,
    ship_type: ShipType
}

impl <'a> Ship<'a> {
    /// Returns a ship with the type given
    ///
    /// # Arguments
    ///
    /// * `ship_type` - A ShipType  that holds the type of the ship
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use inventory::Ship;
    /// let ship = Ship::build(ShipType::AircraftCarrier);
    /// ```
    pub fn build(ship_type: ShipType) -> Ship<'a> {
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

    ///returns orientation if given or defaults to horizontal
    pub fn get_orientation(&self) -> &Orientation {
        match &self.orientation {
            Some(direction) => direction,
            None => &Orientation::Horizontal,
        }
    }

    ///returns x if origin given or defaults to 0
    pub fn get_origin_x(&self) -> i32 {
        match &self.origin {
            Some(grid) => grid.x,
            None => 0,
        }
    }

    ///returns y if origin given or defaults to 0
    pub fn get_origin_y(&self) -> i32 {
        match &self.origin {
            Some(grid) => grid.y,
            None => 0,
        }
    }

    ///get size of ship
    pub fn get_size(&self) -> i32 {
        self.size
    }

    ///get name of ship
    pub fn get_name(&self) -> &str {
        &self.name
    }

    ///get type of ship
    pub fn get_type(&self) -> &ShipType {
        &self.ship_type
    }

    ///get debug mode string of the ship
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

    ///get ship with horizontal orientation debug mode string
    fn get_horizontal_dms(&self) -> String {
        let mut output = String::new();

        for x in 0..self.size {
            output.push_str(format!("({},{}) ", &self.get_origin_x() + x, &self.get_origin_y()).as_str())
        }

        output
    }

    ///get ship with vertical orientation debug mode string
    fn get_vertical_dms(&self) -> String {
        let mut output = String::new();

        for y in 0..self.size {
            output.push_str(format!("({},{}) ", &self.get_origin_x(), &self.get_origin_y() + y).as_str())
        }

        output
    }
}

///ship cli display
impl fmt::Display for Ship<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ship(name = {}, starting point = {}, orientation = {:?}, size = {}, points = {})", &self.name, &self.origin.as_ref().unwrap(), &self.orientation.as_ref().unwrap(), &self.size, &self.points)
    }
}

///Square
#[derive(Debug,Clone,Copy)]
pub struct Square<'a> {
    origin: GridPoint,
    ship: Option<&'a Ship<'a>>
}

impl fmt::Display for Square<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Square (origin = {}, ship = {})", self.origin, self.get_ship_string())
    }
}

///Default struct value of Square
impl Default for Square<'_> {
    fn default() -> Self {
        Square::build(GridPoint::default())
    }
}

impl <'a> Square<'a> {
    /// Returns a square with the origin given
    ///
    /// # Arguments
    ///
    /// * `origin` - A GridPoint that holds the initial position of the ship on the grid
    ///
    /// # Examples
    ///
    /// ```
    /// 
    /// use inventory::Square;
    /// let square = Square::build(GridPoint{x: 1, y: 2});
    /// ```
    pub fn build(origin: GridPoint) -> Square<'a> {
        Square { origin, ship: None }
    }

    pub fn set_gridpoint(&mut self, origin: GridPoint) {
        self.origin = origin;
    }

    pub fn get_ship(&self) -> &Option<&Ship> {
        &self.ship
    }

    pub fn set_ship(&mut self,ship: &'a Ship){
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

///Grid representation with a 10 by 10, 2 dimensional array as layout
#[derive(Debug)]
pub struct Grid<'a> {
    layout: [[Square<'a>; 10]; 10]
}

impl <'a> Grid<'a> {
    ///generate a blank grid
    pub fn build() -> Grid<'a> {
        Self::initialize_layout()
    }

    ///generate a grid from a previously saved session
    pub fn build_from_layout(layout: [[Square; 10]; 10]) -> Grid {
        Grid { layout }
    }

    ///Add a ship to the grid
    pub fn add_ship(&mut self, ship: &'a Ship) -> bool {
        match ship.get_orientation() {
            Orientation::Horizontal => {
                if (ship.get_origin_x() + ship.get_size()) > 10 {
                    return false;
                }

                for length in 0..ship.get_size() {
                    let grid = GridPoint { x: ship.get_origin_x() + length, y: ship.get_origin_y() };
                    let square = self.get_square(grid);
                    if square.has_ship() {
                        return false;
                    }
                }

                for length in 0..ship.get_size() {
                    let grid = GridPoint { x: ship.get_origin_x() + length, y: ship.get_origin_y() };
                    let square = Square{ origin: grid, ship: Some(ship) };
                    self.set_square(square);
                }
            },
            Orientation::Vertical => {
                if (ship.get_origin_y() + ship.get_size()) > 10 {
                    return false
                }

                for length in 0..ship.get_size() {
                    let grid = GridPoint { x: ship.get_origin_x(), y: ship.get_origin_y() + length };
                    let square = self.get_square(grid);
                    if square.has_ship() {
                        return false;
                    }
                }

                for length in 0..ship.get_size() {
                    let grid = GridPoint { x: ship.get_origin_x(), y: ship.get_origin_y() + length };
                    let square = Square{ origin: grid, ship: Some(ship) };
                    self.set_square(square);
                }
            },
        }

        true
    }

    ///Remove a ship from the grid
    fn remove_ship(&mut self, ship: &'a Ship) -> bool {
        match ship.get_orientation() {
            Orientation::Horizontal => {

                for length in 0..ship.get_size() {
                    let grid = GridPoint { x: ship.get_origin_x() + length, y: ship.get_origin_y() };
                    let square = self.get_square(grid);
                    if !square.has_ship() || square.get_ship().unwrap().get_type() != ship.get_type() {
                        return false;
                    }

                    let square = Square{ origin: grid, ship: None };
                    self.set_square(square);
                }
            },
            Orientation::Vertical => {

                for length in 0..ship.get_size() {
                    let grid = GridPoint { x: ship.get_origin_x(), y: ship.get_origin_y() + length };
                    let square = self.get_square(grid);
                    if !square.has_ship() || square.get_ship().unwrap().get_type() != ship.get_type() {
                        return false;
                    }

                    let square = Square{ origin: grid, ship: None };
                    self.set_square(square);
                }
            },
        }
        true
    }

    pub fn hit_ship(&mut self, grid_point: GridPoint) -> bool {
        let square = self.get_square(grid_point);
        square.has_ship()
    }

    fn get_square(&self, grid_point: GridPoint) -> &'a Square {
        &self.layout[Self::get_arr_pos(grid_point.x)][Self::get_arr_pos(grid_point.y)]
    }

    fn set_square(&mut self, square: Square<'a>) {
        self.layout[Self::get_arr_pos(square.origin.x)][Self::get_arr_pos(square.origin.y)] = square;
    }


    fn initialize_layout() -> Grid<'a>{
        let mut layout: [[Square; 10]; 10] = [[Square::default(); 10]; 10];
        for (y, row) in layout.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                col.set_gridpoint(GridPoint { x: Self::get_grid_pos(x), y: Self::get_grid_pos(y) })
            }
        }
        
        Self::build_from_layout(layout)
    }

    fn get_arr_pos(axis: i32) -> usize {
        (axis - 1).try_into().unwrap()
    }

    fn get_grid_pos(axis: usize) -> i32 {
        (axis + 1).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_ship_on_grid_true() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(&ship);
        let hit_ship = grid.hit_ship(ship.origin.unwrap());
        assert_eq!(true,hit_ship)
    }

    #[test]
    fn hit_ship_on_grid_false() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(&ship);
        let hit_ship = grid.hit_ship(GridPoint { x: 1 , y: 2 });
        assert_eq!(false,hit_ship)
    }

    #[test]
    fn remove_ship_on_grid_true() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(&ship);
        let ship_removed = grid.remove_ship(&ship);
        assert_eq!(true,ship_removed)
    }

    #[test]
    fn remove_ship_on_grid_false() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(&ship);
        grid.remove_ship(&ship);
        let ship_removed = grid.remove_ship(&ship);
        assert_eq!(false,ship_removed)
    }
}


