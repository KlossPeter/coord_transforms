use na::Vector3;
use structs::geo_ellipsoid;
use std::f64;

/// Converts 3-d ENU coordinates to 3-d NED coordinates
/// 
/// # Arguments
/// 
/// * `enu_vec` - Vector3 reference to the ENU vector (x, y, z)
/// 
/// # Return Value
/// 
/// * nalgebra::Vector3<f64> - x, y, z
/// 
/// # Formula
/// 
/// * x = y
/// * y = x
/// * z = -z
pub fn enu2ned(enu_vec: &Vector3<f64>) -> Vector3<f64> {
    let mut ret_vec: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
    ret_vec.x = enu_vec.y;
    ret_vec.y = enu_vec.x;
    ret_vec.z = -enu_vec.z;
    ret_vec
}

/// Converts 3-d NED coordinates to 3-d ENU coordinates
/// 
/// # Arguments
/// 
/// * `ned_vec` - Vector3 reference to the NED vector (x, y, z)
/// 
/// # Return Value
/// 
/// * nalgebra::Vector3<f64> - x, y, z
/// 
/// # Formula
/// 
/// * x = y
/// * y = x
/// * z = -z
pub fn ned2enu(ned_vec: &Vector3<f64>) -> Vector3<f64> {
    let mut ret_vec: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
    ret_vec.x = ned_vec.y;
    ret_vec.y = ned_vec.x;
    ret_vec.z = -ned_vec.z;
    ret_vec
}


/// Converts 3-d LLA coordinates to 3-d ECEF coordinates
/// 
/// # Arguments
/// 
/// * `lla_vec` - Vector3 reference to the LLA vector (latitude, longitude, altitude) (radians, radians, meters)
/// 
/// # Return Value
/// 
/// * nalgebra::Vector3<f64> - x, y, z
/// 
/// # Formula
/// 
/// * x = (N + h) * cos(lat) * cos(lon)
/// * y = (N + h) * cos(lat) * sin(lon)
/// * z = (( b^2 / a^2 ) * N + h) * sin(lat)
pub fn lla2ecef(lla_vec: &Vector3<f64>, ellipsoid: &geo_ellipsoid::geo_ellipsoid) -> Vector3<f64> {
	let mut ret_vec: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
	let N = ellipsoid.get_semi_major_axis() / (1.0 - ellipsoid.get_first_ecc().powi(2) * lla_vec.x.sin().powi(2)).sqrt();
	ret_vec.x = (N + lla_vec.z) * lla_vec.x.cos() * lla_vec.y.cos();
	ret_vec.y = (N + lla_vec.z) * lla_vec.x.cos() * lla_vec.y.sin();
	ret_vec.z = ((ellipsoid.get_semi_minor_axis().powi(2) / ellipsoid.get_semi_major_axis().powi(2)) * N + lla_vec.z) * lla_vec.x.sin();
	ret_vec
}


/// Converts 3-d ECEF coordinates to 3-d LLA coordinates
/// 
/// # Arguments
/// 
/// * `ecef_vec` - Vector3 reference to the ECEF vector (x, y, z)
/// 
/// # Return Value
/// 
/// * nalgebra::Vector3<f64> - lat, long, alt (radians, radians, meters)
/// 
/// # Formula
/// 
/// * x = arctan((z + e'^2 * b * sin^3 (theta)) / (p - e^2 * a * cos^3 (theta)))
/// * y = arctan(y / x)
/// * z = (p  / cos(lat)) - N
pub fn ecef2lla(ecef_vec: &Vector3<f64>, ellipsoid: &geo_ellipsoid::geo_ellipsoid) -> Vector3<f64> {
    let mut ret_vec: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
    let p = (ecef_vec.x.powi(2) + ecef_vec.y.powi(2)).sqrt();
    let theta = (ecef_vec.z * ellipsoid.get_semi_major_axis()).atan2(p * ellipsoid.get_semi_minor_axis());
    let xTop = ecef_vec.z + ellipsoid.get_second_ecc().powi(2) * ellipsoid.get_semi_minor_axis() * theta.sin().powi(3);
    let xBot = p - ellipsoid.get_first_ecc().powi(2) * ellipsoid.get_semi_major_axis() * theta.cos().powi(3);
    ret_vec.x = xTop.atan2(xBot);
    ret_vec.y = ecef_vec.y.atan2(ecef_vec.x);
    let N = ellipsoid.get_semi_major_axis() / (1.0 - ellipsoid.get_first_ecc().powi(2) * (ret_vec.x.sin() * ret_vec.x.sin())).sqrt();
    ret_vec.z = (p / ret_vec.x.cos()) - N;
    ret_vec
}

//Unit tests
#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_enu2ned() {
        let enu_vec: Vector3<f64> = Vector3::new(3.0, 4.0, 5.0);
        let ned_vec = enu2ned(&enu_vec);
        assert_eq!(ned_vec.x, 4.0);
        assert_eq!(ned_vec.y, 3.0);
        assert_eq!(ned_vec.z, -5.0);
    }
    #[test]
    fn test_ned2enu() {
        let ned_vec: Vector3<f64> = Vector3::new(3.0, 4.0, 5.0);
        let enu_vec = ned2enu(&ned_vec);
        assert_eq!(enu_vec.x, 4.0);
        assert_eq!(enu_vec.y, 3.0);
        assert_eq!(enu_vec.z, -5.0);
    }
    #[test]
    fn test_lla2ecef() {
    	let ellipsoid = geo_ellipsoid::geo_ellipsoid::new(geo_ellipsoid::WGS84_SEMI_MAJOR_AXIS_METERS,
    										geo_ellipsoid::WGS84_FLATTENING);
        let latDeg: f64 = 48.856614;
        let lonDeg: f64 = 2.352222;
        let altitudeMeters: f64 = 1000.0;
        let lla_vec: Vector3<f64> = Vector3::new(latDeg.to_radians(), lonDeg.to_radians(), altitudeMeters);
        let ecef_vec = lla2ecef(&lla_vec, &ellipsoid);
        assert_approx_eq!(ecef_vec.x, 4201570.9492264455);
        assert_approx_eq!(ecef_vec.y, 172588.3449531975);
        assert_approx_eq!(ecef_vec.z, 4780835.4317144295);
    }
    #[test]
    fn test_ecef2lla() {
        let ellipsoid = geo_ellipsoid::geo_ellipsoid::new(geo_ellipsoid::WGS84_SEMI_MAJOR_AXIS_METERS,
                                            geo_ellipsoid::WGS84_FLATTENING);
        let ecef_vec: Vector3<f64> = Vector3::new(4201570.9492264455, 172588.3449531975, 4780835.4317144295);
        let lla_vec = ecef2lla(&ecef_vec, &ellipsoid);
        let latDeg: f64 = 48.856614;
        let lonDeg: f64 = 2.352222;
        assert_approx_eq!(lla_vec.x, latDeg.to_radians());
        assert_approx_eq!(lla_vec.y, lonDeg.to_radians());
        assert_approx_eq!(lla_vec.z, 1000.0);
    }
}