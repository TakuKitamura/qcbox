use rand::Rng;

// 振幅
#[derive(Debug, PartialEq)]
struct Amplitude{
  magnitude: f64, // 0.0 ~ 1.0
  relative_topology: f64 // 0 ~ 2π
}

// 複素数
#[derive(Debug, PartialEq)]
struct ComplexNumber{
  real: f64,
  imaginary: f64
}

// キュービット
#[derive(Debug, PartialEq)]
struct Qbit{ // zero + one <= 1
  zero: Option<ComplexNumber>, // zero <= 1
  one: Option<ComplexNumber> // one <= 1
}

// フォトンが検出される確率
#[derive(Debug)]
struct ExistenceProbability{
  zero: f64,
  one: f64
}

const ZERO_QBIT: Qbit = Qbit {
  zero: Some(ComplexNumber{
    real: 1.0,
    imaginary: 0.0
  }),
  one: None
};

const ONE_QBIT: Qbit = Qbit {
  zero: None,
  one: Some(ComplexNumber{
    real: 1.0,
    imaginary: 0.0
  })
};

const PI: f64 = std::f64::consts::PI;
const TWO_PI: f64 = 2.0 * PI;

fn round(x:f64, n: u8) -> f64 {
  let scale = (10.0 as f64).powf(n as f64);
  return (x * scale).round() / scale
} 

fn get_relative_topology(cn: &ComplexNumber) -> f64 {
  let mut relative_topology = (cn.imaginary/cn.real).atan();
  if relative_topology < 0.0 { // +表示に変換
    relative_topology += PI;
  }
  return relative_topology;
}

fn get_amplitude(cn: &Option<ComplexNumber>) -> Option<Amplitude> {
  match cn {
    Some(cn) => {
      let relative_topology: f64 = get_relative_topology(cn);
      return Some(Amplitude {
        magnitude: (cn.real.powf(2.0) + cn.imaginary.powf(2.0)).sqrt(),
        relative_topology: relative_topology 
      })
    },
    None => None
  } 
}

fn get_existence_probability(qc: Qbit) -> ExistenceProbability{
  let zero_probability = match get_amplitude(&qc.zero){
    Some(zero_amplitude) => zero_amplitude.magnitude.powf(2.0),
    None => 0.0
  };
  let one_probability = match get_amplitude(&qc.one){
    Some(one_amplitude) => one_amplitude.magnitude.powf(2.0),
    None => 0.0
  };
  return ExistenceProbability{
    zero: zero_probability,
    one: one_probability
  }
}

fn qc_read(qc: Qbit) -> Qbit {
  let ep: ExistenceProbability = 
    get_existence_probability(qc);

  // 本物の量子コンピュータであれば, 本当の意味での乱数となる
  let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
  let random_float: f64 = rng.gen(); // 0~1
  if random_float < ep.zero { // zero
    return ZERO_QBIT
  } else { // one
    return ONE_QBIT
  }
}

// target: ZERO_QBIT or ONE_QBIT
fn qc_write(qc: Qbit, target_qbit: Qbit) -> Qbit{
  // write命令は, readとnot命令で実現できる
  let read_qbit = qc_read(qc);
  if read_qbit == target_qbit {
    return target_qbit;
  } else {
    return qc_not(read_qbit)
  }
}

fn qc_not(qc: Qbit) -> Qbit{
  // swap するだけ
  return Qbit {
    zero: qc.one,
    one: qc.zero
  }
}

fn qc_had(){}

// radが+の場合, 反時計回りに回転
// TODO: 計算結果の計算誤差をどう扱うか検討
fn qc_phase(qc: Qbit, rad: f64) -> Qbit{
  match qc.one {
    Some(one) => {
      let update_relative_topology: f64 = get_relative_topology(&one) + rad;
      return Qbit {
        zero: qc.zero,
        one: Some(ComplexNumber{
          real: one.real * update_relative_topology.cos(),
          imaginary: one.real * update_relative_topology.sin()
        })
      }
    },
    None => 
      Qbit {
        zero: None,
        one: None
      }
  }
}

fn main() {
  let qc: Qbit = Qbit {
    zero: Some(ComplexNumber{
      real: 0.5,
      imaginary: 0.0
    }),
    one: Some(ComplexNumber{
      real: 0.5,
      imaginary: 0.0
    })
  };
  println!("{:?}\n", round(1.23456, 2));
}

#[test]
fn qc_not_test() {
  let qc: Qbit = Qbit {
    zero: Some(ComplexNumber{
      real: 0.1,
      imaginary: 0.2
    }),
    one: Some(ComplexNumber{
      real: 0.3,
      imaginary: 0.4
    })
  };
  let not_qc = qc_not(qc);
  let not_qc_zero = not_qc.zero.unwrap();
  let not_qc_one = not_qc.one.unwrap();
  assert_eq!(not_qc_zero.real, 0.3);
  assert_eq!(not_qc_zero.imaginary, 0.4);
  assert_eq!(not_qc_one.real, 0.1);
  assert_eq!(not_qc_one.imaginary, 0.2);
}

#[test]
fn qc_write_test() {
  let qc: Qbit = Qbit {
    zero: Some(ComplexNumber{
      real: 0.1,
      imaginary: 0.2
    }),
    one: Some(ComplexNumber{
      real: 0.3,
      imaginary: 0.4
    })
  };
  let zero = qc_write(qc, ZERO_QBIT);
  assert_eq!(zero, ZERO_QBIT);
  let one = qc_write(zero, ONE_QBIT);
  assert_eq!(one, ONE_QBIT);
}

#[test]
fn qc_phase_test() {
  let qc: Qbit = Qbit {
    zero: Some(ComplexNumber{
      real: 0.5,
      imaginary: 0.0
    }),
    one: Some(ComplexNumber{
      real: 0.5,
      imaginary: 0.0
    })
  };
  let phase_qc = qc_phase(qc, PI/2.0);
  let phase_qc_zero = phase_qc.zero.unwrap();
  assert_eq!(phase_qc_zero.real, 0.5);
  assert_eq!(phase_qc_zero.imaginary, 0.0);
  
  let phase_qc_one = phase_qc.one.unwrap();
  assert_eq!(round(phase_qc_one.real, 2), 0.0);
  assert_eq!(phase_qc_one.imaginary, 0.5);

  let qc: Qbit = Qbit {
    zero: Some(ComplexNumber{
      real: 0.5,
      imaginary: 0.0
    }),
    one: Some(ComplexNumber{
      real: 0.5,
      imaginary: 0.0
    })
  };
  let phase_qc = qc_phase(qc, (-PI)/2.0);
  let phase_qc_zero = phase_qc.zero.unwrap();
  assert_eq!(phase_qc_zero.real, 0.5);
  assert_eq!(phase_qc_zero.imaginary, 0.0);
  
  let phase_qc_one = phase_qc.one.unwrap();
  assert_eq!(round(phase_qc_one.real, 2), 0.0);
  assert_eq!(phase_qc_one.imaginary, -0.5);
}
