pub(crate) mod class_reader;

use std::collections::HashMap;
use byteorder::{BE, ReadBytesExt};
use crate::classfile::class_reader::{ConstantInfo, FieldInfo, MethodInfo, Reader};
use crate::classfile::class_reader::ConstantInfo::{ConstantClass, ConstantUTF8};
use crate::error::Error;

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
    pub methods_count:u16,
    pub methods_info: Vec<MethodInfo>,
    pub fields_count:u16,
    pub fields_info:Vec<FieldInfo>,
}