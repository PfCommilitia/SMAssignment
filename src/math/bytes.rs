/// # 比特序列
///
/// 字节存储的比特序列，如果最后一个字节未填满，使用 `last_byte_len` 记录长度。
///
/// 全局使用大端序，可转换为用 0 填充末尾字节的小端序字节序列
///
/// ## 成员
///
/// * `bytes` - 字节序列
/// * `last_byte_len` - 最后一个字节的长度，如果这个字节未填满
///
/// ## 构造方法
///
/// * `new(bytes: Vec<u8>, last_byte_len: u8) -> Self` -
///   从字节序列和最后一个字节的长度构造
/// * `new_empty() -> Self` - 创建一个空序列
/// * `with_bytes(bytes: &[u8]) -> Self` - 从字节序列构造
/// * `try_with_bits(bytes: &[u8], size: u64) -> Result<Self, String>` -
///   从字节序列和长度构造
///
/// ## 实现特征
///
/// * `Clone`
/// * `From<&[u8]>`
/// * `From<BitSequence> -> Vec<u8>`
/// * `PartialEq`
/// * `Eq`
///
/// ## 方法
///
/// * `get_bytes(&self) -> &[u8]` - 获取字节序列
/// * `get_bytes_mut(&mut self) -> &mut [u8]` - 获取字节序列的可变引用
/// * `get_last_byte_len(&self) -> u8` - 获取最后一个字节的长度
/// * `append_bytes(&mut self, bytes: &[u8])` - 追加字节序列
/// * `append_bits(&mut self, bits: &Self)` - 追加比特序列
/// * `into_le_bytes(&self) -> Vec<u8>` - 转换为小端序字节序列
/// * `len(&self) -> u64` - 获取比特序列的长度
/// * `xor(&self, other: &Self) -> Result<Self, &'static str>` - 异或运算
/// * `xor_inplace(&mut self, other: &Self) -> Result<(), &'static str>` -
///   就地异或运算
/// * `slice(&self, start: u64, end: u64) -> Result<Self, &'static str>` - 切片
#[derive(Clone)]
pub struct BitSequence {
  bytes: Vec<u8>,
  last_byte_len: u8
}

impl BitSequence {
  pub fn new(bytes: Vec<u8>, last_byte_len: u8) -> Self {
    Self { bytes, last_byte_len }
  }

  pub fn new_empty() -> Self {
    Self { bytes: Vec::new(), last_byte_len: 0 }
  }

  pub fn with_bytes(bytes: &[u8]) -> Self {
    Self { bytes: bytes.to_vec(), last_byte_len: 0 }
  }

  pub fn try_with_bits(bytes: &[u8], size: u64) -> Result<Self, String> {
    let lower_bound: Result<u64, _> = ((bytes.len() - 1) * 8 + 1).try_into();
    let upper_bound: Result<u64, _> = (bytes.len() * 8).try_into();

    if lower_bound.is_err() || upper_bound.is_err() {
      return Err("Invalid input size".to_string().into());
    }

    let lower_bound = lower_bound.unwrap();
    let upper_bound = upper_bound.unwrap();

    if size < lower_bound || size > upper_bound {
      return Err("Invalid input size".to_string().into());
    }

    Ok(Self { bytes: bytes.to_vec(), last_byte_len: (size % 8) as u8 })
  }

  pub fn get_bytes(&self) -> &[u8] {
    &self.bytes
  }

  pub fn get_bytes_mut(&mut self) -> &mut [u8] {
    &mut self.bytes
  }

  pub fn get_last_byte_len(&self) -> u8 {
    self.last_byte_len
  }

  pub fn append_bytes(&mut self, bytes: &[u8]) {
    if self.last_byte_len == 0 {
      self.bytes.extend_from_slice(bytes);
    } else {
      let l_shift = 8 - self.last_byte_len;
      let r_shift = self.last_byte_len;

      for byte in bytes {
        *self.bytes.last_mut().unwrap() |= byte >> r_shift;
        self.bytes.push(byte << l_shift);
      }
    }
  }

  pub fn append_bits(&mut self, bits: &Self) {
    if bits.bytes.len() == 0 {
      return;
    }

    if bits.last_byte_len == 0 {
      self.append_bytes(&bits.bytes);
      return;
    }

    if bits.bytes.len() > 1 {
      self.append_bytes(&bits.bytes[.. bits.bytes.len() - 1]);
    }

    let last_byte = bits.bytes.last().unwrap();
    let tot = self.last_byte_len + bits.last_byte_len;

    if tot <= 8 {
      *self.bytes.last_mut().unwrap() |= last_byte >> self.last_byte_len;
      self.last_byte_len = tot % 8;
    } else {
      let l_shift = 8 - self.last_byte_len;

      *self.bytes.last_mut().unwrap() |= last_byte >> self.last_byte_len;
      self.bytes.push(last_byte << l_shift);
      self.last_byte_len = tot - 8;
    }
  }

  pub fn into_le_bytes(&self) -> Vec<u8> {
    if self.bytes.len() == 0 {
      return vec![];
    }

    let mut bytes = self.bytes.iter().rev().cloned().collect::<Vec<_>>();

    if self.last_byte_len > 0 {
      *bytes.last_mut().unwrap() >>= 8 - self.last_byte_len;
    }

    bytes
  }

  pub fn len(&self) -> u64 {
    self.bytes.len() as u64 * 8 - 8 + self.last_byte_len as u64
  }

  pub fn xor(&self, other: &Self) -> Result<Self, &'static str> {
    if self.len() != other.len() {
      return Err("Lengths of sequences to XOR must be equal");
    }

    let mut result = self.clone();

    for i in 0 .. self.bytes.len() {
      result.bytes[i] ^= other.bytes[i];
    }

    Ok(result)
  }

  pub fn xor_inplace(&mut self, other: &Self) -> Result<(), &'static str> {
    if self.len() != other.len() {
      return Err("Lengths of sequences to XOR must be equal");
    }

    for i in 0 .. self.bytes.len() {
      self.bytes[i] ^= other.bytes[i];
    }

    Ok(())
  }

  pub fn slice(&self, start: u64, end: u64) -> Result<Self, &'static str> {
    if start > end || end >= self.len() {
      return Err("Invalid slice");
    }

    let mut current_byte = start / 8;
    let byte_pos = start % 8;
    let mut bits_remaining = end - start;

    let mut bytes = Vec::new();

    while bits_remaining >= 8 {
      let mut byte = self.bytes[current_byte as usize] << byte_pos;
      byte |= self.bytes[current_byte as usize + 1] >> (8 - byte_pos);

      bytes.push(byte);
      current_byte += 1;
      bits_remaining -= 8;
    }

    if bits_remaining > 0 {
      let mut byte = if 8 - byte_pos <= bits_remaining {
        bits_remaining -= 8 - byte_pos;
        self.bytes[current_byte as usize] << byte_pos
      } else {
        let temp = 8 - bits_remaining;
        bits_remaining = 0;
        self.bytes[current_byte as usize] << temp
      };

      if bits_remaining > 0 {
        byte |= self.bytes[current_byte as usize + 1] >> (8 - bits_remaining);
      }

      bytes.push(byte);
    }

    Ok(BitSequence { bytes, last_byte_len: ((end - start) % 8) as u8 })
  }
}

impl From<&[u8]> for BitSequence {
  fn from(bytes: &[u8]) -> Self {
    Self::with_bytes(bytes)
  }
}

impl From<BitSequence> for Vec<u8> {
  fn from(sequence: BitSequence) -> Self {
    sequence.get_bytes().to_vec()
  }
}

impl PartialEq for BitSequence {
  fn eq(&self, other: &Self) -> bool {
    self.last_byte_len == other.last_byte_len
      && self.bytes.len() == other.bytes.len()
      && self.bytes == other.bytes
  }
}

impl Eq for BitSequence {
}
