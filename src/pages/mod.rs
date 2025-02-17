macro_rules! export_page {
    ($name:ident) => {
        mod $name;
        #[allow(unused)]
        pub use $name::*;
    };
}

export_page!(home);
