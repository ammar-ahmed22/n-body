
/// Normalize a value from one range to another
///
/// ### Arguments
/// - `value` The value to normalize
/// - `min` The minimum value of the input range
/// - `max` The maximum value of the input range
/// - `map_min` The minimum value of the target range
/// - `map_max` The maximum value of the target range
///
/// ### Returns
/// `f32` The normalized value in the target range
pub fn normalize(value: f32, min: f32, max: f32, map_min: f32, map_max: f32) -> f32 {
    let normalized = (value - min) / (max - min);
    map_min + normalized * (map_max - map_min)
}

