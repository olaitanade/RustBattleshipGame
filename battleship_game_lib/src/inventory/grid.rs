use std::collections::HashMap;
use std::{fmt};
use rand::{thread_rng, Rng};
use crate::runtime::ShotStatus;

use super::ship::{GridPoint, Orientation, Ship, ShipType};


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
#[derive(Debug, Clone, Copy)]
pub struct Grid<'g> {
    layout: [[Square<'g>; 10]; 10]
}

impl <'g> Grid<'g> {
    ///generate a blank grid
    pub fn build() -> Grid<'g> {
        Self::initialize_layout()
    }

    ///generate a grid from a previously saved session
    pub fn build_from_layout(layout: [[Square; 10]; 10]) -> Grid {
        Grid { layout }
    }

    pub fn allocate_ships(&'g mut self, ships:&'g mut HashMap<ShipType, Ship<'g>>) {
        for (_key, ship) in ships.iter_mut() {
            self.shuffle_ship_location(ship);
        }
    }

    fn shuffle_ship_location<'a>(&'a mut self, ship:&'g mut Ship)  -> bool {
        loop {
            let mut rng = thread_rng();
            let x_axis = rng.gen_range(1..=10);
            let y_axis = rng.gen_range(1..=10);

            let orientation: Orientation = rand::random();

            ship.origin = Some(GridPoint{ x: x_axis, y: y_axis });
            ship.orientation = Some(orientation);

            if self.verify_allocation(ship) {
                break;
            }
        }
        
        self.add_ship(ship)
    }

    fn verify_allocation(&self, ship: &Ship) -> bool {
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
            },
        }

        true
    }

    ///Add a ship to the grid
    fn add_ship(&mut self, ship: &'g Ship) -> bool {
        match ship.get_orientation() {
            Orientation::Horizontal => {
                for length in 0..ship.get_size() {
                    let grid = GridPoint { x: ship.get_origin_x() + length, y: ship.get_origin_y() };
                    let square = Square{ origin: grid, ship: Some(ship) };
                    self.set_square(square);
                }
            },
            Orientation::Vertical => {
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
    fn remove_ship<'a>(&'a mut self, ship: &'a Ship) -> bool {
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

    pub fn hit_ship(&mut self, grid_point: GridPoint) -> ShotStatus {
        let square = self.layout[Self::get_arr_pos(grid_point.x)][Self::get_arr_pos(grid_point.y)];
        if square.has_ship(){
            let ship = square.ship.unwrap().clone();
            self.remove_ship(&ship);
            return ShotStatus::Hit(ship.clone());
        }
        ShotStatus::Miss
    }

    pub fn get_ship(&self, grid_point: GridPoint) -> &Option<&Ship> {
        let square = self.get_square(grid_point);
        square.get_ship()
    }

    fn get_square<'a>(&self, grid_point: GridPoint) -> &'a Square {
        &self.layout[Self::get_arr_pos(grid_point.x)][Self::get_arr_pos(grid_point.y)]
    }

    fn set_square(&mut self, square: Square<'g>) {
        self.layout[Self::get_arr_pos(square.origin.x)][Self::get_arr_pos(square.origin.y)] = square;
    }


    fn initialize_layout<'a>() -> Grid<'a>{
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
    use crate::inventory::ship::{GridPoint, Ship, ShipType};

    use super::*;

    #[test]
    fn hit_ship_on_grid_true() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(&ship);
        let hit_ship = grid.hit_ship(ship.origin.unwrap());
        assert_eq!(ShotStatus::Hit(ship),hit_ship)
    }

    #[test]
    fn hit_ship_on_grid_false() {
        let mut grid = Grid::build();
        let mut ship = Ship::build(ShipType::AircraftCarrier);
        ship.origin = Some(GridPoint { x: 1, y: 1 });
        grid.add_ship(&ship);
        let hit_ship = grid.hit_ship(GridPoint { x: 1 , y: 2 });
        assert_eq!(ShotStatus::Miss,hit_ship)
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

