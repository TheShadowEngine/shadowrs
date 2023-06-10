use crate::zip;

#[macro_export]
macro_rules! zip {
	(@map $p:pat => $t:expr) => {
		|$p| $t
	};
	(@map $p:pat => ($($t:tt)*), $_removed:expr$(, $rest:expr )*) => {
		zip!(@map ($p, b) => ($($t)*, b)$(, $rest )*)
	};
	($a:expr$(,)*) => {
		::std::iter::IntoIterator::into_iter($a)
	};
	($a:expr, $b:expr$(,)*) => {
		zip!($a).zip(zip!($b))
	};
	($a:expr$(, $rest:expr)*$(,)*) => {
		zip!($a)$(.zip($rest))*.map(zip!(@map a => (a)$(, $rest)*))
	};
}

#[test]
fn test_two() {
    let x = &[1, 2, 3];
    let y = &[3, 4, 5];

    assert_eq!(
        zip!(x, y).collect::<Vec<_>>(),
        vec![(&1, &3), (&2, &4), (&3, &5)]
    )
}
