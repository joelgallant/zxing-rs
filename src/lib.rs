pub mod bindings {
    #![allow(unused)]
    mod raw {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }

    pub use self::raw::root::zxing::*;
}
