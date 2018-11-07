#[cfg(test)]
mod tests {
    #[test]
    fn hue_for() {
        // We should do a complete turn of the hue colorspace between A and Z.
        assert_eq!(::hue_for(String::from("0")), 0.0);
        assert_eq!(::hue_for(String::from("a")), 10.0);
        assert_eq!(::hue_for(String::from("b")), 11.0);
        assert_eq!(::hue_for(String::from("c")), 12.0);
        assert_eq!(::hue_for(String::from("i")), 18.0);
        assert_eq!(::hue_for(String::from("cocotte")), 144.0);
        assert_eq!(::hue_for(String::from("images")), 108.0);
        assert_eq!(::hue_for(String::from("z")), 35.0);
        //assert_eq!(
        //    ::hue_for(String::from("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz")),
        //    360.11102
        //);
        //assert_eq!(
        //    ::hue_for(String::from("zzzzzzzzzz/zzz//zzzzzzzzzzzzzzzzzzzz")),
        //    358.94437
        //);
        assert_eq!(::hue_for(String::from("cloudops")), 177.0);
        assert_eq!(::hue_for(String::from("devops")), 135.0);
        assert_eq!(::hue_for(String::from("go-gitlab")), 145.0);
        assert_eq!(::hue_for(String::from("runbooks")), 187.0);
        assert_eq!(::hue_for(String::from("ansible")), 125.0);
    }
}
