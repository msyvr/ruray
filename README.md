## &#x1F386; a ray tracer written in rust &#x1F386;

### Usage
Clone this (`msyvr/ruray`) repo, then:
```bash
cargo run -- image-file-name-no-format
```

## Project objective
Build a basic ray tracing engine to exercise my new Rust skills.

#### Goal 0
Write a slice of directly-computed pixels to an image format. No scene, just set up the render capability.
```rust
let r = (0.1 + ((col as f32) * 0.9 / (display.0 as f32 - 1.0))) as f32;
let g = 0.5;
let b = (0.1 + ((row as f32) * 0.9 / (display.1 as f32 - 1.0))) as f32;
```

!["color gradient, no scene"](/static/color_gradient.webp)

#### Goal 1
Generate a photo-realistic image of a configured scene. 
- Start by defining a `world` which includes both a `scene` (with lighting) and a `display` and a 'camera' `viewpoint`. 
- Trace a `ray` that emanates from the `viewpoint`, passes through a `display` `pixel`, and interacts with the `scene`.
  - Each interaction contributes to the final color value of that display `pixel`. 
- Repeat for all `display` pixels. 
- Save the display's pixel values (rgb tuples) to a (formatted) image file.

From an [Nvidia ray tracing post](https://developer.nvidia.com/discover/ray-tracing), an overview of the conceptual layout of the 'world':
!["ray tracing: conceptual layout, with viewpoint, display plane, and scene"](static/concept-ray-tracing.png)

#### Goal 2
Optimize the code for speed.
- Probably by implementing [Rayon](https://docs.rs/rayon/latest/rayon/).
- Look into GPU acceleration (stretch).

#### Goal 3 (stretch)
Create an animation.
- Optimized code essential to be able to step through multiple real-time renders 'live'. 
- Consider running on a cloud provider's more powerful machine (with GPU acceleration).

### LICENSE
Not having looked into licenses for a while, I did a quick search and found [this](https://www.reddit.com/r/opensource/comments/1b5oeq4/comment/kta5hwv/), which seemed to align with my intent - so Apache 2.0 it is. See the [LICENSE](https://github.com/msyvr/ruray?tab=License-1-ov-file) for details.

---

### Implementation details

#### Camera
Typically, a camera is represented as a point, at least initially.

#### Viewport aka display plane
Within the scene, this is where the scene image is formed. The final image will inherit the color values assigned to the display pixels.

This plane is perpendicular to the -z axis in a RHC system where the x axis runs parallel to display rows, and the y axis runs parallel to display columns. Vu and Vv are established to 'traverse' rows and columns of the viewport display. 

The distances between pixel centers are considered, so pixels have an area (i.e., they are not points). To illustrate: if the number of pixels is 16:9 and the display is at z = -1, ranging over x = [-8, 7], y = [4, 4], the pixel x and y dimensions are each 1, so the upper left pixel is at (-7.5, 3.5). Initially, the 'sensing' area of a pixel is considered to be at its center point. Averaging over rays passing through a sample of positions within the pixel deserves consideration for images with small numbers of pixels relative to the display dimensions.
