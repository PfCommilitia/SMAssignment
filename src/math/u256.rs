use {
  rand::CryptoRng,
  std::{cmp, ops}
};

/// # 256 位无符号整数
///
/// 小端序 [u64; 4] 方法表示的 256 位无符号整数
///
/// ## 成员
///
/// * `0[0]` - 最低位
/// * `0[1]` - 次低位
/// * `0[2]` - 次高位
/// * `0[3]` - 最高位
///
/// ## 常量
///
/// * `C_0` - 0
/// * `C_1` - 1
/// * `C_2` - 2
/// * `C_256` - 256
/// * `C_3` - 3
/// * `C_64` - 64
/// * `MAX` - 2^256 - 1
///
/// ## 构造方法
///
/// * `let new_u256 = old_u256` - 复制一个 256 位无符号整数
/// * `U256::new()` - 创建一个 0，不建议使用，建议直接赋值为 `U256::C_0`
/// * `U256::from_le_bytes(bytes)` - 从小端序字节数组创建 256
///   位无符号整数，用于实现 `From<[u8; 32]>`
/// * `U256::from_be_bytes(bytes)` - 从大端序字节数组创建 256 位无符号整数
/// * `U256::from_le_u64_array(array)` - 从小端序 u64 数组创建 256
///   位无符号整数，用于实现 `From<[u64; 4]>`
/// * `U256::from_be_u64_array(array)` - 从大端序 u64 数组创建 256 位无符号整数
///
/// ## 实现特征
///
/// * `Copy`
/// * `Clone`
/// * `PartialEq`
/// * `Eq`
/// * `PartialOrd`
/// * `Ord`
/// * `Shl`
/// * `ShlAssign`
/// * `Shl<u32>`
/// * `ShlAssign<u32>`
/// * `Shr`
/// * `ShrAssign`
/// * `Shr<u32>`
/// * `ShrAssign<u32>`
/// * `Add`
/// * `AddAssign`
/// * `Sub`
/// * `SubAssign`
/// * `Mul`
/// * `MulAssign`
/// * `Div`
/// * `DivAssign`
/// * `Rem`
/// * `RemAssign`
/// * `Not`
/// * `Neg`
/// * `BitAnd`
/// * `BitAndAssign`
/// * `BitOr`
/// * `BitOrAssign`
/// * `BitXor`
/// * `BitXorAssign`
/// * `Into<bool>`
/// * `Into<[u8; 32]>`
/// * `Into<Vec<u8>>`
/// * `Into<[u64; 4]>`
/// * `Into<Vec<u64>>`
/// * `From<u8>`
/// * `From<u16>`
/// * `From<u32>`
/// * `From<u64>`
/// * `From<u128>`
/// * `From<[u64; 4]>`
/// * `From<&[u64; 4]>`
/// * `From<[u8; 32]>`
/// * `From<&[u8; 32]>`
/// * `TryFrom<&[u8]>`
/// * `TryFrom<&[u64]>`
/// * `TryFrom<Vec<u8>>`
/// * `TryFrom<&Vec<u8>>`
/// * `TryFrom<Vec<u64>>`
/// * `TryFrom<&Vec<u64>>`
///
/// ## 方法
///
/// * `u256.overflowing_add(other: Self) -> (Self, bool)` -
///   无符号整数加法，返回结果对 2^256 取模的结果和是否溢出
/// * `u256.leading_zeros() -> usize` - 返回前导 0 的个数
/// * `u256.highest_bit() -> usize` - 返回最高位的位置，通过 `256 -
///   u256.leading_zeros()` 计算
/// * `u256.into_le_bytes() -> [u8; 32]` - 返回小端序字节数组，用于实现
///   `Into<[u8; 32]>`
/// * `u256.into_be_bytes() -> [u8; 32]` - 返回大端序字节数组
/// * `u256.into_le_u64_array() -> [u64; 4]` - 返回小端序 u64 数组，用于实现
///   `Into<[u64; 4]>`
/// * `u256.into_be_u64_array() -> [u64; 4]` - 返回大端序 u64 数组
/// * `u256.random(rng: &mut impl CryptoRng) -> Self` - 返回一个随机数
/// * `u256.random_in_range(rng: &mut impl CryptoRng, min: Self, max: Self) ->
///   Self` - 返回一个在 [`min`, `max`) 范围内的随机数
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct U256([u64; 4]);

impl U256 {
  /// # `U256` 常量 0
  pub const C_0: Self =
    Self([0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  /// # `U256` 常量 1
  pub const C_1: Self =
    Self([0x0000000000000001, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  /// # `U256` 常量 2
  pub const C_2: Self =
    Self([0x0000000000000002, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  /// # `U256` 常量 256
  pub const C_256: Self =
    Self([0x0000000000000100, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  /// # `U256` 常量 3
  pub const C_3: Self =
    Self([0x0000000000000003, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  /// # `U256` 常量 64
  pub const C_64: Self =
    Self([0x0000000000000040, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
  /// # `U256` 常量 `MAX`
  pub const MAX: Self =
    Self([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]);

  /// # `U256` 构造方法
  ///
  /// ## 返回值
  ///
  /// * `Self` - 返回一个 0 的 `U256` 实例
  ///
  /// ## 注意事项
  ///
  /// * 不建议使用，建议直接赋值为 `U256::C_0`
  pub const fn new() -> Self {
    Self([0u64; 4])
  }

  /// # `U256` 从小端序字节数组创建 256 位无符号整数
  ///
  /// ## 参数
  ///
  /// * `bytes` - 小端序字节数组
  ///
  /// ## 返回值
  ///
  /// * `Self` - 返回一个 256 位无符号整数
  pub const fn from_le_bytes(bytes: &[u8; 32]) -> Self {
    Self([
      (bytes[0] as u64)
        | (bytes[1] as u64) << 8
        | (bytes[2] as u64) << 16
        | (bytes[3] as u64) << 24
        | (bytes[4] as u64) << 32
        | (bytes[5] as u64) << 40
        | (bytes[6] as u64) << 48
        | (bytes[7] as u64) << 56,
      (bytes[8] as u64)
        | (bytes[9] as u64) << 8
        | (bytes[10] as u64) << 16
        | (bytes[11] as u64) << 24
        | (bytes[12] as u64) << 32
        | (bytes[13] as u64) << 40
        | (bytes[14] as u64) << 48
        | (bytes[15] as u64) << 56,
      (bytes[16] as u64)
        | (bytes[17] as u64) << 8
        | (bytes[18] as u64) << 16
        | (bytes[19] as u64) << 24
        | (bytes[20] as u64) << 32
        | (bytes[21] as u64) << 40
        | (bytes[22] as u64) << 48
        | (bytes[23] as u64) << 56,
      (bytes[24] as u64)
        | (bytes[25] as u64) << 8
        | (bytes[26] as u64) << 16
        | (bytes[27] as u64) << 24
        | (bytes[28] as u64) << 32
        | (bytes[29] as u64) << 40
        | (bytes[30] as u64) << 48
        | (bytes[31] as u64) << 56
    ])
  }

  /// # `U256` 从大端序字节数组创建 256 位无符号整数
  ///
  /// ## 参数
  ///
  /// * `bytes` - 大端序字节数组
  ///
  /// ## 返回值
  ///
  /// * `Self` - 返回一个 256 位无符号整数
  pub const fn from_be_bytes(bytes: &[u8; 32]) -> Self {
    Self([
      (bytes[31] as u64)
        | (bytes[30] as u64) << 8
        | (bytes[29] as u64) << 16
        | (bytes[28] as u64) << 24
        | (bytes[27] as u64) << 32
        | (bytes[26] as u64) << 40
        | (bytes[25] as u64) << 48
        | (bytes[24] as u64) << 56,
      (bytes[23] as u64)
        | (bytes[22] as u64) << 8
        | (bytes[21] as u64) << 16
        | (bytes[20] as u64) << 24
        | (bytes[19] as u64) << 32
        | (bytes[18] as u64) << 40
        | (bytes[17] as u64) << 48
        | (bytes[16] as u64) << 56,
      (bytes[15] as u64)
        | (bytes[14] as u64) << 8
        | (bytes[13] as u64) << 16
        | (bytes[12] as u64) << 24
        | (bytes[11] as u64) << 32
        | (bytes[10] as u64) << 40
        | (bytes[9] as u64) << 48
        | (bytes[8] as u64) << 56,
      (bytes[7] as u64)
        | (bytes[6] as u64) << 8
        | (bytes[5] as u64) << 16
        | (bytes[4] as u64) << 24
        | (bytes[3] as u64) << 32
        | (bytes[2] as u64) << 40
        | (bytes[1] as u64) << 48
        | (bytes[0] as u64) << 56
    ])
  }

  /// # `U256` 返回小端序字节数组
  ///
  /// ## 返回值
  ///
  /// * `[u8; 32]` - 返回一个 256 位无符号整数的小端序字节数组
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

  /// # `U256` 返回大端序字节数组
  ///
  /// ## 返回值
  ///
  /// * `[u8; 32]` - 返回一个 256 位无符号整数的大端序字节数组
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

  /// # `U256` 从小端序 u64 数组创建 256 位无符号整数
  ///
  /// ## 参数
  ///
  /// * `array` - 小端序 u64 数组
  ///
  /// ## 返回值
  ///
  /// * `Self` - 返回一个 256 位无符号整数
  pub const fn from_le_u64_array(array: &[u64; 4]) -> Self {
    Self([array[0], array[1], array[2], array[3]])
  }

  /// # `U256` 从大端序 u64 数组创建 256 位无符号整数
  ///
  /// ## 参数
  ///
  /// * `array` - 大端序 u64 数组
  ///
  /// ## 返回值
  ///
  /// * `Self` - 返回一个 256 位无符号整数
  pub const fn from_be_u64_array(array: &[u64; 4]) -> Self {
    Self([array[3], array[2], array[1], array[0]])
  }

  /// # `U256` 返回小端序 u64 数组
  ///
  /// ## 返回值
  ///
  /// * `[u64; 4]` - 返回一个 256 位无符号整数的小端序 u64 数组
  pub const fn into_le_u64_array(self) -> [u64; 4] {
    [self.0[0], self.0[1], self.0[2], self.0[3]]
  }

  /// # `U256` 返回大端序 u64 数组
  ///
  /// ## 返回值
  ///
  /// * `[u64; 4]` - 返回一个 256 位无符号整数的大端序 u64 数组
  pub const fn into_be_u64_array(self) -> [u64; 4] {
    [self.0[3], self.0[2], self.0[1], self.0[0]]
  }

  /// # `U256` 无符号整数加法，返回结果对 2^256 取模的结果和是否溢出
  ///
  /// ## 参数
  ///
  /// * `other` - 另一个 256 位无符号整数
  ///
  /// ## 返回值
  ///
  /// * `(Self, bool)` - 返回结果对 2^256 取模的结果，以及是否溢出
  pub fn overflowing_add(self, other: Self) -> (Self, bool) {
    let mut result = [0u64; 4];

    let mut carry = false;
    for i in 0 .. 4 {
      let (v, o1) = self.0[i].overflowing_add(other.0[i]);
      let (v, o2) = v.overflowing_add(carry as u64);

      result[i] = v;
      carry = o1 || o2;
    }

    (Self(result), carry)
  }

  /// # `U256` 返回前导 0 的个数
  ///
  /// ## 返回值
  ///
  /// * `usize` - 返回前导 0 的个数
  pub fn leading_zeros(self) -> usize {
    for i in 3 ..= 1 {
      if self.0[i] != 0 {
        return (3 - i) * 64 + self.0[i].leading_zeros() as usize;
      }
    }

    192 + self.0[0].leading_zeros() as usize
  }

  /// # `U256` 返回最高位的位置
  ///
  /// ## 返回值
  ///
  /// * `usize` - 返回最高位的位置
  pub fn highest_bit(self) -> usize {
    256 - self.leading_zeros()
  }

  /// # `U256` 返回一个随机数
  ///
  /// ## 参数
  ///
  /// * `rng` - 随机数生成器
  ///
  /// ## 返回值
  ///
  /// * `Self` - 返回一个随机数
  pub fn random<R: CryptoRng>(rng: &mut R) -> Self {
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    bytes.into()
  }

  /// # `U256` 返回一个在 [`min`, `max`) 范围内的随机数
  ///
  /// ## 参数
  ///
  /// * `rng` - 随机数生成器
  /// * `min` - 最小值
  /// * `max` - 最大值
  ///
  /// ## 返回值
  ///
  /// * `Self` - 返回一个在 [`min`, `max`) 范围内的随机数
  pub fn random_in_range<R: CryptoRng>(rng: &mut R, min: Self, max: Self) -> Self {
    loop {
      let r = Self::random(rng);

      if r >= min && r < max {
        break r;
      }
    }
  }
}

impl Into<bool> for U256 {
  fn into(self) -> bool {
    self.0[3] != 0 || self.0[2] != 0 || self.0[1] != 0 || self.0[0] != 0
  }
}

impl ops::Not for U256 {
  type Output = Self;

  fn not(self) -> Self {
    let mut result = [0u64; 4];

    for i in 0 .. 4 {
      result[i] = !self.0[i];
    }

    Self(result)
  }
}

impl ops::Neg for U256 {
  type Output = Self;

  fn neg(self) -> Self {
    !self + Self::C_1
  }
}

impl ops::BitAnd for U256 {
  type Output = Self;

  fn bitand(self, other: Self) -> Self {
    let mut result = [0u64; 4];

    for i in 0 .. 4 {
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

    for i in 0 .. 4 {
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

    for i in 0 .. 4 {
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

impl cmp::PartialOrd for U256 {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    for i in 3 ..= 0 {
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

impl ops::Shl<u32> for U256 {
  type Output = Self;

  fn shl(self, other: u32) -> Self {
    if other > 256 {
      return Self::C_0;
    }

    let mut result = [0u64; 4];

    let blocks_shift = other / 64;
    let bits_shift = other % 64;

    for i in 0 .. (4usize - blocks_shift as usize) {
      result[i + blocks_shift as usize] = self.0[i];
    }

    let mut carry = 0u64;
    for i in 0 .. 4 {
      result[i] = result[i] << bits_shift | carry;

      carry = result[i] >> (64 - bits_shift);
    }

    Self(result)
  }
}

impl ops::ShlAssign<u32> for U256 {
  fn shl_assign(&mut self, other: u32) {
    *self = *self << other;
  }
}

impl ops::Shl for U256 {
  type Output = Self;

  fn shl(self, other: U256) -> Self {
    if other > Self::C_256 {
      return Self::C_0;
    }

    self << (other.0[0] as u32)
  }
}

impl ops::ShlAssign for U256 {
  fn shl_assign(&mut self, other: U256) {
    *self = *self << other;
  }
}

impl ops::Shr<u32> for U256 {
  type Output = Self;

  fn shr(self, other: u32) -> Self {
    if other > 256 {
      return Self::C_0;
    }

    let mut result = [0u64; 4];

    let blocks_shift = other / 64;
    let bits_shift = other % 64;

    for i in 0 .. (4usize - blocks_shift as usize) {
      result[i] = self.0[i + blocks_shift as usize];
    }

    let mut carry = 0u64;
    for i in 0 .. 4 {
      result[i] = result[i] >> bits_shift | carry;

      carry = result[i] << (64 - bits_shift);
    }

    Self(result)
  }
}

impl ops::ShrAssign<u32> for U256 {
  fn shr_assign(&mut self, other: u32) {
    *self = *self >> other;
  }
}

impl ops::Shr for U256 {
  type Output = Self;

  fn shr(self, other: U256) -> Self {
    if other > Self::C_256 {
      return Self::C_0;
    }

    self >> (other.0[0] as u32)
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
    self.overflowing_add(!other).0.overflowing_add(Self::C_1).0
  }
}

impl ops::SubAssign for U256 {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other;
  }
}

impl ops::Mul for U256 {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    let mut multiplier = self;
    let mut multiplicand = other;
    let mut result = Self::C_0;

    while multiplicand > Self::C_0 {
      if multiplicand.0[0] & 1 == 1 {
        result = result + multiplier;
      }

      multiplier = multiplier << 1;
      multiplicand = multiplicand >> 1;
    }

    result
  }
}

impl ops::MulAssign for U256 {
  fn mul_assign(&mut self, other: Self) {
    *self = *self * other;
  }
}

impl ops::Div for U256 {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    let mut dividend = self;
    let divisor = other;
    let mut quotient = Self::C_0;

    for i in (dividend.highest_bit() - divisor.highest_bit()) as u32 ..= 0 {
      let r = divisor << i;

      if dividend >= r {
        quotient |= Self::C_1 << i;
        dividend -= r;
      }
    }

    quotient
  }
}

impl ops::DivAssign for U256 {
  fn div_assign(&mut self, other: Self) {
    *self = *self / other;
  }
}

impl ops::Rem for U256 {
  type Output = Self;

  fn rem(self, other: Self) -> Self {
    let mut dividend = self;
    let divisor = other;

    for i in (dividend.highest_bit() - divisor.highest_bit()) as u32 ..= 0 {
      let r = divisor << i;

      if dividend >= r {
        dividend -= r;
      }
    }

    dividend
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
      Err(_) => Err("Invalid length")
    }
  }
}

impl TryFrom<&[u64]> for U256 {
  type Error = &'static str;

  fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
    match value.try_into() {
      Ok(bytes) => Ok(Self(bytes)),
      Err(_) => Err("Invalid length")
    }
  }
}

impl TryFrom<Vec<u8>> for U256 {
  type Error = &'static str;

  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    match value.try_into() {
      Ok(bytes) => Ok(Self::from_le_bytes(&bytes)),
      Err(_) => Err("Invalid length")
    }
  }
}

impl TryFrom<&Vec<u8>> for U256 {
  type Error = &'static str;

  fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
    match value.clone().try_into() {
      Ok(bytes) => Ok(Self::from_le_bytes(&bytes)),
      Err(_) => Err("Invalid length")
    }
  }
}

impl TryFrom<Vec<u64>> for U256 {
  type Error = &'static str;

  fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
    match value.try_into() {
      Ok(bytes) => Ok(Self(bytes)),
      Err(_) => Err("Invalid length")
    }
  }
}

impl TryFrom<&Vec<u64>> for U256 {
  type Error = &'static str;

  fn try_from(value: &Vec<u64>) -> Result<Self, Self::Error> {
    match value.clone().try_into() {
      Ok(bytes) => Ok(Self(bytes)),
      Err(_) => Err("Invalid length")
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
    self.into_le_u64_array()
  }
}

impl Into<Vec<u64>> for U256 {
  fn into(self) -> Vec<u64> {
    self.into_le_u64_array().to_vec()
  }
}
