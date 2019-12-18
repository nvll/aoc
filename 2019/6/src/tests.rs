#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_orbit() {
        let test_data: (&str, u32) = (
            r#"COM)B
              B)C
              C)D
              D)E
              E)F
              B)G
              G)H
              D)I
              E)J
              J)K
              K)L"#,
            42,
        );
        assert_eq!(count_orbits(Some(test_data.0)), test_data.1);
    }

    #[test]
    fn sample_orbit_out_of_order() {
        let test_data: (&str, u32) = (
            r#"B)C
              D)E
              C)D
              E)F
            COM)B
              B)G
              G)H
              D)I
              E)J
              J)K
              K)L"#,
            42,
        );
        assert_eq!(count_orbits(Some(test_data.0)), test_data.1);
    }

    #[test]
    fn orbital_transfers() {
        let test_data: [(&str, u32); 1] = [(
            r#"COM)B
              B)C
              C)D
              D)E
              E)F
              B)G
              G)H
              D)I
              E)J
              J)K
              K)L
              K)YOU
              I)SAN"#,
            4,
        )];

        for test in test_data.iter() {
            assert_eq!(count_orbital_transfers(Some(test.0)), test.1);
        }
    }
}
