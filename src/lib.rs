#[macro_export]
/// See [`std::assert_eq`] for more information.
/// Main differences:
/// 1. Only accepts arguments which `impl AsRef<str>`
/// 2. Output on assertion failure is printed to stdout as a `Vec` of [Chunks](https://docs.rs/dissimilar/*/dissimilar/enum.Chunk.html)
macro_rules! str_assert_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left == right)`
  diff: {:#?}"#, dissimilar::diff(&*left_val, &*right_val))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left == right)`
  diff: {:#?}: {}"#, dissimilar::diff(&*left_val, &*right_val),
                           format_args!($($arg)+))
                }
            }
        }
    });
}

#[macro_export]
/// See [`std::assert_ne`] for more information.
/// Main differences:
/// 1. Only accepts arguments which `impl AsRef<str>`
/// 2. Output on assertion failure is printed to stdout as a `Vec` of [Chunks](https://docs.rs/dissimilar/*/dissimilar/enum.Chunk.html)
macro_rules! str_assert_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left != right)`
  diff: {:#?}"#, dissimilar::diff(&*left_val, &*right_val))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left != right)`
  diff: {:#?}: {}"#, dissimilar::diff(&*left_val, &*right_val),
                           format_args!($($arg)+))
                }
            }
        }
    });
}

#[macro_export]
/// See [`std::debug_assert_eq`] for more information.
/// Main differences:
/// 1. Only accepts arguments which `impl AsRef<str>`
/// 2. Output on assertion failure is printed to stdout as a `Vec` of [Chunks](https://docs.rs/dissimilar/*/dissimilar/enum.Chunk.html)
macro_rules! debug_str_assert_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { str_assert_eq!($($arg)*); })
}

#[macro_export]
/// See [`std::debug_assert_ne`] for more information.
/// Main differences:
/// 1. Only accepts arguments which `impl AsRef<str>`
/// 2. Output on assertion failure is printed to stdout as a `Vec` of [Chunks](https://docs.rs/dissimilar/*/dissimilar/enum.Chunk.html)
macro_rules! debug_str_assert_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { str_assert_ne!($($arg)*); })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        str_assert_eq!("Lorem ipsum doleret", "Lorem ipsum doleret");
    }

    #[test]
    #[should_panic]
    fn has_diff() {
        str_assert_eq!("Lorem ipsum doleret", "Lorem ipsum dolert", "Eror");
    }

    #[test]
    fn expect_diff() {
        str_assert_ne!("Lorem ipsum doleret", "Lorem ipsum dolert");
    }
}
