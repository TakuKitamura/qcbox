use rand::Rng;

// 振幅
#[derive(Debug)]
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
#[derive(Debug)]
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

fn get_existence_probability(qc: Qbit) -> ExistenceProbability{
  return ExistenceProbability{
    zero: qc.zero.magnitude.powf(2.0),
    one: qc.one.magnitude.powf(2.0)
  }
}

fn qc_read(qc: Qbit) -> u32 {
  let ep: ExistenceProbability = 
    get_existence_probability(qc);

  // 本物の量子コンピュータであれば, 本当の意味での乱数となる
  let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
  let random_float: f32 = rng.gen(); // 0~1
  if random_float < ep.zero { // zero
    return 0
  } else { // one
    return 1
  }
}

fn qc_write(){}

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

  println!("{:?}", qc_read(qc))
}

#[test]
fn qc_not_test() {
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