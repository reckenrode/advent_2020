pub struct Ship {
    position: (f64, f64),
}
#[derive(Debug)]
pub enum Action {
    MoveNorth(u32),
    MoveSouth(u32),
    MoveEast(u32),
    MoveWest(u32),
}

impl Ship {
    pub fn new() -> Self {
        Ship { position: (0.0, 0.0) }
    }

    pub fn act(&mut self, action: Action) {
        action.apply(self)
    }

    pub fn orientation(&self) -> f64 {
        0.0
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
    }
}

impl Action {
    fn apply(&self, ship: &mut Ship) {
        match self {
            Self::MoveNorth(dy) => ship.position = (0.0, *dy as f64),
            Self::MoveSouth(dy) => ship.position = (0.0, -(*dy as f64)),
            Self::MoveEast(dx) => ship.position = (*dx as f64, 0.0),
            Self::MoveWest(dx) => ship.position = (-(*dx as f64), 0.0),
        }
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

    #[test]
    fn when_the_ship_is_created_it_faces_east() {
        let expected_orientation = 0.0;
        let ship = Ship::new();
        assert_eq!(ship.orientation(), expected_orientation);
    }

    use proptest::prelude::*;

    fn lateral_movement() -> impl Strategy<Value = Action> {
        prop_oneof![
            any::<u32>().prop_map(Action::MoveNorth),
            any::<u32>().prop_map(Action::MoveSouth),
            any::<u32>().prop_map(Action::MoveEast),
            any::<u32>().prop_map(Action::MoveWest),
        ]
    }

    proptest! {
        #[test]
        fn when_the_action_is_north_the_ship_coordinates_change_by_the_specified_value(dist: u32) {
            let expected_position = (0.0, dist as f64);
            let mut ship = Ship::new();
            ship.act(Action::MoveNorth(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_action_is_south_the_ship_coordinates_change_by_the_specified_value(dist: u32) {
            let expected_position = (0.0, -(dist as f64));
            let mut ship = Ship::new();
            ship.act(Action::MoveSouth(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_action_is_east_the_ship_coordinates_change_by_the_specified_value(dist: u32) {
            let expected_position = (dist as f64, 0.0);
            let mut ship = Ship::new();
            ship.act(Action::MoveEast(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_action_is_west_the_ship_coordinates_change_by_the_specified_value(dist: u32) {
            let expected_position = (-(dist as f64), 0.0);
            let mut ship = Ship::new();
            ship.act(Action::MoveWest(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }
    }
}
