extern crate gladiator;
use crate::gladiator::gladiator_struct::Gladiator;
use crate::gladiator::attacks::Attack;


macro_rules! assert_match {
    ($expected:pat, $actual:expr) => {
        if let $expected = $actual {
            assert!(true);
        } else {
            assert!(false, "Expected to match pattern on {:?}", $actual);
        }
    }
}

#[test]
fn test_is_move_known() {
    let mut gladiator: Gladiator = Gladiator::new("Tester".to_string(),5,5,5);
    assert_match!(0,gladiator.get_move_list().len());
    assert_match!(false,gladiator.is_move_known(&Attack::Stab));
    gladiator.add_move(Attack::Stab);
    assert_match!(1,gladiator.get_move_list().len());
    assert_match!(true,gladiator.is_move_known(&Attack::Stab));
    gladiator.add_move(Attack::Stab);
    assert_match!(1,gladiator.get_move_list().len());
}