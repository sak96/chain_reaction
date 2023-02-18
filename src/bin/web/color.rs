pub const EXPLOSION_HSL: HSLColor = (0, 100, 0);
pub const EMPTY_HSL: HSLColor = (0, 100, 100);

pub type HSLColor = (usize, usize, usize);

pub fn get_hsl_player_color(id: u8, total: u8) -> HSLColor {
    ((id as usize) * 360 / (total as usize), 50, 50)
}
