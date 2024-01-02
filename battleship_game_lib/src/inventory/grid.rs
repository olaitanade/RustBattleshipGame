use std::collections::HashMap;
use std::{fmt};
use rand::{thread_rng, Rng};
use crate::runtime::ShotStatus;

use super::ship::{GridPoint, Orientation, Ship, ShipType};


///Square
#[derive(Debug,Clone,Copy)]
pub struct Square {
    origin: GridPoint,
    ship: Option<ShipType>
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Square (origin = {}, ship = {})", self.origin, self.get_ship_string())
    }
}

///Default struct value of Square
impl Default for Square {
    fn default() -> Self {
        Square::build(GridPoint::default())
    }
}

impl Square {
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
    pub fn build(origin: GridPoint) -> Square {
        Square { origin, ship: None }
    }

    pub fn set_gridpoint(&mut self, origin: GridPoint) {
        self.origin = origin;
    }

    pub fn get_ship(&self) -> Option<ShipType> {
        self.ship
    }

    pub fn set_ship(&mut self,ship: ShipType){
        self.ship = Some(ship);
    }

    pub fn has_ship(&self) -> bool{
        self.ship.is_some()
    }

    pub fn get_ship_string(&self) -> String {
        match &self.ship {
            Some(ship) => format!("{:?}",ship),
            None => format!("None"),
        }
    }
}

///Grid representation with a 10 by 10, 2 dimensional array as layout
#[derive(Debug, Clone)]
pub struct Grid {
    layout: [[Square; 10]; 10],
    ships: HashMap<ShipType,Ship>
}

impl Grid {
    ///generate a blank grid
    pub fn build() -> Grid {
        Self::initialize_layout()
    }

    ///generate a grid from a previously saved session
    pub fn build_from_layout<'a>(layout: [[Square; 10]; 10]) -> Grid {
        Grid { layout , ships: Ship::create_ships()}
    }


    pub fn hit_ship(&mut self, grid_point: GridPoint) -> ShotStatus {
        let square = self.layout[Self::get_arr_pos(grid_point.x)][Self::get_arr_pos(grid_point.y)];
        if square.has_ship(){
            let ship_type = square.ship.unwrap();
            let ship = self.ships.get(&ship_type).unwrap();
            return self.remove_ship(ship_type, grid_point, ship.orientation.unwrap(), ship.get_size());
        }
        ShotStatus::Miss
    }

    pub fn get_ship(&self, grid_point: GridPoint) -> Option<ShipType> {
        let square = self.get_square(grid_point);
        square.get_ship()
    }

    fn get_square<'a>(&self, grid_point: GridPoint) -> &Square {
        &self.layout[Self::get_arr_pos(grid_point.x)][Self::get_arr_pos(grid_point.y)]
    }

    fn set_square(&mut self, square: Square) {
        self.layout[Self::get_arr_pos(square.origin.x)][Self::get_arr_pos(square.origin.y)] = square;
    }


    fn initialize_layout() -> Grid{
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

    pub fn display_ships_location(&self) -> String {
        let mut display = String::new();

        for (_key, ship) in self.ships.iter() {
            display.push_str(&format!("{} \n", ship.get_debug_mode_string()))
        }

        display
    }

    pub fn is_any_ship_left(&self) -> bool{
        for (_key, ship) in self.ships.iter() {
            if !ship.is_destroyed() {
                return true;
            }
        }
        false
    }

    pub fn get_destroyed_ships(&self) -> Vec<Ship>{
        let ships: Vec<Ship> = self.ships.values().cloned().filter(|ship| ship.is_destroyed()).collect();
        ships
    }

    pub fn shuffle_ship_location<'a>(&'a mut self){
        for (_key, ship) in self.ships.clone().iter_mut() {
            let mut rng = thread_rng();
            let mut x_axis = rng.gen_range(1..=10);
            let mut y_axis = rng.gen_range(1..=10);
            let mut orientation: Orientation = rand::random();
            
            loop {
                if self.verify_allocation(ship.get_type(), GridPoint{ x: x_axis, y: y_axis }, orientation, ship.get_size()) {
                    self.add_ship(ship.get_type(), GridPoint{ x: x_axis, y: y_axis }, orientation, ship.get_size());
                    break;
                }

                x_axis = rng.gen_range(1..=10);
                y_axis = rng.gen_range(1..=10);
                orientation = rand::random();
            }
        }
        
    }

    fn verify_allocation(&self, ship_type: ShipType, grid_point: GridPoint, orientation: Orientation, size: i32) -> bool {
        match orientation {
            Orientation::Horizontal => {
                if (grid_point.x + size) > 10 {
                    return false;
                }

                for length in 0..size {
                    let grid = GridPoint { x: grid_point.x + length, y: grid_point.y };
                    let square = self.get_square(grid);
                    if square.has_ship() {
                        return false;
                    }
                }
            },
            Orientation::Vertical => {
                if (grid_point.y + size) > 10 {
                    return false
                }

                for length in 0..size {
                    let grid = GridPoint { x: grid_point.x, y: grid_point.y + length };
                    let square = self.get_square(grid);
                    if square.has_ship() {
                        return false;
                    }
                }
            },
        }

        true
    }

    ///Add a ship to the grid
    fn add_ship(&mut self, ship_type: ShipType, grid_point: GridPoint, orientation: Orientation, size: i32) -> bool {
        match orientation {
            Orientation::Horizontal => {
                for length in 0..size {
                    let grid = GridPoint { x: grid_point.x + length, y: grid_point.y };
                    let square = Square{ origin: grid, ship: Some(ship_type) };
                    self.set_square(square);
                }
            },
            Orientation::Vertical => {
                for length in 0..size {
                    let grid = GridPoint { x: grid_point.x, y: grid_point.y + length };
                    let square = Square{ origin: grid, ship: Some(ship_type) };
                    self.set_square(square);
                }
            },
        }
        self.ships.get_mut(&ship_type).unwrap().origin = Some(grid_point);
        self.ships.get_mut(&ship_type).unwrap().orientation = Some(orientation);
        true
    }

    ///Remove a ship from the grid
    fn remove_ship<'a>(&'a mut self, ship_type: ShipType, grid_point: GridPoint, orientation: Orientation, size: i32) -> ShotStatus {
        match orientation {
            Orientation::Horizontal => {

                for length in 0..size {
                    let grid = GridPoint { x: grid_point.x + length, y: grid_point.y };
                    let square = self.get_square(grid);
                    if !square.has_ship() || square.get_ship().unwrap() != ship_type {
                        return ShotStatus::Miss;
                    }

                    self.set_square(Square{ origin: grid, ship: None });
                }
            },
            Orientation::Vertical => {

                for length in 0..size {
                    let grid = GridPoint { x: grid_point.x, y: grid_point.y + length };
                    let square = self.get_square(grid);
                    if !square.has_ship() || square.get_ship().unwrap() != ship_type {
                        return ShotStatus::Miss;
                    }

                    self.set_square(Square{ origin: grid, ship: None });
                }
            },
        }
        self.ships.get_mut(&ship_type).unwrap().destroy();
        ShotStatus::Hit(ship_type, self.ships.get(&ship_type).unwrap().get_point())
    }
}


#[cfg(test)]
mod tests {
    use crate::inventory::ship::{GridPoint, Ship, ShipType};

    use super::*;

    #[test]
    fn hit_ship_on_grid_true() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(ship.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ship.get_size());
        let hit_ship = grid.hit_ship(ship.origin.unwrap());
        assert_eq!(ShotStatus::Hit(ship.get_type(), ship.get_point()),hit_ship)
    }

    #[test]
    fn hit_ship_on_grid_false() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(ship.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ship.get_size());
        let hit_ship = grid.hit_ship(GridPoint { x: 1 , y: 2 });
        assert_eq!(ShotStatus::Miss,hit_ship)
    }

    #[test]
    fn remove_ship_on_grid_true() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(ship.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ship.get_size());
        let ship_removed = grid.remove_ship(ship.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ship.get_size());
        assert_eq!(ShotStatus::Hit(ship.get_type(), ship.get_point()),ship_removed)
    }

    #[test]
    fn remove_ship_on_grid_false() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(ship.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ship.get_size());
        grid.remove_ship(ship.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ship.get_size());
        let ship_removed = grid.remove_ship(ship.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ship.get_size());
        assert_eq!(ShotStatus::Miss,ship_removed)
    }

    #[test]
    fn add_mul_ships() {
        let mut grid = Grid::build();
        let mut ac = Ship::build(ShipType::AircraftCarrier);
        let mut bat = Ship::build(ShipType::Battleship);
        ac.origin = Some(GridPoint { x: 1, y: 1 });
        ac.orientation = Some(Orientation::Horizontal);
        bat.origin = Some(GridPoint { x: 1, y: 2 });
        bat.orientation = Some(Orientation::Horizontal);
        grid.add_ship(ac.get_type(),GridPoint { x: 1, y: 1 }, Orientation::Horizontal, ac.get_size());
        grid.add_ship(bat.get_type(),GridPoint { x: 1, y: 2 }, Orientation::Horizontal, bat.get_size());
        println!("{}", ac.get_debug_mode_string());
        println!("{}", bat.get_debug_mode_string());
        let hit_ship = grid.hit_ship(ac.origin.unwrap());
        assert_eq!(ShotStatus::Hit(ac.get_type(), ac.get_point()),hit_ship)
    }

    #[test]
    fn shuffle_ship() {
        let mut grid = Grid::build();
        grid.shuffle_ship_location();
        println!("{}",grid.display_ships_location());
        //assert_eq!(false,ship_removed)
    }
}


