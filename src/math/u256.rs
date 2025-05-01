use std::ops;
use std::cmp;

#[derive(Copy, Clone)]
pub struct U256([u64; 4]);

impl U256 {
  pub const MAX: Self = Self([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]);
  pub const C1: Self = Self([0x0000000000000001, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  pub const C0: Self = Self([0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  pub const C64: Self = Self([0x0000000000000040, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  pub const C256: Self = Self([0x0000000000000100, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);

  pub const fn new() -> Self {
    Self([0u64; 4])
  }

  pub const fn from_le_bytes(bytes: &[u8; 32]) -> Self {
    Self([
      (bytes[0] as u64) |
      (bytes[1] as u64) << 8 |
      (bytes[2] as u64) << 16 |
      (bytes[3] as u64) << 24 |
      (bytes[4] as u64) << 32 |
      (bytes[5] as u64) << 40 |
      (bytes[6] as u64) << 48 |
      (bytes[7] as u64) << 56,
      (bytes[8] as u64) |
      (bytes[9] as u64) << 8 |
      (bytes[10] as u64) << 16 |
      (bytes[11] as u64) << 24 |
      (bytes[12] as u64) << 32 |
      (bytes[13] as u64) << 40 |
      (bytes[14] as u64) << 48 |
      (bytes[15] as u64) << 56,
      (bytes[16] as u64) |
      (bytes[17] as u64) << 8 |
      (bytes[18] as u64) << 16 |
      (bytes[19] as u64) << 24 |
      (bytes[20] as u64) << 32 |
      (bytes[21] as u64) << 40 |
      (bytes[22] as u64) << 48 |
      (bytes[23] as u64) << 56,
      (bytes[24] as u64) |
      (bytes[25] as u64) << 8 |
      (bytes[26] as u64) << 16 |
      (bytes[27] as u64) << 24 |
      (bytes[28] as u64) << 32 |
      (bytes[29] as u64) << 40 |
      (bytes[30] as u64) << 48 |
      (bytes[31] as u64) << 56
    ])
  }

  pub const fn from_be_bytes(bytes: &[u8; 32]) -> Self {
    Self([
      (bytes[31] as u64) |
      (bytes[30] as u64) << 8 |
      (bytes[29] as u64) << 16 |
      (bytes[28] as u64) << 24 |
      (bytes[27] as u64) << 32 |
      (bytes[26] as u64) << 40 |
      (bytes[25] as u64) << 48 |
      (bytes[24] as u64) << 56,
      (bytes[23] as u64) |
      (bytes[22] as u64) << 8 |
      (bytes[21] as u64) << 16 |
      (bytes[20] as u64) << 24 |
      (bytes[19] as u64) << 32 |
      (bytes[18] as u64) << 40 |
      (bytes[17] as u64) << 48 |
      (bytes[16] as u64) << 56,
      (bytes[15] as u64) |
      (bytes[14] as u64) << 8 |
      (bytes[13] as u64) << 16 |
      (bytes[12] as u64) << 24 |
      (bytes[11] as u64) << 32 |
      (bytes[10] as u64) << 40 |
      (bytes[9] as u64) << 48 |
      (bytes[8] as u64) << 56,
      (bytes[7] as u64) |
      (bytes[6] as u64) << 8 |
      (bytes[5] as u64) << 16 |
      (bytes[4] as u64) << 24 |
      (bytes[3] as u64) << 32 |
      (bytes[2] as u64) << 40 |
      (bytes[1] as u64) << 48 |
      (bytes[0] as u64) << 56
    ])
  }

  pub const fn into_le_bytes(self) -> [u8; 32] {
    let mut bytes = [0u8; 32];

    bytes[0] = self.0[0] as u8;
    bytes[1] = (self.0[0] >> 8) as u8;
    bytes[2] = (self.0[0] >> 16) as u8;
    bytes[3] = (self.0[0] >> 24) as u8;
    bytes[4] = (self.0[0] >> 32) as u8;
    bytes[5] = (self.0[0] >> 40) as u8;
    bytes[6] = (self.0[0] >> 48) as u8;
    bytes[7] = (self.0[0] >> 56) as u8;
    bytes[8] = self.0[1] as u8;
    bytes[9] = (self.0[1] >> 8) as u8;
    bytes[10] = (self.0[1] >> 16) as u8;
    bytes[11] = (self.0[1] >> 24) as u8;
    bytes[12] = (self.0[1] >> 32) as u8;
    bytes[13] = (self.0[1] >> 40) as u8;
    bytes[14] = (self.0[1] >> 48) as u8;
    bytes[15] = (self.0[1] >> 56) as u8;
    bytes[16] = self.0[2] as u8;
    bytes[17] = (self.0[2] >> 8) as u8;
    bytes[18] = (self.0[2] >> 16) as u8;
    bytes[19] = (self.0[2] >> 24) as u8;
    bytes[20] = (self.0[2] >> 32) as u8;
    bytes[21] = (self.0[2] >> 40) as u8;
    bytes[22] = (self.0[2] >> 48) as u8;
    bytes[23] = (self.0[2] >> 56) as u8;
    bytes[24] = self.0[3] as u8;
    bytes[25] = (self.0[3] >> 8) as u8;
    bytes[26] = (self.0[3] >> 16) as u8;
    bytes[27] = (self.0[3] >> 24) as u8;
    bytes[28] = (self.0[3] >> 32) as u8;
    bytes[29] = (self.0[3] >> 40) as u8;
    bytes[30] = (self.0[3] >> 48) as u8;
    bytes[31] = (self.0[3] >> 56) as u8;

    bytes
  }

  pub const fn into_be_bytes(self) -> [u8; 32] {
    let mut bytes = [0u8; 32];

    bytes[0] = (self.0[3] >> 56) as u8;
    bytes[1] = (self.0[3] >> 48) as u8;
    bytes[2] = (self.0[3] >> 40) as u8;
    bytes[3] = (self.0[3] >> 32) as u8;
    bytes[4] = (self.0[3] >> 24) as u8;
    bytes[5] = (self.0[3] >> 16) as u8;
    bytes[6] = (self.0[3] >> 8) as u8;
    bytes[7] = self.0[3] as u8;
    bytes[8] = (self.0[2] >> 56) as u8;
    bytes[9] = (self.0[2] >> 48) as u8;
    bytes[10] = (self.0[2] >> 40) as u8;
    bytes[11] = (self.0[2] >> 32) as u8;
    bytes[12] = (self.0[2] >> 24) as u8;
    bytes[13] = (self.0[2] >> 16) as u8;
    bytes[14] = (self.0[2] >> 8) as u8;
    bytes[15] = self.0[2] as u8;
    bytes[16] = (self.0[1] >> 56) as u8;
    bytes[17] = (self.0[1] >> 48) as u8;
    bytes[18] = (self.0[1] >> 40) as u8;
    bytes[19] = (self.0[1] >> 32) as u8;
    bytes[20] = (self.0[1] >> 24) as u8;
    bytes[21] = (self.0[1] >> 16) as u8;
    bytes[22] = (self.0[1] >> 8) as u8;
    bytes[23] = self.0[1] as u8;
    bytes[24] = (self.0[0] >> 56) as u8;
    bytes[25] = (self.0[0] >> 48) as u8;
    bytes[26] = (self.0[0] >> 40) as u8;
    bytes[27] = (self.0[0] >> 32) as u8;
    bytes[28] = (self.0[0] >> 24) as u8;
    bytes[29] = (self.0[0] >> 16) as u8;
    bytes[30] = (self.0[0] >> 8) as u8;
    bytes[31] = self.0[0] as u8;

    bytes
  }

  pub fn overflowing_add(self, other: Self) -> (Self, bool) {
    let mut result = [0u64; 4];

    let mut carry = false;
    for i in 0..4 {
      let (v, o1) = self.0[i].overflowing_add(other.0[i]);
      let (v, o2) = v.overflowing_add(carry as u64);

      result[i] = v;
      carry = o1 || o2;
    }

    (Self(result), carry)
  }

  pub fn highest_bit(self) -> usize {
    for i in 3..=1 {
      if self.0[i] != 0 {
        return (i + 1) * 64 - self.0[i].leading_zeros() as usize;
      }
    }

    64 - self.0[0].leading_zeros() as usize
  }
}

impl ops::Not for U256 {
  type Output = Self;

  fn not(self) -> Self {
    let mut result = [0u64; 4];

    for i in 0..4 {
      result[i] = !self.0[i];
    }

    Self(result)
  }
}

impl ops::BitAnd for U256 {
  type Output = Self;

  fn bitand(self, other: Self) -> Self {
    let mut result = [0u64; 4];

    for i in 0..4 {
      result[i] = self.0[i] & other.0[i];
    }

    Self(result)
  }
}

impl ops::BitAndAssign for U256 {
  fn bitand_assign(&mut self, other: Self) {
    *self = *self & other;
  }
}

impl ops::BitOr for U256 {
  type Output = Self;

  fn bitor(self, other: Self) -> Self {
    let mut result = [0u64; 4];

    for i in 0..4 {
      result[i] = self.0[i] | other.0[i];
    }

    Self(result)
  }
}

impl ops::BitOrAssign for U256 {
  fn bitor_assign(&mut self, other: Self) {
    *self = *self | other;
  }
}

impl ops::BitXor for U256 {
  type Output = Self;

  fn bitxor(self, other: Self) -> Self {
    let mut result = [0u64; 4];

    for i in 0..4 {
      result[i] = self.0[i] ^ other.0[i];
    }

    Self(result)
  }
}

impl ops::BitXorAssign for U256 {
  fn bitxor_assign(&mut self, other: Self) {
    *self = *self ^ other;
  }
}

impl cmp::Eq for U256 {}

impl cmp::PartialEq for U256 {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl cmp::PartialOrd for U256 {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    for i in 3..=0 {
      if self.0[i] != other.0[i] {
        return Some(self.0[i].cmp(&other.0[i]));
      }
    }

    Some(cmp::Ordering::Equal)
  }
}

impl cmp::Ord for U256 {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl ops::Shl for U256 {
  type Output = Self;

  fn shl(self, other: U256) -> Self {
    if other > Self::C256 {
      return Self::C0;
    }

    let mut result = [0u64; 4];
    let other = other.0[0];

    let blocks_shift = other / 64;
    let bits_shift = other % 64;

    for i in 0..(4usize - blocks_shift as usize) {
      result[i + blocks_shift as usize] = self.0[i];
    }

    let mut carry = 0u64;
    for i in 0..4 {
      result[i] = result[i] << bits_shift | carry;

      carry = result[i] >> (64 - bits_shift);
    }

    Self(result)
  }
}

impl ops::ShlAssign for U256 {
  fn shl_assign(&mut self, other: U256) {
    *self = *self << other;
  }
}

impl ops::Shr for U256 {
  type Output = Self;

  fn shr(self, other: U256) -> Self {
    if other > Self::C256 {
      return Self::C0;
    }

    let mut result = [0u64; 4];
    let other = other.0[0];

    let blocks_shift = other / 64;
    let bits_shift = other % 64;

    for i in 0..(4usize - blocks_shift as usize) {
      result[i] = self.0[i + blocks_shift as usize];
    }

    let mut carry = 0u64;
    for i in 0..4 {
      result[i] = result[i] >> bits_shift | carry;

      carry = result[i] << (64 - bits_shift);
    }

    Self(result)
  }
}

impl ops::ShrAssign for U256 {
  fn shr_assign(&mut self, other: U256) {
    *self = *self >> other;
  }
}

impl ops::Add for U256 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    self.overflowing_add(other).0
  }
}

impl ops::AddAssign for U256 {
  fn add_assign(&mut self, other: Self) {
    *self = *self + other;
  }
}

impl ops::Sub for U256 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    self.overflowing_add(!other).0.overflowing_add(Self::C1).0
  }
}

impl ops::SubAssign for U256 {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other;
  }
}

impl From<u8> for U256 {
  fn from(value: u8) -> Self {
    Self([value as u64, 0, 0, 0])
  }
}

impl From<u16> for U256 {
  fn from(value: u16) -> Self {
    Self([value as u64, 0, 0, 0])
  }
}

impl From<u32> for U256 {
  fn from(value: u32) -> Self {
    Self([value as u64, 0, 0, 0])
  }
}

impl From<u64> for U256 {
  fn from(value: u64) -> Self {
    Self([value, 0, 0, 0])
  }
}

impl From<u128> for U256 {
  fn from(value: u128) -> Self {
    Self([value as u64, (value >> 64) as u64, 0, 0])
  }
}

impl From<[u64; 4]> for U256 {
  fn from(value: [u64; 4]) -> Self {
    Self(value)
  }
}

impl From<&[u64; 4]> for U256 {
  fn from(value: &[u64; 4]) -> Self {
    Self(value.clone())
  }
}

impl From<[u8; 32]> for U256 {
  fn from(value: [u8; 32]) -> Self {
    Self::from_le_bytes(&value)
  }
}

impl From<&[u8; 32]> for U256 {
  fn from(value: &[u8; 32]) -> Self {
    Self::from_le_bytes(value)
  }
}

impl TryFrom<&[u8]> for U256 {
  type Error = &'static str;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    match value.try_into() {
      Ok(bytes) => Ok(Self::from_le_bytes(&bytes)),
      Err(_) => Err("Invalid length"),
    }
  }
}

impl TryFrom<&[u64]> for U256 {
  type Error = &'static str;

  fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
    match value.try_into() {
      Ok(bytes) => Ok(Self(bytes)),
      Err(_) => Err("Invalid length"),
    }
  }
}

impl TryFrom<Vec<u8>> for U256 {
  type Error = &'static str;

  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    match value.try_into() {
      Ok(bytes) => Ok(Self::from_le_bytes(&bytes)),
      Err(_) => Err("Invalid length"),
    }
  }
}

impl TryFrom<&Vec<u8>> for U256 {
  type Error = &'static str;

  fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
    match value.clone().try_into() {
      Ok(bytes) => Ok(Self::from_le_bytes(&bytes)),
      Err(_) => Err("Invalid length"),
    }
  }
}

impl TryFrom<Vec<u64>> for U256 {
  type Error = &'static str;

  fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
    match value.try_into() {
      Ok(bytes) => Ok(Self(bytes)),
      Err(_) => Err("Invalid length"),
    }
  }
}

impl TryFrom<&Vec<u64>> for U256 {
  type Error = &'static str;

  fn try_from(value: &Vec<u64>) -> Result<Self, Self::Error> {
    match value.clone().try_into() {
      Ok(bytes) => Ok(Self(bytes)),
      Err(_) => Err("Invalid length"),
    }
  }
}

impl Into<[u8; 32]> for U256 {
  fn into(self) -> [u8; 32] {
    self.into_le_bytes()
  }
}

impl Into<Vec<u8>> for U256 {
  fn into(self) -> Vec<u8> {
    self.into_le_bytes().to_vec()
  }
}

impl Into<[u64; 4]> for U256 {
  fn into(self) -> [u64; 4] {
    self.0
  }
}

impl Into<Vec<u64>> for U256 {
  fn into(self) -> Vec<u64> {
    self.0.to_vec()
  }
}
