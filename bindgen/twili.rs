mod ctypes {
    pub type c_void = core::ffi::c_void;
    pub type c_char = u8;
    pub type c_int = i32;
    pub type c_long = i64;
    pub type c_longlong = i64;
    pub type c_schar = i8;
    pub type c_short = i16;
    pub type c_uchar = u8;
    pub type c_uint = u32;
    pub type c_ulong = u64;
    pub type c_ulonglong = u64;
    pub type c_ushort = u16;
    pub type size_t = u64;
    pub type ssize_t = i64;
    pub type c_float = f32;
    pub type c_double = f64;
}/* automatically generated by rust-bindgen */

pub type __uint32_t = ctypes :: c_uint ; pub type Handle = u32 ; pub type Result = u32 ; 
 /// < Uninitialized service. 
 pub const ServiceType_ServiceType_Uninitialized : ServiceType = 0 ; 
 /// < Normal service. 
 pub const ServiceType_ServiceType_Normal : ServiceType = 1 ; 
 /// < Domain. 
 pub const ServiceType_ServiceType_Domain : ServiceType = 2 ; 
 /// < Domain subservice; 
 pub const ServiceType_ServiceType_DomainSubservice : ServiceType = 3 ; 
 /// < Service overriden in the homebrew environment. 
 pub const ServiceType_ServiceType_Override : ServiceType = 4 ; 
 /// Service type. 
 pub type ServiceType = u32 ; 
 /// Service object structure. 
 # [ repr ( C ) ] pub struct Service { pub handle : Handle , pub object_id : u32 , pub type_ : ServiceType , } # [ test ] fn bindgen_test_layout_Service ( ) { assert_eq ! ( :: core :: mem :: size_of :: < Service > ( ) , 12usize , concat ! ( "Size of: " , stringify ! ( Service ) ) ) ; assert_eq ! ( :: core :: mem :: align_of :: < Service > ( ) , 4usize , concat ! ( "Alignment of " , stringify ! ( Service ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: core :: ptr :: null :: < Service > ( ) ) ) . handle as * const _ as usize } , 0usize , concat ! ( "Offset of field: " , stringify ! ( Service ) , "::" , stringify ! ( handle ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: core :: ptr :: null :: < Service > ( ) ) ) . object_id as * const _ as usize } , 4usize , concat ! ( "Offset of field: " , stringify ! ( Service ) , "::" , stringify ! ( object_id ) ) ) ; assert_eq ! ( unsafe { & ( * ( :: core :: ptr :: null :: < Service > ( ) ) ) . type_ as * const _ as usize } , 8usize , concat ! ( "Offset of field: " , stringify ! ( Service ) , "::" , stringify ! ( type_ ) ) ) ; } extern "C" { pub fn twiliWriteNamedPipe ( pipe : * mut Service , ptr : * const ctypes :: c_char , len : usize ) -> Result ; } extern "C" { pub fn twiliCreateNamedOutputPipe ( srv_out : * mut Service , name : * const ctypes :: c_char , len : usize ) -> Result ; } extern "C" { pub fn twiliInitialize ( ) -> Result ; } extern "C" { pub fn twiliExit ( ) ; }