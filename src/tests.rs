#[cfg(test)]
mod tests {
    #[test]
    fn hue_for() {
        // We should do a complete turn of the hue colorspace between A and Z.
        assert_eq!(::hue_for(String::from("0")), 0.0);
        assert_eq!(::hue_for(String::from("a")), 100.0);
        assert_eq!(::hue_for(String::from("b")), 110.0);
        assert_eq!(::hue_for(String::from("c")), 120.0);
        assert_eq!(::hue_for(String::from("i")), 180.0);
        assert_eq!(::hue_for(String::from("cocotte")), 127.0);
        assert_eq!(::hue_for(String::from("images")), 186.0);
        assert_eq!(::hue_for(String::from("z")), 350.0);
        assert_eq!(
            ::hue_for(String::from("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz")),
            360.0
        );
        assert_eq!(
            ::hue_for(String::from("zzzzzzzzzz/zzz//zzzzzzzzzzzzzzzzzzzz")),
            360.0
        );
        assert_eq!(::hue_for(String::from("cloudops")), 126.0);
        assert_eq!(::hue_for(String::from("devops")), 134.0);
        assert_eq!(::hue_for(String::from("go-gitlab")), 167.0);
        assert_eq!(::hue_for(String::from("runbooks")), 279.0);
        assert_eq!(::hue_for(String::from("ansible")), 107.0);
    }
}
