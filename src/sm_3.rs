/// SM3 哈希函数
///
/// # 参数
///
/// * `input` - 输入消息
///   * 类型：`&[u8]` - 一个长度在 u64 范围内的二进制数组，通过 u8
///     数组的不可变借用方式传入
///   * 说明：最后一个 u8 未填充满时，后续位填充为 0
///
/// * `size` - 输入消息的二进制位数
///   * 类型：`u64`
///   * 说明：表示 `input` 中实际有效的二进制位数
///
/// # 返回值
///
/// * `Result<[u8; 32], Box<dyn std::error::Error>>`
///   * 成功时返回一个 256 位（32 字节）的二进制数组，表示哈希结果
///   * 失败时返回错误信息
///
/// # 错误
///
/// 函数可能在以下情况返回错误：
/// * 输入数组长度超出 u64 范围
/// * 输入大小不合法（size 大于实际位数或小于实际位数 -8）
///
/// # 算法步骤
///
/// 1. 输入验证
///    - 检查输入数组长度是否在 u64 范围内
///    - 验证 size 参数是否合法
///    - 确保输入数组尾端没有多余的 0
///
/// 2. 消息填充
///    - 添加一个 1 位
///    - 填充 0 位直到长度是 512 位的倍数
///    - 最后 64 位添加消息长度
///
/// 3. 迭代压缩
///    - 初始化 8 个 32 位字的中间状态
///    - 对每个 512 位分组进行压缩
///    - 使用消息扩展和压缩函数处理
///
/// 4. 输出结果
///    - 将 8 个 32 位字转换为 32 字节的输出
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

/// SM3 压缩函数
///
/// # 参数
///
/// * `last_result` - 上一次压缩的结果或初始值
///   * 类型：`&mut [u32; 8]` - 一个包含 8 个 u32 值的可变引用数组
///   * 说明：表示 256 位的中间哈希值，每个 u32 表示 32 位
///
/// * `message_group` - 待压缩的消息分组
///   * 类型：`&[u8; 64]` - 一个包含 64 个字节的不可变引用数组
///   * 说明：表示 512 位的消息分组，每个字节表示 8 位
///
/// # 处理流程
///
/// 1. 消息扩展：将 512 位的消息分组扩展为 2048 位
///    - 生成 68 个 32 位字 W0 到 W67
///    - 生成 64 个 32 位字 W'0 到 W'63
///    - 使用 P1 置换函数进行扩展
///
/// 2. 压缩处理：进行 64 轮迭代压缩
///    - 每轮使用不同的压缩函数和常量
///    - 更新 8 个 32 位字的中间状态
///
/// # 注意事项
///
/// * 该函数会直接修改 `last_result` 参数，不返回新的值
/// * 函数内部使用了 SM3 标准中定义的置换函数 P0、P1 和布尔函数 FFj、GGj
/// * 每轮迭代都会更新 8 个 32 位字的中间状态
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

/// SM3 置换函数 P0
///
/// # 参数
///
/// * `input` - 输入值
///   * 类型：`u32` - 32 位无符号整数
///
/// # 返回值
///
/// * `u32` - 经过 P0 置换后的 32 位无符号整数
///
/// # 说明
///
/// P0 置换函数是 SM3 算法中的基本置换函数之一，用于消息扩展和压缩过程中。
/// 其计算公式为：P0(X) = X ⊕ (X <<< 9) ⊕ (X <<< 17)
fn p_0(input: u32) -> u32 {
  input ^ input.rotate_left(9) ^ input.rotate_left(17)
}

/// SM3 置换函数 P1
///
/// # 参数
///
/// * `input` - 输入值
///   * 类型：`u32` - 32 位无符号整数
///
/// # 返回值
///
/// * `u32` - 经过 P1 置换后的 32 位无符号整数
///
/// # 说明
///
/// P1 置换函数是 SM3 算法中的基本置换函数之一，用于消息扩展过程中。
/// 其计算公式为：P1(X) = X ⊕ (X <<< 15) ⊕ (X <<< 23)
fn p_1(input: u32) -> u32 {
  input ^ input.rotate_left(15) ^ input.rotate_left(23)
}

/// SM3 常量 Tj
///
/// # 参数
///
/// * `index` - 迭代轮数索引
///   * 类型：`usize` - 无符号整数，表示当前迭代的轮数
///
/// # 返回值
///
/// * `u32` - 对应轮数的常量值
///
/// # 说明
///
/// Tj 是 SM3 算法中用于压缩函数的常量，根据迭代轮数返回不同的值：
/// * 当 j < 16 时，返回 0x79cc4519
/// * 当 j ≥ 16 时，返回 0x7a879d8a
fn t_j(index: usize) -> u32 {
  if index < 16 {
    0x79cc4519
  } else {
    0x7a879d8a
  }
}

/// SM3 布尔函数 FFj
///
/// # 参数
///
/// * `index` - 迭代轮数索引
///   * 类型：`usize` - 无符号整数，表示当前迭代的轮数
/// * `(x, y, z)` - 三个输入值
///   * 类型：`(u32, u32, u32)` - 三个 32 位无符号整数
///
/// # 返回值
///
/// * `u32` - 布尔函数计算结果
///
/// # 说明
///
/// FFj 是 SM3 算法中的布尔函数，根据迭代轮数使用不同的计算方式：
/// * 当 j < 16 时，FFj(X,Y,Z) = X ⊕ Y ⊕ Z
/// * 当 j ≥ 16 时，FFj(X,Y,Z) = (X & Y) | (X & Z) | (Y & Z)
fn ff_j(index: usize, (x, y, z): (u32, u32, u32)) -> u32 {
  if index < 16 {
    x ^ y ^ z
  } else {
    (x & y) | (x & z) | (y & z)
  }
}

/// SM3 布尔函数 GGj
///
/// # 参数
///
/// * `index` - 迭代轮数索引
///   * 类型：`usize` - 无符号整数，表示当前迭代的轮数
/// * `(x, y, z)` - 三个输入值
///   * 类型：`(u32, u32, u32)` - 三个 32 位无符号整数
///
/// # 返回值
///
/// * `u32` - 布尔函数计算结果
///
/// # 说明
///
/// GGj 是 SM3 算法中的布尔函数，根据迭代轮数使用不同的计算方式：
/// * 当 j < 16 时，GGj(X,Y,Z) = X ⊕ Y ⊕ Z
/// * 当 j ≥ 16 时，GGj(X,Y,Z) = (X & Y) | (!X & Z)
fn gg_j(index: usize, (x, y, z): (u32, u32, u32)) -> u32 {
  if index < 16 {
    x ^ y ^ z
  } else {
    (x & y) | (!x & z)
  }
}
