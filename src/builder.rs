// module focused on building walls.
// predefined shapes can be drawn.

pub mod builder {

    use crate::utilities::HALF_PLAYER as HALF_UNIT;
    use crate::utilities::PLAYER_SIZE as UNIT;
    use crate::{
        corner::Corner,
        map::Wall,
        utilities::{Direction, BLUE, GREEN, RED, YELLOW},
    };

    pub const ORIGIN: f64 = 0_f64;

    // to build a new map
    // or at least the intended purpose is this
    // this is a sample map
    pub fn new_map() -> (Vec<Wall>, Vec<Corner>) {
        // UNIT is the size of a unit

        let wall_holder: Box<[Box<[Wall]>]> = Box::new([
            Box::new([Wall::new((UNIT, UNIT), UNIT, HALF_UNIT, BLUE)]),
            Box::new([Wall::new((ORIGIN, 2.5 * UNIT), 2. * UNIT, UNIT, BLUE)]),


            // G shape at top left
            Box::new(U_RIGHT(3. * UNIT, UNIT, 4.5 * UNIT, 4. * UNIT, UNIT, BLUE)),
        ]);

        let mut walls: Vec<Wall> = Vec::new();

        for wall_pattern in wall_holder.iter() {
            for &wall in wall_pattern.iter() {
                walls.push(wall);
            }
        }

        /*
        for &v in w1.iter() {
            walls.push(v);
        }
        for &v in w2.iter() {
            walls.push(v);
        }
        for &v in w3.iter() {
            walls.push(v);
        }
        */

        let corners = vec![
            Corner::new(700., 700., Box::new([Direction::Up, Direction::Right])),
            //Corner::new(x, y, directions)
        ];

        (walls, corners)
    }

    // top left is x, y
    // self explanatory
    #[allow(non_snake_case)]
    pub fn L_RIGHT(
        x: f64,
        y: f64,
        height: f64,
        width: f64,
        thickness: f64,
        color: [f32; 4],
    ) -> [Wall; 2] {
        [
            Wall::new((x, y), thickness, height, color),
            Wall::new(
                (x + thickness, y + height - thickness),
                width - thickness,
                thickness,
                color,
            ),
        ]
    }

    // top right is x, y
    // will draw a wall from that point, taking up thickness, height; left, down
    // a wall of height thickness, and width of width is drawn to the left of the bottom of this wall
    #[allow(non_snake_case)]
    pub fn L_LEFT(
        x: f64,
        y: f64,
        height: f64,
        width: f64,
        thickness: f64,
        color: [f32; 4],
    ) -> [Wall; 2] {
        [
            Wall::new((x - thickness, y), thickness, height, color),
            Wall::new(
                (x - width, y + height - thickness),
                width - thickness,
                thickness,
                color,
            ),
        ]
    }

    // top left is x, y
    // height of U shape is height, width is width
    #[allow(non_snake_case)]
    pub fn U_UP(
        x: f64,
        y: f64,
        height: f64,
        width: f64,
        thickness: f64,
        color: [f32; 4],
    ) -> [Wall; 3] {
        [
            Wall::new((x, y), thickness, height, color),
            Wall::new(
                (x + thickness, y + height - thickness),
                width - thickness,
                thickness,
                color,
            ),
            Wall::new(
                (x + width - thickness, y),
                thickness,
                height - thickness,
                color,
            ),
        ]
    }

    #[allow(non_snake_case)]
    pub fn U_DOWN(
        x: f64,
        y: f64,
        height: f64,
        width: f64,
        thickness: f64,
        color: [f32; 4],
    ) -> [Wall; 3] {
        [
            Wall::new((x, y), thickness, height, color),
            Wall::new((x + thickness, y), width - thickness, thickness, color),
            Wall::new(
                (x + width - thickness, y + thickness),
                thickness,
                height - thickness,
                color,
            ),
        ]
    }

    #[allow(non_snake_case)]
    pub fn U_RIGHT(
        x: f64,
        y: f64,
        height: f64,
        width: f64,
        thickness: f64,
        color: [f32; 4],
    ) -> [Wall; 3] {
        [
            Wall::new((x, y), width, thickness, color),
            Wall::new((x, y + thickness), thickness, height - thickness, color),
            Wall::new(
                (x + thickness, y + height - thickness),
                width - thickness,
                thickness,
                color,
            ),
        ]
    }

    #[allow(non_snake_case)]
    pub fn T_UP(
        x: f64,
        y: f64,
        height: f64,
        width: f64,
        thickness: f64,
        color: [f32; 4],
        body_position: f64,
    ) -> [Wall; 2] {
        assert!(body_position >= 0_f64 && body_position <= 1_f64);
        [
            Wall::new((x, y), width, thickness, color),
            Wall::new(
                ((x - thickness) * body_position, y + thickness), // to offset it.
                // 0.5 will make the middle of the vertical part of the T be in the middle
                thickness,
                height - thickness,
                color,
            ),
        ]
    }
}
