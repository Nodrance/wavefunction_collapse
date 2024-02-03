My implementation of wavefunction collapse

Layout:

main.rs imports a renderer which should define
- fn draw_tilegrid (grid: &TileGrid) // draws a tilegrid to the screen
- enum TileTexture // has one variant for each texture a tile should have, can have one variant if tiles are rendered based on connections
and a wavefunction which should define
- struct TileChoice {
    connections: [Connection; 4], // up down left right
    texture: TileTexture, // the texture that the renderer will render it with
    weight: i32, // The weight of the tile in the collapse function
  }
- impl UndecidedTile {fn new()} // Generates a new blank undecided tile with correct tiles and weights
- fn can_connect (con1: Connection, con2: Connection) // Returns true if two connections can connect to each other, used while collapsing tiles