use std::collections::VecDeque;

use crate::{ValueEnv, interpreter_utils::global};

use compiler::{
    constants::*,
    syntax_trees::{shared::*, x86::*},
};

#[derive(Debug, Default)]
struct Eflags {
    pub zero: bool,     // ZF
    pub carry: bool,    // CF
    pub negative: bool, // SF
    pub overflow: bool, // OF
}

const HEAP_OFFSET: usize = 0x10000;
const FUNCTIONS_OFFSET: usize = 0x20000;

const SPECIAL_FUNCTIONS: [&str; 9] = [
    FN_READ_INT,
    FN_READ_STR,
    FN_PRINT_INT,
    FN_PRINT_STR,
    FN_GC_INITIALIZE,
    FN_GC_COLLECT,
    FN_SUBSCRIPT_ARRAY,
    FN_ASSIGN_TO_ARRAY_ELEM,
    FN_STR_CONCAT,
];

#[derive(Debug)]
struct X86Env {
    vars: ValueEnv,
    regs: [i64; 16],
    stack: [u8; 0x1000],
    eflags: Eflags,
    heap: [u8; 0x10000],
    gc_free_ptr: i64,
    functions: Vec<Identifier>,
    special_function_offset: usize,
}

impl X86Env {
    fn new(mut functions: Vec<Identifier>) -> Self {
        let special_function_offset = functions.len();
        functions.extend(SPECIAL_FUNCTIONS.map(|f| global!(f)));

        let mut vars = ValueEnv::new();
        vars.extend(
            functions
                .iter()
                .enumerate()
                .map(|(idx, f_id)| (f_id.clone(), Value::I64((idx | FUNCTIONS_OFFSET) as i64))),
        );

        let mut ret = Self {
            vars,
            regs: [0; 16],
            stack: [0; _],
            eflags: Eflags::default(),
            heap: [0; _],
            gc_free_ptr: HEAP_OFFSET as i64,
            functions,
            special_function_offset,
        };

        ret.regs[Register::rsp as usize] = ret.stack.len() as i64;
        ret.regs[Register::rbp as usize] = ret.stack.len() as i64;

        ret
    }

    fn write_high_byte(&mut self, reg: Register, value: i64) {
        self.regs[reg as usize] &= 0xFFFF_FFFF_FFFF_00FFu64 as i64; // Zero the "Ah" byte
        self.regs[Register::rax as usize] |= (value & 0x0000_0000_0000_00FFu64 as i64) << 8;
    }

    fn write_low_byte(&mut self, reg: Register, value: i64) {
        self.regs[reg as usize] &= 0xFFFF_FFFF_FFFF_FF00u64 as i64; // Zero the "Ah" byte
        self.regs[Register::rax as usize] |= value & 0x0000_0000_0000_00FFu64 as i64;
    }

    fn write_arg(&mut self, arg: &Arg, value: i64) {
        match &arg.value {
            ArgValue::Reg(reg) => {
                let slot = &mut self.regs[reg.to_quad() as usize];

                let mask_lshift = if reg.is_high_byte() { 8 } else { 0 };
                let mask = arg.width.mask() << mask_lshift;
                let masked_value = (value << mask_lshift) & mask;

                if arg.width == Width::Double {
                    // Writing to a double-word register zeroes out the
                    // upper 32 bits as well. Not true for other widths
                    *slot = 0;
                } else {
                    *slot &= !mask;
                }
                *slot |= masked_value;
            }
            ArgValue::Variable(id) => {
                self.vars
                    .insert(id.clone(), Value::I64(value & arg.width.mask()));
            }
            ArgValue::Deref(reg, offset) => {
                let base = self.regs[*reg as usize];
                let mut addr: usize = (base + (*offset as i64)).try_into().unwrap();

                if (addr & FUNCTIONS_OFFSET) != 0 {
                    panic!("Tried to write to function memory")
                }

                let memory = if addr & HEAP_OFFSET != 0 {
                    self.heap.as_mut_slice()
                } else {
                    self.stack.as_mut_slice()
                };
                addr &= !HEAP_OFFSET;

                let bytes = value.to_le_bytes();
                for (idx, byte) in bytes.iter().enumerate().take(arg.width.bytes()) {
                    if addr + idx >= memory.len() {
                        __breakpoint_target();
                    }
                    memory[addr + idx] = *byte;
                }
            }
            ArgValue::Immediate(_) => panic!("Can't write to an intermediate"),
            ArgValue::Global(name) => {
                if name == &global!(GC_FREE_PTR) {
                    if arg.width != Width::Quad {
                        panic!("Writing to {GC_FREE_PTR} with non-quadword width?")
                    }
                    self.gc_free_ptr = value;
                }
                // No other globals should matter in sim?
            }
        }
    }

    fn read_arg(&self, arg: &Arg) -> i64 {
        match &arg.value {
            ArgValue::Reg(reg) => {
                let slot_value = self.regs[reg.to_quad() as usize];

                if reg.is_high_byte() {
                    (slot_value >> 8) & arg.width.mask()
                } else {
                    slot_value & arg.width.mask()
                }
            }
            ArgValue::Variable(id) => self
                .vars
                .get(id)
                .map(|val| i64::from(val) & arg.width.mask())
                .expect(format!("Unknown x86var identifier: {id:?}").as_str()),
            ArgValue::Deref(reg, offset) => {
                let base = self.regs[*reg as usize];
                let mut addr: usize = (base + (*offset as i64)).try_into().unwrap();

                let mut bytes = [0u8; 8];
                let memory = if addr & HEAP_OFFSET != 0 {
                    self.heap.as_slice()
                } else {
                    self.stack.as_slice()
                };

                addr &= !HEAP_OFFSET;

                for (idx, byte) in bytes.iter_mut().enumerate().take(arg.width.bytes()) {
                    *byte = memory[addr + idx];
                }

                i64::from_le_bytes(bytes)
            }
            ArgValue::Immediate(val) => *val & arg.width.mask(),
            ArgValue::Global(name) => {
                // I think its ok for these all to just be 0 - it'll
                // trigger __gc_collect() every time but whatever
                let zeroable_gc_vars = [
                    global!(GC_FROMSPACE_BEGIN),
                    global!(GC_FROMSPACE_END),
                    global!(GC_ROOTSTACK_BEGIN),
                    global!(GC_ROOTSTACK_END),
                ];

                if name == &global!(GC_FREE_PTR) {
                    self.gc_free_ptr
                } else if zeroable_gc_vars.contains(name) {
                    0
                } else if let Some(func_idx) = self.functions.iter().position(|f| f == name) {
                    (func_idx | FUNCTIONS_OFFSET) as i64
                } else {
                    unimplemented!("Unknown global symbol: `{name:?}`")
                }
            }
        }
    }

    fn compare(&mut self, s: i64, d: i64) {
        let val = d.wrapping_sub(s);
        self.eflags = Eflags {
            zero: val == 0,
            negative: val < 0,
            carry: (d as u64) < (s as u64),
            overflow: ((d ^ s) < 0) && ((d ^ val) < 0),
        }
    }

    fn get_comparison(&self, cc: &Comparison) -> bool {
        match cc {
            Comparison::Equals => self.eflags.zero,
            Comparison::NotEquals => !self.eflags.zero,
            Comparison::Less => self.eflags.negative != self.eflags.overflow,
            Comparison::LessEquals => {
                (self.eflags.negative != self.eflags.overflow) || self.eflags.zero
            }
            Comparison::Greater => {
                !(self.eflags.negative != self.eflags.overflow) && !self.eflags.zero
            }
            Comparison::GreaterEquals => !(self.eflags.negative != self.eflags.overflow),
        }
    }
}

fn execute_special_functions(
    idx: usize,
    inputs: &mut VecDeque<Value>,
    outputs: &mut VecDeque<Value>,
    env: &mut X86Env,
) -> bool {
    if idx < env.special_function_offset {
        return false;
    }

    let arr_idx = idx - env.special_function_offset;
    let label = SPECIAL_FUNCTIONS[arr_idx];

    if label == FN_PRINT_INT {
        let int = env.read_arg(&Arg::new_reg(Register::rdi));
        outputs.push_back(Value::I64(int));
        return true;
    } else if label == FN_PRINT_STR {
        let str_tag_i64 = env.read_arg(&Arg::new_deref(Register::rdi, 0, Width::Quad));
        let array_tag = ArrayTag::from_bits(str_tag_i64 as u64);

        let mut chars = Vec::with_capacity(array_tag.length() as _);
        for i in 0..array_tag.length() {
            let c_i64 = env.read_arg(&Arg::new_deref(
                Register::rdi,
                (size_of_val(&array_tag) + i as usize) as _,
                Width::Byte,
            ));
            let c = char::from_u32(c_i64 as _).expect("Couldn't convert to char");
            chars.push(Value::Char(c));
        }
        outputs.push_back(Value::Array(chars));

        return true;
    } else if label == FN_READ_INT {
        let input = inputs.pop_front().expect("Overflowed input values");
        if let Value::I64(int) = input {
            env.write_arg(&Arg::new_reg(Register::rax), int);
        } else {
            panic!("Expected int input, got {input:?}")
        }
        return true;
    } else if label == FN_READ_STR {
        let input = inputs.pop_front().expect("Overflowed input values");
        if let Value::Array(elems) = &input
            && elems.iter().all(|e| matches!(e, Value::Char(_)))
        {
            let ps = POINTER_SIZE as usize;

            let chars = elems.iter().map(|e| {
                if let Value::Char(c) = e {
                    *c as u8
                } else {
                    unreachable!()
                }
            });
            // Concatenate and allocate new string on heap
            let new_tag = ArrayTag::new()
                .with_length(elems.len() as _)
                .with_elem_size(1)
                .into_bits();

            let alloc_size = size_of_val(&new_tag) + elems.len();
            let out_ptr = env.gc_free_ptr;
            env.gc_free_ptr += alloc_size as i64;

            // Write tag bytes to heap
            let heap_addr = (out_ptr as usize) & !HEAP_OFFSET;
            for (i, byte) in new_tag.to_le_bytes().iter().enumerate() {
                env.heap[heap_addr + i] = *byte;
            }
            // Write string A chars
            for (i, c) in chars.enumerate() {
                env.heap[heap_addr + ps + i] = c;
            }

            // Return pointer in rax
            env.write_arg(&Arg::new_reg(Register::rax), out_ptr);
        } else {
            panic!("Expected string input, got {input:?}")
        }
        return true;
    } else if label == FN_GC_INITIALIZE {
        // Don't need to do anything in sim
        return true;
    } else if label == FN_GC_COLLECT {
        // Don't need to do anything in sim
        return true;
    } else if label == FN_SUBSCRIPT_ARRAY {
        let idx = env.read_arg(&Arg::new_reg(Register::rsi));

        let tag = ArrayTag::from(env.read_arg(&Arg::new_deref(Register::rdi, 0, Width::Quad)));
        if idx >= tag.length() as _ {
            panic!("Tried to index past end of array");
        }

        let elem_mask = Width::from(tag.elem_size()).mask();

        let val = env.read_arg(&Arg::new_deref(
            Register::rdi,
            (POINTER_SIZE + (tag.elem_size() as i64 * idx)) as i32,
            Width::from(tag.elem_size()),
        ));
        env.write_arg(&Arg::new_reg(Register::rax), val & elem_mask);

        return true;
    } else if label == FN_ASSIGN_TO_ARRAY_ELEM {
        let idx = env.read_arg(&Arg::new_reg(Register::rsi));

        let tag = ArrayTag::from(env.read_arg(&Arg::new_deref(Register::rdi, 0, Width::Quad)));
        if idx >= tag.length() as _ {
            panic!("Tried to index past end of array");
        }

        let val = env.read_arg(&Arg::new_reg(Register::rdx));
        env.write_arg(
            &Arg::new_deref(
                Register::rdi,
                (POINTER_SIZE + (tag.elem_size() as i64 * idx)) as i32,
                Width::from(tag.elem_size()),
            ),
            val,
        );

        return true;
    } else if label == FN_STR_CONCAT {
        let ps = POINTER_SIZE as usize;

        // Read string A from rdi
        let a_tag = ArrayTag::from(env.read_arg(&Arg::new_deref(Register::rdi, 0, Width::Quad)));
        let mut a_chars: Vec<u8> = Vec::with_capacity(a_tag.length() as usize);
        for i in 0..a_tag.length() as usize {
            let c = env.read_arg(&Arg::new_deref(Register::rdi, (ps + i) as _, Width::Byte)) as u8;
            if c == 0 {
                break;
            } // Break on null terminator
            a_chars.push(c);
        }

        // Read string B from rsi
        let b_tag = ArrayTag::from(env.read_arg(&Arg::new_deref(Register::rsi, 0, Width::Quad)));
        let mut b_chars: Vec<u8> = Vec::with_capacity(b_tag.length() as usize);
        for i in 0..b_tag.length() as usize {
            let c = env.read_arg(&Arg::new_deref(Register::rsi, (ps + i) as _, Width::Byte)) as u8;
            if c == 0 {
                break;
            } // Break on null terminator
            b_chars.push(c);
        }

        // Concatenate and allocate new string on heap
        let new_len = a_chars.len() + b_chars.len() + 1; // +1 for null terminator
        let new_tag = ArrayTag::new()
            .with_length(new_len as _)
            .with_elem_size(1)
            .into_bits();

        let alloc_size = size_of_val(&new_tag) + new_len;
        let out_ptr = env.gc_free_ptr;
        env.gc_free_ptr += alloc_size as i64;

        // Write tag bytes to heap
        let heap_addr = (out_ptr as usize) & !HEAP_OFFSET;
        for (i, byte) in new_tag.to_le_bytes().iter().enumerate() {
            env.heap[heap_addr + i] = *byte;
        }
        // Write string A chars
        for (i, &c) in a_chars.iter().enumerate() {
            env.heap[heap_addr + ps + i] = c;
        }
        // Write string B chars
        for (i, &c) in b_chars.iter().enumerate() {
            env.heap[heap_addr + ps + a_chars.len() + i] = c;
        }
        // Write null terminator
        env.heap[heap_addr + ps + a_chars.len() + b_chars.len()] = 0;

        // Return pointer in rax
        env.write_arg(&Arg::new_reg(Register::rax), out_ptr);

        return true;
    } else {
        // No match found, must be another call
        return false;
    }
}

enum Continuation {
    Next,
    Jump(Identifier),
    Call(usize),
    Return,
    Exit,
}

fn run_instr(
    instr: &Instr,
    inputs: &mut VecDeque<Value>,
    outputs: &mut VecDeque<Value>,
    env: &mut X86Env,
) -> Continuation {
    match instr {
        Instr::add(s, d) => {
            env.write_arg(d, env.read_arg(s) + env.read_arg(d));
            Continuation::Next
        }
        Instr::sub(s, d) => {
            env.write_arg(d, env.read_arg(d) - env.read_arg(s));
            Continuation::Next
        }
        Instr::imul(s, d) => {
            env.write_arg(d, env.read_arg(d) * env.read_arg(s));
            Continuation::Next
        }
        Instr::neg(d) => {
            env.write_arg(d, -env.read_arg(d));
            Continuation::Next
        }
        Instr::mov(s, d) => {
            env.write_arg(d, env.read_arg(s));
            Continuation::Next
        }
        Instr::push(s) => {
            env.write_arg(
                &Arg::new_reg(Register::rsp),
                env.read_arg(&Arg::new_reg(Register::rsp)) - 8,
            );
            env.write_arg(&Arg::new_deref(Register::rsp, 0, s.width), env.read_arg(s));
            Continuation::Next
        }
        Instr::pop(d) => {
            env.write_arg(d, env.read_arg(&Arg::new_deref(Register::rsp, 0, d.width)));
            env.write_arg(
                &Arg::new_reg(Register::rsp),
                env.read_arg(&Arg::new_reg(Register::rsp)) + 8,
            );
            Continuation::Next
        }
        Instr::call(s, _num_args) => {
            let func = env.read_arg(s);
            let func_idx = (func as usize) & !FUNCTIONS_OFFSET;
            if execute_special_functions(func_idx, inputs, outputs, env) {
                // If this is a special function, then
                // execute_special_functions() will take care of
                // everything including return value, so just `Next` it.
                Continuation::Next
            } else {
                Continuation::Call(func_idx)
            }
        }
        Instr::ret => Continuation::Return,
        Instr::xor(s, d) => {
            env.write_arg(d, env.read_arg(s) ^ env.read_arg(d));
            Continuation::Next
        }
        Instr::cmp(s, d) => {
            env.compare(env.read_arg(s), env.read_arg(d));
            Continuation::Next
        }
        Instr::set(cc, d) => {
            env.write_arg(d, env.get_comparison(cc) as i64);
            Continuation::Next
        }
        Instr::movzx(s, d) => {
            assert!(matches!(d.value, ArgValue::Reg(_)));
            env.write_arg(d, env.read_arg(s));
            Continuation::Next
        }
        Instr::movsx(s, d) => {
            assert!(matches!(d.value, ArgValue::Reg(_)));
            env.write_arg(d, env.read_arg(s));
            Continuation::Next
        }
        Instr::jmp(label) => Continuation::Jump(label.clone()),
        Instr::jmpcc(cc, label) => {
            if env.get_comparison(cc) {
                Continuation::Jump(label.clone())
            } else {
                Continuation::Next
            }
        }
        Instr::sar(s, d) => {
            env.write_arg(d, env.read_arg(d) >> env.read_arg(s));
            Continuation::Next
        }
        Instr::sal(s, d) => {
            env.write_arg(d, env.read_arg(d) << env.read_arg(s));
            Continuation::Next
        }
        Instr::and(s, d) => {
            env.write_arg(d, env.read_arg(d) & env.read_arg(s));
            Continuation::Next
        }
        Instr::lea(s, d) => {
            env.write_arg(d, env.read_arg(s));
            Continuation::Next
        }
        Instr::call_ind(s, _) => {
            let func = env.read_arg(s);
            let func_idx = (func as usize) & !FUNCTIONS_OFFSET;
            if execute_special_functions(func_idx, inputs, outputs, env) {
                // If this is a special function, then
                // execute_special_functions() will take care of
                // everything including return value, so just `Next` it.
                Continuation::Next
            } else {
                Continuation::Call(func_idx)
            }
        }
        Instr::jmp_tail(s, _) => {
            let func = env.read_arg(s);
            let func_idx = (func as usize) & !FUNCTIONS_OFFSET;
            if execute_special_functions(func_idx, inputs, outputs, env) {
                // This is a tail call whose stack frame is already torn
                // down. The special function put its result in RAX, so
                // return to the caller rather than falling through to
                // dead code after the jmp.
                Continuation::Return
            } else {
                Continuation::Jump(env.functions[func_idx].clone())
            }
        }
        Instr::idiv(divisor_arg) => {
            let dividend = env.read_arg(&Arg::new_reg(Register::rax));
            let divisor = env.read_arg(divisor_arg);

            let quotient = dividend / divisor;
            let remainder = dividend % divisor;

            env.write_arg(&Arg::new_reg(Register::rax), quotient);
            env.write_arg(&Arg::new_reg(Register::rdx), remainder);

            Continuation::Next
        }
        Instr::cqto => {
            let rax_val = env.read_arg(&Arg::new_reg(Register::rax));
            let rax_sign_extension = if rax_val >= 0 { 0 } else { !0i64 };
            env.write_arg(&Arg::new_reg(Register::rdx), rax_sign_extension);

            Continuation::Next
        }
    }
}

pub fn interpret_x86(m: &X86Program, inputs: &mut VecDeque<Value>, outputs: &mut VecDeque<Value>) {
    let funcs = m.functions.iter().map(|f| f.name.clone()).collect();
    let mut env = X86Env::new(funcs);

    let mut curr_func = m
        .functions
        .iter()
        .find(|f| f.name == global!(LABEL_MAIN))
        .unwrap();

    let mut curr_block_idx = curr_func
        .blocks
        .iter()
        .position(|b| b.label == curr_func.entry_block)
        .unwrap();
    let mut curr_instr_iter = curr_func.blocks[curr_block_idx].instrs.iter();
    let mut call_stack = VecDeque::new();
    loop {
        let curr_instr = curr_instr_iter.next().unwrap_or_else(|| {
            // Fallthrough for a block
            curr_block_idx += 1;
            curr_instr_iter = curr_func.blocks[curr_block_idx].instrs.iter();
            curr_instr_iter.next().unwrap()
        });

        match run_instr(curr_instr, inputs, outputs, &mut env) {
            Continuation::Next => {}
            Continuation::Jump(label) => {
                if let Some(new_block_idx) = curr_func.blocks.iter().position(|b| b.label == label)
                {
                    // Look for blocks within this function to jump to
                    curr_block_idx = new_block_idx;
                    curr_instr_iter = curr_func.blocks[curr_block_idx].instrs.iter();
                } else if let Some(new_func) = m.functions.iter().find(|f| f.name == label) {
                    // Otherwise, look for functions with this name to
                    // jump to
                    curr_func = new_func;
                    curr_block_idx = curr_func
                        .blocks
                        .iter()
                        .position(|b| b.label == curr_func.entry_block)
                        .unwrap();
                    curr_instr_iter = curr_func.blocks[curr_block_idx].instrs.iter();
                } else {
                    panic!("Couldn't jump to label: {label:?}")
                }
            }
            Continuation::Call(dest_idx) => {
                call_stack.push_back((curr_func, curr_block_idx, curr_instr_iter));
                if let Some(new_func) = m.functions.get(dest_idx) {
                    curr_func = new_func;
                    curr_block_idx = curr_func
                        .blocks
                        .iter()
                        .position(|b| b.label == curr_func.entry_block)
                        .unwrap();
                    curr_instr_iter = curr_func.blocks[curr_block_idx].instrs.iter();
                    println!("Calling to {:?}", curr_func.name);
                } else {
                    panic!("Couldn't find function with dest-idx: {dest_idx}");
                }
            }
            Continuation::Return => {
                if let Some((func, block_idx, iter)) = call_stack.pop_back() {
                    curr_func = func;
                    curr_block_idx = block_idx;
                    curr_instr_iter = iter;
                    println!("Returning to {:?}", curr_func.name);
                } else {
                    break;
                }
            }
            Continuation::Exit => {
                break;
            }
        }
    }
}

#[inline(never)]
fn __breakpoint_target() {
    println!("BREAKPOINT TARGET");
}
