use super::u256::U256;

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

#[derive(PartialEq, Eq)]
pub struct EccParams {
  pub a: U256,
  pub b: U256,
  pub p: U256,
  pub n: U256,
  pub g_x: U256,
  pub g_y: U256
}

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

pub trait EccOps<'a>: Sized {
  fn ecc_add(self, other: Self, params: &'a EccParams) -> Result<Self, String>;
  fn ecc_mul(self, k: U256, params: &'a EccParams) -> Self;
}

impl<'a> EccOps<'a> for EccPoint<'a> {
  fn ecc_add(self, other: Self, params: &'a EccParams) -> Result<Self, String> {
    if self.params != other.params {
      panic!("Incompatible elliptic curve parameters");
    }

    if self == other {
      return Ok(self.ecc_mul(U256::C_2, params));
    }

    match (self.infinity, other.infinity) {
      (true, _) => return Ok(other),
      (_, true) => return Ok(self),
      (false, false) => {}
    }

    let num = other.y + (params.p - self.y);
    let denom = other.x + (params.p - self.x);

    let denom_inv = denom.mod_inv(params.p);
    if denom_inv.is_none() {
      return Err("Denominator is not invertible".to_string());
    }

    let lambda = num * denom_inv.unwrap() % params.p;

    let x3 = (lambda * lambda + (params.p - self.x) + (params.p - other.x)) % params.p;
    let y3 = (lambda * (self.x + (params.p - x3)) + (params.p - self.y)) % params.p;

    Ok(EccPoint::new_simple(x3, y3, params))
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
          Some(r) => Some(r.ecc_add(addend, params).unwrap())
        };
      }
      addend = addend.ecc_add(addend, params).unwrap();
    }

    match res {
      Some(r) => r,
      None => EccPoint::infinity(params)
    }
  }
}
