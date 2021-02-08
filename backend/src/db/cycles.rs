#[derive(Debug, PartialEq)]
pub enum Cycle {
    /// [3 -> 6[
    C1,
    /// [6 -> 9[
    C2,
    /// [9 -> 12[
    C3,
    /// [12 -> ...[
    C4,
}

impl Cycle {
    pub fn from_text(text: &str) -> Cycle {
        match text {
            "C1" => Cycle::C1,
            "C2" => Cycle::C2,
            "C3" => Cycle::C3,
            "C4" => Cycle::C4,
            _ => panic!(format!("Invalid cycle: {}", text)),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Cycle::C1 => "C1",
            Cycle::C2 => "C2",
            Cycle::C3 => "C3",
            Cycle::C4 => "C4",
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_from_text() {
        assert_eq!(Cycle::from_text("C1"), Cycle::C1);
        assert_eq!(Cycle::from_text("C2"), Cycle::C2);
        assert_eq!(Cycle::from_text("C3"), Cycle::C3);
        assert_eq!(Cycle::from_text("C4"), Cycle::C4);
    }

    #[test]
    #[should_panic]
    fn cycle_from_text_invalid() {
        Cycle::from_text("Invalid");
    }

    #[test]
    fn cycle_to_text() {
        assert_eq!(Cycle::C1.to_str(), "C1");
        assert_eq!(Cycle::C2.to_str(), "C2");
        assert_eq!(Cycle::C3.to_str(), "C3");
        assert_eq!(Cycle::C4.to_str(), "C4");
    }
}
