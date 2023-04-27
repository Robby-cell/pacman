// module focused on building walls.
// predefined shapes can be drawn.

pub mod wall_builder {

    use crate::map::Wall;

    // to build a new map
    // or at least the intended purpose is this
    pub fn builder() -> Vec<Wall> {
        let a1 = U_UP(600., 300., 300., 300., 50., [0., 0., 1., 1.]);
        let a2 = L_RIGHT(600., 650., 100., 300., 50., [0., 0., 1., 1.]);
        let a3 = L_LEFT(350., 50., 400., 300., 40., [0., 1., 0., 1.]);

        let mut vect: Vec<Wall> = Vec::new();

        for &v in a1.iter() {
            vect.push(v);
        }
        for &v in a2.iter() {
            vect.push(v);
        }
        for &v in a3.iter() {
            vect.push(v);
        }

        vect
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
