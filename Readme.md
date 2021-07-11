# Ray Caster in 2d hyperbolic space (Rust + Webassembly)
An exploration in non-euclidian geometry. Inspired by [CodeParade's Hyperbolica][1] and [ZenoRogue's HyperRogue][2].
Try the [browser version][4] to play with it instantly or compile on your desktop!

![][image-1]

Project is based on [hydrixos' raycaster-rust][3] - preserved the project skeleton, SDL input handling + graphics output and some of the ray casting code.

## How does it work?
The game takes place in a 2-dimensional space with Gaussian curvature = -1. It differs from "normal" euclidian geometry in that
Euclid's 5th axiom ([Paralell postulate][6]) is not preserved. This has a few interesting consequences:

1. Sum of angles in a triangle is less than 180 degrees
2. Non-intersecting lines have a point of minimum distance and diverge from both sides of that point
3. Ratio of circle's circumference to its diameter is greater than pi
4. Moving up, right, down, left leaves you where you started, but rotated

Projecting this space onto a computer screen, as well as applying transformations (translation, rotation) is tricky. The game does it as follows:

1. Map is stored as an array of walls consisting of 2 points (beginning, end) with coordinates in the [Poincaré  disk model][7]. This is chosen as it's relatively easy to design by placing points on a tesselated Poincaré  disk and writing down coordinates.

2. Next, the map is converted to [Minkowski hyperboloid model][8]. This is done so that transformations of the space with player movement are easy to implement and formulas are analogous to the ones used in Euclidian space. This approach was suggested by ZenoRogue, and after trying to research gyrovectors, I can definitely see why. [Very helpful StackExchange thread][9]

3. To render a frame, current state of the world is again converted to Poincaré  disk model, for ease of casting rays and finding intersections. 

4. For each column of the screen a ray is cast from the origin, iterating through the walls, calculating whether an intersection is found with the circle on which lies the geodesic between the ends of the wall. 

5. Then the hits are filtered and checked whether the intersection point is actually contained within the arc of the geodesic between these two points. 

6. The closest hit according to the Poincaré  metric is chosen and drawn on the screen.

For details on the ray casting part I recommend [hydrixos' writeup][10] on his Swift project. This differs slightly as it's not using a grid map, but the general principle applies.

## Tested Platforms
**Browser:**
iOS 14
Desktop: Chrome, Firefox

**Native:**
Linux, Windows

## Tested toolchain versions
- rustc 1.55.0-nightly (a1411de9d 2021-06-26)
- emsdk 1.39.20

## How to Build
To build the desktop or the browser version you need to install the Rust  compiler first:

```bash
curl https://sh.rustup.rs -sSf | sh
```

During the installation, you may be asked for installation options. Just press Enter to select the default option. After the installation succeeded make sure that all environment variables are set up:

```bash
source ~/.profile
```

You may also need to install libSDL. On macOS you can do this with brew:

```bash
brew install sdl2
```

On Ubuntu / Debian:

```bash
apt install libsdl2-dev
```

### The Desktop Version
You can build an run the desktop version by typing 

```bash
cargo run
```

### The Browser Version
To build the browser version, you need to install the [Emscripten SDK][5]. Create a new folder on your file system and open it in your terminal. Then run the following commands to install the SDK:

```bash
git clone https://github.com/juj/emsdk.git
cd emsdk
./emsdk update
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
embuilder.py build sdl2
```

We also need to setup Webassembly support for Rust:

```bash
rustup target add wasm32-unknown-emscripten
```

Finally, switch back to the ray caster source folder to build the browser version:

```bash
./build-web.sh
```

The compilation result is then stored to the  folder `html`. Since Webassembly can’t be directly embedded to a HTML page you need a web server for running the binary. Just copy the entire `html` folder to your web server and then open the `index.html` page in your browser. You can also run it locally with `emrun index.html`.

## Changing the Map
Map is stored as a JSON file containing an array of HyperWalls - struct representing a wall with two points (beginning, end) in coordinates of the 
Poincaré  disk model, and a color of the wall. It is loaded at compile time.

Location of the maps: `assets/`

Location of the chosen map path: `src/main.rs:25` 

## Changing the Renderer
A top-down view of the Poincaré  disk is available. To switch, in file `main.rs` comment out the line:

```rust
use hyperbolic_renderer::Renderer;
```

and uncomment:

```rust
//use poincare_renderer::Renderer;
```


[1]:	https://www.youtube.com/watch?v=EMKLeS-Uq_8
[2]:	https://roguetemple.com/z/hyper/
[3]:	https://github.com/hydrixos/raycaster-rust
[4]:	https://elo-siema.github.io/hyperbolic-raycaster-rust/
[5]:	https://emscripten.org
[6]:	https://emscripten.org
[7]:	https://en.wikipedia.org/wiki/Poincar%C3%A9_disk_model
[8]:	https://en.wikipedia.org/wiki/Hyperboloid_model
[9]:    https://math.stackexchange.com/questions/1862340/what-are-the-hyperbolic-rotation-matrices-in-3-and-4-dimensions?newreg=0a895728ef9c48ad814e2f06eafb3862
[10]:	https://github.com/hydrixos/raycaster-swift

[image-1]:	doc/demo.gif
