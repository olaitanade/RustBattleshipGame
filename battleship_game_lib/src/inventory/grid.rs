use std::collections::HashMap;
use std::fmt;
use rand::{thread_rng, Rng};
use crate::runtime::{ShotStatus, GridPoint};

use super::ship::{Orientation, Ship, ShipType};


/// Square
///
///
/// ```
/// 
///
/// pub struct Square {
///   origin: GridPoint,
///   ship: Option<ShipType>
/// }  
///
/// impl fmt::Display for Square {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "Square (origin = {}, ship = {})", self.origin, self.get_ship_string())
///     }
/// }
///
/// let square = Square::build(GridPoint{x: 1, y: 2});
///
/// ```
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

/// Default struct value of Square
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

    /// Set origin of square
    pub fn set_gridpoint(&mut self, origin: GridPoint) {
        self.origin = origin;
    }

    /// Get ShipType on square if any, returns Option<ShipType> 
    pub fn get_ship(&self) -> Option<ShipType> {
        self.ship
    }

    /// Set ShipType on square
    pub fn set_ship(&mut self,ship: ShipType){
        self.ship = Some(ship);
    }

    /// Has ShipType on square
    pub fn has_ship(&self) -> bool{
        self.ship.is_some()
    }

    /// Get ShipType string display on square
    pub fn get_ship_string(&self) -> String {
        match &self.ship {
            Some(ship) => format!("{:?}",ship),
            None => format!("None"),
        }
    }
}

/// Grid representation with a 10 by 10, 2 dimensional array as layout
///
///
/// ```
/// 
///
/// pub struct Grid {
///   layout: [Square; 100],
///   ships: HashMap<ShipType,Ship>
/// }  
///
///
///
/// let grid = Grid::build();
///
/// ```
#[derive(Debug, Clone)]
pub struct Grid {
    layout: [Square; 100],
    ships: HashMap<ShipType,Ship>
}

impl Grid {
    /// Generate a blank grid
    pub fn build() -> Grid {
        Self::initialize_layout()
    }

    /// Generate a grid from layout (a previously saved session)
    /// Argument: `layout: [[Square; 10]; 10]``
    pub fn build_from_layout(layout: [Square; 100]) -> Grid {
        Grid { layout , ships: Ship::create_ships()}
    }

    /// Hit ship
    /// Argument: `grid_point: GridPoint`
    /// Return: `ShotStatus`
    pub fn hit_ship(&mut self, grid_point: GridPoint) -> ShotStatus {
        let square = self.layout[Self::get_index(Self::get_arr_pos(grid_point.x), Self::get_arr_pos(grid_point.y))];
        if square.has_ship(){
            let ship_type = square.ship.unwrap();
            let ship = self.ships.get(&ship_type).unwrap();
            return self.remove_ship(ship_type, grid_point, ship.orientation.unwrap(), ship.get_size());
        }
        ShotStatus::Miss
    }

    /// Get ship
    /// Argument: `grid_point: GridPoint`
    /// Return: `Option<ShipType>`
    pub fn get_ship(&self, grid_point: GridPoint) -> Option<ShipType> {
        let square = self.get_square(grid_point);
        square.get_ship()
    }

    /// Get ship locations
    pub fn display_ships_location(&self) -> String {
        let mut display = String::new();

        for (_key, ship) in self.ships.iter() {
            display.push_str(&format!("{} \n", ship.get_debug_mode_string()))
        }

        display
    }


    /// Is any ship left
    pub fn is_any_ship_left(&self) -> bool{
        for (_key, ship) in self.ships.iter() {
            if !ship.is_destroyed() {
                return true;
            }
        }
        false
    }

    /// Get destroyed ships
    pub fn get_destroyed_ships(&self) -> Vec<Ship>{
        let ships: Vec<Ship> = self.ships.values().cloned().filter(|ship| ship.is_destroyed()).collect();
        ships
    }

    /// Shuffle ship locations randomly on the grid
    pub fn shuffle_ship_location<'a>(&'a mut self){
        for (_key, ship) in self.ships.clone().iter_mut() {
            let mut rng = thread_rng();
            let mut x_axis = rng.gen_range(1..=10);
            let mut y_axis = rng.gen_range(1..=10);
            let mut orientation: Orientation = rand::random();
            
            loop {
                if self.verify_allocation(GridPoint{ x: x_axis, y: y_axis }, orientation, ship.get_size()) {
                    self.add_ship(ship.get_type(), GridPoint{ x: x_axis, y: y_axis }, orientation, ship.get_size());
                    break;
                }

                x_axis = rng.gen_range(1..=10);
                y_axis = rng.gen_range(1..=10);
                orientation = rand::random();
            }
        }
        
    }


    fn get_square<'a>(&self, grid_point: GridPoint) -> &Square {
        &self.layout[Self::get_index(Self::get_arr_pos(grid_point.x), Self::get_arr_pos(grid_point.y))]
    }

    fn set_square(&mut self, square: Square) {
        self.layout[Self::get_index(Self::get_arr_pos(square.origin.x), Self::get_arr_pos(square.origin.y))] = square;
    }


    fn initialize_layout() -> Grid{
        let mut layout: [Square;100] = [Square::default(); 100];
        for (n, cell) in layout.iter_mut().enumerate() {
            cell.set_gridpoint(GridPoint { x: Self::get_grid_pos(Self::cal_pos_x(n)), y: Self::get_grid_pos(Self::cal_pos_y(n)) })
            
        }
        
        Self::build_from_layout(layout)
    }

    fn get_index(row: usize, column: usize) -> usize {
        row * 10 + column
    }

    fn cal_pos_y(i: usize) -> usize {
        i/10
    }

    fn cal_pos_x(i: usize) -> usize {
        i%10
    }

    fn get_arr_pos(axis: i32) -> usize {
        (axis - 1).try_into().unwrap()
    }

    fn get_grid_pos(axis: usize) -> i32 {
        (axis + 1).try_into().unwrap()
    }

    fn verify_allocation(&self, grid_point: GridPoint, orientation: Orientation, size: i32) -> bool {
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
    use crate::inventory::ship::{Ship, ShipType};

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


