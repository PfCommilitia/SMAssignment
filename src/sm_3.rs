/// # SM3 哈希函数
///
/// ## 参数
///
/// * `input` - 输入消息；最后一个字节未填充满时，后续位填充为 0
///
/// * `size` - 输入消息的二进制位数；为实际有效的位数，单位为比特
///
/// ## 返回值
///
/// * `Result<[u8; 32], Box<dyn std::error::Error>>` - 成功时返回一个 256 位（32 字节）的二进制数组，表示哈希结果；失败时返回错误信息
///
/// ## 错误
///
/// * `Too long an input` - 输入数组长度超出 u64 范围
/// * `Invalid input size` - 输入大小不合法（size 大于实际位数或小于实际位数 -8）
pub fn hash(input: &[u8], size: u64) -> Result<[u8; 32], Box<dyn std::error::Error>> {
  // 0. 验证输入

  {
    // 确保输入数组大小在 u64 以内
    let capacity = {
      let capacity_bytes_u64: u64 = input.len().try_into()?;

      match 8u64.checked_mul(capacity_bytes_u64) {
        Some(capacity) => capacity,
        None => return Err("Too long an input".to_string().into())
      }
    };

    // 确保 size 合法，且输入数组尾端没有多余的0
    if capacity < size {
      return Err("Invalid input size".to_string().into());
    }

    match size.overflowing_add(8) {
      (added_size, false) => {
        if capacity > added_size {
          return Err("Invalid input size".to_string().into());
        }
      },
      _ => {}
    }
  }

  // 1. 填充

  // 计算填充后长度
  let size_required = size + 65;
  let padded_size =
    if size_required % 512 == 0 { size_required } else { (size_required / 512 + 1) * 512 };

  let input = {
    let mut new_input = vec![0u8; (padded_size as usize) / 8];

    new_input[.. input.len()].copy_from_slice(input);

    // 加入一个1位
    if size % 8 == 0 {
      new_input[(size as usize) / 8] = 0x80;
    } else {
      new_input[(size as usize) / 8] |= 1 << (7 - size % 8);
    }

    // 最后64位为输入长度
    let size_bits = size.to_be_bytes();
    new_input[((padded_size as usize) / 8 - 8) ..].copy_from_slice(&size_bits);

    new_input
  };

  // 2. 迭代过程

  // 初始化V值
  let mut result_array_u32: [u32; 8] = [
    0x7380166f,
    0x4914b2b9,
    0x172442d7,
    0xda8a0600,
    0xa96f30bc,
    0x163138aa,
    0xe38dee4d,
    0xb0fb0e4e
  ];

  // 分组，对每个组调用压缩函数，最终结果保存到 `result_array_u32`
  for i in 0 .. ((padded_size as usize) / 512) {
    // 声明 `message_group` 为 [u8; 64]
    let message_group: [u8; 64] = input[(i * 64) .. (i * 64 + 64)].try_into()?;

    cf(&mut result_array_u32, &message_group);
  }

  // 3. 输出

  // 将迭代的V值转换为u8数组
  let mut result_array_u8 = [0u8; 32];
  for i in 0 .. 8 {
    let bytes = result_array_u32[i].to_be_bytes();
    result_array_u8[i * 4 .. (i + 1) * 4].copy_from_slice(&bytes);
  }

  Ok(result_array_u8)
}

/// # SM3 压缩函数
///
/// ## 参数
///
/// * `last_result` - 上一次压缩的结果或初始值
///
/// * `message_group` - 待压缩的消息分组
///
/// ## 注意事项
///
/// * 该函数直接修改 `last_result`
fn cf(last_result: &mut [u32; 8], message_group: &[u8; 64]) {
  // 1. 消息扩展

  let mut wj = [0u32; 68];

  for j in 0 .. 16 {
    wj[j] = (message_group[j * 4] as u32) << 24
      | (message_group[j * 4 + 1] as u32) << 16
      | (message_group[j * 4 + 2] as u32) << 8
      | (message_group[j * 4 + 3] as u32);
  }

  for j in 16 ..= 67 {
    wj[j] = p_1(wj[j - 16] ^ wj[j - 9] ^ (wj[j - 3].rotate_left(15)))
      ^ (wj[j - 13].rotate_left(7))
      ^ wj[j - 6];
  }

  let mut wj_s = [0u32; 64];

  for j in 0 .. 64 {
    wj_s[j] = wj[j] ^ wj[j + 4];
  }

  // 2. 进行压缩
  for j in 0 .. 64 {
    let ss_1 = last_result[0]
      .rotate_left(12)
      .wrapping_add(last_result[4])
      .wrapping_add(t_j(j).rotate_left(j as u32))
      .rotate_left(7);

    let ss_2 = ss_1 ^ last_result[0].rotate_left(12);

    let tt_1 = ff_j(j, (last_result[0], last_result[1], last_result[2]))
      .wrapping_add(last_result[3])
      .wrapping_add(ss_2)
      .wrapping_add(wj_s[j]);

    let tt_2 = gg_j(j, (last_result[4], last_result[5], last_result[6]))
      .wrapping_add(last_result[7])
      .wrapping_add(ss_1)
      .wrapping_add(wj_s[j]);

    last_result[3] = last_result[2];

    last_result[2] = last_result[1].rotate_left(9);

    last_result[1] = last_result[0];

    last_result[0] = tt_1;

    last_result[7] = last_result[6];

    last_result[6] = last_result[5].rotate_left(19);

    last_result[5] = last_result[4];

    last_result[4] = p_0(tt_2);
  }
}

/// # SM3 置换函数 P0
///
/// ## 参数
///
/// * `input` - 输入值
///
/// ## 返回值
///
/// * `u32` - 经过 P0 置换后的 32 位无符号整数
fn p_0(input: u32) -> u32 {
  // P0(X) = X ⊕ (X <<< 9) ⊕ (X <<< 17)
  input ^ input.rotate_left(9) ^ input.rotate_left(17)
}

/// # SM3 置换函数 P1
///
/// ## 参数
///
/// * `input` - 输入值
///
/// ## 返回值
///
/// * `u32` - 经过 P1 置换后的 32 位无符号整数
fn p_1(input: u32) -> u32 {
  // P1(X) = X ⊕ (X <<< 15) ⊕ (X <<< 23)
  input ^ input.rotate_left(15) ^ input.rotate_left(23)
}

/// # SM3 常量 Tj
///
/// ## 参数
///
/// * `index` - 迭代轮数索引
///
/// ## 返回值
///
/// * `u32` - 对应轮数的常量值
fn t_j(index: usize) -> u32 {
  if index < 16 {
    0x79cc4519
  } else {
    0x7a879d8a
  }
}

/// # SM3 布尔函数 FFj
///
/// ## 参数
///
/// * `index` - 迭代轮数索引
/// * `(x, y, z)` - 三个输入值
///
/// ## 返回值
///
/// * `u32` - 布尔函数计算结果
fn ff_j(index: usize, (x, y, z): (u32, u32, u32)) -> u32 {
  if index < 16 {
    // X ⊕ Y ⊕ Z
    x ^ y ^ z
  } else {
    // (X & Y) | (X & Z) | (Y & Z)
    (x & y) | (x & z) | (y & z)
  }
}

/// #SM3 布尔函数 GGj
///
/// ## 参数
///
/// * `index` - 迭代轮数索引
/// * `(x, y, z)` - 三个输入值
///
/// ## 返回值
///
/// * `u32` - 布尔函数计算结果
fn gg_j(index: usize, (x, y, z): (u32, u32, u32)) -> u32 {
  if index < 16 {
    // X ⊕ Y ⊕ Z
    x ^ y ^ z
  } else {
    // (X & Y) | (!X & Z)
    (x & y) | (!x & z)
  }
}
