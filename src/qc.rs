use num_complex::Complex;
use rand::Rng;

// 振幅
#[derive(Debug, PartialEq)]
pub struct Amplitude {
  magnitude: f64,         // 0.0 ~ 1.0
  relative_topology: f64, // 0 ~ 2π
}

// キュービット
#[derive(Debug, PartialEq)]
pub struct Qbit {
  // zero + one <= 1
  pub zero: Option<Complex<f64>>, // zero <= 1
  pub one: Option<Complex<f64>>,  // one <= 1
}

// フォトンが検出される確率
#[derive(Debug)]
pub struct ExistenceProbability {
  zero: f64,
  one: f64,
}

const ZERO_QBIT: Qbit = Qbit {
  zero: Some(Complex::new(1.0, 0.0)),
  one: None,
};

const ONE_QBIT: Qbit = Qbit {
  zero: None,
  one: Some(Complex::new(1.0, 0.0)),
};

pub const PI: f64 = std::f64::consts::PI;

const SQRT_2: f64 = std::f64::consts::SQRT_2;

fn round(x: f64, n: u8) -> f64 {
  let scale = (10.0 as f64).powf(n as f64);
  return (x * scale).round() / scale;
}

pub fn get_relative_topology(cn: &Complex<f64>) -> f64 {
  // TODO: エラー処理
  if cn.re != 0.0 {
    let mut relative_topology = (cn.im / cn.re).atan();
    if relative_topology < 0.0 {
      // +表示に変換
      relative_topology += PI;
    }
    return relative_topology;
  } else {
    return 0.0;
  }
}

pub fn get_amplitude(cn: &Option<Complex<f64>>) -> Option<Amplitude> {
  match cn {
    Some(cn) => {
      let relative_topology: f64 = get_relative_topology(cn);
      return Some(Amplitude {
        magnitude: (cn.re.powf(2.0) + cn.im.powf(2.0)).sqrt(),
        relative_topology: relative_topology,
      });
    }
    None => None,
  }
}

pub fn get_existence_probability(qc: Qbit) -> ExistenceProbability {
  let zero_probability = match get_amplitude(&qc.zero) {
    Some(zero_amplitude) => zero_amplitude.magnitude.powf(2.0),
    None => 0.0,
  };
  let one_probability = match get_amplitude(&qc.one) {
    Some(one_amplitude) => one_amplitude.magnitude.powf(2.0),
    None => 0.0,
  };
  return ExistenceProbability {
    zero: zero_probability,
    one: one_probability,
  };
}

pub fn qc_read(qc: Qbit) -> Qbit {
  let ep: ExistenceProbability = get_existence_probability(qc);

  // 本物の量子コンピュータであれば, 本当の意味での乱数となる
  let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
  let random_float: f64 = rng.gen(); // 0~1
  if random_float < ep.zero {
    // zero
    return ZERO_QBIT;
  } else {
    // one
    return ONE_QBIT;
  }
}

// target: ZERO_QBIT or ONE_QBIT
pub fn qc_write(qc: Qbit, target_qbit: Qbit) -> Qbit {
  // write命令は, readとnot命令で実現できる
  let read_qbit = qc_read(qc);
  if read_qbit == target_qbit {
    return target_qbit;
  } else {
    return qc_not(read_qbit);
  }
}

pub fn qc_not(qc: Qbit) -> Qbit {
  // swap するだけ
  return Qbit {
    zero: qc.one,
    one: qc.zero,
  };
}

pub fn qc_had(qc: Qbit) -> Qbit {
  return match qc.zero {
    Some(qc_zero) => match qc.one {
      Some(qc_one) => {
        return Qbit {
          zero: Some((qc_zero + qc_one) / SQRT_2),
          one: Some((qc_zero - qc_one) / SQRT_2),
        };
      }
      None => Qbit {
        zero: None,
        one: None,
      },
    },
    None => Qbit {
      zero: None,
      one: None,
    },
  };
}

// radが+の場合, 反時計回りに回転
// TODO: 計算結果の計算誤差をどう扱うか検討
// TODO: 回転行列の利用の検討
pub fn qc_phase(qc: Qbit, rad: f64) -> Qbit {
  match qc.one {
    Some(one) => {
      let update_relative_topology: f64 = get_relative_topology(&one) + rad;
      return Qbit {
        zero: qc.zero,
        one: Some(Complex::new(
          one.re * update_relative_topology.cos(),
          one.re * update_relative_topology.sin(),
        )),
      };
    }
    None => Qbit {
      zero: None,
      one: None,
    },
  }
}

#[test]
fn qc_not_test() {
  let qc: Qbit = Qbit {
    zero: Some(Complex::new(0.1, 0.2)),
    one: Some(Complex::new(0.3, 0.4)),
  };
  let not_qc = qc_not(qc);
  let not_qc_zero = not_qc.zero.unwrap();
  let not_qc_one = not_qc.one.unwrap();
  assert_eq!(not_qc_zero.re, 0.3);
  assert_eq!(not_qc_zero.im, 0.4);
  assert_eq!(not_qc_one.re, 0.1);
  assert_eq!(not_qc_one.im, 0.2);
}
#[test]
fn qc_write_test() {
  let qc: Qbit = Qbit {
    zero: Some(Complex::new(0.1, 0.2)),
    one: Some(Complex::new(0.3, 0.4)),
  };
  let zero = qc_write(qc, ZERO_QBIT);
  assert_eq!(zero, ZERO_QBIT);
  let one = qc_write(zero, ONE_QBIT);
  assert_eq!(one, ONE_QBIT);
}
#[test]
fn qc_phase_test() {
  let qc: Qbit = Qbit {
    zero: Some(Complex::new(0.5, 0.0)),
    one: Some(Complex::new(0.5, 0.0)),
  };
  let phase_qc = qc_phase(qc, PI / 2.0);
  let phase_qc_zero = phase_qc.zero.unwrap();
  assert_eq!(phase_qc_zero.re, 0.5);
  assert_eq!(phase_qc_zero.im, 0.0);
  let phase_qc_one = phase_qc.one.unwrap();
  assert_eq!(round(phase_qc_one.re, 2), 0.0);
  assert_eq!(phase_qc_one.im, 0.5);

  let qc: Qbit = Qbit {
    zero: Some(Complex::new(0.5, 0.0)),
    one: Some(Complex::new(0.5, 0.0)),
  };
  let phase_qc = qc_phase(qc, (-PI) / 2.0);
  let phase_qc_zero = phase_qc.zero.unwrap();
  assert_eq!(phase_qc_zero.re, 0.5);
  assert_eq!(phase_qc_zero.im, 0.0);
  let phase_qc_one = phase_qc.one.unwrap();
  assert_eq!(round(phase_qc_one.re, 2), 0.0);
  assert_eq!(phase_qc_one.im, -0.5);
}
#[test]
fn qc_had_test() {
  let qc: Qbit = Qbit {
    zero: Some(Complex::new(0.0, 0.0)),
    one: Some(Complex::new(1.0, 0.0)),
  };
  let qc = qc_had(qc);
  let zero_amplitude = get_amplitude(&qc.zero).unwrap();
  let one_amplitude = get_amplitude(&qc.one).unwrap();
  assert_eq!(round(zero_amplitude.magnitude, 5), 0.70711);
  assert_eq!(zero_amplitude.relative_topology, 0.0);
  assert_eq!(round(one_amplitude.magnitude, 5), 0.70711);
  assert_eq!(one_amplitude.relative_topology, 0.0);

  let qc = qc_had(qc);
  let zero_amplitude = get_amplitude(&qc.zero).unwrap();
  let one_amplitude = get_amplitude(&qc.one).unwrap();
  assert_eq!(zero_amplitude.magnitude, 0.0);
  assert_eq!(zero_amplitude.relative_topology, 0.0);
  assert_eq!(round(one_amplitude.magnitude, 2), 1.0);
  assert_eq!(one_amplitude.relative_topology, 0.0);
}
