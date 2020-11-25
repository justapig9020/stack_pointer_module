use std::cell::RefCell;
use std::rc::Rc;
use std::io;

struct SpRegs {
    psp: Rc<RefCell<u32>>,
    msp: Rc<RefCell<u32>>,
}

#[derive(Debug)]
enum Cmd {
    UseMsp,
    UsePsp,
    SpAdd(u32),
    Exit,
    Null,
}

fn main() {
    let sp_regs = SpRegs {
        psp: Rc::new(RefCell::new(0)),
        msp: Rc::new(RefCell::new(0)),
    };
    let mut sp = sp_regs.msp.clone();
    let mut exit = false;
    while !exit {
        let cmd = get_cmd();
        match cmd {
            Cmd::UseMsp => {
                sp = sp_regs.msp.clone();
            },
            Cmd::UsePsp => {
                sp = sp_regs.psp.clone();
            },
            Cmd::SpAdd(val) => {
                *sp.borrow_mut() += val;
            },
            Cmd::Exit => {
                exit = true;
            },
            Cmd::Null => {},
        }
        let mut indicator: [String; 2] = [String::from(""), String::from("")];
        if sp == sp_regs.msp {
            indicator[0].push_str(" <");
        } else {
            indicator[1].push_str(" <");
        }
        println!("MSP: {}, Rc cnt: {} {}",sp_regs.msp.borrow(), Rc::strong_count(&sp_regs.msp), indicator[0]);
        println!("PSP: {}, Rc cnt: {} {}",sp_regs.psp.borrow(), Rc::strong_count(&sp_regs.psp), indicator[1]);
        println!("====================");
        println!("");
    }
}

fn get_cmd() -> Cmd {
    let mut cmd = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut cmd).expect("Read error");
    let cmd = cmd.trim();
    match cmd {
        "msp" => return Cmd::UseMsp,
        "psp" => return Cmd::UsePsp,
        "exit" => return Cmd::Exit,
        "help" => {
            println!("Usage: ");
            println!("  msp: use msp as sp");
            println!("  psp: use psp as sp");
            println!("  <u32>: Add <u32> to sp");
            println!("  help: show help message");
            println!("  exit: exit function");
            return Cmd::Null;
        }
        _ => {
            return match cmd.parse::<u32>() {
                Ok(n) => Cmd::SpAdd(n),
                Err(e) => {
                    println!("Cmd err: {}", e);
                    Cmd::Null
                }
            }
        },
    }
}
