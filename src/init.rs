use crate::map;

fn set_fov(FOV: i32) -> f32{
    return 3.141592 / FOV as f32;
}

pub fn initialize_player() -> (
    f32,
    f32,
    f32
){
    let mut player_X:       f32 = 8.0;
    let mut player_Y:       f32 = 8.0;
    let mut player_angle:   f32 = 0.0;

    return(
        player_X,
        player_Y,
        player_angle
    )
}

pub fn initialize_map<'lifetime>() -> (
    [&'lifetime str; 256],
    i32,
    i32
){
    let map:         [&str; 256] = map::generate_map();
    let map_width:   i32 = 16;
    let map_height:  i32 = 16;

    return(
        map,
        map_width,
        map_height
    )
}

pub fn initialize_settings() -> (
    f32,
    f32,
    f32,
    f32
){
    let horizontal_sensitivity:  f32 = 4.0;
    let player_speed:            f32 = 4.0;
    let FOV:                     f32 = set_fov(4);
    let depth_of_field:          f32 = 16.0;

    return(
        horizontal_sensitivity,
        player_speed,
        FOV,
        depth_of_field
    )
}
