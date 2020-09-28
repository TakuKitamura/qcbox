mod qc;
use num_complex::Complex;
fn main() {
  let qc: qc::Qbit = qc::Qbit {
    zero: Some(Complex::new(0.0, 0.0)),
    one: Some(Complex::new(1.0, 0.0)),
  };
  println!("had(had(qc))");
  println!("{:?}", qc::get_amplitude(&qc.zero));
  println!("{:?}", qc::get_amplitude(&qc.one));

  let qc = qc::qc_had(qc);
  println!("{:?}", qc::get_amplitude(&qc.zero));
  println!("{:?}", qc::get_amplitude(&qc.one));

  let qc = qc::qc_had(qc);
  println!("{:?}", qc::get_amplitude(&qc.zero));
  println!("{:?}", qc::get_amplitude(&qc.one));
}
