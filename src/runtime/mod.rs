use std::cell::Cell;
use std::fmt::Error;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use crate::classfile::class_reader::{AttributeInfo, MethodInfo};
use crate::instructions::bitcode_reader::BytecodeReader;
use crate::instructions::{new_instruction, Instruction};

#[derive(Clone, Debug)]
pub struct Object {}

pub struct Thread {
    pub pc: i32,
    pub stack: Stack,
}

pub struct Stack {
    max_size: usize,
    size: usize,
    _top: *mut Frame,
}

pub struct LocalVars(Vec<Slot>);

pub struct Frame {
    pub lower: *mut Frame,
    pub local_vars: LocalVars,
    pub operand_stack: OperandStack,
    pub next_pc: i32,
    pub thread: *mut Thread,
}

// TODO clone option/box
#[derive(Clone, Debug)]
pub struct Slot {
    num: i32,
    reference: Option<Rc<Object>>,
}

pub struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl Thread {
    pub fn new_thread() -> Thread {
        return Thread {
            pc: 0,
            stack: Stack::new_stack(1024),
        };
    }

    pub fn pc(&self) -> i32 {
        return self.pc;
    }

    pub fn set_pc(&mut self, pc: i32) {
        self.pc = pc;
    }

    pub fn pop_frame(&mut self) -> *mut Frame {
        return self.stack.pop();
    }

    pub fn current_frame(&mut self) -> *mut Frame {
        return self.stack.top();
    }

    pub fn push_frame(&mut self, frame: *mut Frame) {
        return self.stack.push(frame);
    }
}

impl Stack {
    fn new_stack(max_size: usize) -> Stack {
        return Stack {
            max_size,
            size: 0,
            _top: std::ptr::null_mut(),
        };
    }

    pub fn push(&mut self, frame: *mut Frame) {
        if self.size >= self.max_size {
            panic!("stackOverflow");
        }
        if !self._top.is_null() {
            unsafe { (*frame).lower = self._top }
        }
        self._top = frame;
        self.size += 1;
    }

    pub fn pop(&mut self) -> *mut Frame {
        if self._top.is_null() {
            panic!("stack is empty");
        }
        let top = self._top;
        unsafe {
            self._top = (*top).lower;
            (*top).lower = std::ptr::null_mut()
        }
        self.size -= 1;
        return top;
    }

    pub fn top(&mut self) -> *mut Frame {
        if self._top.is_null() {
            panic!("stack is empty");
        }
        return self._top;
    }
}

impl Frame {
    pub fn new_frame(thread: *mut Thread, max_local: usize, max_stack: usize) -> Frame {
        return Frame {
            lower: std::ptr::null_mut(),
            local_vars: LocalVars::new_local_vars(max_local),
            operand_stack: OperandStack::new_operand_stack(max_stack),
            next_pc: 0,
            thread,
        };
    }
}

impl OperandStack {
    fn new_operand_stack(max_stack: usize) -> OperandStack {
        return OperandStack {
            size: 0,
            slots: vec![Slot::new_slot(); max_stack],
        };
    }

    pub(crate) fn push_int(&mut self, value: i32) {
        self.slots[self.size].num = value;
        // TOOD remove
        println!("OperandStack push_int[{}] {}", self.size, value);
        self.size += 1;
    }

    pub(crate) fn pop_int(&mut self) -> i32 {
        self.size -= 1;
        println!("OperandStack pop_int[{}] {}", self.size, self.slots[self.size].num);
        return self.slots[self.size].num;
    }

    pub(crate) fn push_float(&mut self, value: f32) {
        self.slots[self.size].num = value.to_bits() as i32;
        self.size += 1;
    }

    pub(crate) fn pop_float(&mut self) -> f32 {
        self.size -= 1;
        return f32::from_bits(self.slots[self.size].num as u32);
    }

    pub(crate) fn push_double(&mut self, value: f64) {
        self.push_long(value.to_bits() as i64)
    }

    pub(crate) fn pop_double(&mut self) -> f64 {
        return f64::from_bits(self.pop_long() as u64);
    }

    pub(crate) fn push_long(&mut self, value: i64) {
        self.slots[self.size].num = value as i32;
        self.slots[self.size + 1].num = (value >> 32) as i32;
        self.size += 2;
    }

    pub(crate) fn pop_long(&mut self) -> i64 {
        self.size -= 2;
        let low = self.slots[self.size].num as u32 as i64;
        let high = self.slots[self.size + 1].num as i64;
        return high << 32 | low;
    }

    pub(crate) fn push_ref(&mut self, value: Option<Rc<Object>>) {
        self.slots[self.size].reference = value;
        self.size += 1;
    }

    pub(crate) fn pop_ref(&mut self) -> Option<Rc<Object>> {
        self.size -= 1;
        let slot_removed = self.slots.remove(self.size);
        self.slots[self.size] = (Slot {
            num: slot_removed.num,
            reference: Option::None,
        });
        return slot_removed.reference;
    }

    pub(crate) fn push_slot(&mut self, value: Slot) {
        self.slots[self.size] = value;
        self.size += 1;
    }

    pub(crate) fn pop_slot(&mut self) -> Slot {
        self.size -= 1;
        let slot_removed = self.slots.remove(self.size);
        self.push_slot(Slot {
            num: 0,
            reference: Option::None,
        });
        return slot_removed;
    }
}

impl Slot {
    fn new_slot() -> Slot {
        return Slot {
            num: 0,
            reference: None,
        };
    }
}

impl LocalVars {
    fn new_local_vars(max_size: usize) -> LocalVars {
        return LocalVars(vec![Slot::new_slot(); max_size]);
    }

    pub fn set_int(&mut self, index: usize, value: i32) {
        self.0[index].num = value;
        // TODO remove
        println!("LocalVars set_int[{}] value = {}", index, value);
    }

    pub fn get_int(&self, index: usize) -> i32 {
        return self.0[index].num;
    }

    pub(crate) fn get_float(&self, index: usize) -> f32 {
        return f32::from_bits(self.0[index].num as u32);
    }

    pub(crate) fn set_float(&mut self, index: usize, value: f32) {
        self.0[index].num = value.to_bits() as i32;
    }

    pub(crate) fn set_long(&mut self, index: usize, value: i64) {
        self.0[index].num = value as i32;
        self.0[index + 1].num = (value >> 32) as i32;
    }

    pub(crate) fn get_long(&self, index: usize) -> i64 {
        let low = self.0[index].num as u32 as i64;
        let high = self.0[index + 1].num as i64;
        return high << 32 | low;
    }

    pub(crate) fn set_double(&mut self, index: usize, value: f64) {
        self.set_long(index, value.to_bits() as i64)
    }

    pub(crate) fn get_double(&self, index: usize) -> f64 {
        return f64::from_bits(self.get_long(index) as u64);
    }

    pub(crate) fn set_ref(&mut self, index: usize, value: Option<Rc<Object>>) {
        self.0[index].reference = value;
    }

    pub(crate) fn get_ref(&mut self, index: usize) -> Option<Rc<Object>> {
        let slot_removed = self.0.remove(index);
        self.0.push(Slot {
            num: slot_removed.num,
            reference: slot_removed.reference.clone(),
        });
        return slot_removed.reference;
    }
}

pub fn interpret(method: &MethodInfo) {
    let attribute_list = &method.attribute_info;
    for attribute in attribute_list {
        if let AttributeInfo::CodeAttribute{max_stacks, max_locals,code_length,code,exception_table,attributes} = attribute {
            let mut thread = Thread::new_thread();
            let mut frame = Frame::new_frame(std::ptr::addr_of_mut!(thread), *max_locals as usize, *max_stacks as usize);
            thread.push_frame(std::ptr::addr_of_mut!(frame));
            // TODO copy???
            innerLoop(&mut thread, code.to_owned());
        }
    }
}

pub fn innerLoop(thread: &mut Thread, bytecode: Vec<u8>){
    let frame = thread.pop_frame();
    let mut reader = BytecodeReader {
        content: bytecode,
        cursor: Cell::new(0),
    };
    while true {
        let next_pc = unsafe { (*frame).next_pc };
        thread.set_pc(next_pc);
        reader.reset( next_pc);
        let opcode = reader.read_u8().unwrap();
	    let mut inst =  new_instruction(opcode);
		inst.fetchOperands(&reader);
        let pc = reader.cursor.get();
        unsafe {
            (*frame).next_pc = pc;
		    println!("pc:{} inst:{:?}", pc, inst);
		    // execute
		    inst.execute(&mut *frame);
            for local in &(*frame).local_vars.0 {
                println!("local vars = {}", local.num);
            }
            println!("");
        }
        sleep(Duration::from_millis(10))
    }
    // todo
}
