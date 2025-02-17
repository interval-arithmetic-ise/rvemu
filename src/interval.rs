use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Interval32 {
    lower: f32,
    upper: f32,
}

impl Interval32 {
    pub const fn new(lower: f32, upper: f32) -> Self {
        assert!(upper >= lower);
        Self { lower, upper }
    }

    #[inline(always)]
    fn lower(&self) -> f32 {
        self.lower
    }

    #[inline(always)]
    fn upper(&self) -> f32 {
        self.upper
    }

    pub fn unpack(value: f64) -> Self {
        let [a,b,c,d,e,f,g,h] = value.to_le_bytes();
        let lower = f32::from_le_bytes([a,b,c,d]);
        let upper = f32::from_le_bytes([e,f,g,h]);

        Self::new(lower, upper)
    }

    pub fn pack(&self) -> f64 {
        let [a,b,c,d] = self.lower().to_le_bytes();
        let [e,f,g,h] = self.upper().to_le_bytes();

        f64::from_le_bytes([a,b,c,d,e,f,g,h])
    }
}

impl Add for Interval32 {
    type Output = Interval32;

    fn add(self, rhs: Self) -> Self::Output {
        let lower = self.lower + rhs.lower;
        let upper = self.upper + rhs.upper;
        Interval32::new(lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_bounds() {
        let _ = Interval32::new(0.0, 0.0);
        let _ = Interval32::new(1.0, 1.0);
        let _ = Interval32::new(-1.0, 1.0);
        let _ = Interval32::new(-2.0, 2.0);
    }

    #[test]
    #[should_panic]
    fn denies_invalid_bounds() {
        let _ = Interval32::new(1.0, -1.0);
    }

    #[test]
    fn packs_unpacks() {
        let interval1 = Interval32::new(-1.0, 1.0);
        let interval2 = Interval32::unpack(interval1.pack());

        assert_eq!(interval1.lower(), interval2.lower());
        assert_eq!(interval1.upper(), interval2.upper());
    }

    #[test]
    fn unpacks_packs() {
        let interval1 = -0.007812505573383532;
        let interval2 = Interval32::unpack(interval1).pack();

        assert_eq!(interval1, interval2);
    }

    #[test]
    fn adds() {
        let a = Interval32::new(-1.0, 1.0);
        let b = Interval32::new(-2.0, 2.0);
        let c = a + b; 

        assert_eq!(c.lower(), -3.0);
        assert_eq!(c.upper(), 3.0);
    }
}
