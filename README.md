# RustBattleshipGame

A rust battleship game on CLI based on the classic game Battleships.

## Goal

The main goal of this project is to use all I have learnt in the rust book and then use this game as a means to learn how to use runtime on different platforms and build architectures

1. CLI based game
1. Mobile and Desktop based game using Flutter, FFI, Protobuf and tokio many more ....
1. Web assembly build on the web.
1. Substrate pallet, Ink contract and blockchain with a runtime to support classic games like this as contracts.

## How it works

The computer will randomly plack 5 different ships into a 10 by 10 grid. The player will then be able to select an individual square fro the grid to fire at. If a ship is on theat square the ship is sunk,the player gains points. After the player has had ten shots the game ends and the player's score is displayed. The game can end before ten moves if the player sinks all the ships.

Although in the game there is never any reason to display the grid.  The computer randomly assigns five ships to the grid ensuring that they do not overlap. The ships can be placed horizontally or vertically but not diagonally.  The ships are all different sizes and worth different amount of points if shot as shown below.

| Ship  | No of squares | Points |
| ------------- | ------------- |  ------------- |
| Aircraft Carrier  | 5  | 2 |
| Battleship  | 4  | 4 |
|  Submarine | 3  | 6 |
| Destroyer  | 2  | 8 |
| Patrol boat  | 1  | 10 |

So after the computer has allocated the ships the grid might look like this

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

## Tasks

- [ ] Add game data structures
- [ ] Add basic game runtime - Grid and Gameplay
- [ ] Prevent the user from firing at the same square twice
If the user tries to enter a square they have already fired at they should be told they can’t fire at that square and the attempt does not count as one of their ten shots.
- [ ] Add Save Game Status:
The player should be able to stop the game part way through and the game status is saved. When the program exits and starts up again the player should be asked if they wish to start a new game or load a saved game.
- [ ] Add High Score Table:
The program should keep a list of names and high scores.  The high score table must be maintained when the program has been shut down and started up again and so it is able to save users from different sessions.
- [ ] Add tests
- [ ] Convert game runtime into a library crate with compatible interface for other User interface like Flutter for UI and Wasm on the web
