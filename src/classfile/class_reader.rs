use std::any::Any;
use std::cell::Cell;
use std::collections::HashMap;
use std::iter::Map;
use std::thread::sleep;
use byteorder::{BE, ReadBytesExt};
use crate::classfile::class_reader::AttributeInfo::{*};
use crate::classfile::class_reader::ConstantInfo::{ConstantClass, ConstantDouble, ConstantFieldReference, ConstantFloat, ConstantInteger, ConstantInterfaceMethodReference, ConstantInvokeDynamic, ConstantLong, ConstantMethodHandle, ConstantMethodReference, ConstantMethodType, ConstantNameAndType, ConstantString, ConstantUTF8};
use crate::classfile::ClassFile;
use crate::error::Error;
use crate::error::Error::UnKnownConstantType;

pub struct Reader {
    pub(crate) content: Vec<u8>,
    pub(crate) cursor: Cell<usize>,
}

pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub  bootstrap_arguments: Vec<u16>
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
    start_pc: u16,
    line_number:u16
}

pub struct LocalVariableTableEntry {
    start_pc: u16,
    length:u16,
    name_index:u16,
    descriptor_index:u16,
    index:u16,
}

pub struct LocalVariablTypeEntry {
    start_pc: u16,
    length:u16,
    name_index:u16,
    signature_index:u16,
    index:u16,
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

const ACC_PUBLIC: u32 = 0x0001;
const ACC_FINAL: u32 = 0x0010;
const ACC_SUPER: u32 = 0x0020;
const ACC_INTERFACE: u32 = 0x0200;
const ACC_ABSTRACT: u32 = 0x0400;
const ACC_SYNTHETIC: u32 = 0x1000;
const ACC_ANNOTATION: u32 = 0x2000;
const ACC_ENUM: u32 = 0x4000;

#[derive(Debug)]
pub enum ConstantInfo {
    ConstantInteger {
        value: u32
    },
    ConstantFloat {
        value: u32
    },
    ConstantLong {
        value: u64
    },
    ConstantDouble {
        value: u64
    },
    ConstantUTF8 {
        value: String
    },
    ConstantString {
        index: u16
    },
    ConstantClass {
        index: u16
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
        descriptor_index: u16
    },
    ConstantMethodHandle {
        ref_kind: u8,
        ref_kind_index: u16,
    },
    ConstantInvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

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

        let mut constant_pool = HashMap::new();
        // constant pool
        let mut index = 1;
        while index < constant_pool_size {
            println!("loop index = {}", index);
            let tag = self.read_u8()?;
            println!("constant tag is {}", tag);

            match tag {
                CONSTANT_INTEGER => {
                    let v = self.read_u32()?;
                    constant_pool.insert(index, ConstantInteger {
                        value: v
                    });
                    println!("value = {}", v);
                }
                CONSTANT_FLOAT => {
                    let v = self.read_u32()?;
                    constant_pool.insert(index, ConstantFloat {
                        value: v
                    });
                    println!("value = {}", v);
                }
                CONSTANT_LONG => {
                    let v = self.read_u64()?;
                    constant_pool.insert(index, ConstantLong {
                        value: v
                    });
                    println!("index before is {}", index);
                    index = index + 1;
                    println!("index after is {}", index);
                    println!("value = {}", v);
                }
                CONSTANT_DOUBLE => {
                    let v = self.read_u64()?;
                    constant_pool.insert(index, ConstantDouble {
                        value: v
                    });
                    index = index + 1;
                    println!("value = {}", v);
                }
                CONSTANT_UTF8 => {
                    let length = self.read_u16()?;
                    let content = self.read_bytes(length as usize)?;
                    let string = std::str::from_utf8(content.into_boxed_slice().as_ref()).unwrap().to_string();
                    println!("CONSTANT_UTF8 utf8 length = {}, content = {}", length, string);
                    constant_pool.insert(index, ConstantUTF8 {
                        value: string
                    });
                }
                CONSTANT_STRING => {
                    let str_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantString {
                        index: str_index
                    });
                    println!("CONSTANT_STRING string index = {} ", str_index);
                }

                CONSTANT_CLASS => {
                    let class_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantClass {
                        index: class_index
                    });
                    println!("CONSTANT_CLASS class index = {} ", class_index);
                }

                CONSTANT_NAME_AND_TYPE => {
                    let name_index = self.read_u16()?;
                    let descriptor_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantNameAndType {
                        name_index,
                        descriptor_index,
                    });
                    println!("CONSTANT_NAME_AND_TYPE name_index = {},  descriptor_index= {}", name_index, descriptor_index);
                }

                CONSTANT_FIELD_REF => {
                    let class_index = self.read_u16()?;
                    let name_and_type_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantFieldReference {
                        class_index,
                        name_and_type_index,
                    });
                    println!("CONSTANT_FIELD_REF class_index = {},  name_and_type_index= {}", class_index, name_and_type_index);
                }

                CONSTANT_METHOD_REF => {
                    let class_index = self.read_u16()?;
                    let name_and_type_index = self.read_u16()?;
                    println!("CONSTANT_METHOD_REF class_index = {},  name_and_type_index= {}", class_index, name_and_type_index);
                    constant_pool.insert(index, ConstantMethodReference {
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_INTERFACE_METHOD_REF => {
                    let class_index = self.read_u16()?;
                    let name_and_type_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantInterfaceMethodReference {
                        class_index,
                        name_and_type_index,
                    });
                    println!("CONSTANT_INTERFACE_METHOD_REF class_index = {},  name_and_type_index= {}", class_index, name_and_type_index);
                }

                CONSTANT_METHOD_TYPE => {
                    let descriptor_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantMethodType {
                        descriptor_index
                    });
                    println!(" CONSTANT_METHOD_TYPE  descriptor_index= {}", descriptor_index);
                }

                CONSTANT_METHOD_HANDLE => {
                    let ref_kind = self.read_u8()?;
                    let ref_kind_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantMethodHandle {
                        ref_kind,
                        ref_kind_index,
                    });
                    println!("CONSTANT_METHOD_HANDLE ref_kind = {}, ref_kind_index= {}", ref_kind, ref_kind_index);
                }
                CONSTANT_INVOKE_DYN => {
                    let bootstrap_method_attr_index = self.read_u16()?;
                    let name_and_type_index = self.read_u16()?;
                    constant_pool.insert(index, ConstantInvokeDynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    });
                    println!(" CONSTANT_INVOKE_DYN bootstrap_method_attr_index = {},  name_and_type_index= {}", bootstrap_method_attr_index, name_and_type_index);
                }
                _ => {
                    return Err(UnKnownConstantType(tag));
                }
            }
            index += 1;
        }
        let access_flags = self.read_u16()?;
        let this_class = self.read_u16()?;
        let super_class = self.read_u16()?;
        let interface_count = self.read_u16()?;

        let mut interfaces = Vec::<String>::new();
        for index in 0..interface_count {
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
            let attributes = self.parse_attributes(&constant_pool)?;
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
            let attributes = self.parse_attributes(&constant_pool)?;
            method_info.push(MethodInfo {
                name: get_utf8(&constant_pool, &name_index).unwrap(),
                access_flag: access_flags,
                descriptor_index,
                attribute_info:attributes
            });
        }

        let attributes_info = self.parse_attributes(&constant_pool)?;

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

    fn parse_attributes(&self, constant_pool: &HashMap<u16, ConstantInfo>) -> Result<Vec<AttributeInfo>, Error> {
        let attributes_count = self.read_u16()?;
        println!(" attributes count = {} ", attributes_count);
        let mut result = Vec::new();
        for _index in 0..attributes_count {
            let attr_name_index = self.read_u16()?;
            let name = get_utf8(&constant_pool, &attr_name_index).unwrap();
            let length = self.read_u32()?;
            println!("parse attr {:?}, length {:?}", name, length);
            match name.as_str() {
                "BootstrapMethods" => {
                    let mut bootMethods = Vec::new();
                    let number = self.read_u16()?;
                    for _i in  0..number{
                        bootMethods.push(BootstrapMethod {
                            bootstrap_method_ref: self.read_u16()?,
                            bootstrap_arguments: self.read_u16s()?
                        })
                    }
                    result.push(BootstrapMethodsAttribute {
                        boostrap_methods: bootMethods
                    })
                },
                "Code" => {
                    let max_stacks = self.read_u16()?;
                    let max_locals = self.read_u16()?;
                    let code_length = self.read_u32()?;
                    let code = self.read_bytes(code_length as usize)?;
                    let exception_num = self.read_u16()?;
                    let mut exception_table = Vec::new();
                    for _i in  0..exception_num {
                        exception_table.push(ExceptionTableEntry {
                            start_pc: self.read_u16()?,
                            end_pc:self.read_u16()?,
                            handle_pc:self.read_u16()?,
                            catch_type:self.read_u16()?
                        })
                    }
                    let attributes =  self.parse_attributes(constant_pool)?;
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
                    result.push(ConstantValueAttribute { value_index: self.read_u16()? })
                },
                "EnclosingMethod" => {
                    result.push(EnclosingMethodAttribute { class_index: self.read_u16()?, name_index: self.read_u16()? })
                },
                "Exceptions" => {
                    result.push(ExceptionsAttribute { index_table: self.read_u16s()? })
                },
                "InnerClasses" => {
                    let number = self.read_u16()?;
                    let mut class_vec = Vec::new();
                    for _ in 0..number {
                        class_vec.push(InnerClassInfo{
                            innner_class_index: self.read_u16()?,
                            outter_class_index: self.read_u16()?,
                            inner_class_name_index: self.read_u16()?,
                            inner_class_access_flags: self.read_u16()?
                        })
                    }
                    result.push(InnerClassesAttribute { inner_classes: class_vec })
                },
                "LineNumberTable" => {
                    let line_number = self.read_u16()?;
                    let mut line_table = Vec::new();
                    for _i in 0..line_number {
                        line_table.push(LineNumberEntry {
                            start_pc: self.read_u16()?,
                            line_number:self.read_u16()?
                        })
                    }
                    result.push(LineNumberTableAttribute { line_number_table: line_table })
                },
                "LocalVariableTable" => {
                    let line_number = self.read_u16()?;
                    let mut line_table = Vec::new();
                    for _i in 0..line_number {
                        line_table.push(LocalVariableTableEntry {
                            start_pc: self.read_u16()?,
                            length:self.read_u16()?,
                            name_index: self.read_u16()?,
                            descriptor_index:self.read_u16()?,
                            index:self.read_u16()?
                        })
                    }
                    result.push(LocalVariableTableAttribute { local_variable_table: line_table })
                },
                "LocalVariableTypeTable" => {
                    let line_number = self.read_u16()?;
                    let mut line_table = Vec::new();
                    for _i in 0..line_number {
                        line_table.push(LocalVariablTypeEntry {
                            start_pc: self.read_u16()?,
                            length:self.read_u16()?,
                            name_index: self.read_u16()?,
                            signature_index:self.read_u16()?,
                            index:self.read_u16()?
                        })
                    }
                    result.push(LocalVariableTypeTableAttribute { local_variable_type_table:line_table })
                },
                "Signature" => {
                    result.push(SignatureAttribute { signature_index: self.read_u16()? })
                },
                "SourceFile" => {
                    let source_file_index = self.read_u16()?;
                    println!("SourceFile is {:?} ", get_utf8(constant_pool, &source_file_index));
                    result.push(SourceFileAttribute { source_file: source_file_index })
                },
                "Synthetic" => {
    
                },
                "Deprecated" => {
                    
                },
                _ => {
                    self.read_bytes(length as usize)?;
                    println!("dont parse attr {:?}, length {:?}", name, length)
                }
            }
           
        }
        return Ok(result);
    }

    fn read_u8(&self) -> Result<u8, Error> {
        let content = (&self.content[self.cursor.get()..]).read_u8()?;
        self.cursor.set(self.cursor.get() + 1);
        return Ok(content);
    }

    fn read_u16(&self) -> Result<u16, Error> {
        let content = (&self.content[self.cursor.get()..self.cursor.get() + 2]).read_u16::<BE>()?;
        self.cursor.set(self.cursor.get() + 2);
        return Ok(content);
    }

    fn read_u16s(&self) -> Result<Vec<u16>, Error> {
        let n = (&self.content[self.cursor.get()..self.cursor.get() + 2]).read_u16::<BE>()?;
        self.cursor.set(self.cursor.get() + 2);
        let mut content = Vec::new();
        for _i in 0..n {
            let item = self.read_u16()?;
            content.push(item);
        }
        return Ok(content);
    }

    fn read_u32(&self) -> Result<u32, Error> {
        let content = (&self.content[self.cursor.get()..self.cursor.get() + 4]).read_u32::<BE>()?;
        self.cursor.set(self.cursor.get() + 4);
        return Ok(content);
    }

    fn read_u64(&self) -> Result<u64, Error> {
        let content = (&self.content[self.cursor.get()..self.cursor.get() + 8]).read_u64::<BE>()?;
        self.cursor.set(self.cursor.get() + 8);
        return Ok(content);
    }
    fn read_bytes(&self, size: usize) -> Result<Vec<u8>, Error> {
        let content = &self.content[self.cursor.get()..self.cursor.get() + size];
        self.cursor.set(self.cursor.get() + size);
        return Ok(content.to_vec());
    }
}

pub fn get_utf8(constant_pool: &HashMap<u16, ConstantInfo>, index: &u16) -> Option<String> {
    if let ConstantUTF8 { value } = constant_pool.get(&index).unwrap() {
        println!("constant utf8 = {}", value);
        return Some(String::from(value));
    };
    return None;
}

pub fn get_class_name(constant_pool: &HashMap<u16, ConstantInfo>, this_class: &u16) -> Option<String> {
    println!("this_class {}", this_class);
    if !constant_pool.contains_key(&this_class) {
        return None;
    }
    if let ConstantClass { index } = constant_pool.get(&this_class).unwrap() {
        if  !constant_pool.contains_key(&index){
            return None;
        }
        if let ConstantUTF8 { value } = constant_pool.get(&index).unwrap() {
            return Some(String::from(value));
        }
    }
    return None;
}