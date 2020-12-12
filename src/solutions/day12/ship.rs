pub struct Ship {
    position: (f64, f64),
}
#[derive(Debug)]
pub enum Action {
    MoveNorth(u16),
    MoveSouth(u16),
    MoveEast(u16),
    MoveWest(u16),
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
        let (x, y) = ship.position;
        match self {
            Self::MoveNorth(dy) => ship.position = (x, y + *dy as f64),
            Self::MoveSouth(dy) => ship.position = (x, y - *dy as f64),
            Self::MoveEast(dx) => ship.position = (x + *dx as f64, y),
            Self::MoveWest(dx) => ship.position = (x - *dx as f64, y),
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
            any::<u16>().prop_map(Action::MoveNorth),
            any::<u16>().prop_map(Action::MoveSouth),
            any::<u16>().prop_map(Action::MoveEast),
            any::<u16>().prop_map(Action::MoveWest),
        ]
    }

    fn position_from_action(action: &Action) -> (f64, f64) {
        match action {
            Action::MoveNorth(dy) => (0.0, *dy as f64),
            Action::MoveSouth(dy) => (0.0, -(*dy as f64)),
            Action::MoveEast(dx) => (*dx as f64, 0.0),
            Action::MoveWest(dx) => (-(*dx as f64), 0.0),
        }
    }

    proptest! {
        #[test]
        fn when_the_action_is_north_the_ship_coordinates_change_by_the_specified_value(dist: u16) {
            let expected_position = (0.0, dist as f64);
            let mut ship = Ship::new();
            ship.act(Action::MoveNorth(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_action_is_south_the_ship_coordinates_change_by_the_specified_value(dist: u16) {
            let expected_position = (0.0, -(dist as f64));
            let mut ship = Ship::new();
            ship.act(Action::MoveSouth(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_action_is_east_the_ship_coordinates_change_by_the_specified_value(dist: u16) {
            let expected_position = (dist as f64, 0.0);
            let mut ship = Ship::new();
            ship.act(Action::MoveEast(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_action_is_west_the_ship_coordinates_change_by_the_specified_value(dist: u16) {
            let expected_position = (-(dist as f64), 0.0);
            let mut ship = Ship::new();
            ship.act(Action::MoveWest(dist));
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_ship_moves_it_starts_from_its_current_position(a1 in lateral_movement(), a2 in lateral_movement()) {
            let a1 = Action::MoveNorth(1);
            let a2 = Action::MoveWest(0);
            let (a1x, a1y) = position_from_action(&a1);
            let (a2x, a2y) = position_from_action(&a2);
            let expected_position = (a1x + a2x, a1y + a2y);
            let mut ship = Ship::new();
            ship.act(a1);
            ship.act(a2);
            prop_assert_eq!(ship.position(), expected_position);
        }
    }
}
