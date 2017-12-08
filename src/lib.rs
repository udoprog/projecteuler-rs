#![feature(test)]

extern crate test;

#[macro_export]
macro_rules! problem {
    (tests => [$($name:ident => ($exp:expr, $test:expr),)*]; $($rest:tt)*) => {
        #[cfg(test)]
        mod benches {
            #[allow(unused)]
            use super::*;

            $(
            #[bench]
            fn $name(b: &mut ::test::Bencher) {
                b.iter(|| $exp);
            }
            )*
        }

        #[cfg(test)]
        mod tests {
            #[allow(unused)]
            use super::*;

            $(
            #[test]
            fn $name() {
                assert_eq!($test, $exp);
            }
            )*
        }

        pub fn run_all(name: &str) {
            println!("# {}", name);
            $(println!("{:10} => {} = {}", stringify!($name), stringify!($test), $test);)*
        }

        problem!($($rest)*);
    };

    () => {
    };
}

macro_rules! modules {
    ($($mod:ident,)*) => {
        $(mod $mod;)*

        pub fn run_all() {
            $(self::$mod::run_all(stringify!($mod));)*
        }
    }
}

modules![
    p001,
    p002,
    p003,
];
