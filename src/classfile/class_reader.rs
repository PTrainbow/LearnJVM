use std::cell::Cell;
use std::collections::HashMap;
use byteorder::{BE, ReadBytesExt};
use crate::classfile::attribute::{*};
use crate::classfile::constant_pool::parse_constant_pool;
use crate::classfile::ClassFile;
use crate::error::Error;

use super::attribute::AttributeInfo::{self, *};
use super::constant_pool::ConstantInfo::{self, *};

pub struct Reader {
    pub(crate) content: Vec<u8>,
    pub(crate) cursor: Cell<usize>,
}


pub struct MethodInfo {
    pub name: String,
    pub access_flag:u16,
    pub descriptor_index:u16,
    pub attribute_info:Vec<AttributeInfo>
}

pub struct FieldInfo {
    pub name: String,
    pub access_flag:u16,
    pub descriptor_index:u16,
    pub attribute_info:Vec<AttributeInfo>
}

const ACC_PUBLIC: u32 = 0x0001;
const ACC_FINAL: u32 = 0x0010;
const ACC_SUPER: u32 = 0x0020;
const ACC_INTERFACE: u32 = 0x0200;
const ACC_ABSTRACT: u32 = 0x0400;
const ACC_SYNTHETIC: u32 = 0x1000;
const ACC_ANNOTATION: u32 = 0x2000;
const ACC_ENUM: u32 = 0x4000;

impl Reader {
    pub(crate) fn parse_classfile(&self) -> Result<ClassFile, Error> {
        let magic = self.read_u32()?;
        println!("magic number is {:#x}", magic);

        let minor_version = self.read_u16()?;
        println!("minor version is {}", minor_version);

        let major_version = self.read_u16()?;
        println!("major version is {}", major_version);

        let constant_pool_size = self.read_u16()?;
        println!("constant_pool_size is {}", constant_pool_size);

        let constant_pool = parse_constant_pool(self, constant_pool_size)?;
        
        println!("just a test {:?}", constant_pool);
        let access_flags = self.read_u16()?;
        let this_class = self.read_u16()?;
        let super_class = self.read_u16()?;
        let interface_count = self.read_u16()?;

        let mut interfaces = Vec::<String>::new();
        for _i in 0..interface_count {
            let class_info_index = self.read_u16()?;
            println!(" interfaces  class_index= {}", class_info_index);
            interfaces.push(get_class_name(&constant_pool, &class_info_index).unwrap());
        }

        let fields_count = self.read_u16()?;
        println!(" fields count = {} ", fields_count);

        let mut fields_info = Vec::<FieldInfo>::new();
        for field_index in 0..fields_count {
            let access_flags = self.read_u16()?;
            let name_index = self.read_u16()?;
            let descriptor_index = self.read_u16()?;
            let attributes = parse_attributes(self,&constant_pool)?;
            fields_info.push(FieldInfo {
                name: get_utf8(&constant_pool, &name_index).unwrap(),
                access_flag: access_flags,
                descriptor_index,
                attribute_info:attributes
            });
        }

        let methods_count = self.read_u16()?;
        println!(" methods count = {} ", methods_count);
        let mut method_info = Vec::<MethodInfo>::new();
        for method_index in 0..methods_count {
            let access_flags = self.read_u16()?;
            let name_index = self.read_u16()?;
            let descriptor_index = self.read_u16()?;
            let attributes = parse_attributes(self, &constant_pool)?;
            method_info.push(MethodInfo {
                name: get_utf8(&constant_pool, &name_index).unwrap(),
                access_flag: access_flags,
                descriptor_index,
                attribute_info:attributes
            });
        }

        let attributes_info = parse_attributes(self, &constant_pool)?;

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count: constant_pool.len() as u16,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count: interface_count,
            interfaces,
            fields_count,
            fields_info,
            methods_count,
            methods_info: method_info,
            attributes_count: attributes_info.len() as u16,
            attributes_info
        })
    }

    pub fn read_u8(&self) -> Result<u8, Error> {
        let content = (&self.content[self.cursor.get()..]).read_u8()?;
        self.cursor.set(self.cursor.get() + 1);
        return Ok(content);
    }

    pub fn read_u16(&self) -> Result<u16, Error> {
        let content = (&self.content[self.cursor.get()..self.cursor.get() + 2]).read_u16::<BE>()?;
        self.cursor.set(self.cursor.get() + 2);
        return Ok(content);
    }

    pub fn read_u16s(&self) -> Result<Vec<u16>, Error> {
        let n = (&self.content[self.cursor.get()..self.cursor.get() + 2]).read_u16::<BE>()?;
        self.cursor.set(self.cursor.get() + 2);
        let mut content = Vec::new();
        for _i in 0..n {
            let item = self.read_u16()?;
            content.push(item);
        }
        return Ok(content);
    }

    pub fn read_u32(&self) -> Result<u32, Error> {
        let content = (&self.content[self.cursor.get()..self.cursor.get() + 4]).read_u32::<BE>()?;
        self.cursor.set(self.cursor.get() + 4);
        return Ok(content);
    }

    pub fn read_u64(&self) -> Result<u64, Error> {
        let content = (&self.content[self.cursor.get()..self.cursor.get() + 8]).read_u64::<BE>()?;
        self.cursor.set(self.cursor.get() + 8);
        return Ok(content);
    }
    pub fn read_bytes(&self, size: usize) -> Result<Vec<u8>, Error> {
        let content = &self.content[self.cursor.get()..self.cursor.get() + size];
        self.cursor.set(self.cursor.get() + size);
        return Ok(content.to_vec());
    }
}

pub fn get_utf8(constant_pool: &Vec<ConstantInfo>, index: &u16) -> Option<String> {
    if let ConstantUTF8 { value } = &constant_pool[*index as usize] {
        println!("constant utf8 = {}", value);
        return Some(String::from(value));
    };
    return None;
}

pub fn get_class_name(constant_pool: &Vec<ConstantInfo>, this_class: &u16) -> Option<String> {
    println!("this_class {}", this_class);
    if let ConstantClass { index } = &constant_pool[*this_class as usize] {
        if let ConstantUTF8 { value } = &constant_pool[*index as usize] {
            return Some(String::from(value));
        }
    }
    return None;
}