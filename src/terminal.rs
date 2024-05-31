pub fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}

pub trait CharRepeat {
    fn repeat(self, times: isize) -> String;
}

impl CharRepeat for char {
    fn repeat(self, times: isize) -> String {
        if times > 0 {
            self.to_string().repeat(times.unsigned_abs())
        } else {
            "".to_string()
        }
    }
}

pub const SPACE: char = ' ';

pub const LINE_HORIZONTAL: char = '─';
pub const LINE_VERTICAL: char = '│';

pub const JOINT_TOP_LEFT: char = '┌';
pub const JOINT_TOP_RIGHT: char = '┐';
pub const JOINT_BOTTOM_LEFT: char = '└';
pub const JOINT_BOTTOM_RIGHT: char = '┘';

pub const BALL: char = '🔵';

// pub const PRIZE: char = '🎁';
// pub const SPARKLES: char = '✨';

// pub const BOMB: char = '💣';
// pub const SKULL: char = '💀';
