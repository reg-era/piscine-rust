# Geometrical Shapes Drawing

This project is a Rust-based exercise that involves drawing various geometrical shapes and saving the result as a PNG image. The goal of the project is to work with traits, structs, and image processing libraries to implement drawing functionality for different shapes like points, lines, triangles, rectangles, and circles.

## Project Structure

The project consists of the following main parts:

1. **Main File (`main.rs`)**:
    - The main entry point of the application.
    - Uses the `geometrical_shapes` module to generate and draw shapes onto an image.

2. **Module (`geometrical_shapes.rs`)**:
    - Contains the definitions for shapes and traits used for drawing and displaying shapes.

3. **Dependencies**:
    - The `raster` library is used to handle image processing.
    - The `rand` library is used to generate a randoms.

## Instructions

### 1. Setup the Project

1. **Clone the repository** or create a new Rust project and copy the necessary files.

2. **Add dependencies to `Cargo.toml`**:
    In order to compile and run the program, add the following dependencies in your `Cargo.toml` file:

    ```toml
    [dependencies]
    rand = "0.9.1"
    raster = "0.2.0"
    ```

3. **Start drawing**
    After creating a image with height and width using `Image::blank` you can create difrent shapes from geometrical_shapes like lines, points, rectangles, circles, pentagones, cubes, stars and pyramids.

    *Note*: you need to implement `display` for `Image` so it can set each pixel with there color on the image.

```rust
use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    gs::Circle::random(image.width, image.height).draw(&mut image);

    gs::Pentagon::random(image.width, image.height).draw(&mut image);

    gs::Star::random(image.width, image.height).draw(&mut image);

    gs::Cube::new(gs::Point::new(300, 300), 200).draw(&mut image);

    gs::Pyramid::new(gs::Point::new(300, 300), 200).draw(&mut image);

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
```

### 2. Team members

- `ifoukahi` Ilyass Mohamed Foukahi
- `iichi` Ismail Ichi
- `tsaadsal` Tounsadi Saad Salah
