use std::collections::HashMap;

use super::class_reader::Reader;
use crate::classfile::ConstantInfo::{*};
use crate::error::Error;
use crate::error::Error::UnKnownConstantType;

#[derive(Debug)]
pub enum ConstantInfo {
    ConstantInteger {
        value: u32,
    },
    ConstantFloat {
        value: u32,
    },
    ConstantLong {
        value: u64,
    },
    ConstantDouble {
        value: u64,
    },
    ConstantUTF8 {
        value: String,
    },
    ConstantString {
        index: u16,
    },
    ConstantClass {
        index: u16,
    },
    ConstantNameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    ConstantFieldReference {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantMethodReference {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantInterfaceMethodReference {
        class_index: u16,
        name_and_type_index: u16,
    },
    ConstantMethodType {
        descriptor_index: u16,
    },
    ConstantMethodHandle {
        ref_kind: u8,
        ref_kind_index: u16,
    },
    ConstantInvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    ConstantEmpty {
       
    },
}

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELD_REF: u8 = 9;
const CONSTANT_METHOD_REF: u8 = 10;
const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAME_AND_TYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_METHOD_HANDLE: u8 = 15;
const CONSTANT_METHOD_TYPE: u8 = 16;
const CONSTANT_INVOKE_DYN: u8 = 18;

pub fn parse_constant_pool(reader: &Reader, constant_pool_size: u16) -> Result<Vec<ConstantInfo>, Error> {
    let mut constant_pool =  Vec::new();
    // constant pool
    let mut index = 1;
    constant_pool.push(ConstantEmpty{});
    while index < constant_pool_size {
        println!("loop index = {}", index);
        let tag = reader.read_u8()?;
        println!("constant tag is {}", tag);

        match tag {
            CONSTANT_INTEGER => {
                let v = reader.read_u32()?;
                constant_pool.push( ConstantInteger { value: v });
                println!("value = {}", v);
            }
            CONSTANT_FLOAT => {
                let v = reader.read_u32()?;
                constant_pool.push( ConstantFloat { value: v });
                println!("value = {}", v);
            }
            CONSTANT_LONG => {
                let v = reader.read_u64()?;
                constant_pool.push( ConstantLong { value: v });
                println!("index before is {}", index);
                index = index + 1;
                constant_pool.push(ConstantEmpty{});
                println!("index after is {}", index);
                println!("value = {}", v);
            }
            CONSTANT_DOUBLE => {
                let v = reader.read_u64()?;
                constant_pool.push( ConstantDouble { value: v });
                index = index + 1;
                constant_pool.push(ConstantEmpty{});
                println!("value = {}", v);
            }
            CONSTANT_UTF8 => {
                let length = reader.read_u16()?;
                let content = reader.read_bytes(length as usize)?;
                let string = std::str::from_utf8(content.into_boxed_slice().as_ref())
                    .unwrap()
                    .to_string();
                println!(
                    "CONSTANT_UTF8 utf8 length = {}, content = {}",
                    length, string
                );
                constant_pool.push( ConstantUTF8 { value: string });
            }
            CONSTANT_STRING => {
                let str_index = reader.read_u16()?;
                constant_pool.push( ConstantString { index: str_index });
                println!("CONSTANT_STRING string index = {} ", str_index);
            }

            CONSTANT_CLASS => {
                let class_index = reader.read_u16()?;
                constant_pool.push( ConstantClass { index: class_index });
                println!("CONSTANT_CLASS class index = {} ", class_index);
            }

            CONSTANT_NAME_AND_TYPE => {
                let name_index = reader.read_u16()?;
                let descriptor_index = reader.read_u16()?;
                constant_pool.push(
                    ConstantNameAndType {
                        name_index,
                        descriptor_index,
                    },
                );
                println!(
                    "CONSTANT_NAME_AND_TYPE name_index = {},  descriptor_index= {}",
                    name_index, descriptor_index
                );
            }

            CONSTANT_FIELD_REF => {
                let class_index = reader.read_u16()?;
                let name_and_type_index = reader.read_u16()?;
                constant_pool.push(
                    ConstantFieldReference {
                        class_index,
                        name_and_type_index,
                    },
                );
                println!(
                    "CONSTANT_FIELD_REF class_index = {},  name_and_type_index= {}",
                    class_index, name_and_type_index
                );
            }

            CONSTANT_METHOD_REF => {
                let class_index = reader.read_u16()?;
                let name_and_type_index = reader.read_u16()?;
                println!(
                    "CONSTANT_METHOD_REF class_index = {},  name_and_type_index= {}",
                    class_index, name_and_type_index
                );
                constant_pool.push(
                    ConstantMethodReference {
                        class_index,
                        name_and_type_index,
                    },
                );
            }

            CONSTANT_INTERFACE_METHOD_REF => {
                let class_index = reader.read_u16()?;
                let name_and_type_index = reader.read_u16()?;
                constant_pool.push(
                    ConstantInterfaceMethodReference {
                        class_index,
                        name_and_type_index,
                    },
                );
                println!(
                    "CONSTANT_INTERFACE_METHOD_REF class_index = {},  name_and_type_index= {}",
                    class_index, name_and_type_index
                );
            }

            CONSTANT_METHOD_TYPE => {
                let descriptor_index = reader.read_u16()?;
                constant_pool.push( ConstantMethodType { descriptor_index });
                println!(
                    " CONSTANT_METHOD_TYPE  descriptor_index= {}",
                    descriptor_index
                );
            }

            CONSTANT_METHOD_HANDLE => {
                let ref_kind = reader.read_u8()?;
                let ref_kind_index = reader.read_u16()?;
                constant_pool.push(
                    ConstantMethodHandle {
                        ref_kind,
                        ref_kind_index,
                    },
                );
                println!(
                    "CONSTANT_METHOD_HANDLE ref_kind = {}, ref_kind_index= {}",
                    ref_kind, ref_kind_index
                );
            }
            CONSTANT_INVOKE_DYN => {
                let bootstrap_method_attr_index = reader.read_u16()?;
                let name_and_type_index = reader.read_u16()?;
                constant_pool.push(
                    ConstantInvokeDynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    },
                );
                println!(" CONSTANT_INVOKE_DYN bootstrap_method_attr_index = {},  name_and_type_index= {}", bootstrap_method_attr_index, name_and_type_index);
            }
            _ => {
                return Err(UnKnownConstantType(tag));
            }
        }
        index += 1;
    }
    return Ok(constant_pool);
}
