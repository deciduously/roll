#[cfg(test)]
use command::*;
#[cfg(test)]
use roll::Roll;

#[test]
fn test_single_roll_command() {
    assert_eq!(
        Command::Roll(vec![Roll::new("2d6").unwrap()]),
        validate_input(&vec!["2d6".to_string()]).unwrap()
    );
}

#[test]
fn test_mult_command() {
    assert_eq!(
        Command::Multiplier(2, vec!["2d6".to_string(), "1d4".to_string()]),
        validate_input(&vec!["2".to_string(), "2d6".to_string(), "1d4".to_string()]).unwrap()
    )
}

#[test]
fn test_lookup_command() {
    assert_eq!(
        Command::Lookup("blello".to_string()),
        validate_input(&vec!["blello".to_string()]).unwrap()
    );
}
