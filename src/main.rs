use win32console::{
    structs::coord::Coord,
    console::WinConsole,

};
use crossterm::{
    event::{
        read,
        Event, 
        KeyCode, 
        KeyEvent, 
        KeyModifiers,
        KeyEventKind, KeyEventState
    }
};

use std::time::{SystemTime, Duration};

use console::Term;
use yore::code_pages::CP437;

mod constants;

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
    time_one: SystemTime
)   -> (Duration, SystemTime)
{
    let time_two: SystemTime = SystemTime::now();
    return (
        time_one.elapsed().unwrap() - time_two.elapsed().unwrap(),
        time_two
    );
}

fn calculate_shading(distance: f32, depth_of_field: f32) -> u8{
    if distance <= depth_of_field / 4.0{
        return 35;
    }
    else if distance <= depth_of_field / 3.0 {
        return 64;
    }
    else if distance <= depth_of_field / 2.0 {
        return 38;
    }
    else if distance <= depth_of_field{
        return 42;
    }    
    return 32;
}

fn main(){
    let mut SCREEN: [u8; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize] = [40; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize];

    let map: [&str; 256] = constants::map();

    let mut now: SystemTime = SystemTime::now();

    let map_width: i32 = 16;
    let map_height: i32 = 16;

    let FOV: f32 = constants::set_fov(4);
    let depth_of_field: f32 = 16.0;
 
    let mut player_X: f32 = 8.0;
    let mut player_Y: f32 = 8.0;
    let mut player_angle: f32 = 0.0;
    let horizontal_sensitivity: f32 = 4.0;
    let player_speed: f32 = 4.0;

    let stdout: Term = Term::buffered_stdout();

    WinConsole::output().clear().unwrap();    

    loop {
        let (elapsed, next) = calculate_fps(now);
        let FPS: f32 = elapsed.as_secs_f32();
        now = next;

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
                else {
                    if map[(ray_y * map_width + ray_x) as usize] == "#"{
                        break;
                    }
                }
            }

            let ceiling = (SCREEN_HEIGHT as f32 / 2.0) - SCREEN_HEIGHT as f32 / (distance_to_wall as f32);
            let floor= SCREEN_HEIGHT - ceiling as i32;

            let shading = calculate_shading(distance_to_wall, depth_of_field);

            for y in 0..SCREEN_HEIGHT{

                if y < ceiling as i32{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = 32;
                }
                else if y > ceiling as i32 && y <= floor{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = shading;
                }
                else{
                    SCREEN[(y*SCREEN_WIDTH+x) as usize] = 32;
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