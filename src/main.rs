mod qc;
fn main() {
  let undefined_qc: qc::Qbit = qc::Qbit {
    zero: None,
    one: None,
  };
  let qc = qc::qc_write(undefined_qc, qc::ONE_QBIT);
  println!("-- WRITE 1 --");
  println!("{:?}", qc::get_amplitude(&qc.zero).unwrap());
  println!("{:?}", qc::get_amplitude(&qc.one).unwrap());
  let qc = qc::qc_had(qc);
  println!("-- HAD --");
  println!("{:?}", qc::get_amplitude(&qc.zero).unwrap());
  println!("{:?}", qc::get_amplitude(&qc.one).unwrap());
  let qc = qc::qc_had(qc);
  println!("-- HAD --");
  println!("{:?}", qc::get_amplitude(&qc.zero).unwrap());
  println!("{:?}", qc::get_amplitude(&qc.one).unwrap());
  println!("-- READ --");
  if qc::qc_read(qc) == qc::ZERO_QBIT {
    println!("READ ZERO!!")
  } else {
    println!("READ ONE!!")
  };
}
