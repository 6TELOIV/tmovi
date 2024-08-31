#![no_std]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
use agb::{
    display::object::{Graphics, OamManaged, Object, Tag}, include_aseprite, interrupt::VBlank
};

// Globals
const MAX_X: i32 = agb::display::WIDTH - 16;
const MAX_Y: i32 = agb::display::HEIGHT - 16;

// Load GFX from aseprite file
static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");
static PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
static PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");
static BALL: &Tag = GRAPHICS.tags().get("Ball");

// Wrap the 3 paddle sprites together
struct Paddle<'obj> {
    start: Object<'obj>,
    mid: Object<'obj>,
    end: Object<'obj>,
}
impl<'obj> Paddle<'obj> {
    fn new(object: &'obj OamManaged<'_>, start_x: i32, start_y: i32, hflip: bool) -> Self {
        let mut paddle_start = object.object_sprite(PADDLE_END.sprite(0));
        let mut paddle_mid = object.object_sprite(PADDLE_MID.sprite(0));
        let mut paddle_end = object.object_sprite(PADDLE_END.sprite(0));

        paddle_start.set_hflip(hflip).show();
        paddle_mid.set_hflip(hflip).show();
        paddle_end.set_vflip(true).set_hflip(hflip).show();

        let mut paddle = Self {
            start: paddle_start,
            mid: paddle_mid,
            end: paddle_end
        };

        paddle.set_position(start_x, start_y);

        paddle
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.start.set_position((x, y));
        self.mid.set_position((x, y + 16));
        self.end.set_position((x, y + 32));
    }
}

pub fn run_pong(mut gba: agb::Gba) -> ! {
    // System
    let vblank = VBlank::get();
    let object: OamManaged<'_> = gba.display.object.get_managed();
    let mut input = agb::input::ButtonController::new();

    // GFX
    let mut ball = object.object_sprite(BALL.sprite(0));
    ball.set_x(50).set_y(50).show();
    let mut paddle_a = Paddle::new(&object, 8, 8, false);
    let mut paddle_b = Paddle::new(&object, MAX_X - 8, 8, true);

    // Game State
    let mut ball_x = 50;
    let mut ball_y = 50;
    let mut ball_x_velocity = 2;
    let mut ball_y_velocity = 2;

    let paddle_a_x = 8;
    let mut paddle_a_y = 8;
    let paddle_b_x = MAX_X - 8;
    let mut paddle_b_y = 8;

    // Main Loop
    loop {
        
        // Move ball
        ball_x = (ball_x + ball_x_velocity).clamp(0, MAX_X);
        ball_y = (ball_y + ball_y_velocity).clamp(0, MAX_Y);
        
        // Bounce off paddles
        if (ball_x == 24 && ball_y.abs_diff(paddle_a_y + 24) < 24) || (ball_x == MAX_X - 24 && ball_y.abs_diff(paddle_b_y + 24) < 24) {
            ball_x_velocity = -ball_x_velocity;
        }
        
        // Bounce off walls
        if ball_y == 0 || ball_y == MAX_Y {
            ball_y_velocity = -ball_y_velocity;
        }

        // End game if misses paddle
        if ball_x == 0 || ball_x == MAX_X {
            drop(paddle_a);
            drop(paddle_b);
            drop(ball);
            vblank.wait_for_vblank();
            object.commit();
            loop {}
        }

        // Move player paddle by input
        paddle_a_y += input.y_tri() as i32;
        paddle_a_y = paddle_a_y.clamp(0, MAX_Y - 32);

        // Move bot paddle
        if ball_x > MAX_X / 2 {
            // If ball is on bot side, move torward where ball is going
            paddle_b_y += if (paddle_b_y + 24) < ball_y + ball_y_velocity * (MAX_X - 24 - ball_x) {
                1
            } else if (paddle_b_y + 24) > ball_y + ball_y_velocity * (MAX_X - 24 - ball_x) {
                -1
            } else {
                0
            };
        } else {
            // if ball is not on bot side, move torward middle
            paddle_b_y += if (paddle_b_y + 24) < MAX_Y / 2 {
                1
            } else if (paddle_b_y + 24) > MAX_Y / 2 {
                -1
            } else {
                0
            };
        }
        paddle_b_y = paddle_b_y.clamp(0, MAX_Y - 32);
        
        // Move sprites
        ball.set_x(ball_x as u16).set_y(ball_y as u16);
        paddle_a.set_position(paddle_a_x, paddle_a_y);
        paddle_b.set_position(paddle_b_x, paddle_b_y);

        // Frame updates
        vblank.wait_for_vblank();
        object.commit();
        input.update();
    }
}