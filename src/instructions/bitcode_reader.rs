use std::cell::Cell;

use byteorder::{BE, ReadBytesExt};

use crate::error::Error;

pub struct BytecodeReader {
    pub(crate) content: Vec<u8>,
    pub(crate) cursor: Cell<i32>,
}

impl BytecodeReader {
    pub fn skip_padding(&self) {
       while (self.cursor.get() % 4) != 0 {
           self.read_u8().unwrap();
       }
    }

    pub fn reset(&mut self,  pc: i32) {
        self.cursor.set(pc);
    }

    pub fn read_u8(&self) -> Result<u8, Error> {
        let content = (&self.content[self.cursor.get() as usize..]).read_u8()?;
        self.cursor.set(self.cursor.get() + 1);
        return Ok(content);
    }

    pub fn read_i8(&self) -> Result<i8, Error> {
        let content = (&self.content[self.cursor.get() as usize..]).read_i8()?;
        self.cursor.set(self.cursor.get() + 1);
        return Ok(content);
    }

    pub fn read_u16(&self) -> Result<u16, Error> {
        let content = (&self.content[self.cursor.get() as usize..self.cursor.get() as usize + 2]).read_u16::<BE>()?;
        self.cursor.set(self.cursor.get() + 2);
        return Ok(content);
    }

    pub fn read_i16(&self) -> Result<i16, Error> {
        let content = (&self.content[self.cursor.get() as usize..self.cursor.get() as usize + 2]).read_i16::<BE>()?;
        self.cursor.set(self.cursor.get() + 2);
        return Ok(content);
    }

    pub fn read_u32(&self) -> Result<u32, Error> {
        let content = (&self.content[self.cursor.get() as usize..self.cursor.get() as usize + 4]).read_u32::<BE>()?;
        self.cursor.set(self.cursor.get() + 4);
        return Ok(content);
    }

    pub fn read_i32s(&self, size:i32) -> Result<Vec<i32>, Error> {
        let mut vector = Vec::new();
        for _ in  0..size {
            vector.push( self.read_i32()?)
        }
        return Ok(vector);
    }

    pub fn read_i32(&self) -> Result<i32, Error> {
        let content = (&self.content[self.cursor.get() as usize ..self.cursor.get() as usize + 4]).read_i32::<BE>()?;
        self.cursor.set(self.cursor.get() + 4);
        return Ok(content);
    }
}