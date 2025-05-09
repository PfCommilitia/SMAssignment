use {
  super::{bytes::BitSequence, u256::U256},
  std::{
    cmp,
    ops::{self}
  }
};

/// # 模逆运算
///
/// ## 方法
///
/// * `mod_inv(self, modulus) -> Option<Self>` - 计算模逆
///
/// ## 注意事项
///
/// 由于实际只需对 `U256` 实现，所以使用 `Sized` 约束
pub trait ModInv: Sized {
  fn mod_inv(self, modulus: Self) -> Option<Self>;
}

impl ModInv for U256 {
  fn mod_inv(self, modulus: Self) -> Option<Self> {
    let mut a = self;
    let mut modulus = modulus;
    let (mut x0, mut x1) = (U256::C_0, U256::C_1);

    if modulus == U256::C_1 {
      return Some(U256::C_0);
    }

    while a > U256::C_1 {
      if modulus == U256::C_0 {
        return None;
      }
      let q = a / modulus;
      let t = modulus;
      modulus = a % modulus;
      a = t;
      let t = x0;
      x0 = x1 - q * x0;
      x1 = t;
    }

    if x1 > modulus {
      x1 = x1 + modulus;
    }

    if a == U256::C_1 {
      Some((x1 + modulus) % modulus)
    } else {
      None
    }
  }
}

/// # 椭圆曲线参数结构体
///
/// ## 成员
///
/// * `a` - 椭圆曲线参数 a
/// * `b` - 椭圆曲线参数 b
/// * `p` - 椭圆曲线参数 p
/// * `n` - 椭圆曲线参数 n
/// * `g_x` - 椭圆曲线参数 Gx
/// * `g_y` - 椭圆曲线参数 Gy
/// ## 构造方法
///
/// * `EccParams { a, b, p, n, g_x, g_y }` - 初始化椭圆曲线参数结构体
///
/// ## 实现特征
///
/// * `PartialEq`
/// * `Eq`
///
/// ## 注意事项
///
/// 确保所有 `EccPoint` 的生命周期与 `EccParams` 一致，否则会出现生命周期问题
#[derive(PartialEq, Eq)]
pub struct EccParams {
  pub a: U256,
  pub b: U256,
  pub p: U256,
  pub n: U256,
  pub g_x: U256,
  pub g_y: U256
}

/// # 椭圆曲线点结构体
///
/// ## 成员
///
/// * `x` - 椭圆曲线点 x 坐标；无穷远点为 0
/// * `y` - 椭圆曲线点 y 坐标；无穷远点为 0
/// * `params` - 椭圆曲线参数结构体的引用
/// * `infinity` - 椭圆曲线点是否为无穷远点
///
/// ## 构造方法
///
/// * `let new_ecc_point = old_ecc_point` - 复制一个椭圆曲线点
/// * `EccPoint { x, y, params, infinity }` - 直接初始化，不推荐
/// * `EccPoint::new(x, y, params, infinity)` - 创建一个椭圆曲线点
/// * `EccPoint::infinity(params)` - 创建一个无穷远点
/// * `EccPoint::new_simple(x, y, params)` - 创建一个非无穷远椭圆曲线点
///
/// ## 实现特征
///
/// * `Clone`
/// * `Copy`
/// * `PartialEq`
/// * `Eq`
/// * `EccOps` - 椭圆曲线相关运算
/// * `From<EccPoint<'a>> -> Vec<u8>`
/// * `From<EccPoint<'a>> -> BitSequence`
///
/// ## 方法
///
/// * `from_bytes(bytes: &[u8; 65], params: &'a EccParams) -> Self` -
///   从字节序列构造椭圆曲线点
/// * `validate_on_curve(self) -> bool` - 验证椭圆曲线点是否在曲线上
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EccPoint<'a> {
  pub x: U256,
  pub y: U256,
  pub params: &'a EccParams,
  pub infinity: bool
}

impl<'a> EccPoint<'a> {
  pub fn new(x: U256, y: U256, params: &'a EccParams, infinity: bool) -> Self {
    if infinity {
      Self { x: U256::C_0, y: U256::C_0, params, infinity }
    } else {
      Self { x, y, params, infinity: false }
    }
  }

  pub fn infinity(params: &'a EccParams) -> Self {
    Self { x: U256::C_0, y: U256::C_0, params, infinity: true }
  }

  pub fn new_simple(x: U256, y: U256, params: &'a EccParams) -> Self {
    Self::new(x, y, params, false)
  }

  pub fn from_bytes(bytes: &[u8; 65], params: &'a EccParams) -> Self {
    let x = U256::from_le_bytes(&bytes[1 .. 33].try_into().unwrap());
    let y = U256::from_le_bytes(&bytes[33 .. 65].try_into().unwrap());

    Self::new_simple(x, y, params)
  }

  pub fn validate_on_curve(self) -> bool {
    // y^2 = x^3 + ax + b (mod p)
    self.infinity
      || self.y.mod_mul(self.y, self.params.p)
        == self
          .x
          .mod_mul(self.x, self.params.p)
          .mod_mul(self.x, self.params.p)
          .mod_add(self.x.mod_mul(self.params.a, self.params.p), self.params.p)
          .mod_add(self.params.b.modded(self.params.p), self.params.p)
  }

  pub fn validate_on_given_curve(self, params: &EccParams) -> bool {
    self.params == params && self.validate_on_curve()
  }
}

/// # 椭圆曲线相关运算
///
/// ## 方法
///
/// * `ecc_add(self, other: Self, params: &'a EccParams) -> Self` - 椭圆曲线加法
/// * `ecc_mul(self, k: U256, params: &'a EccParams) -> Self` - 椭圆曲线数乘
pub trait EccOps<'a> {
  /// # 椭圆曲线加法
  ///
  /// ## 参数
  ///
  /// * `self` - 椭圆曲线点
  /// * `other` - 椭圆曲线点
  /// * `params` - 椭圆曲线参数结构体的引用
  ///
  /// ## 返回值
  ///
  /// * `Self` - 椭圆曲线点
  ///
  /// ## 特殊情况
  ///
  /// * 当两个椭圆曲线点参数不兼容时，崩溃
  /// * 当 `self` 和 `other` 中有一个为无穷远点时，返回另一个点
  fn ecc_add(self, other: Self, params: &'a EccParams) -> Self;

  /// # 椭圆曲线数乘
  ///
  /// ## 参数
  ///
  /// * `self` - 椭圆曲线点
  /// * `k` - 数乘因子
  /// * `params` - 椭圆曲线参数结构体的引用
  ///
  /// ## 返回值
  ///
  /// * `Self` - 椭圆曲线点
  ///
  /// ## 特殊情况
  ///
  /// * 当 `k` 为 0 或 `self` 为无穷远点时，返回无穷远点
  fn ecc_mul(self, k: U256, params: &'a EccParams) -> Self;
}

impl<'a> EccOps<'a> for EccPoint<'a> {
  fn ecc_add(self, other: Self, params: &'a EccParams) -> Self {
    if self.params != other.params {
      panic!("Incompatible elliptic curve parameters");
    }

    if self == other {
      return self.ecc_mul(U256::C_2, params);
    }

    match (self.infinity, other.infinity) {
      (true, _) => return other,
      (_, true) => return self,
      (false, false) => {}
    }

    let num = other.y.mod_add(params.p - self.y, params.p);
    let denom = other.x.mod_add(params.p - self.x, params.p);

    let denom_inv = denom.mod_inv(params.p).unwrap();

    let lambda = num.mod_mul(denom_inv, params.p);

    let x3 = lambda
      .mod_mul(lambda, params.p)
      .mod_add(params.p - self.x, params.p)
      .mod_add(params.p - other.x, params.p);
    let y3 = lambda
      .mod_mul(self.x.mod_add(params.p - x3, params.p), params.p)
      .mod_add(params.p - self.y, params.p);

    EccPoint::new_simple(x3, y3, params)
  }

  fn ecc_mul(self, k: U256, params: &'a EccParams) -> Self {
    if k == U256::C_0 || self.infinity {
      return EccPoint::infinity(params);
    }

    let mut res: Option<Self> = None;
    let mut addend = self;
    let k_words = k.into_le_u64_array();

    for i in 0 .. 256 {
      if (k_words[i / 64] >> (i % 64)) & 1 == 1 {
        res = match res {
          None => Some(addend.clone()),
          Some(r) => Some(r.ecc_add(addend, params))
        };
      }
      addend = addend.ecc_add(addend, params);
    }

    match res {
      Some(r) => r,
      None => EccPoint::infinity(params)
    }
  }
}

/// # 带模/域内运算
///
/// 实现加法和乘法的域内运算，确保取模结果正确
///
/// ## 方法
///
/// * `mod_add(self, other: Self, modulus: Self) -> Self` - 模加
/// * `mod_mul(self, other: Self, modulus: Self) -> Self` - 模乘
/// * `mod_sub(self, other: Self, modulus: Self) -> Self` - 模减，占位
/// * `mod_div(self, other: Self, modulus: Self) -> Self` - 模除，占位
/// * `modded(self, modulus: Self) -> Self` - 取模
pub trait ModOps: Sized {
  /// # 模加
  ///
  /// 对两个数进行模加运算，确保返回正确的取模后结果，即便加法结果溢出
  ///
  /// ## 参数
  ///
  /// * `self` - 被加数
  /// * `other` - 加数
  /// * `modulus` - 模
  ///
  /// ## 返回值
  ///
  /// * `Self` - 结果
  fn mod_add(self, other: Self, modulus: Self) -> Self;

  /// # 模乘
  ///
  /// 对两个数进行模乘运算，确保返回正确的取模后结果，即便乘法结果溢出
  ///
  /// ## 参数
  ///
  /// * `self` - 被乘数
  /// * `other` - 乘数
  /// * `modulus` - 模
  ///
  /// ## 返回值
  ///
  /// * `Self` - 结果
  fn mod_mul(self, other: Self, modulus: Self) -> Self;

  /// # 模减
  ///
  /// 对两个数进行模减运算，确保返回正确的取模后结果；由于减法操作一定不会溢出，
  /// 所以等价于 `(self - other) % modulus`
  ///
  /// ## 参数
  ///
  /// * `self` - 被减数
  /// * `other` - 减数
  /// * `modulus` - 模
  ///
  /// ## 返回值
  ///
  /// * `Self` - 结果
  fn mod_sub(self, other: Self, modulus: Self) -> Self;

  /// # 模除
  ///
  /// 对两个数进行模除运算，确保返回正确的取模后结果；由于除法操作一定不会溢出，
  /// 所以等价于 `(self / other) % modulus`
  ///
  /// ## 参数
  ///
  /// * `self` - 被除数
  /// * `other` - 除数
  /// * `modulus` - 模
  ///
  /// ## 返回值
  ///
  /// * `Self` - 结果
  fn mod_div(self, other: Self, modulus: Self) -> Self;

  /// # 取模
  ///
  /// 对一个数进行取模运算，确保返回正确的取模后结果；等价于 `self % modulus`
  ///
  /// ## 参数
  ///
  /// * `self` - 被取模数
  /// * `modulus` - 模
  ///
  /// ## 返回值
  ///
  /// * `Self` - 结果
  fn modded(self, modulus: Self) -> Self;
}

/// # 512 位无符号整数辅助结构体，小端序
///
/// ## 成员
///
/// * `0[0]` - 第 1 位
/// * `0[1]` - 第 2 位
/// * `0[2]` - 第 3 位
/// * `0[3]` - 第 4 位
/// * `0[4]` - 第 5 位
/// * `0[5]` - 第 6 位
/// * `0[6]` - 第 7 位
/// * `0[7]` - 第 8 位
///
/// ## 常量
///
/// * `C_1` - 1
///
/// ## 构造方法
///
/// * `let new_u512_helper = old_u512_helper` - 复制一个 512
///   位无符号整数辅助结构体
/// * `U512Helper::new()` - 创建一个 0
///
/// ## 实现特征
///
/// * `Copy`
/// * `Clone`
/// * `PartialEq`
/// * `Eq`
/// * `PartialOrd`
/// * `Ord`
/// * `Not`
/// * `Neg`
/// * `Add`
/// * `Sub`
/// * `Shl<u32>`
/// * `Shr<u32>`
/// * `From<U256>`
/// * `From<U512Helper> -> U256`
/// * `ModOps` - 带模/域内运算
///
/// ## 方法
///
/// * `leading_zeros(self) -> usize` - 返回前导 0 的个数
/// * `highest_bit(self) -> usize` - 返回最高位 1 的位置
#[derive(Clone, Copy, PartialEq, Eq)]
struct U512Helper([u64; 8]);

impl U512Helper {
  pub const C_1: Self = Self([1, 0, 0, 0, 0, 0, 0, 0]);

  pub fn new() -> Self {
    Self([0; 8])
  }

  pub fn leading_zeros(self) -> usize {
    for i in 7 ..= 1 {
      if self.0[i] != 0 {
        return (7 - i) * 64 + self.0[i].leading_zeros() as usize;
      }
    }

    448 + self.0[0].leading_zeros() as usize
  }

  pub fn highest_bit(self) -> usize {
    512 - self.leading_zeros()
  }
}

impl From<U256> for U512Helper {
  fn from(value: U256) -> Self {
    let blocks = value.into_le_u64_array();

    U512Helper([blocks[0], blocks[1], blocks[2], blocks[3], 0, 0, 0, 0])
  }
}

impl From<U512Helper> for U256 {
  fn from(helper: U512Helper) -> Self {
    let blocks = helper.0;

    U256::from_le_u64_array(&[blocks[0], blocks[1], blocks[2], blocks[3]])
  }
}

impl cmp::PartialOrd for U512Helper {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    for i in 7 ..= 0 {
      if self.0[i] != other.0[i] {
        return Some(self.0[i].cmp(&other.0[i]));
      }
    }

    Some(cmp::Ordering::Equal)
  }
}

impl cmp::Ord for U512Helper {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl ops::Not for U512Helper {
  type Output = Self;

  fn not(self) -> Self {
    Self(self.0.map(|x| !x))
  }
}

impl ops::Neg for U512Helper {
  type Output = Self;

  fn neg(self) -> Self {
    !self + U512Helper::C_1
  }
}

impl ops::Add for U512Helper {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    let mut result = U512Helper::new();

    let mut carry = false;
    for i in 0 .. 8 {
      let (v, o1) = self.0[i].overflowing_add(other.0[i]);
      let (v, o2) = v.overflowing_add(carry as u64);

      result.0[i] = v;
      carry = o1 || o2;
    }

    result
  }
}

impl ops::Sub for U512Helper {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    self + (-other)
  }
}

impl ops::Shl<u32> for U512Helper {
  type Output = Self;

  fn shl(self, other: u32) -> Self {
    if other > 512 {
      return U512Helper::new();
    }

    let mut result = [0u64; 8];

    let blocks_shift = other / 64;
    let bits_shift = other % 64;

    for i in 0 .. (8usize - blocks_shift as usize) {
      result[i + blocks_shift as usize] = self.0[i];
    }

    let mut carry = 0u64;
    for i in 0 .. 8 {
      result[i] = result[i] << bits_shift | carry;

      carry = result[i] >> (64 - bits_shift);
    }

    Self(result)
  }
}

impl ops::Shr<u32> for U512Helper {
  type Output = Self;

  fn shr(self, other: u32) -> Self {
    if other > 512 {
      return U512Helper::new();
    }

    let mut result = [0u64; 8];

    let blocks_shift = other / 64;
    let bits_shift = other % 64;

    for i in 0 .. (8usize - blocks_shift as usize) {
      result[i] = self.0[i + blocks_shift as usize];
    }

    let mut carry = 0u64;
    for i in 0 .. 8 {
      result[i] = result[i] >> bits_shift | carry;

      carry = result[i] << (64 - bits_shift);
    }

    Self(result)
  }
}

impl ModOps for U256 {
  fn mod_add(self, other: Self, modulus: Self) -> Self {
    let (result, carry) = self.overflowing_add(other);

    if carry || result >= modulus {
      result - modulus
    } else {
      result
    }
  }

  fn mod_mul(self, other: Self, modulus: Self) -> Self {
    let mut multiplier: U512Helper = self.into();
    let mut multiplicand = other;
    let mut result = U512Helper::new();

    while multiplicand > U256::C_0 {
      if multiplicand.into_le_bytes()[0] & 1 == 1 {
        result = result + multiplier;
      }

      multiplier = multiplier << 1;
      multiplicand = multiplicand >> 1;
    }

    let mut dividend = result;
    let divisor = modulus;

    for i in (dividend.highest_bit() - divisor.highest_bit()) as u32 ..= 0 {
      let r = U512Helper::from(divisor) << i;

      if dividend >= r {
        dividend = dividend - r;
      }
    }

    dividend.into()
  }

  fn mod_sub(self, other: Self, modulus: Self) -> Self {
    (self - other).modded(modulus)
  }

  fn mod_div(self, other: Self, modulus: Self) -> Self {
    (self / other).modded(modulus)
  }

  fn modded(self, modulus: Self) -> Self {
    self % modulus
  }
}

impl<'a> From<EccPoint<'a>> for Vec<u8> {
  fn from(point: EccPoint<'a>) -> Self {
    let mut result = Vec::with_capacity(65);

    result.push(0x04);
    result.extend_from_slice(&point.x.into_le_bytes());
    result.extend_from_slice(&point.y.into_le_bytes());

    result
  }
}

impl<'a> From<EccPoint<'a>> for BitSequence {
  fn from(point: EccPoint<'a>) -> Self {
    Vec::<u8>::from(point)[..].into()
  }
}
