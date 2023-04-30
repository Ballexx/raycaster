use win32console::{
    structs::coord::Coord,
    console::WinConsole,

};
use std::time::{
    SystemTime, 
    Duration
};

use console::Term;
use yore::code_pages::CP437;

mod map;
mod init;

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

fn calculate_fps
(
    system_time_one: SystemTime
)   -> (Duration, SystemTime)
{
    let system_time_two: SystemTime = SystemTime::now();
    return (
        system_time_one.elapsed().unwrap() - system_time_two.elapsed().unwrap(),
        system_time_two
    );
}

fn calculate_wall_shading(distance: f32, depth_of_field: f32) -> u8{
    if distance <= depth_of_field / 4.0{
        return 127;
    }
    else if distance <= depth_of_field / 3.0 {
        return 38;
    }
    else if distance <= depth_of_field / 2.0 {
        return 35;
    }
    else if distance <= depth_of_field{
        return 43;
    }    
    return 32;
}
fn calculate_floor_shading(distance: f32) -> u8{
    if distance < 0.25{
        return 35;
    }
    else if distance < 0.5{
        return 120;
    }
    else if distance < 0.75{
        return 46;
    }
    else if distance < 0.9{
        return 39;
    }    
    return 32;
}

fn clear_console(){
    let console_clear_result = WinConsole::output().clear();

    let console_clear = match console_clear_result {
        Ok(clear) => clear,
        Err(err) => {
            println!("{}", err);
        }
    };
}

fn main(){
    let mut SCREEN: [u8; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize] = [40; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize];

    let (
        mut player_X,
        mut player_Y,
        mut player_angle,
    ) = init::initialize_player();

    let (
        map,
        map_width,
        map_height
    ) = init::initialize_map();

    let (
        horizontal_sensitivity,
        player_speed,
        FOV,
        depth_of_field
    ) = init::initialize_settings();

    let mut current_system_time:   SystemTime = SystemTime::now();

    let stdout: Term = Term::buffered_stdout();
    let console_clear: () = clear_console();

    loop {
        let (elapsed, next_system_time) = calculate_fps(current_system_time);
        let FPS: f32 = elapsed.as_secs_f32();
        current_system_time = next_system_time;

        for x in 0..SCREEN_WIDTH{
            let ray_angle: f32 = (player_angle - FOV / 2.0) + (x as f32 / SCREEN_WIDTH as f32);
            
            let eye_X: f32 = ray_angle.sin();
            let eye_Y: f32 = ray_angle.cos();

            let mut distance_to_wall: f32 = 0.0;

            while distance_to_wall < depth_of_field{
                distance_to_wall += 0.1;

                let ray_x: i32 = (player_X + eye_X * distance_to_wall) as i32;
                let ray_y: i32 = (player_Y + eye_Y * distance_to_wall) as i32;

                if get_ray_bounds(ray_x, ray_y, map_height, map_width){
                    distance_to_wall = depth_of_field;
                    break;
                }
                if map[(ray_y * map_width + ray_x) as usize] == "#"{
                    break;
                }
            }

            let ceiling = (SCREEN_HEIGHT as f32 / 2.0) - SCREEN_HEIGHT as f32 / (distance_to_wall as f32);
            let floor= SCREEN_HEIGHT - ceiling as i32;

            let wall_shading = calculate_wall_shading(distance_to_wall, depth_of_field);

            for y in 0..SCREEN_HEIGHT{

                if y < ceiling as i32{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = 32;
                }
                else if y > ceiling as i32 && y <= floor{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = wall_shading;
                }
                else{
                    let floor_distance: f32 = 1.0 - ((y as f32 - SCREEN_HEIGHT as f32 / 2.0) / (SCREEN_HEIGHT as f32 /2.0));
                    let floor_shading: u8 = calculate_floor_shading(floor_distance);

                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = floor_shading;
                }
            }            
        }
        WinConsole::output().write_output_character(&SCREEN, Coord::new(0, 0)).unwrap();

        if let Ok(input) = stdout.read_char() {
            match input {
                'w' => {
                    player_X += player_angle.sin() * player_speed * FPS;
                    player_Y += player_angle.cos() * player_speed * FPS;
                },
                's' => {
                    player_X -= player_angle.sin() * player_speed * FPS;
                    player_Y -= player_angle.cos() * player_speed * FPS;
                },
                'a' => player_angle -= 0.1 * (FPS * horizontal_sensitivity),
                
                'd' => player_angle += 0.1 * (FPS * horizontal_sensitivity),
                _ => (),
            }
        }

    }
}
