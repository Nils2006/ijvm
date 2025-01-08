use std::collections::LinkedList;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let mut file = File::open(&args[1])?;

    // Create a buffer to store the bytes
    let mut buffer = Vec::new();

    // Read the file content into the buffer
    file.read_to_end(&mut buffer)?;


    // Get the first 8 bytes from the buffer
    let constant_pool_length = buffer[0];
    buffer.remove(0);
    let constant_pool = buffer[0..constant_pool_length as usize].to_vec();

    for _ in 0..constant_pool_length {
        buffer.remove(0);
    }

    let mut stack: LinkedList<i8> = LinkedList::new();
    let mut pc = 0;
    let mut lv:[i8; 1000] = [0;1000];

    while pc != buffer.len() {
        match buffer[pc] {
            0b00010000 => {
                //BIPUSH
                stack.push_front(buffer[pc + 1] as i8);
                pc += 2;
            },
            0b01011001 => {
                //DUP
                let top = stack.front().unwrap().clone();
                stack.push_front(top);
                pc += 1;
            },
            0b10100111 => {
                //GOTO
                let offset: i16 = (buffer[pc + 1] as i16) << 8 | buffer[pc + 2] as i16;
                pc += offset as usize;
            },
            0b01100000 => {
                //IADD
                if let (Some(a), Some(b)) = (stack.pop_front(), stack.pop_front()) {
                    stack.push_front(a + b);
                } else {
                    println!("ERROR: Not enough elements in the stack for IADD");
                }
                pc += 1;
            },
            0b01111110 => {
                //IAND
                if let (Some(a), Some(b)) = (stack.pop_front(), stack.pop_front()) {
                    stack.push_front(a & b);
                } else {
                    println!("ERROR: Not enough elements in the stack for IAND");
                }
                pc += 1;
            },
            0b10011001 => {
                //IFEQ
                if let Some(a) = stack.pop_front() {
                    if a == 0 {
                        let offset: i16 = (buffer[pc + 1] as i16) << 8 | buffer[pc + 2] as i16;
                        pc += offset as usize;
                    } else {
                        pc += 3;
                    }
                } else {
                    println!("ERROR: Not enough elements in the stack for IFEQ");
                }
            },
            0b10011011 => {
                //IFLT
                if let Some(a) = stack.pop_front() {
                    if a < 0 {
                        let offset: i16 = (buffer[pc + 1] as i16) << 8 | buffer[pc + 2] as i16;
                        pc += offset as usize;
                    }else {
                        pc += 3;
                    }
                } else {
                    println!("ERROR: Not enough elements in the stack for IFLT");
                }
            },
            0b10011111 =>{
                //IF_ICMPEQ
                if let (Some(a), Some(b)) = (stack.pop_front(), stack.pop_front()) {
                    if a == b {
                        let offset: i16 = (buffer[pc + 1] as i16) << 8 | buffer[pc + 2] as i16;
                        pc += offset as usize;
                    } else {
                        pc += 3;
                    }
                } else {
                    println!("ERROR: Not enough elements in the stack for IF_ICMPEQ");
                }
            },
            0b10000100 => {
                //IINC
                lv[pc+1] += buffer[pc + 2] as i8;
                pc += 3;
            },
            0b00010101 => {
                //ILOAD
                stack.push_front(lv[buffer[pc+1] as usize]);
                pc += 2;
            },
            0b10110110 => {
                //INVOKEVIRTUAL
                //TODO: implement
            },
            0b10000000 => {
                //IOR
                if let (Some(a), Some(b)) = (stack.pop_front(), stack.pop_front()) {
                    stack.push_front(a | b);
                } else {
                    println!("ERROR: Not enough elements in the stack for IOR");
                }
                pc += 1;
            },
            0b10101100 => {
                //IRETURN
                //TODO: implement
            },
            0b00110110 => {
                //ISTORE
                lv[buffer[pc+1] as usize] = *stack.front().unwrap();
                stack.pop_front();
                pc += 2;
            },
            0b01100100 => {
                //ISUB
                if let (Some(a), Some(b)) = (stack.pop_front(), stack.pop_front()) {
                    stack.push_front(a - b);
                } else {
                    println!("ERROR: Not enough elements in the stack for ISUB");
                }
                pc += 1;
            },
            0b00010011 => {
                //LDC_W
                let index: u16 = (buffer[pc + 1] as u16) << 8 | buffer[pc + 2] as u16;
                stack.push_front(constant_pool[index as usize] as i8);
                pc += 3;
            },
            0b00000000 => {
                //NOP
                pc += 1;
            },
            0b01010111 => {
                //POP
                stack.pop_front();
                pc += 1;
            },
            0b01011111 => {
                //SWAP
                if let (Some(a), Some(b)) = (stack.pop_front(), stack.pop_front()) {
                    stack.push_front(a);
                    stack.push_front(b);
                } else {
                    println!("ERROR: Not enough elements in the stack for SWAP");
                }
                pc += 1;
            }
            _ => {}
        }
    }

    println!("Stack: {:?}", stack);

    Ok(())
}
