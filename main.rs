use rand::Rng;

// 振幅
#[derive(Debug, PartialEq)]
struct Amplitude{
  magnitude: f32, // 0.0 ~ 1.0
  relative_topology: f32 // 0 ~ 2π
}

// 複素数
struct ComplexNumber{
  real: f32,
  imaginary: f32
}

// キュービット
#[derive(Debug, PartialEq)]
struct Qbit{ // zero + one <= 1
  zero: Amplitude, // zero <= 1
  one: Amplitude // one <= 1
}

// フォトンが検出される確率
#[derive(Debug)]
struct ExistenceProbability{
  zero: f32,
  one: f32
}

const ZERO_QBIT: Qbit = Qbit {
  zero: Amplitude{
    magnitude: 1.0,
    relative_topology: 0.0
  },
  one: Amplitude{
    magnitude: 0.0,
    relative_topology: 0.0
  }
};

const ONE_QBIT: Qbit = Qbit {
  zero: Amplitude{
    magnitude: 0.0,
    relative_topology: 0.0
  },
  one: Amplitude{
    magnitude: 1.0,
    relative_topology: 0.0
  }
};

fn get_existence_probability(qc: Qbit) -> ExistenceProbability{
  return ExistenceProbability{
    zero: qc.zero.magnitude.powf(2.0),
    one: qc.one.magnitude.powf(2.0)
  }
}

fn qc_read(qc: Qbit) -> Qbit {
  let ep: ExistenceProbability = 
    get_existence_probability(qc);

  // 本物の量子コンピュータであれば, 本当の意味での乱数となる
  let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
  let random_float: f32 = rng.gen(); // 0~1
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

fn qc_phase(){}

fn main() {
  let qc: Qbit = Qbit {
    zero: Amplitude{
      magnitude: 0.707,
      relative_topology: 0.0
    },
    one: Amplitude{
      magnitude: 0.707,
      relative_topology: 0.0
    }
  };

  println!("{:?}", qc_write(qc, ONE_QBIT))
}

#[test]
fn qc_not_test() {
  // あくまでもテスト用のQbit
  let qc = Qbit {
    zero: Amplitude{
      magnitude: 0.1,
      relative_topology: 0.2
    },
    one: Amplitude{
      magnitude: 0.3,
      relative_topology: 0.4
    }
  };
  let not_qc = qc_not(qc);
  assert_eq!(not_qc.zero.magnitude, 0.3);
  assert_eq!(not_qc.zero.relative_topology, 0.4);
  assert_eq!(not_qc.one.magnitude, 0.1);
  assert_eq!(not_qc.one.relative_topology, 0.2);
}

#[test]
fn qc_write_test() {
  let qc: Qbit = Qbit {
    zero: Amplitude{
      magnitude: 0.707,
      relative_topology: 0.0
    },
    one: Amplitude{
      magnitude: 0.707,
      relative_topology: 0.0
    }
  };
  let zero = qc_write(qc, ZERO_QBIT);
  assert_eq!(zero, ZERO_QBIT);
  let one = qc_write(zero, ONE_QBIT);
  assert_eq!(one, ONE_QBIT);
}