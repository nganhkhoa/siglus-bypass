use falcon::loader::Pe;
use falcon::loader::Loader;
use falcon::il::{Function, Instruction, Operation::Store};

use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

use std::error::Error;

fn read_wstring(pe: &Pe, address: u64) -> Option<String> {
    let mut wstr = vec![];
    let mut shift = 0;
    loop {
        let b1 = pe.memory().ok()?.get8(address + shift)?;
        let b2 = pe.memory().ok()?.get8(address + shift + 1)?;
        if b2 != 0 || (b1 == 0 && b2 == 0) {
            break;
        }
        shift += 2;
        wstr.push(b1);
    }
    String::from_utf8(wstr).ok()
}

fn get_constant_inst(instruction: &Instruction) -> Option<u64> {
    if let Store { src, .. } = instruction.operation() {
        let c = src.get_constant()?;
        c.value_u64()
    } else {
        None
    }
}

fn find_fix_function(pe: &Pe) -> Result<Function, Box<dyn Error>> {
    let program = pe.program_recursive()?;
    for function in program.functions() {
        for block in function.blocks() {
            for instruction in block.instructions() {
                let wstr = get_constant_inst(instruction)
                    .and_then(|c| read_wstring(&pe, c));
                if Some(String::from("This Game is Japan Only\n\n")) == wstr {
                    return Ok(function.clone());
                }
            }
        }
    }
    Err("Cannot find check function".into())
}

fn find_offset_to(pe: &Pe, function: &Function) -> Option<u64> {
    // hack because current falcon can't patch binary
    // manually find file offset to fix
    let goblin_pe = pe.pe();
    goblin_pe.sections
        .iter()
        .find_map(|section| {
            let file_size = section.size_of_raw_data as u64;
            let section_address = section.virtual_address as u64 + goblin_pe.image_base as u64;
            let section_address_end = section_address + file_size;
            if function.address() >= section_address && function.address() < section_address_end {
                let file_offset = section.pointer_to_raw_data as u64;
                Some(function.address() - section_address + file_offset)
            } else {
                None
            }
        })
}

fn patch_ret1_at(offset: u64) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).open("SiglusEngine.exe")?;
    file.seek(SeekFrom::Start(offset))?;
    file.write(&[
        0x0c, 0x01, // or al, 1
        0xc3        // ret
    ])?;
    Ok(())
}

fn process() -> Result<(), Box<dyn Error>> {
    let pe = Pe::from_file(Path::new("SiglusEngine.exe"))?;
    let function = find_fix_function(&pe)?;
    let offset = find_offset_to(&pe, &function)
        .ok_or("Cannot find file offset to function")?;
    patch_ret1_at(offset)
}

fn main() {
    let result = process();
    if let Err(err) = result {
        println!("Error occured: {:?}", err);
    } else {
        println!("Patch is complete");
    }
}
