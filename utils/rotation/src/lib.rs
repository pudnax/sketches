use nannou::prelude::*;

pub fn rotate_x(point: Point3, degree: f32, orientation: Point3) -> Point3 {
    let x = point.x;
    let y = (point.y - orientation.y) * degree.to_radians().cos()
        - (point.z - orientation.z) * degree.to_radians().sin()
        + orientation.y;
    let z = (point.y - orientation.y) * degree.to_radians().sin()
        + (point.z - orientation.z) * degree.to_radians().cos()
        + orientation.z;

    pt3(x, y, z)
}

pub fn rotate_y(point: Point3, degree: f32, orientation: Point3) -> Point3 {
    let x = (point.x - orientation.x) * degree.to_radians().cos()
        + (point.z - orientation.z) * degree.to_radians().sin()
        + orientation.x;
    let y = point.y;
    let z = -(point.x - orientation.x) * degree.to_radians().sin()
        + (point.z - orientation.z) * degree.to_radians().cos()
        + orientation.z;

    pt3(x, y, z)
}

pub fn rotate_z(point: Point3, degree: f32, orientation: Point3) -> Point3 {
    let x = (point.x - orientation.x) * degree.to_radians().cos()
        - (point.y - orientation.y) * degree.to_radians().sin()
        + orientation.x;
    let y = (point.x - orientation.x) * degree.to_radians().sin()
        + (point.y - orientation.y) * degree.to_radians().cos()
        + orientation.y;
    let z = point.z;

    pt3(x, y, z)
}

pub fn rotate(point: Point3, degree: Point3, orientation: Point3) -> Point3 {
    let x_rot = rotate_x(point, degree.x, orientation);
    let y_rot = rotate_y(x_rot, degree.y, orientation);
    rotate_z(y_rot, degree.z, orientation)
}
