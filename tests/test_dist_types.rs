use dimstack_rs::{dim, dist, tol};

#[test]
fn test_positive_dist() {
    let measurements = vec![1.1, 0.9];
    let measurements_dist = dist::Normal::fit(&measurements);
    let tol = tol::Bilateral::symmetric(0.3);
    let reviewed_dim = dim::Reviewed {
        dim: dim::Basic {
            nom: dim::Vec1D::new(1.0),
            tol,
            a: 0.0,
            name: "1".to_owned(),
            desc: "".to_owned(),
        },
        target_process_sigma: 0.0,
        distribution: Some(dist::Distribution::Normal(measurements_dist)),
    };

    assert_eq!(reviewed_dim.dim.nom.x, 1.0);
    assert_eq!(reviewed_dim.dim.absolute_lower(), 0.7);
    assert_eq!(reviewed_dim.dim.absolute_upper(), 1.3);

    match reviewed_dim.distribution {
        Some(dist::Distribution::Normal(ref d)) => assert_eq!(d.mean, 1.0),
        _ => panic!("Expected Normal distribution"),
    }

    assert!((reviewed_dim.yield_probability() - 0.9973).abs() < 0.0001);
}

#[test]
fn test_negative_dist() {
    let measurements = vec![-1.1, -0.9];
    let measurements_dist = dist::Normal::fit(&measurements);
    let reviewed_dim = dim::Reviewed {
        dim: dim::Basic {
            nom: dim::Vec1D::new(-1.0),
            tol: tol::Bilateral::symmetric(0.3),
            a: 1.0,
            name: "1".to_owned(),
            desc: "".to_owned(),
        },
        target_process_sigma: 0.0,
        distribution: Some(dist::Distribution::Normal(measurements_dist)),
    };

    assert_eq!(reviewed_dim.dim.nom.x, -1.0);
    assert_eq!(reviewed_dim.dim.absolute_lower(), -1.3);
    assert_eq!(reviewed_dim.dim.absolute_upper(), -0.7);

    match reviewed_dim.distribution {
        Some(dist::Distribution::Normal(ref d)) => assert_eq!(d.mean, -1.0),
        _ => panic!("Expected Normal distribution"),
    }

    assert!((reviewed_dim.yield_probability() - 0.9973).abs() < 0.0001);
}
