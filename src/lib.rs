extern crate proc_macro;

#[macro_export]
macro_rules! promise {
    ($b:block) => {
        unsafe { $b }
    };
    ($s:stmt) => {
        unsafe { $s }
    };
    ($e:expr) => {
        unsafe { $e }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = 0;
        let p: *mut i32 = &mut v;
        promise!(*p = 4);
        assert_eq!(v, 4);

        assert_eq!(
            promise!({
                *p = 3;
                *p
            }),
            3
        );

        assert_eq!(promise!(*p), 3);
    }
}
