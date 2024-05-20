pub(crate) mod class_reader;
pub(crate) mod constant_pool;
pub(crate) mod attribute;

use std::collections::HashMap;
use self::constant_pool::ConstantInfo;
use self::class_reader::{FieldInfo, MethodInfo};
use self::attribute::AttributeInfo;


pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version:u16,
    pub constant_pool_count:u16,
    pub constant_pool: HashMap<u16, ConstantInfo>,
    pub access_flags: u16,
    pub this_class : u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<String>,
    pub fields_count:u16,
    pub fields_info:Vec<FieldInfo>,
    pub methods_count:u16,
    pub methods_info: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes_info: Vec<AttributeInfo>
}