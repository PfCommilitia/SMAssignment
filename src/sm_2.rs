use crate::math::{
  ecc::{EccOps, EccParams, EccPoint, ModOps},
  u256::U256
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

pub struct KeyPair<'a> {
  private_key: U256,
  public_key: EccPoint<'a>
}

pub fn key_gen(params: &EccParams) -> KeyPair {
  let d = U256::random_in_range(&mut rand::rng(), U256::C_1, params.n - U256::C_1);

  let p = SM2_G.ecc_mul(d, params);

  KeyPair { private_key: d, public_key: p }
}

pub fn pubkey_validate<'a>(p: &EccPoint<'a>) -> bool {
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
