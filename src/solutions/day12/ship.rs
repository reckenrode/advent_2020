pub struct Ship {

}

impl Ship {
    pub fn new() -> Self {
        Ship {}
    }

    pub fn position(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_the_ship_is_created_it_is_positioned_at_the_origin() {
        let expected_position = (0.0, 0.0);
        let ship = Ship::new();
        assert_eq!(ship.position(), expected_position);
    }

    // use proptest::prelude::*;

    // proptest! {
    //     #[test]
    //     fn when_the_action_is_north_the_ship_coordinates_change_by_the_specified_value(dist: u32) {
    //         let expectedPosition = (0, dist);
    //         let ship = Ship::new();
    //         ship.move(Action(dist));
    //         prop_assert_eq!(ship.position, expectedPosition);
    //     }
    // }
}
