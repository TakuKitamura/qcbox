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
struct Qbit{
  zero: Amplitude,
  one: Amplitude
}

fn qc_read(){}

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
  let qc = Qbit {
    zero: Amplitude{
      magnitude: 1.0,
      relative_topology: 0.0
    },
    one: Amplitude{
      magnitude: 0.0,
      relative_topology: 0.0
    }
  };
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