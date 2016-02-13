extern crate memcached_sys;

#[test]
fn test() {
    unsafe {
        memcached_sys::memcached_lib_version();
    }
}
