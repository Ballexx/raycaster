use win32console::console::WinConsole;
use win32console::structs::coord::Coord;

mod constants;
mod input;

const SCREEN_WIDTH: i32 = 120;
const SCREEN_HEIGHT: i32 = 40;

fn get_ray_bounds
(
    ray_x:      i32,
    ray_y:      i32, 
    map_height: i32, 
    map_width:  i32
) -> bool
{
    if ray_x < 0 || ray_x >= map_height || ray_y < 0 || ray_y >= map_width{
        return true;
    };
    return false;
}

fn main(){
    let mut SCREEN: [u8; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize] = [40; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize];

    let map: [&str; 256] = constants::map();

    let map_width: i32 = 16;
    let map_height: i32 = 16;

    let FOV: f32 = constants::set_fov(4);
    let depth_of_field: f32 = 16.0;
 
    let mut player_X: f32 = 8.0;
    let mut player_Y: f32 = 8.0;
    let mut player_angle: f32 = 0.0;

    WinConsole::output().clear().unwrap();    

    loop {
        for x in 0..SCREEN_WIDTH{
            let ray_angle: f32 = (player_angle - FOV / 2.0) + (x as f32 / SCREEN_WIDTH as f32);
            
            let eye_X: f32 = ray_angle.sin();
            let eye_Y: f32 = ray_angle.cos();

            let mut distance_to_wall: f32 = 0.0;

            let mut wall_collision: bool = false;

            while !wall_collision && distance_to_wall < depth_of_field{
                distance_to_wall += 0.1;

                let ray_x: i32 = (player_X + eye_X * distance_to_wall) as i32;
                let ray_y: i32 = (player_Y + eye_Y * distance_to_wall) as i32;

                if get_ray_bounds(ray_x, ray_y, map_height, map_width){
                    wall_collision = true;
                    distance_to_wall = depth_of_field;
                }
                else {
                    if map[(ray_y * map_width + ray_x) as usize] == "#"{
                        wall_collision = true;
                    }
                }

            }

            let ceiling = (SCREEN_HEIGHT as f32 / 2.0) - SCREEN_HEIGHT as f32 / (distance_to_wall as f32);
            let floor= SCREEN_HEIGHT - ceiling as i32;

            for y in 0..SCREEN_HEIGHT{

                if y < ceiling as i32{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = 32;
                }
                else if y > ceiling as i32 && y <= floor{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = 35;
                }
                else{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = 32;
                }

                
            }            
        }

        WinConsole::output().write_output_character(&SCREEN, Coord::new(0, 0)).unwrap();
    }
}