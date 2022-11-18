/// Precomputed sampling table for the probabilistic segment search algorithm.
///
/// Associate a lambda factor from the [Poisson distribution](https://en.wikipedia.org/wiki/Poisson_distribution) to a sampling size.
///
/// The table is precomputed for a range of segments and a range of `top` params.
///
/// TODO attach proper python code to generate the table
/// Python code to generate the table:
/// from scipy.stats import poisson
/// q = 0.999 # probability to cover full top in all segments
/// res = []
/// for s in range(2, 1000): # Number of segments
///   for n in range(100, 10000, 50): # top param
///     lmbda = n * (1/s)
///     k = poisson.ppf(q**(1/s), lmbda)
///     res.append((lmbda, int(k)))
/// res = sorted(res, key=lambda x: x[0])
///
/// with additional code to remove duplicates and values within 5% of each other.
const POISSON_DISTRIBUTION_SEARCH_SAMPLING: [(f64, usize); 121] = [
    (0.19342359767891684, 4),
    (0.398406374501992, 5),
    (0.6666666666666667, 6),
    (0.9900990099009901, 7),
    (1.3513513513513513, 8),
    (1.7543859649122806, 9),
    (2.2222222222222223, 10),
    (2.7027027027027026, 11),
    (3.125, 12),
    (3.7037037037037033, 13),
    (4.166666666666666, 14),
    (4.761904761904762, 15),
    (5.263157894736842, 16),
    (5.88235294117647, 17),
    (6.25, 18),
    (7.142857142857142, 19),
    (7.6923076923076925, 20),
    (9.090909090909092, 22),
    (10.0, 24),
    (11.538461538461538, 26),
    (12.5, 28),
    (14.285714285714285, 30),
    (15.384615384615385, 32),
    (16.666666666666668, 34),
    (18.181818181818183, 36),
    (20.0, 38),
    (21.428571428571427, 40),
    (23.076923076923077, 43),
    (25.0, 46),
    (27.777777777777775, 49),
    (30.0, 52),
    (33.33333333333333, 55),
    (33.333333333333336, 58),
    (37.5, 61),
    (39.285714285714285, 65),
    (42.857142857142854, 69),
    (45.83333333333333, 73),
    (50.0, 77),
    (50.0, 81),
    (56.25, 86),
    (60.0, 91),
    (66.66666666666666, 96),
    (70.0, 101),
    (75.0, 107),
    (78.57142857142857, 113),
    (83.33333333333333, 119),
    (90.0, 125),
    (94.44444444444444, 132),
    (100.0, 139),
    (106.25, 146),
    (111.53846153846155, 154),
    (117.6470588235294, 162),
    (125.0, 171),
    (137.5, 180),
    (143.75, 190),
    (150.0, 200),
    (162.5, 211),
    (170.83333333333331, 222),
    (183.33333333333331, 234),
    (191.66666666666666, 246),
    (200.0, 259),
    (215.0, 272),
    (230.0, 286),
    (242.85714285714283, 301),
    (257.1428571428571, 317),
    (271.4285714285714, 333),
    (285.0, 350),
    (300.0, 368),
    (318.1818181818182, 387),
    (340.0, 407),
    (360.0, 429),
    (380.0, 451),
    (400.0, 474),
    (421.4285714285714, 498),
    (443.75, 523),
    (468.1818181818182, 551),
    (494.4444444444444, 579),
    (519.2307692307693, 608),
    (550.0, 639),
    (576.6666666666666, 671),
    (614.2857142857142, 706),
    (650.0, 742),
    (678.5714285714286, 780),
    (716.6666666666666, 820),
    (759.0909090909091, 864),
    (800.0, 908),
    (844.4444444444443, 954),
    (892.8571428571428, 1003),
    (941.6666666666666, 1054),
    (991.6666666666666, 1107),
    (1043.75, 1164),
    (1100.0, 1224),
    (1175.0, 1289),
    (1233.3333333333333, 1355),
    (1291.6666666666665, 1423),
    (1364.2857142857142, 1500),
    (1440.0, 1576),
    (1516.6666666666665, 1658),
    (1600.0, 1741),
    (1687.5, 1832),
    (1780.0, 1931),
    (1875.0, 2028),
    (1975.0, 2132),
    (2083.333333333333, 2240),
    (2200.0, 2356),
    (2316.6666666666665, 2482),
    (2437.5, 2611),
    (2575.0, 2744),
    (2716.6666666666665, 2896),
    (2866.6666666666665, 3051),
    (3016.6666666666665, 3205),
    (3183.333333333333, 3377),
    (3375.0, 3568),
    (3550.0, 3748),
    (3750.0, 3953),
    (3950.0, 4158),
    (4175.0, 4389),
    (4400.0, 4620),
    (4650.0, 4876),
    (4900.0, 5132),
    (f64::MAX, usize::MAX),
];

/// Uses binary search to find the sampling size for a given lambda.
pub fn find_search_sampling_over_point_distribution(n: f64, p: f64) -> Option<usize> {
    let target_lambda = p * n;
    let k = POISSON_DISTRIBUTION_SEARCH_SAMPLING
        .binary_search_by(|&(lambda, _sampling)| lambda.partial_cmp(&target_lambda).unwrap());
    match k {
        Ok(k) => Some(POISSON_DISTRIBUTION_SEARCH_SAMPLING[k].1),
        Err(insert) => Some(POISSON_DISTRIBUTION_SEARCH_SAMPLING[insert].1),
    }
}