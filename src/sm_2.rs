use crate::math::u256::U256;

/// SM2 常数
static SM2_P: U256 = U256::from_be_u64_array(&[
  0xfffffffeffffffff,
  0xffffffffffffffff,
  0xffffffff00000000,
  0xffffffffffffffff
]);

static SM2_A: U256 = U256::from_be_u64_array(&[
  0xfffffffeffffffff,
  0xffffffffffffffff,
  0xffffffff00000000,
  0xfffffffffffffffc
]);

static SM2_B: U256 = U256::from_be_u64_array(&[
  0x28e9fa9e9d9f5e34,
  0x4d5a9e4bcf6509a7,
  0xf39789f515ab8f92,
  0xddbcbd414d940e93
]);

static SM2_N: U256 = U256::from_be_u64_array(&[
  0xfffffffeffffffff,
  0xffffffffffffffff,
  0x7203df6b21c6052b,
  0x53bbf40939d54123
]);

static SM2_GX: U256 = U256::from_be_u64_array(&[
  0x32c4ae2c1f198119,
  0x5f9904466a39c994,
  0x8fe30bbff2660be1,
  0x715a4589334c74c7
]);

static SM2_GY: U256 = U256::from_be_u64_array(&[
  0xbc3736a2f4f6779c,
  0x59bdcee36b692153,
  0xd0a9877cc62a4740,
  0x02df32e52139f0a0
]);
