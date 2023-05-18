/// See http://numbbo.github.io/coco/testsuites/bbob for more info.
#[derive(Copy, Clone)]
pub enum BBOBFunctionType {
    /// Index: 1.
    Sphere,

    /// Index: 2.
    SeparableEllipsoidal,

    /// Index: 3.
    Rastrigin,

    /// Index: 4.
    BucheRastrigin,

    /// Index: 5.
    LinearSlope,

    /// Index: 6.
    AttractiveSector,

    /// Index: 7.
    StepEllipsoidal,

    /// Index: 8.
    RosenbrockFunction,

    /// Index: 9.
    RosenbrockFunctionRotated,

    /// Index: 10.
    Ellipsoidal,

    /// Index: 11.
    Discus,

    /// Index: 12.
    BentCigar,

    /// Index: 13.
    SharpRidge,

    /// Index: 14.
    DifferentPowers,

    /// Index: 15.
    RastriginMultiModal,

    /// Index: 16.
    Weierstrass,

    /// Index: 17.
    SchafferF7,

    /// Index: 18.
    SchafferF7IllConditioned,

    /// Index: 19.
    CompositeGriewankRosenbrockF8F2,

    /// Index: 20.
    Schwefel,

    /// Index: 21.
    GallagherGaussian101MePeaks,

    /// Index: 22.
    GallagherGaussian21HiPeaks,

    /// Index: 23.
    Katsuura,

    /// Index: 24.
    LunacekBiRastrigin,
}

pub const ALL_BBOB_FUNCTIONS: [BBOBFunctionType; 24] = [
    BBOBFunctionType::Sphere,
    BBOBFunctionType::SeparableEllipsoidal,
    BBOBFunctionType::Rastrigin,
    BBOBFunctionType::BucheRastrigin,
    BBOBFunctionType::LinearSlope,
    BBOBFunctionType::AttractiveSector,
    BBOBFunctionType::StepEllipsoidal,
    BBOBFunctionType::RosenbrockFunction,
    BBOBFunctionType::RosenbrockFunctionRotated,
    BBOBFunctionType::Ellipsoidal,
    BBOBFunctionType::Discus,
    BBOBFunctionType::BentCigar,
    BBOBFunctionType::SharpRidge,
    BBOBFunctionType::DifferentPowers,
    BBOBFunctionType::RastriginMultiModal,
    BBOBFunctionType::Weierstrass,
    BBOBFunctionType::SchafferF7,
    BBOBFunctionType::SchafferF7IllConditioned,
    BBOBFunctionType::CompositeGriewankRosenbrockF8F2,
    BBOBFunctionType::Schwefel,
    BBOBFunctionType::GallagherGaussian101MePeaks,
    BBOBFunctionType::GallagherGaussian21HiPeaks,
    BBOBFunctionType::Katsuura,
    BBOBFunctionType::LunacekBiRastrigin,
];

impl BBOBFunctionType {
    /// See http://numbbo.github.io/coco/testsuites/bbob for more info.
    #[allow(dead_code)]
    pub fn from_function_index(index: usize) -> Option<Self> {
        match index {
            1 => Some(BBOBFunctionType::Sphere),
            2 => Some(BBOBFunctionType::SeparableEllipsoidal),
            3 => Some(BBOBFunctionType::Rastrigin),
            4 => Some(BBOBFunctionType::BucheRastrigin),
            5 => Some(BBOBFunctionType::LinearSlope),
            6 => Some(BBOBFunctionType::AttractiveSector),
            7 => Some(BBOBFunctionType::StepEllipsoidal),
            8 => Some(BBOBFunctionType::RosenbrockFunction),
            9 => Some(BBOBFunctionType::RosenbrockFunctionRotated),
            10 => Some(BBOBFunctionType::Ellipsoidal),
            11 => Some(BBOBFunctionType::Discus),
            12 => Some(BBOBFunctionType::BentCigar),
            13 => Some(BBOBFunctionType::SharpRidge),
            14 => Some(BBOBFunctionType::DifferentPowers),
            15 => Some(BBOBFunctionType::RastriginMultiModal),
            16 => Some(BBOBFunctionType::Weierstrass),
            17 => Some(BBOBFunctionType::SchafferF7),
            18 => Some(BBOBFunctionType::SchafferF7IllConditioned),
            19 => Some(BBOBFunctionType::CompositeGriewankRosenbrockF8F2),
            20 => Some(BBOBFunctionType::Schwefel),
            21 => Some(BBOBFunctionType::GallagherGaussian101MePeaks),
            22 => Some(BBOBFunctionType::GallagherGaussian21HiPeaks),
            23 => Some(BBOBFunctionType::Katsuura),
            24 => Some(BBOBFunctionType::LunacekBiRastrigin),
            _ => None,
        }
    }

    /// See http://numbbo.github.io/coco/testsuites/bbob for more info.
    pub fn index(&self) -> usize {
        match self {
            BBOBFunctionType::Sphere => 1,
            BBOBFunctionType::SeparableEllipsoidal => 2,
            BBOBFunctionType::Rastrigin => 3,
            BBOBFunctionType::BucheRastrigin => 4,
            BBOBFunctionType::LinearSlope => 5,
            BBOBFunctionType::AttractiveSector => 6,
            BBOBFunctionType::StepEllipsoidal => 7,
            BBOBFunctionType::RosenbrockFunction => 8,
            BBOBFunctionType::RosenbrockFunctionRotated => 9,
            BBOBFunctionType::Ellipsoidal => 10,
            BBOBFunctionType::Discus => 11,
            BBOBFunctionType::BentCigar => 12,
            BBOBFunctionType::SharpRidge => 13,
            BBOBFunctionType::DifferentPowers => 14,
            BBOBFunctionType::RastriginMultiModal => 15,
            BBOBFunctionType::Weierstrass => 16,
            BBOBFunctionType::SchafferF7 => 17,
            BBOBFunctionType::SchafferF7IllConditioned => 18,
            BBOBFunctionType::CompositeGriewankRosenbrockF8F2 => 19,
            BBOBFunctionType::Schwefel => 20,
            BBOBFunctionType::GallagherGaussian101MePeaks => 21,
            BBOBFunctionType::GallagherGaussian21HiPeaks => 22,
            BBOBFunctionType::Katsuura => 23,
            BBOBFunctionType::LunacekBiRastrigin => 24,
        }
    }

    pub fn name(&self) -> String {
        match self {
            BBOBFunctionType::Sphere => "Sphere",
            BBOBFunctionType::SeparableEllipsoidal => "SeparableEllipsoidal",
            BBOBFunctionType::Rastrigin => "Rastrigin",
            BBOBFunctionType::BucheRastrigin => "BucheRastrigin",
            BBOBFunctionType::LinearSlope => "LinearSlope",
            BBOBFunctionType::AttractiveSector => "AttractiveSector",
            BBOBFunctionType::StepEllipsoidal => "StepEllipsoidal",
            BBOBFunctionType::RosenbrockFunction => "RosenbrockFunction",
            BBOBFunctionType::RosenbrockFunctionRotated => {
                "RosenbrockFunctionRotated"
            }
            BBOBFunctionType::Ellipsoidal => "Ellipsoidal",
            BBOBFunctionType::Discus => "Discus",
            BBOBFunctionType::BentCigar => "BentCigar",
            BBOBFunctionType::SharpRidge => "SharpRidge",
            BBOBFunctionType::DifferentPowers => "DifferentPowers",
            BBOBFunctionType::RastriginMultiModal => "RastriginMultiModal",
            BBOBFunctionType::Weierstrass => "Weierstrass",
            BBOBFunctionType::SchafferF7 => "SchafferF7",
            BBOBFunctionType::SchafferF7IllConditioned => {
                "SchafferF7IllConditioned"
            }
            BBOBFunctionType::CompositeGriewankRosenbrockF8F2 => {
                "CompositeGriewankRosenbrockF8F2"
            }
            BBOBFunctionType::Schwefel => "Schwefel",
            BBOBFunctionType::GallagherGaussian101MePeaks => {
                "GallagherGaussian101MePeaks"
            }
            BBOBFunctionType::GallagherGaussian21HiPeaks => {
                "GallagherGaussian21HiPeaks"
            }
            BBOBFunctionType::Katsuura => "Katsuura",
            BBOBFunctionType::LunacekBiRastrigin => "LunacekBiRastrigin",
        }
        .to_string()
    }

    /// Global minimum for the given function.
    /// Retrieved using `print(makeBBOBFunction(40, index, 2023))` in `R`.
    pub fn global_minimum(&self) -> f64 {
        match self {
            BBOBFunctionType::Sphere => 21.1000,
            BBOBFunctionType::SeparableEllipsoidal => 26.9100,
            BBOBFunctionType::Rastrigin => 311.6000,
            // (not an error, both have the same minimum)
            BBOBFunctionType::BucheRastrigin => 311.6000,
            BBOBFunctionType::LinearSlope => -48.4700,
            BBOBFunctionType::AttractiveSector => -91.3600,
            BBOBFunctionType::StepEllipsoidal => 32.4900,
            BBOBFunctionType::RosenbrockFunction => 71.6000,
            BBOBFunctionType::RosenbrockFunctionRotated => -356.7000,
            BBOBFunctionType::Ellipsoidal => 51.0300,
            BBOBFunctionType::Discus => -96.6500,
            BBOBFunctionType::BentCigar => 553.3900,
            BBOBFunctionType::SharpRidge => 9.8800,
            BBOBFunctionType::DifferentPowers => 405.4700,
            BBOBFunctionType::RastriginMultiModal => 64.2500,
            BBOBFunctionType::Weierstrass => -43.2800,
            BBOBFunctionType::SchafferF7 => 227.5100,
            BBOBFunctionType::SchafferF7IllConditioned => 227.5100,
            BBOBFunctionType::CompositeGriewankRosenbrockF8F2 => 73.0600,
            BBOBFunctionType::Schwefel => -123.8100,
            BBOBFunctionType::GallagherGaussian101MePeaks => -44.4200,
            BBOBFunctionType::GallagherGaussian21HiPeaks => 222.1000,
            BBOBFunctionType::Katsuura => -1000.0000,
            BBOBFunctionType::LunacekBiRastrigin => -1.3300,
        }
    }
}
