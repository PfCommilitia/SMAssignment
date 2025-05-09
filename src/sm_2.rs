use crate::{
  math::{
    bytes::BitSequence,
    ecc::{EccOps, EccParams, EccPoint, ModInv, ModOps},
    u256::U256
  },
  sm_3::hash
};

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
  // 随机生成私钥
  let d = U256::random_in_range(&mut rand::rng(), U256::C_1, params.n - U256::C_1);

  // 计算对应的公钥
  let g = EccPoint::new(params.g_x, params.g_y, params, false);
  let p = g.ecc_mul(d, params);

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

  if !p.validate_on_curve() {
    return false;
  }

  // 如果 [n]p 是无穷远点，则公钥有效
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

  // ID.bit_len() as u16 || ID || a || b || Gx || Gy || Px || Py
  bits.append_bytes(
    &((id.get_bytes().len() - 8 + id.get_last_byte_len() as usize) as u16).to_le_bytes()
  );
  bits.append_bits(id);
  bits.append_bytes(&params.a.into_le_bytes());
  bits.append_bytes(&params.b.into_le_bytes());
  bits.append_bytes(&params.g_x.into_le_bytes());
  bits.append_bytes(&params.g_y.into_le_bytes());
  bits.append_bytes(&public_key.x.into_le_bytes());
  bits.append_bytes(&public_key.y.into_le_bytes());

  // Z
  hash(&bits).into()
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
/// * `SigningInput::new(params, id, public_key, private_key)` -
///   创建签名输入结构体
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
  pub fn new(
    params: &'a EccParams,
    id: BitSequence,
    public_key: EccPoint<'a>,
    private_key: U256
  ) -> Self {
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
  // m_bar = Za || M

  let mut m_bar =
    BitSequence::new(get_z(input.params, &input.id, &input.public_key).into_le_bytes().to_vec(), 0);
  m_bar.append_bits(message);

  // e = H(m_bar)
  let e = hash(&m_bar).into();

  let g = EccPoint::new(input.params.g_x, input.params.g_y, input.params, false);
  loop {
    let k = U256::random_in_range(&mut rand::rng(), U256::C_1, input.params.n);

    let x1 = g.ecc_mul(k, input.params).x;

    let r = x1.mod_add(e, input.params.p);

    // r == 0 或 r + k == n，重新生成
    if r == U256::C_0 || r.mod_add(k, input.params.p) == input.params.n {
      continue;
    }

    // s = (k + 1)^-1 * (k - d_a * r) mod n
    let s = input
      .private_key
      .mod_add(U256::C_1, input.params.p)
      .mod_inv(input.params.p)
      .unwrap()
      .mod_mul(
        k.mod_add(-input.private_key.mod_mul(r, input.params.p), input.params.p),
        input.params.p
      )
      .modded(input.params.n);

    // s == 0，重新生成
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
/// * `SigningVerificationInput::new(params, id, public_key)` -
///   创建签名验证输入结构体
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
pub fn verify_signature(
  input: &SigningVerificationInput,
  message: &BitSequence,
  signature: ([u8; 32], [u8; 32])
) -> bool {
  let r = U256::from_le_bytes(&signature.0);
  let s = U256::from_le_bytes(&signature.1);

  // 检验是否在域内
  if r >= input.params.n || s >= input.params.n {
    return false;
  }

  // t = (t + s) mod n，t == 0 则验证失败
  let t = r.mod_add(s, input.params.p).modded(input.params.n);
  if t == U256::C_0 {
    return false;
  }

  // x1 = [s]G + [t]Pa
  let g = EccPoint::new(input.params.g_x, input.params.g_y, input.params, false);
  let x1 =
    g.ecc_mul(s, input.params).ecc_add(input.public_key.ecc_mul(t, input.params), input.params).x;

  // m_bar = Za || M
  let mut m_bar =
    BitSequence::new(get_z(input.params, &input.id, &input.public_key).into_le_bytes().to_vec(), 0);
  m_bar.append_bits(message);

  // e = H(m_bar)
  let e: U256 = hash(&m_bar).into();

  // R = (e + x1) mod n == r
  e.mod_add(x1, input.params.p).modded(input.params.n) == r
}

/// # SM2 密钥派生函数
///
/// 使用给定的比特序列和目标长度，生成一个比特序列
///
/// ## 参数
///
/// * `sequence` - 比特序列
/// * `target_length` - 目标长度
///
/// ## 返回
///
/// 返回一个比特序列
fn key_derivation_function(sequence: &BitSequence, target_length: u64) -> BitSequence {
  let mut result = BitSequence::new_empty();

  // 遍历次数 klen.div_ceil(v)，其中 v 为 256
  let blocks: u32 = target_length.div_ceil(256) as u32;

  // i 即为计数器 ct 计数，从 1 开始，到 blocks 结束
  for i in 1 ..= if target_length % 256 != 0 { blocks - 1 } else { blocks } {
    // Ha_i = H(Z || ct)

    let mut temp_sequence = sequence.clone();

    temp_sequence.append_bytes(&i.to_le_bytes());

    result.append_bytes(&hash(&temp_sequence));
  }

  // 如果 target_length % 256 != 0，则需要处理最后一个块，长度为 klen - (v * (klen
  // / v))
  if target_length % 256 != 0 {
    let mut temp_sequence = sequence.clone();

    temp_sequence.append_bytes(&blocks.to_le_bytes());

    // assertEq!(klen - (v * (klen / v)), klen % v)
    let target_len = target_length % 256;

    // 取 hash 结果前 target_len / 8 字节，其余舍弃
    let mut hash_result = hash(&temp_sequence)[.. target_len.div_ceil(8) as usize].to_vec();

    // 计算最后一个字节需要舍弃的位数
    let discarded_bits = 8 - target_len % 8;

    let last_byte = hash_result.last_mut().unwrap();

    // 仅舍弃右侧 discarded_bits 位，剩余位位置不变
    *last_byte >>= discarded_bits;
    *last_byte <<= discarded_bits;

    result
      .append_bits(&BitSequence::try_with_bits(hash_result.as_ref(), target_len as u64).unwrap());
  }

  result
}

/// # SM2 密钥交换输入结构体
///
/// 密钥交换输入结构体，包含椭圆曲线参数、用户ID、公钥和对方用户ID、对方公钥
///
/// ## 参数
///
/// * `params` - 椭圆曲线参数
/// * `id` - 用户ID
/// * `public_key` - 公钥
/// * `other_id` - 对方用户ID
/// * `other_public_key` - 对方公钥
pub struct ExchangeKeyInput<'a> {
  params: &'a EccParams,
  id: BitSequence,
  public_key: EccPoint<'a>,
  other_id: BitSequence,
  other_public_key: EccPoint<'a>
}

/// # SM2 密钥交换发起者状态结构体
///
/// 记录密钥交换发起者的状态，包含随机点、随机数
///
/// ## 参数
///
/// * `r_point` - 随机点
/// * `r` - 随机数
pub struct ExchangeKeyStateInitiator<'a> {
  r_point: EccPoint<'a>,
  r: U256
}

/// # SM2 密钥交换接收者状态结构体
///
/// 记录密钥交换接收者的状态，包含随机点、随机数、计算点、对方随机点
///
/// ## 参数
///
/// * `r_point` - 随机点
/// * `r` - 随机数
/// * `v` - 计算点
/// * `r_point_other` - 对方随机点
pub struct ExchangeKeyStateReceiver<'a> {
  r_point: EccPoint<'a>,
  v: EccPoint<'a>,
  r_point_other: EccPoint<'a>
}

/// # SM2 密钥交换生成输出结构体
///
/// 密钥交换生成输出结构体，包含密钥和发送到对方的负载
///
/// ## 参数
///
/// * `key` - 密钥
/// * `payload` - 发送到对方的负载
pub struct ExchangeKeyGenerateOutput<'a> {
  pub key: BitSequence,
  pub payload: ExchangeKeyGeneratePayload<'a>
}

/// # SM2 密钥交换生成负载结构体
///
/// 密钥交换生成负载结构体，包含自己的随机点、验证参数
///
/// ## 参数
///
/// * `received` - 随机点
/// * `validator` - 验证参数
pub struct ExchangeKeyGeneratePayload<'a> {
  pub received: EccPoint<'a>,
  pub validator: [u8; 32]
}

/// # SM2 密钥交换确认输出结构体
///
/// 密钥交换确认输出结构体，包含密钥和验证参数
///
/// ## 参数
///
/// * `key` - 密钥
/// * `payload` - 验证参数
pub struct ExchangeKeyConfirmOutput {
  pub key: BitSequence,
  pub payload: [u8; 32]
}

/// # SM2 密钥交换初始化函数
///
/// 初始化密钥交换，生成随机点
///
/// ## 参数
///
/// * `input` - 密钥交换输入结构体
///
/// ## 返回
///
/// 返回一个元组，包含随机点和一个状态结构体
pub fn exchange_key_initiate<'a>(
  input: &'a ExchangeKeyInput
) -> (EccPoint<'a>, ExchangeKeyStateInitiator<'a>) {
  let g = EccPoint::new(input.params.g_x, input.params.g_y, input.params, false);
  let r = U256::random_in_range(&mut rand::rng(), U256::C_1, input.params.n);
  let r_point = g.ecc_mul(r, input.params);

  (r_point, ExchangeKeyStateInitiator { r_point, r })
}

/// # SM2 密钥交换生成函数
///
/// 生成密钥交换的响应
///
/// ## 参数
///
/// * `input` - 密钥交换输入结构体
/// * `received` - 对方生成的随机点
/// * `private_key` - 私钥
/// * `klen` - 密钥长度
///
/// ## 返回
///
/// 如果协商成功，返回一个元组，包含密钥和发送到对方的负载，以及自己的状态结构体
///
/// 如果协商失败，返回一个错误
pub fn exchange_key_generate<'a>(
  input: &'a ExchangeKeyInput,
  received: &'a EccPoint,
  private_key: U256,
  klen: u64
) -> Result<(ExchangeKeyGenerateOutput<'a>, ExchangeKeyStateReceiver<'a>), &'static str> {
  // 提前验证 B5 前半部分，接收到的点是否在椭圆曲线上
  if !received.validate_on_given_curve(input.params) {
    return Err("Invalid received point");
  }

  // 生成随机数
  let r = U256::random_in_range(&mut rand::rng(), U256::C_1, input.params.n);

  // 计算随机点
  let g = EccPoint::new(input.params.g_x, input.params.g_y, input.params, false);
  let r_point = g.ecc_mul(r, input.params);

  // omega = ceil(log2(n)).div_ceil(2) - 1
  let omega = input.params.n.highest_bit().div_ceil(2) as u64 - 1;

  // x1_bar = 2^omega + (received.x & (2^omega - 1))
  let x1_bar = (U256::C_1 << omega as u32) + (received.x & (U256::C_1 << omega as u32 - 1));

  // x2_bar = 2^omega + (r_point.x & (2^omega - 1))
  let x2_bar = (U256::C_1 << omega as u32) + (r_point.x & (U256::C_1 << omega as u32 - 1));

  // t = (private_key + x2_bar * r) mod n
  let t =
    private_key.mod_add(x2_bar.mod_mul(r, input.params.p), input.params.p).modded(input.params.n);

  // V = [h \cdot t](input_other_public_key + [x1_bar]received)
  let v = input
    .other_public_key
    .ecc_add(received.ecc_mul(x1_bar, input.params), input.params)
    .ecc_mul(t, input.params);

  // 如果 V 是无穷远点，则验证失败
  if v.infinity {
    return Err("Invalid received point");
  }

  // Z = v.x || v.y || Za || Zb
  let mut sequence = BitSequence::new_empty();
  sequence.append_bytes(&v.x.into_le_bytes());
  sequence.append_bytes(&v.y.into_le_bytes());
  sequence
    .append_bytes(&get_z(input.params, &input.other_id, &input.other_public_key).into_le_bytes());
  sequence.append_bytes(&get_z(input.params, &input.id, &input.public_key).into_le_bytes());

  // K = KDF(Z, klen)
  let key = key_derivation_function(&sequence, klen);

  // 中间结果 Internal = v.x || Za || Zb || received.x || received.y || r_point.x
  // || r_point.y
  let mut to_hash_sequence_internal = BitSequence::new_empty();
  to_hash_sequence_internal.append_bytes(&v.x.into_le_bytes());
  to_hash_sequence_internal
    .append_bytes(&get_z(input.params, &input.id, &input.public_key).into_le_bytes());
  to_hash_sequence_internal
    .append_bytes(&get_z(input.params, &input.other_id, &input.other_public_key).into_le_bytes());
  to_hash_sequence_internal.append_bytes(&received.x.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&received.y.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&r_point.x.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&r_point.y.into_le_bytes());

  // 验证参数 S = H(0x02 || v.y || Internal)
  let mut to_hash_sequence = BitSequence::new_empty();
  to_hash_sequence.append_bytes(&[0x02]);
  to_hash_sequence.append_bytes(&v.y.into_le_bytes());
  to_hash_sequence.append_bytes(&hash(&to_hash_sequence_internal));
  let validator = hash(&to_hash_sequence);

  Ok((
    ExchangeKeyGenerateOutput {
      key,
      payload: ExchangeKeyGeneratePayload { received: r_point, validator }
    },
    ExchangeKeyStateReceiver { r_point, v, r_point_other: received.clone() }
  ))
}

/// # SM2 密钥交换确认函数
///
/// 确认密钥交换
///
/// ## 参数
///
/// * `input` - 密钥交换输入结构体
/// * `state` - 状态结构体
/// * `respond` - 对方发送的载荷
/// * `private_key` - 私钥
/// * `klen` - 密钥长度
///
/// ## 返回
///
/// 如果协商成功，返回一个元组，包含密钥和验证参数
///
/// 如果协商失败，返回一个错误
pub fn exchange_key_confirm<'a>(
  input: &'a ExchangeKeyInput,
  state: &'a ExchangeKeyStateInitiator<'a>,
  respond: &'a ExchangeKeyGeneratePayload,
  private_key: U256,
  klen: u64
) -> Result<ExchangeKeyConfirmOutput, &'static str> {
  // 提前验证 A6 前半部分，接收到的点是否在椭圆曲线上
  if !respond.received.validate_on_given_curve(input.params) {
    return Err("Invalid received point");
  }

  // omega = ceil(log2(n)).div_ceil(2) - 1
  let omega: u64 = (input.params.n.highest_bit() - 1).div_ceil(2) as u64 - 1;

  // x1_bar = 2^omega + (state.r_point.x & (2^omega - 1))
  let x1_bar = (U256::C_1 << omega as u32) + (state.r_point.x & (U256::C_1 << omega as u32 - 1));

  // x2_bar = 2^omega + (respond.received.x & (2^omega - 1))
  let x2_bar = (U256::C_1 << omega as u32) + (respond.received.x & (U256::C_1 << omega as u32 - 1));

  // t = (private_key + x1_bar * r) mod n
  let t = private_key
    .mod_add(x1_bar.mod_mul(state.r, input.params.p), input.params.p)
    .modded(input.params.n);

  // U = [h \cdot t](input_other_public_key + [x2_bar]respond.received)
  let u = input
    .other_public_key
    .ecc_add(respond.received.ecc_mul(x2_bar, input.params), input.params)
    .ecc_mul(t, input.params);

  // 如果 U 是无穷远点，则验证失败
  if u.infinity {
    return Err("Invalid received point");
  }

  // Z = u.x || u.y || Za || Zb
  let mut sequence = BitSequence::new_empty();
  sequence.append_bytes(&u.x.into_le_bytes());
  sequence.append_bytes(&u.y.into_le_bytes());
  sequence
    .append_bytes(&get_z(input.params, &input.other_id, &input.other_public_key).into_le_bytes());
  sequence.append_bytes(&get_z(input.params, &input.id, &input.public_key).into_le_bytes());

  // K = KDF(Z, klen)
  let key = key_derivation_function(&sequence, klen);

  // 中间结果 Internal = u.x || Za || Zb || received.x || received.y || r_point.x
  // || r_point.y
  let mut to_hash_sequence_internal = BitSequence::new_empty();
  to_hash_sequence_internal.append_bytes(&u.x.into_le_bytes());
  to_hash_sequence_internal
    .append_bytes(&get_z(input.params, &input.id, &input.public_key).into_le_bytes());
  to_hash_sequence_internal
    .append_bytes(&get_z(input.params, &input.other_id, &input.other_public_key).into_le_bytes());
  to_hash_sequence_internal.append_bytes(&state.r_point.x.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&state.r_point.y.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&respond.received.x.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&respond.received.y.into_le_bytes());

  // 验证参数 Sb = H(0x02 || u.y || Internal)
  let mut to_hash_sequence_1 = BitSequence::new_empty();
  to_hash_sequence_1.append_bytes(&[0x02]);
  to_hash_sequence_1.append_bytes(&u.y.into_le_bytes());
  to_hash_sequence_1.append_bytes(&hash(&to_hash_sequence_internal));
  let validator_1 = hash(&to_hash_sequence_1);

  // 如果验证参数不匹配，则验证失败
  if validator_1 != respond.validator {
    return Err("Invalid validator");
  }

  // 验证参数 Sa = H(0x03 || u.y || Internal)
  let mut to_hash_sequence_2 = BitSequence::new_empty();
  to_hash_sequence_2.append_bytes(&[0x03]);
  to_hash_sequence_2.append_bytes(&u.y.into_le_bytes());
  to_hash_sequence_2.append_bytes(&hash(&to_hash_sequence_internal));
  let validator_2 = hash(&to_hash_sequence_2);

  Ok(ExchangeKeyConfirmOutput { key, payload: validator_2 })
}

/// # SM2 密钥交换验证函数
///
/// 验证密钥交换
///
/// ## 参数
///
/// * `input` - 密钥交换输入结构体
/// * `state` - 状态结构体
/// * `respond` - 对方发送的验证参数
///
/// ## 返回
///
/// 如果验证成功，返回 true
pub fn exchange_key_validate<'a>(
  input: &'a ExchangeKeyInput,
  state: &'a ExchangeKeyStateReceiver<'a>,
  respond: &[u8; 32]
) -> bool {
  // 中间结果 Internal = v.x || Za || Zb || state.r_point_other.x ||
  // state.r_point_other.y || state.r_point.x || state.r_point.y
  let mut to_hash_sequence_internal = BitSequence::new_empty();
  to_hash_sequence_internal.append_bytes(&state.v.x.into_le_bytes());
  to_hash_sequence_internal
    .append_bytes(&get_z(input.params, &input.id, &input.public_key).into_le_bytes());
  to_hash_sequence_internal
    .append_bytes(&get_z(input.params, &input.other_id, &input.other_public_key).into_le_bytes());
  to_hash_sequence_internal.append_bytes(&state.r_point_other.x.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&state.r_point_other.y.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&state.r_point.x.into_le_bytes());
  to_hash_sequence_internal.append_bytes(&state.r_point.y.into_le_bytes());

  // 验证参数 Sa = H(0x03 || v.y || Internal)
  let mut to_hash_sequence = BitSequence::new_empty();
  to_hash_sequence.append_bytes(&[0x03]);
  to_hash_sequence.append_bytes(&state.v.y.into_le_bytes());
  to_hash_sequence.append_bytes(&hash(&to_hash_sequence_internal));

  // 如果验证参数不匹配，则验证失败
  hash(&to_hash_sequence) == *respond
}

/// # SM2 加密函数
///
/// 加密消息
///
/// ## 参数
///
/// * `params` - 椭圆曲线参数
/// * `message` - 消息
/// * `public_key` - 公钥
///
/// ## 返回
///
/// 如果加密成功，返回密文
///
/// 如果加密失败，返回错误
pub fn encrypt(
  params: &EccParams,
  message: &BitSequence,
  public_key: &EccPoint
) -> Result<BitSequence, &'static str> {
  let g = EccPoint::new(params.g_x, params.g_y, params, false);

  let (c1, intermediate, t) = loop {
    let k = U256::random_in_range(&mut rand::rng(), U256::C_1, params.n);

    let c1 = g.ecc_mul(k, params);

    if c1.infinity {
      return Err("Invalid c1");
    }

    if public_key.ecc_mul(params.n, params).infinity {
      return Err("Invalid s");
    }

    let intermediate = public_key.ecc_mul(k, params);

    let mut sequence = BitSequence::new_empty();
    sequence.append_bits(&BitSequence::from(c1));
    sequence.append_bits(&BitSequence::from(intermediate));

    let t = key_derivation_function(&sequence, message.len());

    if t.get_bytes().iter().find(|b| **b != 0).is_none() {
      continue;
    }

    break (c1, intermediate, t);
  };

  // c2 = M ^ t
  let c2 = message.xor(&t).unwrap();

  // c3 = H(x2 || M || y2)
  let mut sequence = BitSequence::new_empty();
  sequence.append_bits(&BitSequence::try_with_bits(&intermediate.x.into_le_bytes(), 256).unwrap());
  sequence.append_bits(&message);
  sequence.append_bits(&BitSequence::try_with_bits(&intermediate.y.into_le_bytes(), 256).unwrap());

  let c3 = BitSequence::try_with_bits(&hash(&sequence), 256).unwrap();

  let mut result = BitSequence::new_empty();
  result.append_bits(&BitSequence::from(c1));
  result.append_bits(&c2);
  result.append_bits(&c3);

  Ok(result)
}

/// # SM2 解密函数
///
/// 解密消息
///
/// ## 参数
///
/// * `params` - 椭圆曲线参数
/// * `cipher_text` - 密文
/// * `private_key` - 私钥
///
/// ## 返回
///
/// 如果解密成功，返回明文
///
/// 如果解密失败，返回错误
pub fn decrypt(
  params: &EccParams,
  cipher_text: &BitSequence,
  private_key: U256
) -> Result<BitSequence, &'static str> {
  let key_length = cipher_text.len() - 65 - 256;
  let c1 = EccPoint::from_bytes(&cipher_text.get_bytes()[0 .. 65].try_into().unwrap(), params);

  if !c1.validate_on_curve() {
    return Err("Invalid c1");
  }

  if c1.ecc_mul(params.n, params).infinity {
    return Err("Invalid s");
  }

  let p2 = c1.ecc_mul(private_key, params);

  let mut sequence = BitSequence::new_empty();
  sequence.append_bits(&BitSequence::try_with_bits(&p2.x.into_le_bytes(), 256).unwrap());
  sequence.append_bits(&BitSequence::try_with_bits(&p2.y.into_le_bytes(), 256).unwrap());

  let t = key_derivation_function(&sequence, key_length);

  if t.get_bytes().iter().all(|b| *b == 0) {
    return Err("Invalid t");
  }

  let c2 = cipher_text.slice(65, cipher_text.len() - 256).unwrap();

  let result = c2.xor(&t).unwrap();

  let mut to_hash_sequence = BitSequence::new_empty();
  to_hash_sequence.append_bits(&BitSequence::try_with_bits(&p2.x.into_le_bytes(), 256).unwrap());
  to_hash_sequence.append_bits(&result);
  to_hash_sequence.append_bits(&BitSequence::try_with_bits(&p2.y.into_le_bytes(), 256).unwrap());

  let u = BitSequence::try_with_bits(&hash(&to_hash_sequence), 256).unwrap();

  if u != cipher_text.slice(cipher_text.len() - 256, cipher_text.len()).unwrap() {
    return Err("Invalid u");
  }

  Ok(result)
}
