My implementation of wavefunction collapse

# Layout

main.rs imports a renderer which should define
- pub fn draw_tilegrid (grid: &TileGrid, textures: &HashMap<&str, Texture2D>, tile_size: Vec2, texture_limits: Vec2, offset: i32, render_every: i32) 
- - grid has a lot of stuff, notable height width and tilegrid
- - textures is a hashmap of textures, it can be blank if you don't use texture based rendering but will always be passed
- - tile_size is the size of each tile in pixels
- - texture limits is the edges of the currently active texture, don't draw tiles outside this
- - offset is the current frame count, used with render_every to ensure different tiles are rendered each time
- - render_every is the number of tiles that should be skipped between each one that is rendered. When it is 10, 1/10 tiles should be rendered. This is for performance. 
- - - An upcoming change may rewrite the render engine so it's given a rectangle to draw inside
- pub fn draw_tile_opt (x: f32, y: f32, tile_size: Vec2, tileopt: &TileChoice, textures: &HashMap<&str, Texture2D>)
- - Draws a tile at the given XY co-ords and size. All logic relating to a tile being OOB or offscreen should happen in draw_tilegrid, and all logic relating to which texture gets drawn and how it gets reoriented should happen in here
- pub async fn load_textures_paths<T: Hash + Eq + Clone>(paths: &[&str], keys: &[T]) -> HashMap<T, Texture2D>
- - If your renderer uses programmatic non-texture based rendering, this can be an empty map, or contain entirely missing textures

It also imports a wavefunction which should define
- `struct TileChoice {`
  `  connections: [Connection; 4],` // up down left right
  `  weight: i32,` // The weight of the tile in the collapse function
    // other things the renderer might need, for example texture:TileTexture
  `}`
- enum Connection // Has one variant for each connection
- fn can_connect (con1: Connection, con2: Connection) // Returns true if two connections can connect to each other, used while collapsing tiles. Order must not matter.
- impl UndecidedTile {fn new()} // Generates a new blank undecided tile with correct tile options and weights
- impl TileGrid {fn pick_index(&mut self) -> (i32, i32) /*x,y*/} //Picks the x and y index into the grid that should be collapsed next for best results.

# Controls
- WASD to move the bottom left corner
- UDLR to shift the whole grid
- Space (hold) to autogenerate
- IO to change render mode
- P to toggle grid
- click to collapse a tile
- right click to uncollapse a tile, fails if there are decided tiles locking it in
- 1234567890 to pick option 1-10 from the tile your mouse is over (see bottom corner)
- - Hold Lshift to add 10, Lalt for 20, Lcontrol for 40, does not work for R
- F (hold) for FPS (unsmoothed, might need to work on that lol)

todo:
add indicators for shift, alt, ctrl, add support for right sides too
add train and town/city wavefunctions
profile and reduce ram usage
better (onscreen w/ mouse) controls (shift, expand, zoom, autogen speed, rerender, toggle grid, reset)
smooth fps
add panning
fix debug rerender and clear hotkeys
make grid on different layer
ui that lists all tile variants not just ones in the tile
better right click uncollapse
pull texture definitions into wavefunction.rs
move rendering functions into tilegrid
add sensible defaults for new tileopts, grids so i can just to ..Default when I need to make a new one 
music and sfx
make it so tileopts are way easier to define, instead of the current thing

upload to the web

add the ability to switch wavefunctions without closing and reopening (main menu?)