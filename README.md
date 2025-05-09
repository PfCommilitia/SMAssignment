# SM2、SM3、SM4 算法实现

## 项目进度

* [x] SM2 椭圆曲线公钥密码算法
  * [x] 大整数数据类型
  * [x] 椭圆曲线基础运算
  * [x] 椭圆曲线公钥密码算法
  * [x] 椭圆曲线数字签名算法
  * [x] 椭圆曲线密钥交换协议
* [x] SM3 密码杂凑算法
* [x] SM4 分组密码算法
* [ ] 测试

在项目完全完成前，不保证代码可正常运行。

## 项目结构

```
SMAssignment
├── src              - 源代码目录
│   ├── math         - 数学相关模块
│   │   ├── ecc.rs   - 椭圆曲线运算相关模块
│   │   ├── mod.rs
│   │   └── u256.rs  - 256 位整数相关模块
│   ├── lib.rs
│   ├── main.rs
│   ├── sm_2.rs      - SM2 相关模块
│   ├── sm_3.rs      - SM3 相关模块
│   └── sm_4.rs      - SM4 相关模块
├── .gitignore
├── Cargo.toml
├── Changelog.md
├── LICENSE
├── README.md
└── rustfmt.toml
```

从 v0.1.0 版本开始，commit 信息只包含版本号，详细信息请参考 [此文件](Changelog.md)。

## 构建

1. [安装 Rust 工具链](https://www.rust-lang.org/tools/install)。
2. 使用 `cargo build` 构建项目。

## 开源与许可证

本项目在 [GitHub](https://github.com/PfCommilitia/SMAssignment) 开源，使用 [MIT 许可证](LICENSE)。

## 参考及 Crate 使用

* 主要参考文献为国家密码管理局发布的技术文档。
* [rand](https://crates.io/crates/rand) 用于生成随机数。
* 为 `U256` 类型实现四则运算时，参考了 [此页面](https://rgb-24bit.github.io/blog/2019/bitop.html)。
* 使用了以 `Claude-3.5-Sonnet` 为主的多个 AI 模型辅助开发。所有代码均由本人编写，或已经过本人审阅修改。
