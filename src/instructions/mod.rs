use core::{fmt, str};
use std::{
    borrow::{Borrow, BorrowMut}, cell::Cell, ops::{Deref, DerefMut}, rc::Rc, thread::sleep
};

use zip::read;

use crate::runtime::{Frame, LocalVars, Object, OperandStack, Slot, Thread};

use self::bitcode_reader::BytecodeReader;

pub(crate) mod bitcode_reader;

pub trait Instruction {
    fn execute(&self, frame: &mut Frame);

    // TODO
    fn fetchOperands(&mut self, reader: &BytecodeReader){
       
    }

    fn get_name(&self) -> &'static str {
        return std::any::type_name::<Self>();
    }
}

impl fmt::Debug for dyn Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_name();
        write!(f, "The name: {}", name)
    }
}

pub struct NoOperandsInstruction;

// CONST
pub struct AConstNull;
pub struct DConst0;
pub struct DConst1;
pub struct FConst0;
pub struct FConst1;
pub struct FConst2;
pub struct IConstM1;
pub struct IConst0;
pub struct IConst1;
pub struct IConst2;
pub struct IConst3;
pub struct IConst4;
pub struct IConst5;
pub struct LConst0;
pub struct LConst1;

// BIPUSH SIPUSH
pub struct BIPush {
    value: Cell<i8>,
}
pub struct SIPush {
    value: Cell<i16>,
}

// LOAD
pub struct ALoad {
    index: usize,
}
pub struct ALoad0;
pub struct ALoad1;
pub struct ALoad2;
pub struct ALoad3;
pub struct DLoad {
    index: usize,
}
pub struct DLoad0;
pub struct DLoad1;
pub struct DLoad2;
pub struct DLoad3;
pub struct FLoad {
    index: usize,
}
pub struct FLoad0;
pub struct FLoad1;
pub struct FLoad2;
pub struct FLoad3;
pub struct ILoad {
    index: usize,
}
pub struct ILoad0;
pub struct ILoad1;
pub struct ILoad2;
pub struct ILoad3;
pub struct LLoad {
    index: usize,
}
pub struct LLoad0;
pub struct LLoad1;
pub struct LLoad2;
pub struct LLoad3;

// STORE
pub struct AStore {
    index: usize,
}
pub struct AStore0;
pub struct AStore1;
pub struct AStore2;
pub struct AStore3;
pub struct DStore {
    index: usize,
}
pub struct DStore0;
pub struct DStore1;
pub struct DStore2;
pub struct DStore3;

pub struct FStore {
    index: usize,
}
pub struct FStore0;
pub struct FStore1;
pub struct FStore2;
pub struct FStore3;
pub struct IStore {
    index: usize,
}
pub struct IStore0;
pub struct IStore1;
pub struct IStore2;
pub struct IStore3;
pub struct LStore {
    index: usize,
}
pub struct LStore0;
pub struct LStore1;
pub struct LStore2;
pub struct LStore3;

// stack
pub struct Dup;
pub struct DupX1;
pub struct DupX2;
pub struct Dup2;
pub struct Dup2X1;
pub struct Dup2X2;
pub struct Pop;
pub struct Pop2;
pub struct Swap;

// math
pub struct DAdd;
pub struct FAdd;
pub struct IAdd;
pub struct LAdd;
pub struct DSub;
pub struct FSub;
pub struct ISub;
pub struct LSub;
pub struct DDiv;
pub struct FDiv;
pub struct IDiv;
pub struct LDiv;
pub struct DMul;
pub struct FMul;
pub struct IMul;
pub struct LMul;
pub struct IAnd;
pub struct LAnd;
pub struct IINC {
    index: usize,
    value: i32
}
pub struct DNeg;
pub struct FNeg;
pub struct INeg;
pub struct LNeg;
pub struct IOR;
pub struct LOR;
pub struct DRem;
pub struct FRem;
pub struct IRem;
pub struct LRem;
pub struct ISHL;
pub struct ISHR;
pub struct IUSHR;

pub struct LSHL;
pub struct LSHR;
pub struct LUSHR;

pub struct IXOR;
pub struct LXOR;

// cast
pub struct D2F;
pub struct D2I;
pub struct D2L;
pub struct F2D;
pub struct F2I;
pub struct F2L;
pub struct I2D;
pub struct I2B;
pub struct I2C;
pub struct I2S;
pub struct I2F;
pub struct I2L;
pub struct L2F;
pub struct L2I;
pub struct L2D;

// control
pub struct GOTO {
    offset: i32
}
pub struct LookUpSwitch {
    default_offset: i32,
    n_pairs: i32,
    match_offsets: Vec<i32>
}
pub struct TableSwitch {
    default_offset: i32,
    low: i32,
    high: i32,
    jump_offsets: Vec<i32>
}

// compare
pub struct DCMPG;
pub struct DCMPL;
pub struct FCMPG;
pub struct FCMPL;
pub struct LCMP;
pub struct IFACMPEQ {
    offset: i32
}
pub struct IFACMPNE {
    offset: i32
}
pub struct IFICMPEQ {
    offset: i32
}

pub struct IFICMPNE {
    offset: i32
}
pub struct IFICMPLT {
    offset: i32
}
pub struct IFICMPLE {
    offset: i32
}
pub struct IFICMPGT {
    offset: i32
}
pub struct IFICMPGE {
    offset: i32
}
pub struct IFEQ {
    offset: i32
}
pub struct IFNE {
    offset: i32
}
pub struct IFLT {
    offset: i32
}
pub struct IFLE {
    offset: i32
}
pub struct IFGT {
    offset: i32
}
pub struct IFGE {
    offset: i32
}
pub struct IFNULL{
    offset: i32
}
pub struct IFNOTNULL{
    offset: i32
}
pub struct WIDE {
    modified_inst: Box<dyn Instruction>
}
pub struct GOTO_W {
    offset:i32
}
impl Instruction for NoOperandsInstruction {
    fn execute(&self, frame: &mut Frame) {}
}

impl Instruction for AConstNull {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_ref(Option::None)
    }
}

impl Instruction for DConst0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_double(0.0)
    }
}

impl Instruction for DConst1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_double(1.0)
    }
}

impl Instruction for FConst1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_float(1.0)
    }
}

impl Instruction for FConst2 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_float(2.0)
    }
}

impl Instruction for FConst0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_float(0.0)
    }
}

impl Instruction for IConst0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(0)
    }
}

impl Instruction for IConst1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(1)
    }
}

impl Instruction for IConst2 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(2)
    }
}

impl Instruction for IConst3 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(3)
    }
}

impl Instruction for IConst4 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(4)
    }
}

impl Instruction for IConst5 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(5)
    }
}

impl Instruction for LConst0 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_long(0)
    }
}

impl Instruction for LConst1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_long(1)
    }
}

impl Instruction for IConstM1 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(-1)
    }
}

impl Instruction for BIPush {
    fn execute(&self, frame: &mut Frame) {
        let v = self.value.get();
        println!("BIPush {}", v);
        frame.operand_stack.push_int(v as i32);
    }

    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.value.set(reader.read_i8().unwrap());
    }
}

impl Instruction for SIPush {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(self.value.get().into());
    }

    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.value.set(reader.read_i16().unwrap());
    }
}

impl Instruction for ALoad {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_u8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_ref(self.index);
        frame.operand_stack.push_ref(reference)
    }
}

impl Instruction for ALoad0 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_ref(0);
        frame.operand_stack.push_ref(reference)
    }
}

impl Instruction for ALoad1 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_ref(1);
        frame.operand_stack.push_ref(reference)
    }
}

impl Instruction for ALoad2 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_ref(2);
        frame.operand_stack.push_ref(reference)
    }
}

impl Instruction for ALoad3 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_ref(3);
        frame.operand_stack.push_ref(reference)
    }
}

impl Instruction for DLoad {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_u8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_double(self.index);
        frame.operand_stack.push_double(reference)
    }
}

impl Instruction for DLoad0 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_double(0);
        frame.operand_stack.push_double(reference)
    }
}

impl Instruction for DLoad1 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_double(1);
        frame.operand_stack.push_double(reference)
    }
}

impl Instruction for DLoad2 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_double(2);
        frame.operand_stack.push_double(reference)
    }
}

impl Instruction for DLoad3 {
    fn execute(&self, frame: &mut Frame) {
        let reference = frame.local_vars.get_double(3);
        frame.operand_stack.push_double(reference)
    }
}

impl Instruction for FLoad {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_u8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_float(self.index);
        frame.operand_stack.push_float(value)
    }
}

impl Instruction for FLoad0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_float(0);
        frame.operand_stack.push_float(value)
    }
}

impl Instruction for FLoad1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_float(1);
        frame.operand_stack.push_float(value)
    }
}

impl Instruction for FLoad2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_float(2);
        frame.operand_stack.push_float(value)
    }
}

impl Instruction for FLoad3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_float(3);
        frame.operand_stack.push_float(value)
    }
}

impl Instruction for ILoad {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_u8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_int(self.index);
        frame.operand_stack.push_int(value)
    }
}

impl Instruction for ILoad0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_int(0);
        frame.operand_stack.push_int(value)
    }
}

impl Instruction for ILoad1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_int(1);
        frame.operand_stack.push_int(value)
    }
}

impl Instruction for ILoad2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_int(2);
        frame.operand_stack.push_int(value)
    }
}

impl Instruction for ILoad3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_int(3);
        frame.operand_stack.push_int(value)
    }
}

impl Instruction for LLoad {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_u8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_long(self.index);
        frame.operand_stack.push_long(value)
    }
}

impl Instruction for LLoad0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_long(0);
        frame.operand_stack.push_long(value)
    }
}

impl Instruction for LLoad1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_long(1);
        frame.operand_stack.push_long(value)
    }
}

impl Instruction for LLoad2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_long(2);
        frame.operand_stack.push_long(value)
    }
}

impl Instruction for LLoad3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.local_vars.get_long(3);
        frame.operand_stack.push_long(value)
    }
}

impl Instruction for AStore {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_i8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_ref();
        frame.local_vars.set_ref(self.index, value)
    }
}

impl Instruction for AStore0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_ref();
        frame.local_vars.set_ref(0, value)
    }
}

impl Instruction for AStore1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_ref();
        frame.local_vars.set_ref(1, value)
    }
}

impl Instruction for AStore2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_ref();
        frame.local_vars.set_ref(2, value)
    }
}

impl Instruction for AStore3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_ref();
        frame.local_vars.set_ref(3, value)
    }
}

impl Instruction for DStore {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_i8().unwrap() as usize;
    }
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_double();
        frame.local_vars.set_double(self.index, value)
    }
}

impl Instruction for DStore0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_double();
        frame.local_vars.set_double(0, value)
    }
}

impl Instruction for DStore1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_double();
        frame.local_vars.set_double(1, value)
    }
}

impl Instruction for DStore2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_double();
        // TODO wtf???
        frame.local_vars.set_double(2, value)
    }
}

impl Instruction for DStore3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_double();
        // TODO wtf???
        frame.local_vars.set_double(3, value)
    }
}

impl Instruction for FStore {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_i8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_float();
        frame.local_vars.set_float(self.index, value)
    }
}

impl Instruction for FStore0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_float();
        frame.local_vars.set_float(0, value)
    }
}

impl Instruction for FStore1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_float();
        frame.local_vars.set_float(1, value)
    }
}

impl Instruction for FStore2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_float();
        frame.local_vars.set_float(2, value)
    }
}

impl Instruction for FStore3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_float();
        frame.local_vars.set_float(3, value)
    }
}

impl Instruction for IStore {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_i8().unwrap() as usize;
    }

    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_int();
        frame.local_vars.set_int(self.index, value)
    }
}

impl Instruction for IStore0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_int();
        frame.local_vars.set_int(0, value)
    }
}

impl Instruction for IStore1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_int();
        frame.local_vars.set_int(1, value)
    }
}

impl Instruction for IStore2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_int();
        frame.local_vars.set_int(2, value)
    }
}

impl Instruction for IStore3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_int();
        frame.local_vars.set_int(3, value)
    }
}

impl Instruction for LStore {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_i8().unwrap() as usize;
    }
    
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_long();
        frame.local_vars.set_long(self.index, value)
    }
}

impl Instruction for LStore0 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_long();
        frame.local_vars.set_long(0, value)
    }
}

impl Instruction for LStore1 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_long();
        frame.local_vars.set_long(1, value)
    }
}

impl Instruction for LStore2 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_long();
        frame.local_vars.set_long(2, value)
    }
}

impl Instruction for LStore3 {
    fn execute(&self, frame: &mut Frame) {
        let value = frame.operand_stack.pop_long();
        frame.local_vars.set_long(3, value)
    }
}

impl Instruction for Dup {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.operand_stack.borrow_mut();
        let value = stack.pop_slot();
        stack.push_slot(value)
    }
}

impl Instruction for DupX1 {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.operand_stack.borrow_mut();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot3 = slot1.clone();
        stack.push_slot(slot1);
        stack.push_slot(slot2);
        stack.push_slot(slot3);
    }
}

impl Instruction for DupX2 {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.operand_stack.borrow_mut();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot3 = stack.pop_slot();
        let slot1_cloned = slot1.clone();
        stack.push_slot(slot1);
        stack.push_slot(slot3);
        stack.push_slot(slot2);
        stack.push_slot(slot1_cloned);
    }
}

impl Instruction for Dup2 {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.operand_stack.borrow_mut();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot1_cloned = slot1.clone();
        let slot2_cloned = slot2.clone();
        stack.push_slot(slot2);
        stack.push_slot(slot1);
        stack.push_slot(slot2_cloned);
        stack.push_slot(slot1_cloned);
    }
}

impl Instruction for Dup2X1 {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.operand_stack.borrow_mut();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot3 = stack.pop_slot();
        let slot1_cloned = slot1.clone();
        let slot2_cloned = slot2.clone();
        stack.push_slot(slot2);
        stack.push_slot(slot1);
        stack.push_slot(slot3);
        stack.push_slot(slot2_cloned);
        stack.push_slot(slot1_cloned);
    }
}

impl Instruction for Dup2X2 {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.operand_stack.borrow_mut();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        let slot3 = stack.pop_slot();
        let slot4 = stack.pop_slot();
        let slot1_cloned = slot1.clone();
        let slot2_cloned = slot2.clone();
        stack.push_slot(slot2);
        stack.push_slot(slot1);
        stack.push_slot(slot4);
        stack.push_slot(slot3);
        stack.push_slot(slot2_cloned);
        stack.push_slot(slot1_cloned);
    }
}

impl Instruction for Pop {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.pop_slot();
    }
}

impl Instruction for Pop2 {
    fn execute(&self, frame: &mut Frame) {
        frame.operand_stack.pop_slot();
        frame.operand_stack.pop_slot();
    }
}

impl Instruction for Swap {
    fn execute(&self, frame: &mut Frame) {
        let stack = frame.operand_stack.borrow_mut();
        let slot1 = stack.pop_slot();
        let slot2 = stack.pop_slot();
        stack.push_slot(slot1);
        stack.push_slot(slot2);
    }
}

impl Instruction for DAdd {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        let slot2 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(slot1+slot2);
    }
}

impl Instruction for FAdd {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        let slot2 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(slot1+slot2);
    }
}

impl Instruction for IAdd {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot1+slot2);
    }
}

impl Instruction for LAdd {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot1+slot2);
    }
}

impl Instruction for DSub {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        let slot2 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(slot2-slot1);
    }
}

impl Instruction for FSub {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        let slot2 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(slot2-slot1);
    }
}

impl Instruction for ISub{
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot2-slot1);
    }
}

impl Instruction for LSub {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot2-slot1);
    }
}

impl Instruction for DDiv {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        let slot2 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(slot2/slot1);
    }
}

impl Instruction for FDiv{
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        let slot2 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(slot2/slot1);
    }
}

impl Instruction for IDiv {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot2/slot1);
    }
}

impl Instruction for LDiv {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot2/slot1);
    }
}

impl Instruction for DMul {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        let slot2 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(slot1*slot2);
    }
}

impl Instruction for FMul {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        let slot2 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(slot1*slot2);
    }
}

impl Instruction for IMul {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot1*slot2);
    }
}

impl Instruction for LMul {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot1*slot2);
    }
}

impl Instruction for IAnd {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot1 & slot2);
    }
}

impl Instruction for LAnd {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot1 & slot2);
    }
}

impl Instruction for IINC {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.index = reader.read_u8().unwrap().into();
        self.value = reader.read_u8().unwrap().into();
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.local_vars.get_int(self.index);
        frame.local_vars.set_int(self.index, slot1 + self.value);
    }
}

impl Instruction for DNeg {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(-slot1);
    }
}

impl Instruction for FNeg {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(-slot1);
    }
}

impl Instruction for INeg {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(-slot1);
    }
}

impl Instruction for LNeg {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(-slot1);
    }
}

impl Instruction for IOR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot1 | slot2);
    }
}

impl Instruction for LOR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot1 | slot2);
    }
}

impl Instruction for DRem {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        let slot2 = frame.operand_stack.pop_double();
        frame.operand_stack.push_double(slot2 % slot1);
    }
}

impl Instruction for FRem {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        let slot2 = frame.operand_stack.pop_float();
        frame.operand_stack.push_float(slot2 % slot1);
    }
}

impl Instruction for IRem {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot2 % slot1);
    }
}

impl Instruction for LRem {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot2 % slot1);
    }
}

impl Instruction for ISHL {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot2 << ((slot1 as u32) & 0x1f));
    }
}

impl Instruction for ISHR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot2 >> ((slot1 as u32) & 0x1f));
    }
}

impl Instruction for IUSHR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(((slot2 as u32) >> ((slot1 as u32) & 0x1f)) as i32);
    }
}

impl Instruction for LSHL {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot2 << ((slot1 as u32) & 0x3f));
    }
}

impl Instruction for LSHR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(((slot2  as u64)>> ((slot1 as u32) & 0x3f)) as i64);
    }
}

impl Instruction for LUSHR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot2 >> ((slot1 as u32) & 0x3f));
    }
}

impl Instruction for IXOR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        frame.operand_stack.push_int(slot1 ^ slot2);
    }
}

impl Instruction for LXOR {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        frame.operand_stack.push_long(slot1 ^ slot2);
    }
}

impl Instruction for D2F {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        frame.operand_stack.push_float(slot1 as f32);
    }
}

impl Instruction for D2I {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        frame.operand_stack.push_int(slot1 as i32);
    }
}

impl Instruction for D2L {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        frame.operand_stack.push_long(slot1 as i64);
    }
}

impl Instruction for F2D {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        frame.operand_stack.push_double(slot1 as f64);
    }
}

impl Instruction for F2I {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        frame.operand_stack.push_int(slot1 as i32);
    }
}

impl Instruction for F2L {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        frame.operand_stack.push_long(slot1 as i64);
    }
}

impl Instruction for I2D {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        frame.operand_stack.push_double(slot1 as f64);
    }
}

impl Instruction for I2B {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int() as i8;
        frame.operand_stack.push_int(slot1 as i32);
    }
}

impl Instruction for I2C {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int() as u16;
        frame.operand_stack.push_int(slot1 as i32);
    }
}

impl Instruction for I2S {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int() as i16;
        frame.operand_stack.push_int(slot1 as i32);
    }
}

impl Instruction for I2F {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        frame.operand_stack.push_float(slot1 as f32);
    }
}

impl Instruction for I2L {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        frame.operand_stack.push_long(slot1 as i64);
    }
}

impl Instruction for L2D {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        frame.operand_stack.push_double(slot1 as f64);
    }
}

impl Instruction for L2F {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        frame.operand_stack.push_float(slot1 as f32);
    }
}

impl Instruction for L2I {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        frame.operand_stack.push_int(slot1 as i32);
    }
}

impl Instruction for GOTO {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }
    fn execute(&self, frame: &mut Frame) {
        // TODO 
        unsafe {
            frame.next_pc = (*(frame.thread)).pc + self.offset;
        }
    }
}

impl Instruction for LookUpSwitch {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32().unwrap();
        self.n_pairs = reader.read_i32().unwrap();
        self.match_offsets = reader.read_i32s(self.n_pairs * 2).unwrap();
    }

    fn execute(&self, frame: &mut Frame) {
        let key = frame.operand_stack.pop_int();
        for i in (0..self.n_pairs * 2).step_by(2) {
            if self.match_offsets[i as usize] == key {
                unsafe {
                    frame.next_pc = ( *frame.thread ).pc + (self.match_offsets[(i+1) as usize]);
                }
                return
            }
        }    
        unsafe {
            frame.next_pc = ( *frame.thread ).pc + (self.default_offset);
        }
    }
}

impl Instruction for TableSwitch {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32().unwrap();
        self.low = reader.read_i32().unwrap();
        self.high = reader.read_i32().unwrap();
        self.jump_offsets = reader.read_i32s(self.high - self.low +1).unwrap();
    }

    fn execute(&self, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int();
        let mut offset = self.default_offset;
        if index >= self.low && index <= self.high {
            offset = self.jump_offsets[(index - self.low) as usize]
        }
        unsafe {
            frame.next_pc = ( *frame.thread ).pc + (offset);
        }
    }
}

impl Instruction for DCMPG {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        let slot2 = frame.operand_stack.pop_double();
        if slot1.is_nan() || slot2.is_nan() {
            frame.operand_stack.push_int(1);
            return;
        }
        if slot1 < slot2 {
            frame.operand_stack.push_int(1)
        } else if slot1 == slot2{
            frame.operand_stack.push_int(0)
        } else {
            frame.operand_stack.push_int(-1)
        }
    }
}

impl Instruction for DCMPL {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_double();
        let slot2 = frame.operand_stack.pop_double();
        if slot1.is_nan() || slot2.is_nan() {
            frame.operand_stack.push_int(-1);
            return;
        }
        if slot1 < slot2 {
            frame.operand_stack.push_int(1)
        } else if slot1 == slot2{
            frame.operand_stack.push_int(0)
        } else {
            frame.operand_stack.push_int(-1)
        }
    }
}

impl Instruction for FCMPG {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        let slot2 = frame.operand_stack.pop_float();
        if slot1.is_nan() || slot2.is_nan() {
            frame.operand_stack.push_int(1);
            return;
        }
        if slot1 < slot2 {
            frame.operand_stack.push_int(1)
        } else if slot1 == slot2{
            frame.operand_stack.push_int(0)
        } else {
            frame.operand_stack.push_int(-1)
        }
    }
}

impl Instruction for FCMPL {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_float();
        let slot2 = frame.operand_stack.pop_float();
        if slot1.is_nan() || slot2.is_nan() {
            frame.operand_stack.push_int(-1);
            return;
        }
        if slot1 < slot2 {
            frame.operand_stack.push_int(1)
        } else if slot1 == slot2{
            frame.operand_stack.push_int(0)
        } else {
            frame.operand_stack.push_int(-1)
        }
    }
}

impl Instruction for LCMP {
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_long();
        let slot2 = frame.operand_stack.pop_long();
        if slot1 < slot2 {
            frame.operand_stack.push_int(1)
        } else if slot1 == slot2{
            frame.operand_stack.push_int(0)
        } else {
            frame.operand_stack.push_int(-1)
        }
    }
}

impl Instruction for IFICMPEQ {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        if slot1 == slot2{
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFICMPNE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        if slot1 != slot2{
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFICMPLT {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        if slot1 > slot2{
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFICMPLE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        if slot1 >= slot2{
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFICMPGT {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        println!("IFICMPGT offset = {}", self.offset);
        if slot1 < slot2{
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFICMPGE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        let slot2 = frame.operand_stack.pop_int();
        if slot1 <= slot2{
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFACMPEQ {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = &frame.operand_stack.pop_ref().unwrap();
        let slot2 = &frame.operand_stack.pop_ref().unwrap();
        if Rc::ptr_eq(slot1, slot2){
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFACMPNE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = &frame.operand_stack.pop_ref().unwrap();
        let slot2 = &frame.operand_stack.pop_ref().unwrap();
        if Rc::ptr_eq(slot1, slot2){
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFEQ {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        if slot1 == 0 {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFNE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        if slot1 != 0 {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFLT {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        if slot1 < 0 {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFLE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        if slot1 <= 0 {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFGT {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }

    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        if slot1 > 0 {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFGE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }
    fn execute(&self, frame: &mut Frame) {
        let slot1 = frame.operand_stack.pop_int();
        if slot1 >= 0 {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for GOTO_W {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i32().unwrap();
    }
    fn execute(&self, frame: &mut Frame) {
        unsafe {
            frame.next_pc = ( *frame.thread ).pc + (self.offset);
        }
    }
}

impl Instruction for IFNULL {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }
    fn execute(&self, frame: &mut Frame) {
        let refernce = frame.operand_stack.pop_ref();
        if refernce.is_none() {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for IFNOTNULL {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        self.offset = reader.read_i16().unwrap() as i32;
    }
    fn execute(&self, frame: &mut Frame) {
        let refernce = frame.operand_stack.pop_ref();
        if refernce.is_some() {
            unsafe {
                frame.next_pc = ( *frame.thread ).pc + (self.offset);
            }
        }
    }
}

impl Instruction for WIDE {
    fn fetchOperands(&mut self, reader: &BytecodeReader) {
        let opcode = reader.read_u8().unwrap();
        match opcode {
            0x15=> {
                let mut inst: ILoad = ILoad{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
               
            0x16 => {
                let mut inst = LLoad{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x17  => {
                let mut inst = FLoad{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x18  => {
                let mut inst = DLoad{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x19  => {
                let mut inst = ALoad{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x36  => {
                let mut inst = IStore{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x37  => {
                let mut inst = LStore{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x38  => {
                let mut inst = FStore{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x39  => {
                let mut inst = DStore{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x3a  => {
                let mut inst = AStore{index:0};
                inst.index = reader.read_u16().unwrap() as usize;
                self.modified_inst = Box::new(inst)
            }
            0x84  => {
                let mut inst = IINC{index:0,value:0};
                inst.index = reader.read_u16().unwrap() as usize;
                inst.value = reader.read_u16().unwrap() as i32;
                self.modified_inst = Box::new(inst)

            }
            _=>{
                panic!("Unsupported opcode: {}!", opcode)
            }
        }
    }

    fn execute(&self, frame: &mut Frame) {
        self.modified_inst.execute(frame);
    }

}


pub fn new_instruction(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x00=> {
            return Box::new(NoOperandsInstruction)
        }
    
        0x01=> {
            return Box::new(AConstNull)
        }
    
        0x02=> {
            return Box::new(IConstM1)
        }
    
        0x03=> {
            return Box::new(IConst0)
        }
    
        0x04=> {
            return Box::new(IConst1)
        }
    
        0x05=> {
            return Box::new(IConst2)
        }
    
        0x06=> {
            return Box::new(IConst3)
        }
    
        0x07=> {
            return Box::new(IConst4)
        }
    
        0x08=> {
            return Box::new(IConst5)
        }
    
        0x09=> {
            return Box::new(LConst0)
        }
    
        0x0a=> {
            return Box::new(LConst1)
        }
    
        0x0b=> {
            return Box::new(FConst0)
        }
    
        0x0c=> {
            return Box::new(FConst1)
        }
    
        0x0d=> {
            return Box::new(FConst2)
        }
    
        0x0e=> {
            return Box::new(DConst0)
        }
    
        0x0f=> {
            return Box::new(DConst1)
        }
    
        0x10=> {
            return Box::new(BIPush {value:Cell::new(0)})
        }
    
        0x11=> {
            return Box::new(SIPush{value:Cell::new(0)})
        }
    
        // 0x12=> {
        // 	return &LDC{}
        //}
    
        // 0x13=> {
        // 	return &LDC_W{}
        //}
    
        // 0x14=> {
        // 	return &LDC2_W{}
        //}
    
        0x15=> {
            return Box::new(ILoad{index:0})
        }
    
        0x16=> {
            return Box::new(ILoad{index:0})
        }
    
        0x17=> {
            return Box::new(FLoad{index:0})
        }
    
        0x18=> {
            return Box::new(DLoad{index:0})
        }
    
        0x19=> {
            return Box::new(ALoad{index:0})
        }
    
        0x1a=> {
            return Box::new(ILoad0)
        }
    
        0x1b=> {
            return Box::new(ILoad1)
        }
    
        0x1c=> {
            return Box::new(ILoad2)
        }
    
        0x1d=> {
            return Box::new(ILoad3)
        }
    
        0x1e=> {
            return Box::new(LLoad0)
        }
    
        0x1f=> {
            return Box::new(LLoad1)
        }
    
        0x20=> {
            return Box::new(LLoad2)
        }
    
        0x21=> {
            return Box::new(LLoad3)
        }
    
        0x22=> {
            return Box::new(FLoad0)
        }
    
        0x23=> {
            return Box::new(FLoad1)
        }
    
        0x24=> {
            return Box::new(FLoad2)
        }
    
        0x25=> {
            return Box::new(FLoad3)
        }
    
        0x26=> {
            return Box::new(DLoad0)
        }
    
        0x27=> {
            return Box::new(DLoad1)
        }
    
        0x28=> {
            return Box::new(DLoad2)
        }
    
        0x29=> {
            return Box::new(DLoad3)
        }
    
        0x2a=> {
            return Box::new(ALoad0)
        }
    
        0x2b=> {
            return Box::new(ALoad1)
        }
    
        0x2c=> {
            return Box::new(ALoad2)
        }
    
        0x2d=> {
            return Box::new(ALoad3)
        }
    
        // 0x2e=> {
        // 	return iaload
        //}
    
        // 0x2f=> {
        // 	return laload
        //}
    
        // 0x30=> {
        // 	return faload
        //}
    
        // 0x31=> {
        // 	return daload
        //}
    
        // 0x32=> {
        // 	return aaload
        //}
    
        // 0x33=> {
        // 	return baload
        //}
    
        // 0x34=> {
        // 	return caload
        //}
    
        // 0x35=> {
        // 	return saload
        //}
    
        0x36=> {
            return Box::new(IStore{index:0})
        }
    
        0x37=> {
            return Box::new(LStore{index:0})
        }
    
        0x38=> {
            return Box::new(FStore{index:0})
        }
    
        0x39=> {
            return Box::new(DStore{index:0})
        }
    
        0x3a=> {
            return Box::new(AStore{index:0})
        }
    
        0x3b=> {
            return Box::new(IStore0)
        }
    
        0x3c=> {
            return Box::new(IStore1)
        }
    
        0x3d=> {
            return Box::new(IStore2)
        }
    
        0x3e=> {
            return Box::new(IStore3)
        }
    
        0x3f=> {
            return Box::new(LStore0)
        }
    
        0x40=> {
            return Box::new(LStore1)
        }
    
        0x41=> {
            return Box::new(LStore2)
        }
    
        0x42=> {
            return Box::new(LStore3)
        }
    
        0x43=> {
            return Box::new(FStore0)
        }
    
        0x44=> {
            return Box::new(FStore1)
        }
    
        0x45=> {
            return Box::new(FStore2)
        }
    
        0x46=> {
            return Box::new(FStore3)
        }
    
        0x47=> {
            return Box::new(DStore0)
        }
    
        0x48=> {
            return Box::new(DStore1)
        }
    
        0x49=> {
            return Box::new(DStore2)
        }
    
        0x4a=> {
            return Box::new(DStore3)
        }
    
        0x4b=> {
            return Box::new(AStore0)
        }
    
        0x4c=> {
            return Box::new(AStore1)
        }
    
        0x4d=> {
            return Box::new(AStore2)
        }
    
        0x4e=> {
            return Box::new(AStore3)
        }
    
        // 0x4f=> {
        // 	return iastore
        //}
    
        // 0x50=> {
        // 	return lastore
        //}
    
        // 0x51=> {
        // 	return fastore
        //}
    
        // 0x52=> {
        // 	return dastore
        //}
    
        // 0x53=> {
        // 	return aastore
        //}
    
        // 0x54=> {
        // 	return bastore
        //}
    
        // 0x55=> {
        // 	return castore
        //}
    
        // 0x56=> {
        // 	return sastore
        //}
    
        0x57=> {
            return Box::new(Pop)
        }
    
        0x58=> {
            return Box::new(Pop2)
        }
    
        0x59=> {
            return Box::new(Dup)
        }
    
        0x5a=> {
            return Box::new(DupX1)
        }
    
        0x5b=> {
            return Box::new(DupX2)
        }
    
        0x5c=> {
            return Box::new(Dup2)
        }
    
        0x5d=> {
            return Box::new(Dup2X1)
        }
    
        0x5e=> {
            return Box::new(Dup2X2)
        }
    
        0x5f=> {
            return Box::new(Swap)
        }
    
        0x60=> {
            return Box::new(IAdd)
        }
    
        0x61=> {
            return Box::new(LAdd)
        }
    
        0x62=> {
            return Box::new(FAdd)
        }
    
        0x63=> {
            return Box::new(DAdd)
        }
    
        0x64=> {
            return Box::new(ISub)
        }
    
        0x65=> {
            return Box::new(LSub)
        }
    
        0x66=> {
            return Box::new(FSub)
        }
    
        0x67=> {
            return Box::new(DSub)
        }
    
        0x68=> {
            return Box::new(IMul)
        }
    
        0x69=> {
            return Box::new(LMul)
        }
    
        0x6a=> {
            return Box::new(FMul)
        }
    
        0x6b=> {
            return Box::new(DMul)
        }
    
        0x6c=> {
            return Box::new(IDiv)
        }
    
        0x6d=> {
            return Box::new(LDiv)
        }
    
        0x6e=> {
            return Box::new(FDiv)
        }
    
        0x6f=> {
            return Box::new(DDiv)
        }
    
        0x70=> {
            return Box::new(IRem)
        }
    
        0x71=> {
            return Box::new(LRem)
        }
    
        0x72=> {
            return Box::new(FRem)
        }
    
        0x73=> {
            return Box::new(DRem)
        }
    
        0x74=> {
            return Box::new(INeg)
        }
    
        0x75=> {
            return Box::new(LNeg)
        }
    
        0x76=> {
            return Box::new(FNeg)
        }
    
        0x77=> {
            return Box::new(DNeg)
        }
    
        0x78=> {
            return Box::new(ISHL)
        }
    
        0x79=> {
            return Box::new(LSHL)
        }
    
        0x7a=> {
            return Box::new(ISHR)
        }
    
        0x7b=> {
            return Box::new(LSHR)
        }
    
        0x7c=> {
            return Box::new(IUSHR)
        }
    
        0x7d=> {
            return Box::new(LUSHR)
        }
    
        0x7e=> {
            return Box::new(IAnd)
        }
    
        0x7f=> {
            return Box::new(LAnd)
        }
    
        0x80=> {
            return Box::new(IOR)
        }
    
        0x81=> {
            return Box::new(LOR)
        }
    
        0x82=> {
            return Box::new(IXOR)
        }
    
        0x83=> {
            return Box::new(LXOR)
        }
    
        0x84=> {
            return Box::new(IINC{index:0, value:0})
        }
    
        0x85=> {
            return Box::new(I2L)
        }
    
        0x86=> {
            return Box::new(I2F)
        }
    
        0x87=> {
            return Box::new(I2D)
        }
    
        0x88=> {
            return Box::new(L2I)
        }
    
        0x89=> {
            return Box::new(L2F)
        }
    
        0x8a=> {
            return Box::new(L2D)
        }
    
        0x8b=> {
            return Box::new(F2I)
        }
    
        0x8c=> {
            return Box::new(F2L)
        }
    
        0x8d=> {
            return Box::new(F2D)
        }
    
        0x8e=> {
            return Box::new(D2I)
        }
    
        0x8f=> {
            return Box::new(D2L)
        }
    
        0x90=> {
            return Box::new(D2F)
        }
    
        0x91=> {
            return Box::new(I2B)
        }
    
        0x92=> {
            return Box::new(I2C)
        }
    
        0x93=> {
            return Box::new(I2S)
        }
    
        0x94=> {
            return Box::new(LCMP)
        }
    
        0x95=> {
            return Box::new(FCMPL)
        }
    
        0x96=> {
            return Box::new(FCMPG)
        }
    
        0x97=> {
            return Box::new(DCMPL)
        }
    
        0x98=> {
            return Box::new(DCMPG)
        }
    
        0x99=> {
            return Box::new(IFEQ{offset:0})
        }
    
        0x9a=> {
            return Box::new(IFNE{offset:0})
        }
    
        0x9b=> {
            return Box::new(IFLT{offset:0})
        }
    
        0x9c=> {
            return Box::new(IFGE{offset:0})
        }
    
        0x9d=> {
            return Box::new(IFGT{offset:0})
        }
    
        0x9e=> {
            return Box::new(IFLE{offset:0})
        }
    
        0x9f=> {
            return Box::new(IFICMPEQ{offset:0})
        }
    
        0xa0=> {
            return Box::new(IFICMPNE{offset:0})
        }
    
        0xa1=> {
            return Box::new(IFICMPLT{offset:0})
        }
    
        0xa2=> {
            return Box::new(IFICMPGE{offset:0})
        }
    
        0xa3=> {
            return Box::new(IFICMPGT{offset:0})
        }
    
        0xa4=> {
            return Box::new(IFICMPLE{offset:0})
        }
    
        0xa5=> {
            return Box::new(IFACMPEQ{offset:0})
        }
    
        0xa6=> {
            return Box::new(IFACMPNE{offset:0})
        }
    
        0xa7=> {
            return Box::new(GOTO{offset:0})
        }
    
        // 0xa8=> {
        // 	return &JSR{}
        //}
    
        // 0xa9=> {
        // 	return &RET{}
        //}
    
        0xaa=> {
            return Box::new(TableSwitch{default_offset:0, low:0, high:0, jump_offsets:Vec::new()})
        }
    
        0xab=> {
            return Box::new(LookUpSwitch{default_offset:0,n_pairs:0,match_offsets:Vec::new()})
        }
    
        // 0xac=> {
        // 	return ireturn
        //}
    
        // 0xad=> {
        // 	return lreturn
        //}
    
        // 0xae=> {
        // 	return freturn
        //}
    
        // 0xaf=> {
        // 	return dreturn
        //}
    
        // 0xb0=> {
        // 	return areturn
        //}
    
        // 0xb1=> {
        // 	return _return
        //}
    
        //	0xb2=> {
        //		return &GET_STATIC{}
        //}
    
        // 0xb3=> {
        // 	return &PUT_STATIC{}
        //}
    
        // 0xb4=> {
        // 	return &GET_FIELD{}
        //}
    
        // 0xb5=> {
        // 	return &PUT_FIELD{}
        //}
    
        //	0xb6=> {
        //		return &INVOKE_VIRTUAL{}
        //}
    
        // 0xb7=> {
        // 	return &INVOKE_SPECIAL{}
        //}
    
        // 0xb8=> {
        // 	return &INVOKE_STATIC{}
        //}
    
        // 0xb9=> {
        // 	return &INVOKE_INTERFACE{}
        //}
    
        // 0xba=> {
        // 	return &INVOKE_DYNAMIC{}
        //}
    
        // 0xbb=> {
        // 	return &NEW{}
        //}
    
        // 0xbc=> {
        // 	return &NEW_ARRAY{}
        //}
    
        // 0xbd=> {
        // 	return &ANEW_ARRAY{}
        //}
    
        // 0xbe=> {
        // 	return arraylength
        //}
    
        // 0xbf=> {
        // 	return athrow
        //}
    
        // 0xc0=> {
        // 	return &CHECK_CAST{}
        //}
    
        // 0xc1=> {
        // 	return &INSTANCE_OF{}
        //}
    
        // 0xc2=> {
        // 	return monitorenter
        //}
    
        // 0xc3=> {
        // 	return monitorexit
        //}
    
        0xc4=> {
            return Box::new(WIDE{modified_inst:Box::new(NoOperandsInstruction)})
        }
    
        // 0xc5=> {
        // 	return &MULTI_ANEW_ARRAY{}
        //}
    
        0xc6=> {
            return Box::new(IFNULL{offset:0})
        }
    
        0xc7=> {
            return Box::new(IFNOTNULL{offset:0})
        }
    
        0xc8=> {
            return Box::new(GOTO_W{offset:0})
        }
        _ => {
            panic!("not parse!")
        }
    }
}