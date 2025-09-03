use expect_test::expect;

use super::check_infer;

#[test]
fn opaque_generics() {
    check_infer(
        r#"
//- minicore: iterator
pub struct Grid {}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a ();

    type IntoIter = impl Iterator<Item = &'a ()>;

    fn into_iter(self) -> Self::IntoIter {
    }
}
    "#,
        expect![[r#"
            150..154 'self': &'a Grid
            174..181 '{     }': impl Iterator<Item = &'a ()>
        "#]],
    );
}

#[test]
fn normalization() {
    check_infer(
        r#"
//- minicore: iterator, iterators
fn main() {
    _ = [0i32].into_iter().filter_map(|_n| Some(1i32));
}
    "#,
        expect![[r#"
            10..69 '{     ...2)); }': ()
            16..17 '_': FilterMap<IntoIter<i32, 1>, impl FnMut(i32) -> Option<i32>>
            16..66 '_ = [0...1i32))': ()
            20..26 '[0i32]': [i32; 1]
            20..38 '[0i32]...iter()': IntoIter<i32, 1>
            20..66 '[0i32]...1i32))': FilterMap<IntoIter<i32, 1>, impl FnMut(i32) -> Option<i32>>
            21..25 '0i32': i32
            50..65 '|_n| Some(1i32)': impl FnMut(i32) -> Option<i32>
            51..53 '_n': i32
            55..59 'Some': fn Some<i32>(i32) -> Option<i32>
            55..65 'Some(1i32)': Option<i32>
            60..64 '1i32': i32
        "#]],
    );
}
