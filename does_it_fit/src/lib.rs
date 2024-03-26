mod areas_volumes;
pub use crate::areas_volumes::*;

type GS = GeometricalShapes;
type GV = GeometricalVolumes;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let target_area = x * y;
    let object_area = times * match objects {
        GS::Square => square_area(a),
        GS::Circle => circle_area(a).ceil() as usize,
        GS::Rectangle => rectangle_area(a, b),
        GS::Triangle => triangle_area(a, b).ceil() as usize,
    };
    object_area <= target_area
}
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
let target_volume = x * y * z;
let object_volume = times * match objects {
    GV::Cube => cube_volume(a),
    GV::Sphere => sphere_volume(a).ceil() as usize,
    GV::Cone => cone_volume(a, b).ceil() as usize,
    GV::Pyramid => triangular_pyramid_volume(a as f64, b).ceil() as usize,
    GV::Parallelepiped => parallelepiped_volume(a, b, c),
};
object_volume <= target_volume
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::areas_volumes::*;

    type GS = GeometricalShapes;
    type GV = GeometricalVolumes;

    #[test]
    fn area_test() {
        let f = |o: GS| area_fit(10, 10, o, 2, 5, 15);
        assert!(f(GS::Square));
        assert!(!f(GS::Circle));
        assert!(!f(GS::Rectangle));
        assert!(f(GS::Triangle));
    }

    #[test]
    fn volume_test() {
        let f = |o: GV| volume_fit(10, 10, 10, o, 2, 5, 20, 5);
        assert!(f(GV::Cube));
        assert!(!f(GV::Cone));
        assert!(f(GV::Parallelepiped));
        assert!(f(GV::Pyramid));
        assert!(!f(GV::Sphere));
    }
}