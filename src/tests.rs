#[cfg(test)]
mod tests {
    #[test]
    fn hue_for() {
        // We should do a complete turn of the hue colorspace between A and Z.
        assert_eq!(::hue_for(String::from("0")), 0.0);
        // assert_eq!(hue_for(String::from("z")), 350);
        assert_eq!(
            ::hue_for(String::from("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz")),
            360.11102
        );
        assert_eq!(
            ::hue_for(String::from("zzzzzzzzzz/zzz//zzzzzzzzzzzzzzzzzzzz")),
            358.94437
        );
    }
}
