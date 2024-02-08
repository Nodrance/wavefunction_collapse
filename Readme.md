My implementation of wavefunction collapse

# Layout

main.rs imports a renderer which should define
- `fn draw_tilegrid (grid: &TileGrid)` // draws a tilegrid to the screen
- //Anything else it needs in order to render, such as `enum TileTexture`
and a wavefunction which should define
- `struct TileChoice {`
  `  connections: [Connection; 4],` // up down left right
  `  weight: i32,` // The weight of the tile in the collapse function
    // other things the renderer might need, for example texture:TileTexture
  `}`
- enum Connection // Has one variant for each connection
- fn can_connect (con1: Connection, con2: Connection) // Returns true if two connections can connect to each other, used while collapsing tiles. Order should not matter.
- impl UndecidedTile {fn new()} // Generates a new blank undecided tile with correct tile options and weights
- impl TileGrid {fn pick_index(&mut self) -> (i32, i32) /*x,y*/} //Picks the x and y index into the grid that should be collapsed next for best results.

# Controls
- WASD to move the bottom left corner
- UDLR to shift the whole grid
- Space (hold) to autogenerate
- IOP to change render mode
- G to toggle grid
- click to collapse a tile
- right click to uncollapse a tile, fails if there are decided tiles locking it in
- 1234567890 to pick option 1-10 from the tile your mouse is over (internal order, no easy way to tell other than trial and error)
- - Hold Lshift to add 10, Lalt for 20, Lcontrol for 40,  

todo: make beach into generic renderer
add indicators for shift, alt, ctrl
add puzzle, castle, and train/town wavefunctions
profile and reduce ram usage
better (onscreen w/ mouse) UI

