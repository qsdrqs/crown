
extern "C" {
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn memset(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn memcmp(
        _: *const std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn cosf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sinf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn kmDegreesToRadians(degrees: std::os::raw::c_float) -> std::os::raw::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
    ) -> *mut kmVec3;
    fn kmVec3Subtract(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
    ) -> *mut kmVec3;
    fn kmVec3MultiplyMat4(
        pOut: *mut kmVec3,
        pV: *const kmVec3,
        pM: *const kmMat4,
    ) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    static KM_VEC3_NEG_Z: kmVec3;
    static KM_VEC3_POS_Z: kmVec3;
    static KM_VEC3_POS_Y: kmVec3;
    static KM_VEC3_POS_X: kmVec3;
    fn kmQuaternionRotationAxisAngle(
        pOut: *mut kmQuaternion,
        pV: *const kmVec3,
        angle: std::os::raw::c_float,
    ) -> *mut kmQuaternion;
    fn kmQuaternionRotationMatrix(
        pOut: *mut kmQuaternion,
        pIn: *const kmMat3,
    ) -> *mut kmQuaternion;
    fn kmQuaternionToAxisAngle(
        pIn: *const kmQuaternion,
        pVector: *mut kmVec3,
        pAngle: *mut std::os::raw::c_float,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [std::os::raw::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [std::os::raw::c_float; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmPlane {
    pub a: std::os::raw::c_float,
    pub b: std::os::raw::c_float,
    pub c: std::os::raw::c_float,
    pub d: std::os::raw::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmQuaternion {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
    pub w: std::os::raw::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Fill(
    mut pOut: *mut kmMat4,
    mut pMat: *const std::os::raw::c_float,
) -> *mut kmMat4 {
    memcpy(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        pMat as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Identity(mut pOut: *mut kmMat4) -> *mut kmMat4 {
    memset(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    let ref mut fresh0 = (*pOut).mat[15 as std::os::raw::c_int as usize];
    *fresh0 = 1.0f32;
    let ref mut fresh1 = (*pOut).mat[10 as std::os::raw::c_int as usize];
    *fresh1 = *fresh0;
    let ref mut fresh2 = (*pOut).mat[5 as std::os::raw::c_int as usize];
    *fresh2 = *fresh1;
    (*pOut).mat[0 as std::os::raw::c_int as usize] = *fresh2;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Inverse(
    mut pOut: *mut kmMat4,
    mut pM: *const kmMat4,
) -> *mut kmMat4 {
    let mut tmp = kmMat4 { mat: [0.; 16] };
    let mut det: std::os::raw::c_double = 0.;
    let mut i: std::os::raw::c_int = 0;
    tmp
        .mat[0 as std::os::raw::c_int
        as usize] = (*pM).mat[5 as std::os::raw::c_int as usize]
        * (*pM).mat[10 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[13 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        - (*pM).mat[13 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[4 as std::os::raw::c_int
        as usize] = -(*pM).mat[4 as std::os::raw::c_int as usize]
        * (*pM).mat[10 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[8 as std::os::raw::c_int
        as usize] = (*pM).mat[4 as std::os::raw::c_int as usize]
        * (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[5 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[5 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[12 as std::os::raw::c_int
        as usize] = -(*pM).mat[4 as std::os::raw::c_int as usize]
        * (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[10 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[5 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[5 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[1 as std::os::raw::c_int
        as usize] = -(*pM).mat[1 as std::os::raw::c_int as usize]
        * (*pM).mat[10 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[1 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[13 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        + (*pM).mat[13 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[5 as std::os::raw::c_int
        as usize] = (*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[10 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[9 as std::os::raw::c_int
        as usize] = -(*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[13 as std::os::raw::c_int
        as usize] = (*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[10 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[2 as std::os::raw::c_int
        as usize] = (*pM).mat[1 as std::os::raw::c_int as usize]
        * (*pM).mat[6 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[1 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[13 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[7 as std::os::raw::c_int as usize]
        - (*pM).mat[13 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[6 as std::os::raw::c_int
        as usize] = -(*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[6 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[7 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[10 as std::os::raw::c_int
        as usize] = (*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[15 as std::os::raw::c_int as usize]
        - (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        - (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[15 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[7 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[5 as std::os::raw::c_int as usize];
    tmp
        .mat[14 as std::os::raw::c_int
        as usize] = -(*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[14 as std::os::raw::c_int as usize]
        + (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[14 as std::os::raw::c_int as usize]
        - (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[13 as std::os::raw::c_int as usize]
        - (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[6 as std::os::raw::c_int as usize]
        + (*pM).mat[12 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[5 as std::os::raw::c_int as usize];
    tmp
        .mat[3 as std::os::raw::c_int
        as usize] = -(*pM).mat[1 as std::os::raw::c_int as usize]
        * (*pM).mat[6 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
        + (*pM).mat[1 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize]
        + (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        - (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize]
        - (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[7 as std::os::raw::c_int as usize]
        + (*pM).mat[9 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[7 as std::os::raw::c_int
        as usize] = (*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[6 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
        - (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize]
        - (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[7 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[11 as std::os::raw::c_int
        as usize] = -(*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[11 as std::os::raw::c_int as usize]
        + (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[7 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[11 as std::os::raw::c_int as usize]
        - (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[7 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[3 as std::os::raw::c_int as usize]
            * (*pM).mat[5 as std::os::raw::c_int as usize];
    tmp
        .mat[15 as std::os::raw::c_int
        as usize] = (*pM).mat[0 as std::os::raw::c_int as usize]
        * (*pM).mat[5 as std::os::raw::c_int as usize] * (*pM).mat[10 as std::os::raw::c_int as usize]
        - (*pM).mat[0 as std::os::raw::c_int as usize] * (*pM).mat[6 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize]
        - (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[10 as std::os::raw::c_int as usize]
        + (*pM).mat[4 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[9 as std::os::raw::c_int as usize]
        + (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[1 as std::os::raw::c_int as usize]
            * (*pM).mat[6 as std::os::raw::c_int as usize]
        - (*pM).mat[8 as std::os::raw::c_int as usize] * (*pM).mat[2 as std::os::raw::c_int as usize]
            * (*pM).mat[5 as std::os::raw::c_int as usize];
    det = ((*pM).mat[0 as std::os::raw::c_int as usize] * tmp.mat[0 as std::os::raw::c_int as usize]
        + (*pM).mat[1 as std::os::raw::c_int as usize] * tmp.mat[4 as std::os::raw::c_int as usize]
        + (*pM).mat[2 as std::os::raw::c_int as usize] * tmp.mat[8 as std::os::raw::c_int as usize]
        + (*pM).mat[3 as std::os::raw::c_int as usize] * tmp.mat[12 as std::os::raw::c_int as usize])
        as std::os::raw::c_double;
    if det == 0 as std::os::raw::c_int as std::os::raw::c_double {
        return 0 as *mut kmMat4;
    }
    det = 1.0f64 / det;
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        (*pOut)
            .mat[i
            as usize] = (tmp.mat[i as usize] as std::os::raw::c_double * det) as std::os::raw::c_float;
        i += 1;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4IsIdentity(mut pIn: *const kmMat4) -> std::os::raw::c_int {
    static mut identity: [std::os::raw::c_float; 16] = [
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
    ];
    return (memcmp(
        identity.as_mut_ptr() as *const std::os::raw::c_void,
        ((*pIn).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Transpose(
    mut pOut: *mut kmMat4,
    mut pIn: *const kmMat4,
) -> *mut kmMat4 {
    let mut x: std::os::raw::c_int = 0;
    let mut z: std::os::raw::c_int = 0;
    z = 0 as std::os::raw::c_int;
    while z < 4 as std::os::raw::c_int {
        x = 0 as std::os::raw::c_int;
        while x < 4 as std::os::raw::c_int {
            (*pOut)
                .mat[(z * 4 as std::os::raw::c_int + x)
                as usize] = (*pIn).mat[(x * 4 as std::os::raw::c_int + z) as usize];
            x += 1;
        }
        z += 1;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Multiply(
    mut pOut: *mut kmMat4,
    mut pM1: *const kmMat4,
    mut pM2: *const kmMat4,
) -> *mut kmMat4 {
    let mut mat: [std::os::raw::c_float; 16] = [0.; 16];
    let mut m1 = ((*pM1).mat).as_ptr();
    let mut m2 = ((*pM2).mat).as_ptr();
    mat[0 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[1 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[2 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[3 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[4 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[5 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[6 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[7 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[8 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[9 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[10 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[11 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[12 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    mat[13 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    mat[14 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    mat[15 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    memcpy(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        mat.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Assign(
    mut pOut: *mut kmMat4,
    mut pIn: *const kmMat4,
) -> *mut kmMat4 {
    if pOut != pIn as *mut kmMat4
        && !(b"You have tried to self-assign!!\0" as *const u8 as *const std::os::raw::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pOut != pIn && \"You have tried to self-assign!!\"\0" as *const u8
                as *const std::os::raw::c_char,
            b"../kazmath/mat4.c\0" as *const u8 as *const std::os::raw::c_char,
            272 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[std::os::raw::c_char; 47],
            >(b"kmMat4 *kmMat4Assign(kmMat4 *, const kmMat4 *)\0"))
                .as_ptr(),
        );
    }
    memcpy(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        ((*pIn).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4AssignMat3(
    mut pOut: *mut kmMat4,
    mut pIn: *const kmMat3,
) -> *mut kmMat4 {
    kmMat4Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = (*pIn).mat[0 as std::os::raw::c_int as usize];
    (*pOut).mat[1 as std::os::raw::c_int as usize] = (*pIn).mat[1 as std::os::raw::c_int as usize];
    (*pOut).mat[2 as std::os::raw::c_int as usize] = (*pIn).mat[2 as std::os::raw::c_int as usize];
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = (*pIn).mat[3 as std::os::raw::c_int as usize];
    (*pOut).mat[5 as std::os::raw::c_int as usize] = (*pIn).mat[4 as std::os::raw::c_int as usize];
    (*pOut).mat[6 as std::os::raw::c_int as usize] = (*pIn).mat[5 as std::os::raw::c_int as usize];
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = (*pIn).mat[6 as std::os::raw::c_int as usize];
    (*pOut).mat[9 as std::os::raw::c_int as usize] = (*pIn).mat[7 as std::os::raw::c_int as usize];
    (*pOut).mat[10 as std::os::raw::c_int as usize] = (*pIn).mat[8 as std::os::raw::c_int as usize];
    (*pOut).mat[11 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4AreEqual(
    mut pMat1: *const kmMat4,
    mut pMat2: *const kmMat4,
) -> std::os::raw::c_int {
    let mut i = 0 as std::os::raw::c_int;
    if pMat1 != pMat2
        && !(b"You are comparing the same thing!\0" as *const u8 as *const std::os::raw::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pMat1 != pMat2 && \"You are comparing the same thing!\"\0" as *const u8
                as *const std::os::raw::c_char,
            b"../kazmath/mat4.c\0" as *const u8 as *const std::os::raw::c_char,
            308 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[std::os::raw::c_char; 51],
            >(b"int kmMat4AreEqual(const kmMat4 *, const kmMat4 *)\0"))
                .as_ptr(),
        );
    }
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        if !((*pMat1).mat[i as usize] as std::os::raw::c_double + 0.0001f64
            > (*pMat2).mat[i as usize] as std::os::raw::c_double
            && (*pMat1).mat[i as usize] as std::os::raw::c_double - 0.0001f64
                < (*pMat2).mat[i as usize] as std::os::raw::c_double)
        {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationAxisAngle(
    mut pOut: *mut kmMat4,
    mut axis: *const kmVec3,
    mut radians: std::os::raw::c_float,
) -> *mut kmMat4 {
    let mut quat = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionRotationAxisAngle(&mut quat, axis, radians);
    kmMat4RotationQuaternion(pOut, &mut quat);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationX(
    mut pOut: *mut kmMat4,
    radians: std::os::raw::c_float,
) -> *mut kmMat4 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[6 as std::os::raw::c_int as usize] = sinf(radians);
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[9 as std::os::raw::c_int as usize] = -sinf(radians);
    (*pOut).mat[10 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationY(
    mut pOut: *mut kmMat4,
    radians: std::os::raw::c_float,
) -> *mut kmMat4 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as std::os::raw::c_int as usize] = -sinf(radians);
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = sinf(radians);
    (*pOut).mat[9 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationZ(
    mut pOut: *mut kmMat4,
    radians: std::os::raw::c_float,
) -> *mut kmMat4 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as std::os::raw::c_int as usize] = sinf(radians);
    (*pOut).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = -sinf(radians);
    (*pOut).mat[5 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[9 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationYawPitchRoll(
    mut pOut: *mut kmMat4,
    pitch: std::os::raw::c_float,
    yaw: std::os::raw::c_float,
    roll: std::os::raw::c_float,
) -> *mut kmMat4 {
    let mut yaw_matrix = kmMat4 { mat: [0.; 16] };
    kmMat4RotationY(&mut yaw_matrix, yaw);
    let mut pitch_matrix = kmMat4 { mat: [0.; 16] };
    kmMat4RotationX(&mut pitch_matrix, pitch);
    let mut roll_matrix = kmMat4 { mat: [0.; 16] };
    kmMat4RotationZ(&mut roll_matrix, roll);
    kmMat4Multiply(pOut, &mut pitch_matrix, &mut roll_matrix);
    kmMat4Multiply(pOut, &mut yaw_matrix, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationQuaternion(
    mut pOut: *mut kmMat4,
    mut pQ: *const kmQuaternion,
) -> *mut kmMat4 {
    let mut xx = ((*pQ).x * (*pQ).x) as std::os::raw::c_double;
    let mut xy = ((*pQ).x * (*pQ).y) as std::os::raw::c_double;
    let mut xz = ((*pQ).x * (*pQ).z) as std::os::raw::c_double;
    let mut xw = ((*pQ).x * (*pQ).w) as std::os::raw::c_double;
    let mut yy = ((*pQ).y * (*pQ).y) as std::os::raw::c_double;
    let mut yz = ((*pQ).y * (*pQ).z) as std::os::raw::c_double;
    let mut yw = ((*pQ).y * (*pQ).w) as std::os::raw::c_double;
    let mut zz = ((*pQ).z * (*pQ).z) as std::os::raw::c_double;
    let mut zw = ((*pQ).z * (*pQ).w) as std::os::raw::c_double;
    (*pOut)
        .mat[0 as std::os::raw::c_int
        as usize] = (1 as std::os::raw::c_int as std::os::raw::c_double
        - 2 as std::os::raw::c_int as std::os::raw::c_double * (yy + zz)) as std::os::raw::c_float;
    (*pOut)
        .mat[1 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xy + zw)) as std::os::raw::c_float;
    (*pOut)
        .mat[2 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xz - yw)) as std::os::raw::c_float;
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_float;
    (*pOut)
        .mat[4 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xy - zw)) as std::os::raw::c_float;
    (*pOut)
        .mat[5 as std::os::raw::c_int
        as usize] = (1 as std::os::raw::c_int as std::os::raw::c_double
        - 2 as std::os::raw::c_int as std::os::raw::c_double * (xx + zz)) as std::os::raw::c_float;
    (*pOut)
        .mat[6 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (yz + xw)) as std::os::raw::c_float;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut)
        .mat[8 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xz + yw)) as std::os::raw::c_float;
    (*pOut)
        .mat[9 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (yz - xw)) as std::os::raw::c_float;
    (*pOut)
        .mat[10 as std::os::raw::c_int
        as usize] = (1 as std::os::raw::c_int as std::os::raw::c_double
        - 2 as std::os::raw::c_int as std::os::raw::c_double * (xx + yy)) as std::os::raw::c_float;
    (*pOut).mat[11 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[12 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 1.0f64 as std::os::raw::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Scaling(
    mut pOut: *mut kmMat4,
    x: std::os::raw::c_float,
    y: std::os::raw::c_float,
    mut z: std::os::raw::c_float,
) -> *mut kmMat4 {
    memset(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    (*pOut).mat[0 as std::os::raw::c_int as usize] = x;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = y;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = z;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Translation(
    mut pOut: *mut kmMat4,
    x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
    z: std::os::raw::c_float,
) -> *mut kmMat4 {
    memset(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    (*pOut).mat[0 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[12 as std::os::raw::c_int as usize] = x;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = y;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = z;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetUpVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &KM_VEC3_POS_Y, pIn);
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetRightVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &KM_VEC3_POS_X, pIn);
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetForwardVec3RH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &KM_VEC3_NEG_Z, pIn);
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetForwardVec3LH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    kmVec3MultiplyMat4(pOut, &KM_VEC3_POS_Z, pIn);
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4PerspectiveProjection(
    mut pOut: *mut kmMat4,
    mut fovY: std::os::raw::c_float,
    mut aspect: std::os::raw::c_float,
    mut zNear: std::os::raw::c_float,
    mut zFar: std::os::raw::c_float,
) -> *mut kmMat4 {
    let mut r = kmDegreesToRadians(fovY / 2 as std::os::raw::c_int as std::os::raw::c_float);
    let mut deltaZ = zFar - zNear;
    let mut s = sin(r as std::os::raw::c_double) as std::os::raw::c_float;
    let mut cotangent = 0 as std::os::raw::c_int as std::os::raw::c_float;
    if deltaZ == 0 as std::os::raw::c_int as std::os::raw::c_float
        || s == 0 as std::os::raw::c_int as std::os::raw::c_float
        || aspect == 0 as std::os::raw::c_int as std::os::raw::c_float
    {
        return 0 as *mut kmMat4;
    }
    cotangent = (cos(r as std::os::raw::c_double) / s as std::os::raw::c_double) as std::os::raw::c_float;
    kmMat4Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = cotangent / aspect;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = cotangent;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = -(zFar + zNear) / deltaZ;
    (*pOut).mat[11 as std::os::raw::c_int as usize] = -(1 as std::os::raw::c_int) as std::os::raw::c_float;
    (*pOut)
        .mat[14 as std::os::raw::c_int
        as usize] = -(2 as std::os::raw::c_int) as std::os::raw::c_float * zNear * zFar / deltaZ;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4OrthographicProjection(
    mut pOut: *mut kmMat4,
    mut left: std::os::raw::c_float,
    mut right: std::os::raw::c_float,
    mut bottom: std::os::raw::c_float,
    mut top: std::os::raw::c_float,
    mut nearVal: std::os::raw::c_float,
    mut farVal: std::os::raw::c_float,
) -> *mut kmMat4 {
    let mut tx = -((right + left) / (right - left));
    let mut ty = -((top + bottom) / (top - bottom));
    let mut tz = -((farVal + nearVal) / (farVal - nearVal));
    kmMat4Identity(pOut);
    (*pOut)
        .mat[0 as std::os::raw::c_int
        as usize] = 2 as std::os::raw::c_int as std::os::raw::c_float / (right - left);
    (*pOut)
        .mat[5 as std::os::raw::c_int
        as usize] = 2 as std::os::raw::c_int as std::os::raw::c_float / (top - bottom);
    (*pOut)
        .mat[10 as std::os::raw::c_int
        as usize] = -(2 as std::os::raw::c_int) as std::os::raw::c_float / (farVal - nearVal);
    (*pOut).mat[12 as std::os::raw::c_int as usize] = tx;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = ty;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = tz;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4LookAt(
    mut pOut: *mut kmMat4,
    mut pEye: *const kmVec3,
    mut pCenter: *const kmVec3,
    mut pUp: *const kmVec3,
) -> *mut kmMat4 {
    let mut f = kmVec3 { x: 0., y: 0., z: 0. };
    let mut up = kmVec3 { x: 0., y: 0., z: 0. };
    let mut s = kmVec3 { x: 0., y: 0., z: 0. };
    let mut u = kmVec3 { x: 0., y: 0., z: 0. };
    let mut translate = kmMat4 { mat: [0.; 16] };
    kmVec3Subtract(&mut f, pCenter, pEye);
    kmVec3Normalize(&mut f, &mut f);
    kmVec3Assign(&mut up, pUp);
    kmVec3Normalize(&mut up, &mut up);
    kmVec3Cross(&mut s, &mut f, &mut up);
    kmVec3Normalize(&mut s, &mut s);
    kmVec3Cross(&mut u, &mut s, &mut f);
    kmVec3Normalize(&mut s, &mut s);
    kmMat4Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = s.x;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = s.y;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = s.z;
    (*pOut).mat[1 as std::os::raw::c_int as usize] = u.x;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = u.y;
    (*pOut).mat[9 as std::os::raw::c_int as usize] = u.z;
    (*pOut).mat[2 as std::os::raw::c_int as usize] = -f.x;
    (*pOut).mat[6 as std::os::raw::c_int as usize] = -f.y;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = -f.z;
    kmMat4Translation(&mut translate, -(*pEye).x, -(*pEye).y, -(*pEye).z);
    kmMat4Multiply(pOut, pOut, &mut translate);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4ExtractRotation(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat4,
) -> *mut kmMat3 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = (*pIn).mat[0 as std::os::raw::c_int as usize];
    (*pOut).mat[1 as std::os::raw::c_int as usize] = (*pIn).mat[1 as std::os::raw::c_int as usize];
    (*pOut).mat[2 as std::os::raw::c_int as usize] = (*pIn).mat[2 as std::os::raw::c_int as usize];
    (*pOut).mat[3 as std::os::raw::c_int as usize] = (*pIn).mat[4 as std::os::raw::c_int as usize];
    (*pOut).mat[4 as std::os::raw::c_int as usize] = (*pIn).mat[5 as std::os::raw::c_int as usize];
    (*pOut).mat[5 as std::os::raw::c_int as usize] = (*pIn).mat[6 as std::os::raw::c_int as usize];
    (*pOut).mat[6 as std::os::raw::c_int as usize] = (*pIn).mat[8 as std::os::raw::c_int as usize];
    (*pOut).mat[7 as std::os::raw::c_int as usize] = (*pIn).mat[9 as std::os::raw::c_int as usize];
    (*pOut).mat[8 as std::os::raw::c_int as usize] = (*pIn).mat[10 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationToAxisAngle(
    mut pAxis: *mut kmVec3,
    mut radians: *mut std::os::raw::c_float,
    mut pIn: *const kmMat4,
) -> *mut kmVec3 {
    let mut temp = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut rotation = kmMat3 { mat: [0.; 9] };
    kmMat4ExtractRotation(&mut rotation, pIn);
    kmQuaternionRotationMatrix(&mut temp, &mut rotation);
    kmQuaternionToAxisAngle(&mut temp, pAxis, radians);
    return pAxis;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationTranslation(
    mut pOut: *mut kmMat4,
    mut rotation: *const kmMat3,
    mut translation: *const kmVec3,
) -> *mut kmMat4 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = (*rotation).mat[0 as std::os::raw::c_int as usize];
    (*pOut).mat[1 as std::os::raw::c_int as usize] = (*rotation).mat[1 as std::os::raw::c_int as usize];
    (*pOut).mat[2 as std::os::raw::c_int as usize] = (*rotation).mat[2 as std::os::raw::c_int as usize];
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = (*rotation).mat[3 as std::os::raw::c_int as usize];
    (*pOut).mat[5 as std::os::raw::c_int as usize] = (*rotation).mat[4 as std::os::raw::c_int as usize];
    (*pOut).mat[6 as std::os::raw::c_int as usize] = (*rotation).mat[5 as std::os::raw::c_int as usize];
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = (*rotation).mat[6 as std::os::raw::c_int as usize];
    (*pOut).mat[9 as std::os::raw::c_int as usize] = (*rotation).mat[7 as std::os::raw::c_int as usize];
    (*pOut).mat[10 as std::os::raw::c_int as usize] = (*rotation).mat[8 as std::os::raw::c_int as usize];
    (*pOut).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[12 as std::os::raw::c_int as usize] = (*translation).x;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = (*translation).y;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = (*translation).z;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4ExtractPlane(
    mut pOut: *mut kmPlane,
    mut pIn: *const kmMat4,
    plane: std::os::raw::c_uint,
) -> *mut kmPlane {
    let mut t = 1.0f32;
    match plane {
        1 => {
            (*pOut)
                .a = (*pIn).mat[3 as std::os::raw::c_int as usize]
                - (*pIn).mat[0 as std::os::raw::c_int as usize];
            (*pOut)
                .b = (*pIn).mat[7 as std::os::raw::c_int as usize]
                - (*pIn).mat[4 as std::os::raw::c_int as usize];
            (*pOut)
                .c = (*pIn).mat[11 as std::os::raw::c_int as usize]
                - (*pIn).mat[8 as std::os::raw::c_int as usize];
            (*pOut)
                .d = (*pIn).mat[15 as std::os::raw::c_int as usize]
                - (*pIn).mat[12 as std::os::raw::c_int as usize];
        }
        0 => {
            (*pOut)
                .a = (*pIn).mat[3 as std::os::raw::c_int as usize]
                + (*pIn).mat[0 as std::os::raw::c_int as usize];
            (*pOut)
                .b = (*pIn).mat[7 as std::os::raw::c_int as usize]
                + (*pIn).mat[4 as std::os::raw::c_int as usize];
            (*pOut)
                .c = (*pIn).mat[11 as std::os::raw::c_int as usize]
                + (*pIn).mat[8 as std::os::raw::c_int as usize];
            (*pOut)
                .d = (*pIn).mat[15 as std::os::raw::c_int as usize]
                + (*pIn).mat[12 as std::os::raw::c_int as usize];
        }
        2 => {
            (*pOut)
                .a = (*pIn).mat[3 as std::os::raw::c_int as usize]
                + (*pIn).mat[1 as std::os::raw::c_int as usize];
            (*pOut)
                .b = (*pIn).mat[7 as std::os::raw::c_int as usize]
                + (*pIn).mat[5 as std::os::raw::c_int as usize];
            (*pOut)
                .c = (*pIn).mat[11 as std::os::raw::c_int as usize]
                + (*pIn).mat[9 as std::os::raw::c_int as usize];
            (*pOut)
                .d = (*pIn).mat[15 as std::os::raw::c_int as usize]
                + (*pIn).mat[13 as std::os::raw::c_int as usize];
        }
        3 => {
            (*pOut)
                .a = (*pIn).mat[3 as std::os::raw::c_int as usize]
                - (*pIn).mat[1 as std::os::raw::c_int as usize];
            (*pOut)
                .b = (*pIn).mat[7 as std::os::raw::c_int as usize]
                - (*pIn).mat[5 as std::os::raw::c_int as usize];
            (*pOut)
                .c = (*pIn).mat[11 as std::os::raw::c_int as usize]
                - (*pIn).mat[9 as std::os::raw::c_int as usize];
            (*pOut)
                .d = (*pIn).mat[15 as std::os::raw::c_int as usize]
                - (*pIn).mat[13 as std::os::raw::c_int as usize];
        }
        5 => {
            (*pOut)
                .a = (*pIn).mat[3 as std::os::raw::c_int as usize]
                - (*pIn).mat[2 as std::os::raw::c_int as usize];
            (*pOut)
                .b = (*pIn).mat[7 as std::os::raw::c_int as usize]
                - (*pIn).mat[6 as std::os::raw::c_int as usize];
            (*pOut)
                .c = (*pIn).mat[11 as std::os::raw::c_int as usize]
                - (*pIn).mat[10 as std::os::raw::c_int as usize];
            (*pOut)
                .d = (*pIn).mat[15 as std::os::raw::c_int as usize]
                - (*pIn).mat[14 as std::os::raw::c_int as usize];
        }
        4 => {
            (*pOut)
                .a = (*pIn).mat[3 as std::os::raw::c_int as usize]
                + (*pIn).mat[2 as std::os::raw::c_int as usize];
            (*pOut)
                .b = (*pIn).mat[7 as std::os::raw::c_int as usize]
                + (*pIn).mat[6 as std::os::raw::c_int as usize];
            (*pOut)
                .c = (*pIn).mat[11 as std::os::raw::c_int as usize]
                + (*pIn).mat[10 as std::os::raw::c_int as usize];
            (*pOut)
                .d = (*pIn).mat[15 as std::os::raw::c_int as usize]
                + (*pIn).mat[14 as std::os::raw::c_int as usize];
        }
        _ => {
            if 0 as std::os::raw::c_int != 0
                && !(b"Invalid plane index\0" as *const u8 as *const std::os::raw::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"0 && \"Invalid plane index\"\0" as *const u8
                        as *const std::os::raw::c_char,
                    b"../kazmath/mat4.c\0" as *const u8 as *const std::os::raw::c_char,
                    779 as std::os::raw::c_int as std::os::raw::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 82],
                        &[std::os::raw::c_char; 82],
                    >(
                        b"struct kmPlane *kmMat4ExtractPlane(kmPlane *, const kmMat4 *, const unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    t = sqrtf((*pOut).a * (*pOut).a + (*pOut).b * (*pOut).b + (*pOut).c * (*pOut).c);
    (*pOut).a /= t;
    (*pOut).b /= t;
    (*pOut).c /= t;
    (*pOut).d /= t;
    return pOut;
}
