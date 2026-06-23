/// Clamp `v` into `[lo, hi]`.
pub fn clamp(v: i64, lo: i64, hi: i64) -> i64 {
    v.max(lo).min(hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps() {
        assert_eq!(clamp(10, 0, 5), 5);
        assert_eq!(clamp(-3, 0, 5), 0);
    }
}
