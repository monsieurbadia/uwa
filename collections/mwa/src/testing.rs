/// this represents a simple unit test wrapper
pub macro unit {
  ( $( $e:item )* ) => { $( $e )* },
}

/// this represents a test case
pub macro test {
  ( $name:ident , $assertions:expr ) => (
    #[test]
    fn $name() { $assertions }
  )
}

/// this represents an assertion
pub macro must {
  ( die : $rhs:expr ) => ({
    let catched = std::panic::catch_unwind(|| { $rhs; });

    must!(catched.is_err());
  }),
  ( falsy : $rhs:expr ) => ({ assert!(false == $rhs); }),
  ( truthy : $rhs:expr ) => ({ assert!(true == $rhs); }),
  ( ne : $lhs:expr , $rhs:expr ) => ({ assert_ne!($lhs, $rhs); }),
  ( eq : $lhs:expr , $rhs:expr ) => ({ assert_eq!($lhs, $rhs); }),
  ( $lhs:expr ) => ({ assert!($lhs == true); }),
}
