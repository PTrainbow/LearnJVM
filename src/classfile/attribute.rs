use std::collections::HashMap;

use crate::classfile::class_reader::get_utf8;
use crate::error::Error;
use crate::classfile::AttributeInfo::{*};
use super::{class_reader::Reader, constant_pool::ConstantInfo};

pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>
}

pub struct ExceptionTableEntry{
    pub start_pc: u16,
    pub end_pc: u16,
    pub handle_pc: u16,
    pub catch_type: u16
}

pub struct InnerClassInfo{
    pub innner_class_index: u16,
    pub outter_class_index: u16,
    pub inner_class_name_index: u16,
    pub inner_class_access_flags: u16
}

pub struct LineNumberEntry {
    pub start_pc: u16,
    pub line_number:u16
}

pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length:u16,
    pub name_index:u16,
    pub descriptor_index:u16,
    pub index:u16,
}

pub struct LocalVariablTypeEntry {
    pub start_pc: u16,
    pub length:u16,
    pub name_index:u16,
    pub signature_index:u16,
    pub index:u16,
}

pub enum AttributeInfo {
    BootstrapMethodsAttribute{
        boostrap_methods: Vec<BootstrapMethod>
    },
    CodeAttribute {
        max_stacks:u16,
        max_locals:u16,
        code_length:u32,
        code:Vec<u8>,
        exception_table:Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeInfo>
    },
    ConstantValueAttribute{
        value_index:u16
    },
    AttrDeprecated{

    },
    EnclosingMethodAttribute {
        class_index:u16,
        name_index:u16
    },
    ExceptionsAttribute{
        index_table:Vec<u16>
    },
    InnerClassesAttribute {
        inner_classes: Vec<InnerClassInfo>
    },
    LineNumberTableAttribute {
        line_number_table: Vec<LineNumberEntry>
    },
    LocalVariableTableAttribute {
        local_variable_table: Vec<LocalVariableTableEntry>
    },
    LocalVariableTypeTableAttribute {
        local_variable_type_table: Vec<LocalVariablTypeEntry>
    },
    SignatureAttribute{
        signature_index:u16
    },
    SourceFileAttribute{
        source_file:u16
    },
    UnparsedAttribute {
       
    },
    DeprecatedAttribute{},
    SyntheticAttribute{}
}

pub fn parse_attributes(reader: &Reader, constant_pool: &Vec<ConstantInfo>) -> Result<Vec<AttributeInfo>, Error> {
    let attributes_count = reader.read_u16()?;
    println!(" attributes count = {} ", attributes_count);
    let mut result = Vec::new();
    for _index in 0..attributes_count {
        let attr_name_index = reader.read_u16()?;
        let name = get_utf8(&constant_pool, &attr_name_index).unwrap();
        let length = reader.read_u32()?;
        println!("parse attr {:?}, length {:?}", name, length);
        match name.as_str() {
            "BootstrapMethods" => {
                let mut bootMethods = Vec::new();
                let number = reader.read_u16()?;
                for _i in  0..number{
                    bootMethods.push(BootstrapMethod {
                        bootstrap_method_ref: reader.read_u16()?,
                        bootstrap_arguments: reader.read_u16s()?
                    })
                }
                result.push(BootstrapMethodsAttribute {
                    boostrap_methods: bootMethods
                })
            },
            "Code" => {
                let max_stacks = reader.read_u16()?;
                let max_locals = reader.read_u16()?;
                let code_length = reader.read_u32()?;
                let code = reader.read_bytes(code_length as usize)?;
                let exception_num = reader.read_u16()?;
                let mut exception_table = Vec::new();
                for _i in  0..exception_num {
                    exception_table.push(ExceptionTableEntry {
                        start_pc: reader.read_u16()?,
                        end_pc:reader.read_u16()?,
                        handle_pc:reader.read_u16()?,
                        catch_type:reader.read_u16()?
                    })
                }
                let attributes =  parse_attributes(reader, constant_pool)?;
                result.push(CodeAttribute { 
                    max_stacks,
                    max_locals,
                    code_length,
                    code,
                    exception_table,
                    attributes: attributes
                })
            },
            "ConstantValue" => {
                result.push(ConstantValueAttribute { value_index: reader.read_u16()? })
            },
            "EnclosingMethod" => {
                result.push(EnclosingMethodAttribute { class_index: reader.read_u16()?, name_index: reader.read_u16()? })
            },
            "Exceptions" => {
                result.push(ExceptionsAttribute { index_table: reader.read_u16s()? })
            },
            "InnerClasses" => {
                let number = reader.read_u16()?;
                let mut class_vec = Vec::new();
                for _ in 0..number {
                    class_vec.push(InnerClassInfo{
                        innner_class_index: reader.read_u16()?,
                        outter_class_index: reader.read_u16()?,
                        inner_class_name_index: reader.read_u16()?,
                        inner_class_access_flags: reader.read_u16()?
                    })
                }
                result.push(InnerClassesAttribute { inner_classes: class_vec })
            },
            "LineNumberTable" => {
                let line_number = reader.read_u16()?;
                let mut line_table = Vec::new();
                for _i in 0..line_number {
                    line_table.push(LineNumberEntry {
                        start_pc: reader.read_u16()?,
                        line_number:reader.read_u16()?
                    })
                }
                result.push(LineNumberTableAttribute { line_number_table: line_table })
            },
            "LocalVariableTable" => {
                let line_number = reader.read_u16()?;
                let mut line_table = Vec::new();
                for _i in 0..line_number {
                    line_table.push(LocalVariableTableEntry {
                        start_pc: reader.read_u16()?,
                        length:reader.read_u16()?,
                        name_index: reader.read_u16()?,
                        descriptor_index:reader.read_u16()?,
                        index:reader.read_u16()?
                    })
                }
                result.push(LocalVariableTableAttribute { local_variable_table: line_table })
            },
            "LocalVariableTypeTable" => {
                let line_number = reader.read_u16()?;
                let mut line_table = Vec::new();
                for _i in 0..line_number {
                    line_table.push(LocalVariablTypeEntry {
                        start_pc: reader.read_u16()?,
                        length:reader.read_u16()?,
                        name_index: reader.read_u16()?,
                        signature_index:reader.read_u16()?,
                        index:reader.read_u16()?
                    })
                }
                result.push(LocalVariableTypeTableAttribute { local_variable_type_table:line_table })
            },
            "Signature" => {
                result.push(SignatureAttribute { signature_index: reader.read_u16()? })
            },
            "SourceFile" => {
                let source_file_index = reader.read_u16()?;
                println!("SourceFile is {:?} ", get_utf8(constant_pool, &source_file_index));
                result.push(SourceFileAttribute { source_file: source_file_index })
            },
            "Synthetic" => {

            },
            "Deprecated" => {
                
            },
            _ => {
                reader.read_bytes(length as usize)?;
                println!("dont parse attr {:?}, length {:?}", name, length)
            }
        }
       
    }
    return Ok(result);
}
