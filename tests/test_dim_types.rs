use dimstack_rs::{dim, tol};

// TODO: move these to unit tests in dimstack_rs::tol?

#[test]
fn test_positive_unequal_bilateral() {
    let t = tol::Bilateral::unequal(0.005, -0.004);
    assert_eq!(t.upper, 0.005);
    assert_eq!(t.lower, -0.004);
    let d = dim::Basic {
        nom: dim::Vec1D::new(1.0),
        tol: t,
        a: 0.0,
        name: "a".to_string(),
        desc: "".to_string(),
    };
    assert_eq!(d.absolute_upper(), 1.005);
    assert_eq!(d.absolute_lower(), 0.996);
}

#[test]
fn test_negative_unequal_bilateral() {
    let t = tol::Bilateral::unequal(0.005, -0.004);
    assert_eq!(t.upper, 0.005);
    assert_eq!(t.lower, -0.004);
    let d = dim::Basic {
        nom: dim::Vec1D::new(-1.0),
        tol: t,
        a: 0.0,
        name: "a".to_string(),
        desc: "".to_string(),
    };
    assert_eq!(d.nom.x.abs(), 1.0);
    assert_eq!(d.nom.x, -1.0);

    assert_eq!(d.absolute_upper_tol(), 0.004);
    assert_eq!(d.absolute_lower_tol(), -0.005);

    assert_eq!(d.absolute_upper(), -0.996);
    assert_eq!(d.absolute_lower(), -1.005);
}
