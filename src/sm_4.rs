/// SM4 S盒
///
/// 用于非线性变换的 16x16 字节替换表
static SBOX: [[u8; 16]; 16] = [
  [0xd6, 0x90, 0xe9, 0xfe, 0xcc, 0xe1, 0x3d, 0xb7, 0x16, 0xb6, 0x14, 0xc2, 0x28, 0xfb, 0x2c, 0x05],
  [0x2b, 0x67, 0x9a, 0x76, 0x2a, 0xbe, 0x04, 0xc3, 0xaa, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99],
  [0x9c, 0x42, 0x50, 0xf4, 0x91, 0xef, 0x98, 0x7a, 0x33, 0x64, 0x0b, 0x43, 0xed, 0xcf, 0xac, 0x62],
  [0xe4, 0xb3, 0x1c, 0xa9, 0xc9, 0x08, 0xe8, 0x95, 0x80, 0xdf, 0x94, 0xfa, 0x75, 0x8f, 0x3f, 0xa6],
  [0x47, 0x07, 0xa7, 0xfc, 0xf3, 0x73, 0x17, 0xba, 0x83, 0x59, 0x3c, 0x19, 0xe6, 0x85, 0x4f, 0xa8],
  [0x68, 0x6b, 0x81, 0xb2, 0x71, 0x64, 0xda, 0x8b, 0xf8, 0xeb, 0x0f, 0x4b, 0x70, 0x56, 0x9d, 0x35],
  [0x1e, 0x24, 0x0e, 0x5e, 0x63, 0x58, 0xd1, 0xa2, 0x25, 0x22, 0x7c, 0x3b, 0x01, 0x21, 0x78, 0x87],
  [0xd4, 0x00, 0x46, 0x57, 0x9f, 0xd3, 0x27, 0x52, 0x4c, 0x36, 0x02, 0xe7, 0xa0, 0xc4, 0xc8, 0x9e],
  [0xea, 0xbf, 0x8a, 0xd2, 0x40, 0xc7, 0x38, 0xb5, 0xa3, 0xf7, 0xf2, 0xce, 0xf9, 0x61, 0x15, 0xa1],
  [0xe0, 0xae, 0x5d, 0xa4, 0x9b, 0x34, 0x1a, 0x55, 0xad, 0x93, 0x32, 0x30, 0xf5, 0x8c, 0xb1, 0xe3],
  [0x1d, 0xf6, 0xe2, 0x2e, 0x82, 0x66, 0xca, 0x60, 0xc0, 0x29, 0x23, 0xab, 0x0d, 0x53, 0x4e, 0x6f],
  [0xd5, 0xdb, 0x37, 0x45, 0xde, 0xfd, 0x8e, 0x2f, 0x03, 0xff, 0x6a, 0x72, 0x6d, 0x6c, 0x5b, 0x51],
  [0x8d, 0x1b, 0xaf, 0x92, 0xbb, 0xdd, 0xbc, 0x7f, 0x11, 0xd9, 0x5c, 0x41, 0x1f, 0x10, 0x5a, 0xd8],
  [0x0a, 0xc1, 0x31, 0x88, 0xa5, 0xcd, 0x7b, 0xbd, 0x2d, 0x74, 0xd0, 0x12, 0xb8, 0xe5, 0xb4, 0xb0],
  [0x89, 0x69, 0x97, 0x4a, 0x0c, 0x96, 0x77, 0x7e, 0x65, 0xb9, 0xf1, 0x09, 0xc5, 0x6e, 0xc6, 0x84],
  [0x18, 0xf0, 0x7d, 0xec, 0x3a, 0xdc, 0x4d, 0x20, 0x79, 0xee, 0x5f, 0x3e, 0xd7, 0xcb, 0x39, 0x48]
];

/// SM4 系统参数 FK
///
/// 用于密钥扩展的系统参数
static FK: [u32; 4] = [0xa3b1bac6, 0x56aa3350, 0x677d9197, 0xb27022dc];

/// SM4 固定参数 CK
///
/// 用于密钥扩展的固定参数
static CK: [u32; 32] = [
  0x00070e15,
  0x1c232a31,
  0x383f464d,
  0x545b6269,
  0x70777e85,
  0x8c939aa1,
  0xa8afb6bd,
  0xc4cbd2d9,
  0xe0e7eef5,
  0xdc030a11,
  0x181f262d,
  0x343b4249,
  0x50575e65,
  0x6c737a81,
  0x888f969d,
  0xa4abb2b9,
  0xc0c7ced5,
  0xdce3eaf1,
  0xf8ff060d,
  0x141b2229,
  0x30373e45,
  0x4c535a61,
  0x686f767d,
  0x848b9299,
  0xa0a7aeb5,
  0xbcc3cad1,
  0xd8dfe6ed,
  0xf4fb0209,
  0x10171e25,
  0x2c333a41,
  0x484f565d,
  0x646b7279
];

/// SM4 工作模式
///
/// 定义 SM4 算法的工作模式：加密或解密
#[derive(Clone, Copy, Eq, PartialEq)]
enum Mode {
  Encrypt,
  Decrypt
}

/// SM4 加密函数
///
/// 对16字节的明文数据进行加密，生成16字节的密文。
///
/// # 参数
///
/// * `input` - 待加密的明文
///   * 类型：`&[u8; 16]` - 16 字节的明文数据
/// * `key` - 加密密钥
///   * 类型：`&[u8; 16]` - 16 字节的密钥数据
///
/// # 返回值
///
/// * `[u8; 16]` - 16 字节的密文数据
pub fn encrypt(input: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
  alter_group(input, key, Mode::Encrypt)
}

/// SM4 解密函数
///
/// 对16字节的密文数据进行解密，恢复出原始明文。
///
/// # 参数
///
/// * `input` - 待解密的密文
///   * 类型：`&[u8; 16]` - 16 字节的密文数据
/// * `key` - 解密密钥
///   * 类型：`&[u8; 16]` - 16 字节的密钥数据
///
/// # 返回值
///
/// * `[u8; 16]` - 16 字节的明文数据
///
/// # 说明
///
/// 解密过程与加密过程使用相同的算法结构，但轮密钥的使用顺序相反。
pub fn decrypt(input: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
  alter_group(input, key, Mode::Decrypt)
}

/// SM4 通用变换函数
///
/// # 参数
///
/// * `input` - 输入数据
///   * 类型：`&[u8; 16]` - 16 字节的输入数据
/// * `key` - 密钥
///   * 类型：`&[u8; 16]` - 16 字节的密钥数据
/// * `mode` - 工作模式
///   * 类型：`Mode` - 加密或解密模式
///
/// # 返回值
///
/// * `[u8; 16]` - 16 字节的输出数据
///
/// # 处理流程
///
/// 1. 将输入数据转换为 4 个 32 位字
/// 2. 根据模式生成轮密钥
/// 3. 进行 32 轮变换
/// 4. 将结果转换回字节数组
fn alter_group(input: &[u8; 16], key: &[u8; 16], mode: Mode) -> [u8; 16] {
  // 1. 转换输入数据

  let mut result_array_u32 = {
    let mut new_input = [0u32; 4];

    for i in 0 .. 4 {
      new_input[i] = (input[i * 4] as u32) << 24
        | (input[i * 4 + 1] as u32) << 16
        | (input[i * 4 + 2] as u32) << 8
        | (input[i * 4 + 3] as u32);
    }

    new_input
  };

  // 2. 计算轮密钥，解密时使用反向密钥序

  let round_keys = if mode == Mode::Encrypt {
    expand_key(key)
  } else {
    expand_key(key).into_iter().rev().collect::<Vec<u32>>().try_into().unwrap()
  };

  // 3. 使用轮密钥进行 32 轮迭代

  for i in 0 .. 32 {
    round(&mut result_array_u32, round_keys[i]);
  }

  // 4. 反序变换

  result_array_u32 = result_array_u32.into_iter().rev().collect::<Vec<u32>>().try_into().unwrap();

  // 5. 转换数据返回

  let mut result_array_u8 = [0u8; 16];
  for i in 0 .. 4 {
    let bytes = result_array_u32[i].to_be_bytes();
    result_array_u8[i * 4 .. (i + 1) * 4].copy_from_slice(&bytes);
  }

  result_array_u8
}

/// SM4 密钥扩展函数
///
/// # 参数
///
/// * `key` - 初始密钥
///   * 类型：`&[u8; 16]` - 16 字节的密钥数据
///
/// # 返回值
///
/// * `[u32; 32]` - 32 个轮密钥
///
/// # 处理流程
///
/// 1. 将密钥转换为 4 个 32 位字
/// 2. 使用系统参数 FK 进行初始变换
/// 3. 使用固定参数 CK 生成 32 个轮密钥
fn expand_key(key: &[u8; 16]) -> [u32; 32] {
  let mut mk = [0u32; 4];
  for i in 0 .. 4 {
    mk[i] = (key[i * 4] as u32) << 24
      | (key[i * 4 + 1] as u32) << 16
      | (key[i * 4 + 2] as u32) << 8
      | (key[i * 4 + 3] as u32);
  }

  let mut k = Vec::with_capacity(36);

  for i in 0 .. 4 {
    k[i] = mk[i] ^ FK[i];
  }

  for i in 4 .. 36 {
    k[i] = k[i - 4] ^ t_alter_s(k[i - 3] ^ k[i - 2] ^ k[i - 1] ^ CK[i - 4]);
  }

  k[4 ..].try_into().unwrap()
}

/// SM4 合成置换1
///
/// # 参数
///
/// * `input` - 输入值
///   * 类型：`u32` - 32 位无符号整数
///
/// # 返回值
///
/// * `u32` - 经过合成置换后的 32 位无符号整数
///
/// # 说明
///
/// 用于加密过程中的非线性变换，包含 S 盒变换和线性变换
fn t_alter(input: u32) -> u32 {
  let b = ita(input);

  b ^ b.rotate_left(2) ^ b.rotate_left(10) ^ b.rotate_left(18) ^ b.rotate_left(24)
}

/// SM4 合成置换2
///
/// # 参数
///
/// * `input` - 输入值
///   * 类型：`u32` - 32 位无符号整数
///
/// # 返回值
///
/// * `u32` - 经过合成置换后的 32 位无符号整数
///
/// # 说明
///
/// 用于密钥扩展过程中的非线性变换，包含 S 盒变换和线性变换
fn t_alter_s(input: u32) -> u32 {
  let b = ita(input);

  b ^ b.rotate_left(13) ^ b.rotate_left(23)
}

/// SM4 合成置换非线性变换
///
/// # 参数
///
/// * `input` - 输入值
///   * 类型：`u32` - 32 位无符号整数
///
/// # 返回值
///
/// * `u32` - 经过 S 盒变换后的 32 位无符号整数
///
/// # 说明
///
/// 使用 S 盒对输入的每个字节进行非线性变换
fn ita(input: u32) -> u32 {
  let bytes = input.to_be_bytes();

  let b_bytes = bytes
    .iter()
    .map(|item| SBOX[(*item as usize) / 16][(*item as usize) % 16])
    .collect::<Vec<u8>>();

  (b_bytes[0] as u32) << 24
    | (b_bytes[1] as u32) << 16
    | (b_bytes[2] as u32) << 8
    | (b_bytes[3] as u32)
}

/// SM4 轮函数
///
/// # 参数
///
/// * `input` - 轮函数输入状态
///   * 类型：`&mut [u32; 4]` - 4 个 32 位字的可变引用数组
/// * `round_key` - 轮密钥
///   * 类型：`u32` - 32 位无符号整数
///
/// # 说明
///
/// 执行一轮 SM4 变换，包括：
/// 1. 状态字的循环左移
/// 2. 使用轮密钥和合成置换进行变换
fn round(input: &mut [u32; 4], round_key: u32) {
  let temp = input[0];
  input[0] = input[1];
  input[1] = input[2];
  input[2] = input[3];
  input[3] = temp ^ t_alter(input[0] ^ input[1] ^ input[2] ^ round_key);
}
