macro_rules! export_component {
    ($name:ident) => {
        mod $name;
        #[allow(unused)]
        pub use $name::*;
    };
}

export_component!(navbar);
