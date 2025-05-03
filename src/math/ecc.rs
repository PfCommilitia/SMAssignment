use super::u256::U256;

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
/// 
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

    let num = other.y + (params.p - self.y);
    let denom = other.x + (params.p - self.x);

    let denom_inv = denom.mod_inv(params.p).unwrap();

    let lambda = num * denom_inv % params.p;

    let x3 = (lambda * lambda + (params.p - self.x) + (params.p - other.x)) % params.p;
    let y3 = (lambda * (self.x + (params.p - x3)) + (params.p - self.y)) % params.p;

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
