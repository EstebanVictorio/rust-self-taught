use std::{ io,env };

fn bin2dec(bits_string: &String) -> u8 {
  let mut accum = 0;
  let bits = bits_string.chars().rev().enumerate();
  for (i,c) in bits {
    let bin_pos = i as u32;
    let digit = match c.to_digit(10) {
      Some(1) => 1,
      _ => 0
    };
    accum+= if digit == 0 { 0 } else { 2u8.pow(bin_pos) } ;
  }
  return accum;
}

fn char_is_bit(ctoi: Option<u32>) -> bool {
  match ctoi {
    Some(0) | Some(1) => true,
    _ => false
  }
}

fn is_valid_binary(s: &String) -> bool {
  let bits: Vec<char> = s.chars().collect();
  let mut i = 0;
  let mut ctoi = bits[i].to_digit(10);
  let mut is_bit = char_is_bit(ctoi);

  while is_bit && i < bits.len() {
    i+=1;
    ctoi = bits[i].to_digit(10);
    is_bit = i < bits.len() - 1 && char_is_bit(ctoi);
    if !is_bit {
      return false;
    }
  }

  return true;
}

fn read_from_cmd(bin: &mut String) {
  io::stdin().read_line(bin).expect("Failed to read input");
}


fn main() {
  let _args: Vec<String> = env::args().collect();
  let mut bin: String = String::new();
  read_from_cmd(&mut bin);

  if is_valid_binary(&bin) {
    println!("Invalid binary: {:?}", bin);
  } else {
    let clean_bits = bin.replace('\n', "");
    let dec: u8 = bin2dec(&clean_bits);
    println!("Dec equivalent: {:?}",dec);
  }
  
}