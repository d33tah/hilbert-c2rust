
#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
extern crate libc;
//rotate/flip a quadrant appropriately
unsafe extern "C" fn rot(mut n: libc::c_uint, mut x: *mut libc::c_uint,
                             mut y: *mut libc::c_uint, mut _rx: libc::c_uint,
                             mut _ry: libc::c_uint) -> () {
    if _ry == 0i32 as libc::c_uint {
        if _rx == 1i32 as libc::c_uint {
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
unsafe extern "C" fn _xy2d(mut n: libc::c_uint, mut x: libc::c_uint,
                              mut y: libc::c_uint) -> libc::c_uint {
    let mut _rx: libc::c_uint = 0;
    let mut _ry: libc::c_uint = 0;
    let mut _s: libc::c_uint = 0;
    let mut d: libc::c_uint = 0i32 as libc::c_uint;
    _s = n.wrapping_div(2i32 as libc::c_uint);
    while _s > 0i32 as libc::c_uint {
        _rx = (x & _s > 0i32 as libc::c_uint) as libc::c_int as libc::c_uint;
        _ry = (y & _s > 0i32 as libc::c_uint) as libc::c_int as libc::c_uint;
        d =
            d.wrapping_add(_s.wrapping_mul(_s).wrapping_mul((3i32 as
                                                               libc::c_uint).wrapping_mul(_rx)
                                                              ^ _ry));
        rot(_s, &mut x, &mut y, _rx, _ry);
        _s = _s.wrapping_div(2i32 as libc::c_uint)
    }
    return d;
}
//convert d to (x,y)
unsafe extern "C" fn _d2xy(mut n: libc::c_uint, mut d: libc::c_uint,
                              mut x: *mut libc::c_uint,
                              mut y: *mut libc::c_uint) -> () {
    let mut _rx: libc::c_uint = 0;
    let mut _ry: libc::c_uint = 0;
    let mut _s: libc::c_uint = 0;
    let mut t: libc::c_uint = d;
    *y = 0i32 as libc::c_uint;
    *x = *y;
    _s = 1i32 as libc::c_uint;
    while _s < n {
        _rx = 1i32 as libc::c_uint & t.wrapping_div(2i32 as libc::c_uint);
        _ry = 1i32 as libc::c_uint & (t ^ _rx);
        rot(_s, x, y, _rx, _ry);
        *x = (*x).wrapping_add(_s.wrapping_mul(_rx));
        *y = (*y).wrapping_add(_s.wrapping_mul(_ry));
        t = t.wrapping_div(4i32 as libc::c_uint);
        _s = _s.wrapping_mul(2i32 as libc::c_uint)
    };
}

pub fn xy2d(x: libc::c_uint, y: libc::c_uint, n: libc::c_uint) -> libc::c_uint {
    unsafe {
        return _xy2d(x, y, n);
    }
}

pub fn d2xy(n: libc::c_uint, d: libc::c_uint) -> (libc::c_uint, libc::c_uint) {
    unsafe {
        let mut x: libc::c_uint = 0;
        let mut y: libc::c_uint = 0;
        _d2xy(n, d, &mut x as *mut libc::c_uint, &mut y as *mut libc::c_uint);
        return (x, y);
    }
}

#[test]
fn xy2d_simple() {
    assert_eq!(xy2d(1,1,1), 0);
    assert_eq!(xy2d(1,1,2), 0);
    assert_eq!(xy2d(2,1,4), 3);
}

#[test]
fn d2xy_simple() {
    assert_eq!(d2xy(2, 0), (0, 0));
    assert_eq!(d2xy(2, 1), (0, 1));
    assert_eq!(d2xy(2, 2), (1, 1));
    assert_eq!(d2xy(2, 3), (1, 0));
}
