//! This crates provides a macro `svec` that allows for Dart-style lists in Rust

use proc_macro_hack::proc_macro_hack;

/// A macro that expands to a literal `Vec`
#[proc_macro_hack]
pub use svec_macro::svec;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let n = 38;
        let x = svec![
            0,
            5,
            for i in 0..10 {
                i
            },
            for i in 0..10 {
                if i > 5 { i } else { 0 }
            },
            if n > 100 { 2 } else if n > 60 { 1 } else { 0 },
            if n < 50 { 10 }
        ];

        assert_eq!(
            x,
            vec![0, 5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 6, 7, 8, 9, 0, 10]
        );
    }
}
