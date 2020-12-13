pub struct Ship {
    position: (f64, f64),
    rotation: u16,
}
#[derive(Debug)]
pub enum Action {
    MoveNorth(u16),
    MoveSouth(u16),
    MoveEast(u16),
    MoveWest(u16),
    RotateLeft(u16),
    RotateRight(u16),
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            position: (0.0, 0.0),
            rotation: 0
        }
    }

    pub fn act(&mut self, action: Action) {
        action.apply(self)
    }

    pub fn orientation(&self) -> u16 {
        self.rotation
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
    }

    fn rotate(&mut self, angle: u16, orientation: Orientation) {
        let angle = match orientation {
            Orientation::Right => 360 - (angle % 360),
            Orientation::Left => angle % 360,
        };
        self.rotation = (self.rotation + angle) % 360;
    }
}

enum Orientation {
    Left, Right
}

impl Action {
    fn apply(&self, ship: &mut Ship) {
        let (x, y) = ship.position;
        match self {
            Self::MoveNorth(dy) => ship.position = (x, y + *dy as f64),
            Self::MoveSouth(dy) => ship.position = (x, y - *dy as f64),
            Self::MoveEast(dx) => ship.position = (x + *dx as f64, y),
            Self::MoveWest(dx) => ship.position = (x - *dx as f64, y),
            Self::RotateLeft(theta) => ship.rotate(*theta, Orientation::Left),
            Self::RotateRight(theta) => ship.rotate(*theta, Orientation::Right),
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
        let expected_orientation = 0;
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

    fn rotation() -> impl Strategy<Value = Action> {
        prop_oneof![
            any::<u16>().prop_map(Action::RotateLeft),
            any::<u16>().prop_map(Action::RotateRight),
        ]
    }

    fn add_angles(theta1: i32, theta2: i32) -> u16 {
        let result = (theta1 + theta2) % 360;
        if result < 0 {
            (result + 360) as u16
        } else {
            result as u16
        }
    }

    fn position_from_action(action: &Action) -> (f64, f64) {
        match action {
            Action::MoveNorth(dy) => (0.0, *dy as f64),
            Action::MoveSouth(dy) => (0.0, -(*dy as f64)),
            Action::MoveEast(dx) => (*dx as f64, 0.0),
            Action::MoveWest(dx) => (-(*dx as f64), 0.0),
            _ => panic!("only lateral movements are supported (not rotation or forward)")
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
            let (a1x, a1y) = position_from_action(&a1);
            let (a2x, a2y) = position_from_action(&a2);
            let expected_position = (a1x + a2x, a1y + a2y);
            let mut ship = Ship::new();
            ship.act(a1);
            ship.act(a2);
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_ship_rotates_left_its_orientation_reflects_the_change(rotation: u16) {
            let expected_orientation = rotation % 360;
            let mut ship = Ship::new();
            ship.act(Action::RotateLeft(rotation));
            prop_assert_eq!(ship.orientation(), expected_orientation)
        }

        #[test]
        fn when_the_ship_rotates_right_its_orientation_reflects_the_change(rotation: u16) {
            let expected_orientation = (360 - (rotation % 360)) % 360;
            let mut ship = Ship::new();
            ship.act(Action::RotateRight(rotation));
            prop_assert_eq!(ship.orientation(), expected_orientation)
        }

        #[test]
        fn when_the_ship_rotates_it_rotates_from_its_current_orientation(fst in rotation(), snd in rotation()) {
            let expected_orientation = match (&fst, &snd) {
                (Action::RotateLeft(theta1), Action::RotateLeft(theta2)) => add_angles(*theta1 as i32, *theta2 as i32),
                (Action::RotateLeft(theta1), Action::RotateRight(theta2)) => add_angles(*theta1 as i32, -(*theta2 as i32)),
                (Action::RotateRight(theta1), Action::RotateLeft(theta2)) => add_angles(-(*theta1 as i32), *theta2 as i32),
                (Action::RotateRight(theta1), Action::RotateRight(theta2)) => add_angles(-(*theta1 as i32), -(*theta2 as i32)),
                _ => panic!("shut up rust")
            };
            let mut ship = Ship::new();
            ship.act(fst);
            ship.act(snd);
            prop_assert_eq!(ship.orientation(), expected_orientation)
        }

        #[test]
        fn when_the_ship_rotates_it_stays_at_its_current_position(pos in lateral_movement(), rotation in rotation()) {
            let expected_position = position_from_action(&pos);
            let mut ship = Ship::new();
            ship.act(pos);
            ship.act(rotation);
            prop_assert_eq!(ship.position(), expected_position);
        }

        #[test]
        fn when_the_ship_is_rotated_it_does_not_affect_lateral_movement(pos in lateral_movement(), rotation in rotation()) {
            let expected_position = position_from_action(&pos);
            let mut ship = Ship::new();
            ship.act(rotation);
            ship.act(pos);
            prop_assert_eq!(ship.position(), expected_position);
        }
    }
}
