use crate::*;


#[derive(Debug)]
pub struct BitVec
{
	pub bits: Vec<bool>,
}


impl BitVec
{
	pub fn new() -> BitVec
	{
		BitVec
		{
			bits: Vec::new(),
		}
	}
	
	
	pub fn write(&mut self, index: usize, bit: bool)
	{
		while self.bits.len() <= index
			{ self.bits.push(false); }
			
		self.bits[index] = bit;
	}
	
	
	pub fn write_bigint(&mut self, index: usize, bigint: util::BigInt)
	{
        for i in 0..bigint.size.unwrap()
        {
            self.write(index + i, bigint.get_bit(bigint.size.unwrap() - 1 - i));
        }
    }
	
	
	pub fn write_bitvec(&mut self, index: usize, bitvec: &util::BitVec)
	{
        for i in 0..bitvec.len()
        {
            self.write(index + i, bitvec.read(i));
        }
    }
	
	
	pub fn read(&self, bit_index: usize) -> bool
	{
		if bit_index >= self.bits.len()
			{ false }
		else
			{ self.bits[bit_index] }
    }
    

    pub fn as_bigint(&self) -> util::BigInt
    {
        let mut bigint: util::BigInt = 0.into();

        for i in 0..self.bits.len()
        {
            bigint = bigint.set_bit(i, self.bits[i]);
        }

        bigint
    }
	
	
	pub fn truncate(&mut self, new_len: usize)
	{
		while self.bits.len() > new_len
			{ self.bits.pop(); }
	}
	
	
	pub fn len(&self) -> usize
	{
		self.bits.len()
	}
}


impl std::fmt::LowerHex for BitVec
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        use std::fmt::Write;

        let mut i = 0;
        while i < self.bits.len()
        {
            let mut digit = 0;
            for _ in 0..4
            {
                digit <<= 1;
                digit |= if self.read(i) { 1 } else { 0 };
                i += 1;
            }
            
            let c = if digit < 10
                { ('0' as u8 + digit) as char }
            else
                { ('a' as u8 + digit - 10) as char };
            
            f.write_char(c)?;
        }

        Ok(())
    }
}