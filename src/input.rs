pub enum Direction{
    w,
    a,
    s,
    d
}

pub fn move_character(key: Direction) -> f32{
    match key{
        Direction::d => {
            return 0.1;
        }
        Direction::a => {
            return -0.1;
        }
        Direction::w=> {
            return 0.0;
        }
        Direction::s=>{
            return 0.0;
        }
    }
}