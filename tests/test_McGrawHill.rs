use dimstack_rs::{calc, dim, tol};
mod utils;
use std::f32::EPSILON;
/// # this test is a copy Dimensioning and Tolerancing Handbook by McGraw Hill, Chapter 9
#[test]
fn test_mcgrawhill_1() {
    let m1 = dim::Basic {
        nom: dim::Vec1D::new(-0.375),
        tol: tol::Bilateral::unequal(0.0, -0.031),
        a: 1.0,
        name: "A".to_owned(),
        desc: "Screw thread length".to_owned(),
    };
    let m2 = dim::Basic {
        nom: dim::Vec1D::new(0.032),
        tol: tol::Bilateral::symmetric(0.002),
        a: 1.0,
        name: "B".to_owned(),
        desc: "Washer Length".to_owned(),
    };
    let m3 = dim::Basic {
        nom: dim::Vec1D::new(0.06),
        tol: tol::Bilateral::symmetric(0.003),
        a: 1.0,
        name: "C".to_owned(),
        desc: "Inner bearing cap turned length".to_owned(),
    };
    let m4 = dim::Basic {
        nom: dim::Vec1D::new(0.438),
        tol: tol::Bilateral::unequal(0.0, -0.015),
        a: 1.0,
        name: "D".to_owned(),
        desc: "Bearing length".to_owned(),
    };
    let m5 = dim::Basic {
        nom: dim::Vec1D::new(0.12),
        tol: tol::Bilateral::symmetric(0.005),
        a: 1.0,
        name: "E".to_owned(),
        desc: "Spacer turned length".to_owned(),
    };
    let m6 = dim::Basic {
        nom: dim::Vec1D::new(1.5),
        tol: tol::Bilateral::unequal(0.01, -0.004),
        a: 1.0,
        name: "F".to_owned(),
        desc: "Rotor length".to_owned(),
    };
    let mut m7 = m5.clone();
    m7.name = "G".to_owned();
    let mut m8 = m4.clone();
    m8.name = "H".to_owned();
    let m9 = dim::Basic {
        nom: dim::Vec1D::new(0.450),
        tol: tol::Bilateral::symmetric(0.007),
        a: 1.0,
        name: "I".to_owned(),
        desc: "Pulley casting length".to_owned(),
    };
    let m10 = dim::Basic {
        nom: dim::Vec1D::new(-3.019),
        tol: tol::Bilateral::unequal(0.012, 0.0),
        a: 1.0,
        name: "J".to_owned(),
        desc: "Shaft turned length".to_owned(),
    };
    let m11 = dim::Basic {
        nom: dim::Vec1D::new(0.3),
        tol: tol::Bilateral::symmetric(0.03),
        a: 1.0,
        name: "K".to_owned(),
        desc: "Tapped hole depth".to_owned(),
    };
    let dims = vec![m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11];

    let stack = dim::Stack {
        name: "stacks on stacks".to_owned(),
        description: "".to_owned(),
        dims,
    };

    let wc = calc::wc(&stack);
    assert_delta!(wc.nom.x, 0.0615, EPSILON);
    assert_delta!(wc.tol.t() / 2.0, 0.0955, EPSILON);
    assert_delta!(wc.absolute_lower(), -0.034, EPSILON);
    assert_delta!(wc.absolute_upper(), 0.157, EPSILON);

    let rss = calc::rss(&stack);
    assert_delta!(rss.nom.x, 0.0615, EPSILON);
    assert_delta!(rss.tol.t() / 2.0, 0.03808, EPSILON);
    assert_delta!(rss.absolute_lower(), 0.02342, EPSILON);
    assert_delta!(rss.absolute_upper(), 0.09958, EPSILON);

    let mrss = calc::mrss(&stack);
    assert_delta!(mrss.nom.x, 0.0615, EPSILON);
    assert_delta!(mrss.tol.t() / 2.0, 0.05047, EPSILON);
    assert_delta!(mrss.absolute_lower(), 0.011, EPSILON);
    assert_delta!(mrss.absolute_upper(), 0.112, EPSILON);
}
