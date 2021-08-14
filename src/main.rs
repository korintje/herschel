mod operation;
use operation::Operation;
use core::cell::{Cell, RefCell};

fn main() {

  let w1 = vec![0.0, 1.0, 2.0, 3.0];
  let t1 = vec![2.0, 3.5, 4.2, 9.0];
  let w2 = vec![0.0, 1.0, 2.0, 3.0];
  let t2 = vec![3.0, 1.5, 4.7, 6.1];

  let op1 = operation::Generate{freq: RefCell::new(w1), trans: RefCell::new(t1)};
  let op2 = operation::Generate{freq: RefCell::new(w2), trans: RefCell::new(t2)};
  let op3 = operation::Add{augend: &op1, addend: &op2, coeff: Cell::new(1.0)};
  let op4 = operation::Add{augend: &op3, addend: &op2, coeff: Cell::new(-2.0)};

  println!("Trans: {:?}", op4.get_spectrum().trans);

  op3.coeff.set(-1.0);
  println!("Trans: {:?}", op4.get_spectrum().trans);

  op1.trans.replace(vec![20.0, 23.5, 40.2, 9.0]);
  println!("Trans: {:?}", op4.get_spectrum().trans);

}
