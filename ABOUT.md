## How it works

The computer will randomly place 5 different ships into a 10 by 10 grid. The player will then be able to select an individual square fro the grid to fire at. If a ship is on theat square the ship is sunk,the player gains points. After the player has had ten shots the game ends and the player's score is displayed. The game can end before ten moves if the player sinks all the ships.

Although in the game there is never any reason to display the grid.  The computer randomly assigns five ships to the grid ensuring that they do not overlap. The ships can be placed horizontally or vertically but not diagonally.  The ships are all different sizes and worth different amount of points if shot as shown below.

| Ship  | No of squares | Points |
| ------------- | ------------- |  ------------- |
| Aircraft Carrier  | 5  | 2 |
| Battleship  | 4  | 4 |
|  Submarine | 3  | 6 |
| Destroyer  | 2  | 8 |
| Patrol boat  | 1  | 10 |

The game starts and the player has no points.  They are then asked which square they would like to shoot at (they can’t see the grid).  The program informs the player if they have missed or hit a ship.  If they have hit a ship they are told which ship and how many points it is worth. Once a ship has been hit it “sinks” and so cannot be hit again.

### Game End

There are two ways the game can end. After 10 shots or when all ships have been sunk.  If the player sinks all the ships with 10 or less shots the game ends. If the player hasn’t sunk all the ships the game ends after the tenth shot.

Once the game has ended the program displays which ships were shot and the total number of points the player has.

Debug Mode
When the program starts the player will be asked if they wish to play in debug mode.  If they select yes then the locations of the ships will be displayed when the player chooses his square to fire at.  For example, the display might look like this

| Ship  | No of squares |
| ------------- | ------------- |
| Aircraft Carrier  | (2,2) (3,2) (4,2) (5,2) (6,2) |
| Battleship  | (1,5) (1,6) (1,7) (1,8) |
|  Submarine | (5,8) (5,9) (5,10) |
| Destroyer  | (3,5) (4,5) |
| Patrol boat  | (9,3) |

> Please enter the next square to fire at:

When the program is not in debug mode the locations of the ships are not displayed.
