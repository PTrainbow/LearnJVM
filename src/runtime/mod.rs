use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Object {}

pub struct Thread {
    pub pc: u32,
    pub stack: Box<Stack>,
}

pub struct Stack {
    max_size: usize,
    size: usize,
    _top: Box<Frame>,
}

pub struct LocalVars(Vec<Slot>);

pub struct Frame {
    pub lower: Option<Box<Frame>>,
    pub local_vars: LocalVars,
    pub operand_stack: Box<OperandStack>,
}

// TODO clone option/box
#[derive(Clone, Debug)]
pub struct Slot {
    num: i32,
    reference: Option<Box<Object>>,
}

pub struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl Thread {
    pub fn new_thread() -> Thread {
        return Thread {
            pc: 0,
            stack: Box::new(Stack::new_stack(1024)),
        };
    }
}

impl Stack {
    fn new_stack(max_size: usize) -> Stack {
        return Stack {
            max_size,
            size: 0,
            _top: Box::new(Frame::new_frame(max_size, max_size)),
        };
    }
}

impl Frame {
    pub fn new_frame(max_local: usize, max_stack: usize) -> Frame {
        return Frame {
            lower: None,
            local_vars: LocalVars::new_local_vars(max_local),
            operand_stack: Box::new(OperandStack::new_operand_stack(max_stack)),
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
        self.size += 1;
    }

    pub(crate) fn pop_int(&mut self) -> i32 {
        self.size -= 1;
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

    pub(crate) fn push_ref(&mut self, value: Option<Box<Object>>) {
        self.slots[self.size].reference = value;
        self.size += 1;
    }

    pub(crate) fn pop_ref(&mut self) -> &Option<Box<Object>> {
        self.size -= 1;
        return &self.slots[self.size].reference;
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
    }

    pub fn get_int(&self, index: usize) -> &i32 {
        return &self.0[index].num;
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

    pub(crate) fn set_ref(&mut self, index: usize, value: Option<Box<Object>>) {
        self.0[index].reference = value;
    }

    pub(crate) fn get_ref(&self, index: usize) -> &Option<Box<Object>> {
        return &self.0[index].reference;
    }
}