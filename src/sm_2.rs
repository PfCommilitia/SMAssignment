use crate::math::{
  ecc::{EccOps, EccParams, EccPoint, ModOps, ModInv},
  u256::U256
};
use crate::math::bytes::BitSequence;
use crate::sm_3::hash;

/// # SM2 p 参数
static SM2_P: U256 = U256::from_be_u64_array(&[
  0xfffffffeffffffff,
  0xffffffffffffffff,
  0xffffffff00000000,
  0xffffffffffffffff
]);

/// # SM2 a 参数
static SM2_A: U256 = U256::from_be_u64_array(&[
  0xfffffffeffffffff,
  0xffffffffffffffff,
  0xffffffff00000000,
  0xfffffffffffffffc
]);

/// # SM2 b 参数
static SM2_B: U256 = U256::from_be_u64_array(&[
  0x28e9fa9e9d9f5e34,
  0x4d5a9e4bcf6509a7,
  0xf39789f515ab8f92,
  0xddbcbd414d940e93
]);

/// # SM2 n 参数
static SM2_N: U256 = U256::from_be_u64_array(&[
  0xfffffffeffffffff,
  0xffffffffffffffff,
  0x7203df6b21c6052b,
  0x53bbf40939d54123
]);

/// # SM2 Gx 参数
static SM2_GX: U256 = U256::from_be_u64_array(&[
  0x32c4ae2c1f198119,
  0x5f9904466a39c994,
  0x8fe30bbff2660be1,
  0x715a4589334c74c7
]);

/// # SM2 Gy 参数
static SM2_GY: U256 = U256::from_be_u64_array(&[
  0xbc3736a2f4f6779c,
  0x59bdcee36b692153,
  0xd0a9877cc62a4740,
  0x02df32e52139f0a0
]);

/// # SM2 椭圆曲线参数结构体
static SM2_PARAMS: EccParams =
  EccParams { a: SM2_A, b: SM2_B, p: SM2_P, n: SM2_N, g_x: SM2_GX, g_y: SM2_GY };

/// # SM2 ECC 点 G
static SM2_G: EccPoint<'static> =
  EccPoint { x: SM2_GX, y: SM2_GY, params: &SM2_PARAMS, infinity: false };

/// # SM2 密钥对结构体
pub struct KeyPair<'a> {
  private_key: U256,
  public_key: EccPoint<'a>
}

/// # SM2 密钥对生成函数
/// 
/// 使用给定参数，随机生成私钥及对应公钥
/// 
/// ## 参数
/// 
/// * `params` - 椭圆曲线参数
/// 
/// ## 返回
/// 
/// 返回一个包含私钥和公钥的密钥对
pub fn key_gen(params: &EccParams) -> KeyPair {
  let d = U256::random_in_range(&mut rand::rng(), U256::C_1, params.n - U256::C_1);

  let p = SM2_G.ecc_mul(d, params);

  KeyPair { private_key: d, public_key: p }
}

/// # SM2 公钥验证函数
/// 
/// 验证给定的公钥是否有效
/// 
/// ## 参数
/// 
/// * `p` - 椭圆曲线点，作为公钥
/// 
/// ## 返回
/// 
/// 返回一个布尔值，表示公钥是否有效
pub fn pubkey_validate<'a>(p: &'a EccPoint) -> bool {
  if p.infinity {
    return false;
  }

  let params = p.params;

  if p.y.mod_mul(p.y, params.p)
    != p
      .x
      .mod_mul(p.x, params.p)
      .mod_mul(p.x, params.p)
      .mod_add(params.a.mod_mul(p.x, params.p), params.p)
      .mod_add(params.b % params.p, params.p)
  {
    return false;
  }

  p.ecc_mul(params.n, params).infinity
}

/// # SM2 获取Z值函数
/// 
/// 获取Z值
/// 
/// ## 参数
/// 
/// * `params` - 椭圆曲线参数
/// * `id` - 用户ID，ID为比特序列，最大长度为255比特
/// * `public_key` - 公钥
/// 
/// ## 返回
/// 
/// 返回一个U256值，表示Z值
fn get_z(params: &EccParams, id: &BitSequence, public_key: &EccPoint) -> U256 {
  let mut bits = BitSequence::new_empty();

  bits.append_bytes(&((id.get_bytes().len() - 8 + id.get_last_byte_len() as usize) as u16).to_le_bytes());
  bits.append_bits(id);
  bits.append_bytes(&params.a.into_le_bytes());
  bits.append_bytes(&params.b.into_le_bytes());
  bits.append_bytes(&params.g_x.into_le_bytes());
  bits.append_bytes(&params.g_y.into_le_bytes());
  bits.append_bytes(&public_key.x.into_le_bytes());
  bits.append_bytes(&public_key.y.into_le_bytes());

  hash(&bits).unwrap().into()
}

/// # SM2 签名输入结构体
/// 
/// 签名输入结构体，包含椭圆曲线参数、用户ID、公钥和私钥
/// 
/// ## 参数
/// 
/// * `params` - 椭圆曲线参数
/// * `id` - 用户ID
/// * `public_key` - 公钥
/// * `private_key` - 私钥
/// 
/// ## 构造方法
/// 
/// * `SigningInput::new(params, id, public_key, private_key)` - 创建签名输入结构体
/// 
/// ## 实现特征
/// 
/// * `Clone`
#[derive(Clone)]
pub struct SigningInput<'a> {
  params: &'a EccParams,
  id: BitSequence,
  public_key: EccPoint<'a>,
  private_key: U256
}

impl<'a> SigningInput<'a> {
  pub fn new(params: &'a EccParams, id: BitSequence, public_key: EccPoint<'a>, private_key: U256) -> Self {
    Self { params, id, public_key, private_key }
  }
}

/// # SM2 签名函数
/// 
/// 使用给定的椭圆曲线参数、用户ID、公钥和私钥，对给定的消息进行签名
/// 
/// ## 参数
/// 
/// * `input` - 签名输入结构体
/// * `message` - 比特序列消息
/// 
/// ## 返回
/// 
/// 返回一个元组，包含签名结果的r值和s值
pub fn generate_signature(input: &SigningInput, message: &BitSequence) -> ([u8; 32], [u8; 32]) {
  let mut m_bar = BitSequence::new(get_z(input.params, &input.id, &input.public_key).into_le_bytes().to_vec(), 0);

  m_bar.append_bits(message);

  let e = hash(&m_bar).unwrap().into();

  loop {

    let k = U256::random_in_range(&mut rand::rng(), U256::C_1, input.params.n);

    let x1 = SM2_G.ecc_mul(k, input.params).x;

    let r = x1.mod_add(e, input.params.p);

    if r == U256::C_0 || r.mod_add(k, input.params.p) == U256::C_0 {
      continue;
    }

    let s = input.private_key.mod_add(U256::C_1, input.params.p).mod_inv(input.params.p).unwrap().mod_mul(k.mod_add(-input.private_key.mod_mul(r, input.params.p), input.params.p), input.params.p).modded(input.params.n);

    if s != U256::C_0 {
      break (r.into_le_bytes(), s.into_le_bytes());
    }
  }
}

/// # SM2 签名验证输入结构体
/// 
/// 签名验证输入结构体，包含椭圆曲线参数、用户ID、公钥
/// 
/// ## 参数
/// 
/// * `params` - 椭圆曲线参数
/// * `id` - 用户ID
/// * `public_key` - 公钥
/// 
/// ## 构造方法
/// 
/// * `SigningVerificationInput::new(params, id, public_key)` - 创建签名验证输入结构体
///
/// ## 实现特征
/// 
/// * `Clone`
#[derive(Clone)]
pub struct SigningVerificationInput<'a> {
  params: &'a EccParams,
  id: BitSequence,
  public_key: EccPoint<'a>
}

impl<'a> SigningVerificationInput<'a> {
  pub fn new(params: &'a EccParams, id: BitSequence, public_key: EccPoint<'a>) -> Self {
    Self { params, id, public_key }
  }
}

/// # SM2 签名验证函数
/// 
/// 使用给定的椭圆曲线参数、用户ID、公钥和签名结果，验证签名结果是否有效
/// 
/// ## 参数
///
/// * `input` - 签名验证输入结构体
/// * `message` - 比特序列消息
/// * `signature` - 签名结果
/// 
/// ## 返回
/// 
/// 返回一个布尔值，表示签名结果是否有效
pub fn verify_signature(input: &SigningVerificationInput, message: &BitSequence, signature: ([u8; 32], [u8; 32])) -> bool {
  let r = U256::from_le_bytes(&signature.0);
  let s = U256::from_le_bytes(&signature.1);

  if r >= input.params.n || s >= input.params.n {
    return false;
  }

  let t = r.mod_add(s, input.params.p).modded(input.params.n);

  if t == U256::C_0 {
    return false;
  }

  let x1 = SM2_G.ecc_mul(s, input.params).ecc_add(input.public_key.ecc_mul(t, input.params), input.params).x;

  let mut m_bar = BitSequence::new(get_z(input.params, &input.id, &input.public_key).into_le_bytes().to_vec(), 0);

  m_bar.append_bits(message);

  let e: U256 = hash(&m_bar).unwrap().into();

  e.mod_add(x1, input.params.p).modded(input.params.n) == r
}
