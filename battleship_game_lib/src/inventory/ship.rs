use std::{fmt, collections::HashMap};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::runtime::GridPoint;

/////////////////////////////////////////////////////////////////////////
/// Orientation of ship
/////////////////////////////////////////////////////////////////////////
///
///Horizontal, Vertical
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Distribution<Orientation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Orientation {
        match rng.gen_range(0..=1) {
            0 => Orientation::Horizontal,
            1 => Orientation::Vertical,
            _ => Orientation::Horizontal
        }
    }
}

/////////////////////////////////////////////////////////////////////////
/// Types of Ship
/////////////////////////////////////////////////////////////////////////
///
///AircraftCarrier, Battleship, Submarine, Destroyer, PatrolBoat
#[derive(Debug,Clone,Copy,PartialEq, Eq, Hash)]
pub enum ShipType {
    AircraftCarrier,
    Battleship,
    Submarine,
    Destroyer,
    PatrolBoat
}

/////////////////////////////////////////////////////////////////////////
///Ship
/////////////////////////////////////////////////////////////////////////
///
///Fields
///pub origin: Option<GridPoint>,
///pub orientation: Option<Orientation>,
///size: i32,
///points: i32,
///name: String,
///ship_type: ShipType,
///destroyed: bool
#[derive(Debug,Clone, PartialEq, Eq, Hash)]
pub struct Ship {
    pub origin: Option<GridPoint>,
    pub orientation: Option<Orientation>,
    size: i32,
    points: i32,
    name: String,
    ship_type: ShipType,
    destroyed: bool
}

impl Ship {
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
    pub fn build(ship_type: ShipType) -> Ship {
        match ship_type {
            ShipType::AircraftCarrier => Ship { ship_type, origin: None, orientation: None, size: 5, points: 2, name:String::from("Aircraft Carrier"), destroyed: false},
            ShipType::Battleship => Ship { ship_type, origin: None, orientation: None, size: 4, points: 4, name: String::from("Battleship"), destroyed: false},
            ShipType::Submarine => Ship { ship_type, origin: None, orientation: None, size: 3, points: 6, name: String::from("Submarine"), destroyed: false },
            ShipType::Destroyer => Ship { ship_type, origin: None, orientation: None, size: 2, points: 8, name: String::from("Destroyer"), destroyed: false},
            ShipType::PatrolBoat => Ship { ship_type, origin: None, orientation: None, size: 1, points: 10, name: String::from("Patrol Boat"), destroyed: false},
        }
    }

    pub fn get_point(&self) -> i32 {
        self.points
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
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
    pub fn get_type(&self) -> ShipType {
        self.ship_type
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

    pub fn create_ships() -> HashMap<ShipType, Self> {
        let mut ship_yard = HashMap::new();
        
        ship_yard.insert(ShipType::AircraftCarrier, Ship::build(ShipType::AircraftCarrier));
        ship_yard.insert(ShipType::Battleship, Ship::build(ShipType::Battleship));
        ship_yard.insert(ShipType::Submarine, Ship::build(ShipType::Submarine));
        ship_yard.insert(ShipType::Destroyer, Ship::build(ShipType::Destroyer));
        ship_yard.insert(ShipType::PatrolBoat, Ship::build(ShipType::PatrolBoat));

        ship_yard
    }

    ///get ship with horizontal orientation debug mode string
    fn get_horizontal_dms(&self) -> String {
        let mut output = String::new();

        for x in 0..self.size {
            output.push_str(&format!("({},{}) ", &self.get_origin_x() + x, &self.get_origin_y()))
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
impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ship(name = {}, starting point = {}, orientation = {:?}, size = {}, points = {})", &self.name, &self.origin.as_ref().unwrap(), &self.orientation.as_ref().unwrap(), &self.size, &self.points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship() {
        
    }
}
