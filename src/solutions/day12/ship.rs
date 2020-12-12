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
}
