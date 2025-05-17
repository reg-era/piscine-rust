pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rect_area = areas_volumes::rectangle_area(x, y);

    match kind {
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) * times <= rect_area,
        GeometricalShapes::Circle => {
            areas_volumes::circle_area(a) * times as f64 <= rect_area as f64
        }
        GeometricalShapes::Square => areas_volumes::square_area(a) * times <= rect_area,
        GeometricalShapes::Triangle => {
            areas_volumes::triangle_area(a, b) * times as f64 <= rect_area as f64
        }
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let box_vol = x * y * z;
    match kind {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) * times <= box_vol,
        GeometricalVolumes::Cone => {
            areas_volumes::cone_volume(a, b) * times as f64 <= box_vol as f64
        }
        GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) * times <= box_vol
        }
        GeometricalVolumes::Sphere => {
            areas_volumes::sphere_volume(a) * times as f64 <= box_vol as f64
        }
        GeometricalVolumes::TriangularPyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b) * times as f64 <= box_vol as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
            area_fit((2, 4), GeometricalShapes::Rectangle, 100, (2, 1))
        );
        println!(
            "Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
            area_fit((5, 5), GeometricalShapes::Triangle, 3, (5, 3))
        );
        println!(
            "Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
            volume_fit((5, 5, 5), GeometricalVolumes::Sphere, 3, (2, 0, 0))
        );
        println!(
            "Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
            volume_fit((5, 7, 5), GeometricalVolumes::Parallelepiped, 1, (6, 7, 4))
        );
    }
}
