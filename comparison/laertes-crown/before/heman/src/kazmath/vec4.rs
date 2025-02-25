
extern "C" {
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn kmSQR(s: std::os::raw::c_float) -> std::os::raw::c_float;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [std::os::raw::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec4 {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
    pub w: std::os::raw::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Fill(
    mut pOut: *mut kmVec4,
    mut x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
    mut z: std::os::raw::c_float,
    mut w: std::os::raw::c_float,
) -> *mut kmVec4 {
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Add(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x + (*pV2).x;
    (*pOut).y = (*pV1).y + (*pV2).y;
    (*pOut).z = (*pV1).z + (*pV2).z;
    (*pOut).w = (*pV1).w + (*pV2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Dot(
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> std::os::raw::c_float {
    return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y + (*pV1).z * (*pV2).z
        + (*pV1).w * (*pV2).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Length(mut pIn: *const kmVec4) -> std::os::raw::c_float {
    return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z) + kmSQR((*pIn).w));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4LengthSq(mut pIn: *const kmVec4) -> std::os::raw::c_float {
    return kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z) + kmSQR((*pIn).w);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Lerp(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
    mut t: std::os::raw::c_float,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x + t * ((*pV2).x - (*pV1).x);
    (*pOut).y = (*pV1).y + t * ((*pV2).y - (*pV1).y);
    (*pOut).z = (*pV1).z + t * ((*pV2).z - (*pV1).z);
    (*pOut).w = (*pV1).w + t * ((*pV2).w - (*pV1).w);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Normalize(
    mut pOut: *mut kmVec4,
    mut pIn: *const kmVec4,
) -> *mut kmVec4 {
    if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. && (*pIn).w == 0. {
        return kmVec4Assign(pOut, pIn);
    }
    let mut l = 1.0f32 / kmVec4Length(pIn);
    (*pOut).x = (*pIn).x * l;
    (*pOut).y = (*pIn).y * l;
    (*pOut).z = (*pIn).z * l;
    (*pOut).w = (*pIn).w * l;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Scale(
    mut pOut: *mut kmVec4,
    mut pIn: *const kmVec4,
    s: std::os::raw::c_float,
) -> *mut kmVec4 {
    kmVec4Normalize(pOut, pIn);
    (*pOut).x *= s;
    (*pOut).y *= s;
    (*pOut).z *= s;
    (*pOut).w *= s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Subtract(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x - (*pV2).x;
    (*pOut).y = (*pV1).y - (*pV2).y;
    (*pOut).z = (*pV1).z - (*pV2).z;
    (*pOut).w = (*pV1).w - (*pV2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Mul(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    (*pOut).x = (*pV1).x * (*pV2).x;
    (*pOut).y = (*pV1).y * (*pV2).y;
    (*pOut).z = (*pV1).z * (*pV2).z;
    (*pOut).w = (*pV1).w * (*pV2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Div(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    if (*pV2).x != 0. && (*pV2).y != 0. && (*pV2).z != 0. && (*pV2).w != 0. {
        (*pOut).x = (*pV1).x / (*pV2).x;
        (*pOut).y = (*pV1).y / (*pV2).y;
        (*pOut).z = (*pV1).z / (*pV2).z;
        (*pOut).w = (*pV1).w / (*pV2).w;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4MultiplyMat4(
    mut pOut: *mut kmVec4,
    mut pV: *const kmVec4,
    mut pM: *const kmMat4,
) -> *mut kmVec4 {
    (*pOut)
        .x = (*pV).x * (*pM).mat[0 as std::os::raw::c_int as usize]
        + (*pV).y * (*pM).mat[4 as std::os::raw::c_int as usize]
        + (*pV).z * (*pM).mat[8 as std::os::raw::c_int as usize]
        + (*pV).w * (*pM).mat[12 as std::os::raw::c_int as usize];
    (*pOut)
        .y = (*pV).x * (*pM).mat[1 as std::os::raw::c_int as usize]
        + (*pV).y * (*pM).mat[5 as std::os::raw::c_int as usize]
        + (*pV).z * (*pM).mat[9 as std::os::raw::c_int as usize]
        + (*pV).w * (*pM).mat[13 as std::os::raw::c_int as usize];
    (*pOut)
        .z = (*pV).x * (*pM).mat[2 as std::os::raw::c_int as usize]
        + (*pV).y * (*pM).mat[6 as std::os::raw::c_int as usize]
        + (*pV).z * (*pM).mat[10 as std::os::raw::c_int as usize]
        + (*pV).w * (*pM).mat[14 as std::os::raw::c_int as usize];
    (*pOut)
        .w = (*pV).x * (*pM).mat[3 as std::os::raw::c_int as usize]
        + (*pV).y * (*pM).mat[7 as std::os::raw::c_int as usize]
        + (*pV).z * (*pM).mat[11 as std::os::raw::c_int as usize]
        + (*pV).w * (*pM).mat[15 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Transform(
    mut pOut: *mut kmVec4,
    mut pV: *const kmVec4,
    mut pM: *const kmMat4,
) -> *mut kmVec4 {
    return kmVec4MultiplyMat4(pOut, pV, pM);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4TransformArray(
    mut pOut: *mut kmVec4,
    mut outStride: std::os::raw::c_uint,
    mut pV: *const kmVec4,
    mut vStride: std::os::raw::c_uint,
    mut pM: *const kmMat4,
    mut count: std::os::raw::c_uint,
) -> *mut kmVec4 {
    let mut i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < count {
        let mut in_0 = pV.offset(i.wrapping_mul(vStride) as isize);
        let mut out = pOut.offset(i.wrapping_mul(outStride) as isize);
        kmVec4Transform(out, in_0, pM);
        i = i.wrapping_add(1);
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4AreEqual(
    mut p1: *const kmVec4,
    mut p2: *const kmVec4,
) -> std::os::raw::c_int {
    return (((*p1).x as std::os::raw::c_double) < (*p2).x as std::os::raw::c_double + 0.0001f64
        && (*p1).x as std::os::raw::c_double > (*p2).x as std::os::raw::c_double - 0.0001f64
        && (((*p1).y as std::os::raw::c_double) < (*p2).y as std::os::raw::c_double + 0.0001f64
            && (*p1).y as std::os::raw::c_double > (*p2).y as std::os::raw::c_double - 0.0001f64)
        && (((*p1).z as std::os::raw::c_double) < (*p2).z as std::os::raw::c_double + 0.0001f64
            && (*p1).z as std::os::raw::c_double > (*p2).z as std::os::raw::c_double - 0.0001f64)
        && (((*p1).w as std::os::raw::c_double) < (*p2).w as std::os::raw::c_double + 0.0001f64
            && (*p1).w as std::os::raw::c_double > (*p2).w as std::os::raw::c_double - 0.0001f64))
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Assign(
    mut pOut: *mut kmVec4,
    mut pIn: *const kmVec4,
) -> *mut kmVec4 {
    if pOut != pIn as *mut kmVec4 {} else {
        __assert_fail(
            b"pOut != pIn\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/vec4.c\0" as *const u8 as *const std::os::raw::c_char,
            176 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[std::os::raw::c_char; 47],
            >(b"kmVec4 *kmVec4Assign(kmVec4 *, const kmVec4 *)\0"))
                .as_ptr(),
        );
    }
    memcpy(
        pOut as *mut std::os::raw::c_void,
        pIn as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
