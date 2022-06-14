use crate::prelude::*;

pub struct Obstacle {
    pub x: i32,
    pub gap_y: i32,
    pub size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(5, 20),
            size: i32::max(2, 10 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        // The ground
        for x in 0..SCREEN_WIDTH {
            ctx.set(x, SCREEN_HEIGHT - 1, WHITE, WHITE, to_cp437('#'));
        }

        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        // Top wall
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, WHITE, NAVY, 179);
        }

        // Bottom wall - now leaving room for the ground
        for y in self.gap_y + half_size..SCREEN_HEIGHT - 1 {
            ctx.set(screen_x, y, WHITE, NAVY, 179);
        }
    }

    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        player.x == self.x
            && ((player.y as i32) < self.gap_y - half_size
                || player.y as i32 > self.gap_y + half_size)
    }
}
