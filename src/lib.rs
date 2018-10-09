#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
//rotate/flip a quadrant appropriately
#[no_mangle]
pub unsafe extern "C" fn rot(mut n: libc::c_uint, mut x: *mut libc::c_uint,
                             mut y: *mut libc::c_uint, mut rx: libc::c_uint,
                             mut ry: libc::c_uint) -> () {
    if ry == 0i32 as libc::c_uint {
        if rx == 1i32 as libc::c_uint {
            *x = n.wrapping_sub(1i32 as libc::c_uint).wrapping_sub(*x);
            *y = n.wrapping_sub(1i32 as libc::c_uint).wrapping_sub(*y)
        }
        //Swap x and y
        let mut t: libc::c_uint = *x;
        *x = *y;
        *y = t
    };
}
//convert (x,y) to d
#[no_mangle]
pub unsafe extern "C" fn xy2d(mut n: libc::c_uint, mut x: libc::c_uint,
                              mut y: libc::c_uint) -> libc::c_uint {
    let mut rx: libc::c_uint = 0;
    let mut ry: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut d: libc::c_uint = 0i32 as libc::c_uint;
    s = n.wrapping_div(2i32 as libc::c_uint);
    while s > 0i32 as libc::c_uint {
        rx = (x & s > 0i32 as libc::c_uint) as libc::c_int as libc::c_uint;
        ry = (y & s > 0i32 as libc::c_uint) as libc::c_int as libc::c_uint;
        d =
            d.wrapping_add(s.wrapping_mul(s).wrapping_mul((3i32 as
                                                               libc::c_uint).wrapping_mul(rx)
                                                              ^ ry));
        rot(s, &mut x, &mut y, rx, ry);
        s = s.wrapping_div(2i32 as libc::c_uint)
    }
    return d;
}
//convert d to (x,y)
#[no_mangle]
pub unsafe extern "C" fn d2xy(mut n: libc::c_uint, mut d: libc::c_uint,
                              mut x: *mut libc::c_uint,
                              mut y: *mut libc::c_uint) -> () {
    let mut rx: libc::c_uint = 0;
    let mut ry: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut t: libc::c_uint = d;
    *y = 0i32 as libc::c_uint;
    *x = *y;
    s = 1i32 as libc::c_uint;
    while s < n {
        rx = 1i32 as libc::c_uint & t.wrapping_div(2i32 as libc::c_uint);
        ry = 1i32 as libc::c_uint & (t ^ rx);
        rot(s, x, y, rx, ry);
        *x = (*x).wrapping_add(s.wrapping_mul(rx));
        *y = (*y).wrapping_add(s.wrapping_mul(ry));
        t = t.wrapping_div(4i32 as libc::c_uint);
        s = s.wrapping_mul(2i32 as libc::c_uint)
    };
}
