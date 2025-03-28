// import unittest

// import dimstack

// # this test is a copy of MITCalc User Interface diagram

// m1 = dimstack.dim.Reviewed(
//     dim=dimstack.dim.Basic(
//         nom=208,
//         tol=dimstack.tolerance.SymmetricBilateral(0.036),
//         name="a",
//         desc="Shaft",
//     ),
//     target_process_sigma=6,
// )
// m1.assume_normal_dist_skewed(0.25)
// m2 = dimstack.dim.Reviewed(
//     dim=dimstack.dim.Basic(
//         nom=-1.75,
//         tol=dimstack.tolerance.UnequalBilateral(0, -0.06),
//         name="b",
//         desc="Retainer ring",
//     ),
//     target_process_sigma=3,
// )
// m2.assume_normal_dist()
// m3 = dimstack.dim.Reviewed(
//     dim=dimstack.dim.Basic(nom=-23, tol=dimstack.tolerance.UnequalBilateral(0, -0.12), name="c", desc="Bearing"),
//     target_process_sigma=3,
// )
// m3.assume_normal_dist()
// m4 = dimstack.dim.Reviewed(
//     dim=dimstack.dim.Basic(
//         nom=20,
//         tol=dimstack.tolerance.SymmetricBilateral(0.026),
//         name="d",
//         desc="Bearing Sleeve",
//     ),
//     target_process_sigma=3,
// )
// m4.assume_normal_dist()
// m5 = dimstack.dim.Reviewed(
//     dim=dimstack.dim.Basic(nom=-200, tol=dimstack.tolerance.SymmetricBilateral(0.145), name="e", desc="Case"),
//     target_process_sigma=3,
// )
// m5.assume_normal_dist()
// m6 = dimstack.dim.Reviewed(
//     dim=dimstack.dim.Basic(
//         nom=20,
//         tol=dimstack.tolerance.SymmetricBilateral(0.026),
//         name="f",
//         desc="Bearing Sleeve",
//     ),
//     target_process_sigma=3,
// )
// m6.assume_normal_dist()
// m7 = dimstack.dim.Reviewed(
//     dim=dimstack.dim.Basic(nom=-23, tol=dimstack.tolerance.UnequalBilateral(0, -0.12), name="g", desc="Bearing"),
//     target_process_sigma=3,
// )
// m7.assume_normal_dist()
// dims = [m1, m2, m3, m4, m5, m6, m7]

// stack = dimstack.dim.ReviewedStack(name="stacks on stacks", dims=dims)

// class MITCalc(unittest.TestCase):
//     def test_input(self):
//         self.assertEqual(len(stack.dims), 7)
//         self.assertEqual(stack.dims[0].dim.nominal, 208)
//         self.assertEqual(stack.dims[0].dim.tolerance.upper, 0.036)
//         self.assertEqual(stack.dims[0].dim.tolerance.lower, -0.036)

//     def test_Closed(self):
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.Closed(stack).nominal), 0.25)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.Closed(stack).tolerance.upper), 0.533)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.Closed(stack).tolerance.lower), -0.233)

//     def test_WC(self):
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.WC(stack).nominal), 0.4)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.WC(stack).tolerance.T / 2), 0.383)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.WC(stack).abs_lower), 0.017)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.WC(stack).abs_upper), 0.783)

//     def test_RSS(self):
//         # self.assertEqual(dimstack.utils.nround(stack.RSS.mean), 0.4)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.RSS(stack).nominal), 0.4)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.RSS(stack).tolerance.T / 2), 0.17825)
//         # self.assertEqual(dimstack.utils.nround(stack.RSS.stdev, 6), 0.059417)

//     def test_RSS_assembly(self):
//         eval = dimstack.calc.RSS(stack)
//         eval_dist = dimstack.dim.Reviewed(dim=eval)
//         spec = dimstack.dim.Requirement("spec", "", distribution=eval_dist.distribution, LL=0.05, UL=0.8)

//         self.assertEqual(dimstack.utils.nround(spec.R, 1), 0.0)

//     # def test_MRSS(self):
//     #     self.assertEqual(dimstack.utils.nround(stack.MRSS.mean), 0.4)
//     #     self.assertEqual(dimstack.utils.nround(stack.MRSS.nominal), 0.4)
//     #     # self.assertEqual(dimstack.utils.nround(stack.MRSS.tolerance.T / 2), 0.17825)
//     #     self.assertEqual(dimstack.utils.nround(stack.MRSS.tolerance.T / 2), 0.2405)
//     #     self.assertEqual(dimstack.utils.nround(stack.MRSS.stdev, 6), 0.059417)

//     def test_SixSigma(self):
//         self.assertEqual(dimstack.utils.nround(stack.dims[0].C_p), 2)
//         self.assertEqual(dimstack.utils.nround(stack.dims[0].k), 0.25)
//         self.assertEqual(dimstack.utils.nround(stack.dims[0].C_pk), 1.5)
//         self.assertEqual(dimstack.utils.nround(stack.dims[0].mean_eff), 208.0)
//         self.assertEqual(dimstack.utils.nround(stack.dims[0].stdev_eff), 0.008)

//         self.assertEqual(dimstack.utils.nround(dimstack.calc.SixSigma(stack, at=4.5).dim.nominal), 0.4)
//         # self.assertEqual(dimstack.utils.nround(dimstack.calc.SixSigma(stack, at=4.5).mean_eff), 0.4)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.SixSigma(stack, at=4.5).dim.tolerance.T / 2), 0.26433)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.SixSigma(stack, at=4.5).distribution.stdev, 6), 0.05874)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.SixSigma(stack, at=4.5).dim.abs_lower), 0.13567)
//         self.assertEqual(dimstack.utils.nround(dimstack.calc.SixSigma(stack, at=4.5).dim.abs_upper), 0.66433)

//     def test_SixSigma_assembly(self):
//         eval = dimstack.calc.SixSigma(stack, at=4.5)
//         spec = dimstack.dim.Requirement("spec", "", distribution=eval.distribution, LL=0.05, UL=0.8)

//         # self.assertEqual(dimstack.utils.nround(spec.C_p), 2.12804) # temporarily removed 20230623
//         # self.assertEqual(dimstack.utils.nround(spec.C_pk), 1.98617) # temporarily removed 20230623
//         self.assertEqual(dimstack.utils.nround(spec.R, 1), 0.0)

// if __name__ == "__main__":
//     unittest.main()

#[allow(unused_imports)]
use dimstack_rs::{calc, dim, dist, tol};
mod utils;
use std::f32::EPSILON;
// # this test is a copy Dimensioning and Tolerancing Handbook by McGraw Hill, Chapter 9
// This test is a copy of MITCalc User Interface diagram

#[test]
fn test_mitcalc() {
    let m1b = dim::Basic {
        nom: dim::Vec1D::new(208.0),
        tol: tol::Bilateral::symmetric(0.036),
        a: 1.0,
        name: "a".to_owned(),
        desc: "Shaft".to_owned(),
    };
    let m1 = dim::Reviewed::of_basic(m1b, 6.0).assume_normal_dist_skewed(0.25);
    let m2b = dim::Basic {
        nom: dim::Vec1D::new(-1.75),
        tol: tol::Bilateral::unequal(0.0, -0.06),
        a: 1.0,
        name: "b".to_owned(),
        desc: "Retainer ring".to_owned(),
    };
    let m2 = dim::Reviewed::of_basic(m2b, 3.0).assume_normal_dist();
    let m3b = dim::Basic {
        nom: dim::Vec1D::new(-23.0),
        tol: tol::Bilateral::unequal(0.0, -0.12),
        a: 1.0,
        name: "c".to_owned(),
        desc: "Bearing".to_owned(),
    };
    let m3 = dim::Reviewed::of_basic(m3b, 3.0).assume_normal_dist();
    let m4b = dim::Basic {
        nom: dim::Vec1D::new(20.0),
        tol: tol::Bilateral::symmetric(0.026),
        a: 1.0,
        name: "d".to_owned(),
        desc: "Bearing Sleeve".to_owned(),
    };
    let m4 = dim::Reviewed::of_basic(m4b, 3.0).assume_normal_dist();
    let m5b = dim::Basic {
        nom: dim::Vec1D::new(-200.0),
        tol: tol::Bilateral::symmetric(0.145),
        a: 1.0,
        name: "e".to_owned(),
        desc: "Case".to_owned(),
    };
    let m5 = dim::Reviewed::of_basic(m5b, 3.0).assume_normal_dist();
    let m6b = dim::Basic {
        nom: dim::Vec1D::new(20.0),
        tol: tol::Bilateral::symmetric(0.026),
        a: 1.0,
        name: "f".to_owned(),
        desc: "Bearing Sleeve".to_owned(),
    };
    let m6 = dim::Reviewed::of_basic(m6b, 3.0).assume_normal_dist();
    let m7b = dim::Basic {
        nom: dim::Vec1D::new(-23.0),
        tol: tol::Bilateral::unequal(0.0, -0.12),
        a: 1.0,
        name: "g".to_owned(),
        desc: "Bearing".to_owned(),
    };
    let m7 = dim::Reviewed::of_basic(m7b, 3.0).assume_normal_dist();
    let dims = vec![m1, m2, m3, m4, m5, m6, m7];

    let reviewed_stack = dim::ReviewedStack {
        name: "stacks on stacks".to_owned(),
        description: "".to_owned(),
        dims,
    };

    // test input
    assert_eq!(reviewed_stack.dims.len(), 7);
    assert_eq!(reviewed_stack.dims[0].dim.nom.x, 208.0);
    assert_eq!(reviewed_stack.dims[0].dim.tol.upper, 0.036);
    assert_eq!(reviewed_stack.dims[0].dim.tol.lower, -0.036);

    // test Closed
    let stack = dim::Stack::from_reviewed(&reviewed_stack);
    let closed = calc::closed(&stack);
    assert_eq!(closed.nom.x, 0.25);
    assert_delta!(closed.tol.upper, 0.533, EPSILON);
    assert_delta!(closed.tol.lower, -0.233, EPSILON);

    // test WC
    let wc = calc::wc(&stack);
    assert_delta!(wc.nom.x, 0.4, EPSILON);
    assert_delta!(wc.tol.t() / 2.0, 0.383, EPSILON);
    assert_delta!(wc.absolute_lower(), 0.017, EPSILON);
    assert_delta!(wc.absolute_upper(), 0.783, EPSILON);

    // test RSS
    let rss = calc::rss(&stack);
    // assert_delta!(rss.mean, 0.4, EPSILON);
    assert_delta!(rss.nom.x, 0.4, EPSILON);
    assert_delta!(rss.tol.t() / 2.0, 0.17825, EPSILON);
    // assert_delta!(rss.std_dev, 0.059417, EPSILON);

    // test RSS assembly
    let eval = calc::rss(&stack);
    let eval_dist = dim::Reviewed::of_basic(eval, 0.0);
    let spec = dim::Requirement {
        name: "spec".to_owned(),
        description: "".to_owned(),
        distribution: eval_dist.distribution.clone().unwrap(),
        ll: 0.05,
        ul: 0.8,
    };
    assert_delta!(spec.r(), 0.0, EPSILON);

    // test SixSigma
    assert_delta!(reviewed_stack.dims[0].c_p(), 2.0, EPSILON);
    assert_delta!(reviewed_stack.dims[0].k(), 0.25, EPSILON);
    assert_delta!(reviewed_stack.dims[0].c_pk(), 1.5, EPSILON);
    assert_delta!(reviewed_stack.dims[0].mean_eff(), 208.0, EPSILON);
    assert_delta!(reviewed_stack.dims[0].std_dev_eff(), 0.008, EPSILON);

    let six_sigma = calc::six_sigma(&reviewed_stack, 4.5);
    assert_delta!(six_sigma.dim.nom.x, 0.4, EPSILON);
    // assert_delta!(six_sigma.mean_eff, 0.4, EPSILON);
    assert_delta!(six_sigma.dim.tol.t() / 2.0, 0.26433, EPSILON);
    // assert_delta!(six_sigma.distribution.std_dev, 0.05874, EPSILON);
    match six_sigma.distribution {
        Some(dist::Distribution::Normal(d)) => assert_delta!(d.std_dev, 0.05874, EPSILON),
        _ => panic!("Expected Normal distribution"),
    }
    assert_delta!(six_sigma.dim.absolute_lower(), 0.13567, EPSILON);
    assert_delta!(six_sigma.dim.absolute_upper(), 0.66433, EPSILON);

    // test SixSigma assembly
    let eval = calc::six_sigma(&reviewed_stack, 4.5);
    let spec = dim::Requirement {
        name: "spec".to_owned(),
        description: "".to_owned(),
        distribution: eval.distribution.clone().unwrap(),
        ll: 0.05,
        ul: 0.8,
    };

    // assert_delta!(spec.c_p(), 2.12804, EPSILON); // temporarily removed 20230623
    // assert_delta!(spec.c_pk(), 1.98617, EPSILON); // temporarily removed 20230623
    assert_delta!(spec.r(), 0.0, EPSILON);
}
