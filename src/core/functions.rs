/// See http://numbbo.github.io/coco/testsuites/bbob for more info.
#[derive(Copy, Clone)]
pub enum BBOBFunction {
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

pub const ALL_BBOB_FUNCTIONS: [BBOBFunction; 24] = [
    BBOBFunction::Sphere,
    BBOBFunction::SeparableEllipsoidal,
    BBOBFunction::Rastrigin,
    BBOBFunction::BucheRastrigin,
    BBOBFunction::LinearSlope,
    BBOBFunction::AttractiveSector,
    BBOBFunction::StepEllipsoidal,
    BBOBFunction::RosenbrockFunction,
    BBOBFunction::RosenbrockFunctionRotated,
    BBOBFunction::Ellipsoidal,
    BBOBFunction::Discus,
    BBOBFunction::BentCigar,
    BBOBFunction::SharpRidge,
    BBOBFunction::DifferentPowers,
    BBOBFunction::RastriginMultiModal,
    BBOBFunction::Weierstrass,
    BBOBFunction::SchafferF7,
    BBOBFunction::SchafferF7IllConditioned,
    BBOBFunction::CompositeGriewankRosenbrockF8F2,
    BBOBFunction::Schwefel,
    BBOBFunction::GallagherGaussian101MePeaks,
    BBOBFunction::GallagherGaussian21HiPeaks,
    BBOBFunction::Katsuura,
    BBOBFunction::LunacekBiRastrigin,
];

impl BBOBFunction {
    /// See http://numbbo.github.io/coco/testsuites/bbob for more info.
    #[allow(dead_code)]
    pub fn from_function_index(index: usize) -> Option<Self> {
        match index {
            1 => Some(BBOBFunction::Sphere),
            2 => Some(BBOBFunction::SeparableEllipsoidal),
            3 => Some(BBOBFunction::Rastrigin),
            4 => Some(BBOBFunction::BucheRastrigin),
            5 => Some(BBOBFunction::LinearSlope),
            6 => Some(BBOBFunction::AttractiveSector),
            7 => Some(BBOBFunction::StepEllipsoidal),
            8 => Some(BBOBFunction::RosenbrockFunction),
            9 => Some(BBOBFunction::RosenbrockFunctionRotated),
            10 => Some(BBOBFunction::Ellipsoidal),
            11 => Some(BBOBFunction::Discus),
            12 => Some(BBOBFunction::BentCigar),
            13 => Some(BBOBFunction::SharpRidge),
            14 => Some(BBOBFunction::DifferentPowers),
            15 => Some(BBOBFunction::RastriginMultiModal),
            16 => Some(BBOBFunction::Weierstrass),
            17 => Some(BBOBFunction::SchafferF7),
            18 => Some(BBOBFunction::SchafferF7IllConditioned),
            19 => Some(BBOBFunction::CompositeGriewankRosenbrockF8F2),
            20 => Some(BBOBFunction::Schwefel),
            21 => Some(BBOBFunction::GallagherGaussian101MePeaks),
            22 => Some(BBOBFunction::GallagherGaussian21HiPeaks),
            23 => Some(BBOBFunction::Katsuura),
            24 => Some(BBOBFunction::LunacekBiRastrigin),
            _ => None,
        }
    }

    /// See http://numbbo.github.io/coco/testsuites/bbob for more info.
    pub fn index(&self) -> usize {
        match self {
            BBOBFunction::Sphere => 1,
            BBOBFunction::SeparableEllipsoidal => 2,
            BBOBFunction::Rastrigin => 3,
            BBOBFunction::BucheRastrigin => 4,
            BBOBFunction::LinearSlope => 5,
            BBOBFunction::AttractiveSector => 6,
            BBOBFunction::StepEllipsoidal => 7,
            BBOBFunction::RosenbrockFunction => 8,
            BBOBFunction::RosenbrockFunctionRotated => 9,
            BBOBFunction::Ellipsoidal => 10,
            BBOBFunction::Discus => 11,
            BBOBFunction::BentCigar => 12,
            BBOBFunction::SharpRidge => 13,
            BBOBFunction::DifferentPowers => 14,
            BBOBFunction::RastriginMultiModal => 15,
            BBOBFunction::Weierstrass => 16,
            BBOBFunction::SchafferF7 => 17,
            BBOBFunction::SchafferF7IllConditioned => 18,
            BBOBFunction::CompositeGriewankRosenbrockF8F2 => 19,
            BBOBFunction::Schwefel => 20,
            BBOBFunction::GallagherGaussian101MePeaks => 21,
            BBOBFunction::GallagherGaussian21HiPeaks => 22,
            BBOBFunction::Katsuura => 23,
            BBOBFunction::LunacekBiRastrigin => 24,
        }
    }

    pub fn name(&self) -> String {
        match self {
            BBOBFunction::Sphere => "Sphere",
            BBOBFunction::SeparableEllipsoidal => "SeparableEllipsoidal",
            BBOBFunction::Rastrigin => "Rastrigin",
            BBOBFunction::BucheRastrigin => "BucheRastrigin",
            BBOBFunction::LinearSlope => "LinearSlope",
            BBOBFunction::AttractiveSector => "AttractiveSector",
            BBOBFunction::StepEllipsoidal => "StepEllipsoidal",
            BBOBFunction::RosenbrockFunction => "RosenbrockFunction",
            BBOBFunction::RosenbrockFunctionRotated => {
                "RosenbrockFunctionRotated"
            }
            BBOBFunction::Ellipsoidal => "Ellipsoidal",
            BBOBFunction::Discus => "Discus",
            BBOBFunction::BentCigar => "BentCigar",
            BBOBFunction::SharpRidge => "SharpRidge",
            BBOBFunction::DifferentPowers => "DifferentPowers",
            BBOBFunction::RastriginMultiModal => "RastriginMultiModal",
            BBOBFunction::Weierstrass => "Weierstrass",
            BBOBFunction::SchafferF7 => "SchafferF7",
            BBOBFunction::SchafferF7IllConditioned => "SchafferF7IllConditioned",
            BBOBFunction::CompositeGriewankRosenbrockF8F2 => {
                "CompositeGriewankRosenbrockF8F2"
            }
            BBOBFunction::Schwefel => "Schwefel",
            BBOBFunction::GallagherGaussian101MePeaks => {
                "GallagherGaussian101MePeaks"
            }
            BBOBFunction::GallagherGaussian21HiPeaks => {
                "GallagherGaussian21HiPeaks"
            }
            BBOBFunction::Katsuura => "Katsuura",
            BBOBFunction::LunacekBiRastrigin => "LunacekBiRastrigin",
        }
        .to_string()
    }

    /// Global minimum for the given function.
    /// Retrieved using `print(makeBBOBFunction(40, index, 2023))` in `R`.
    pub fn global_minimum(&self) -> f64 {
        match self {
            BBOBFunction::Sphere => 21.1000,
            BBOBFunction::SeparableEllipsoidal => 26.9100,
            BBOBFunction::Rastrigin => 311.6000,
            // (not an error, both have the same minimum)
            BBOBFunction::BucheRastrigin => 311.6000,
            BBOBFunction::LinearSlope => -48.4700,
            BBOBFunction::AttractiveSector => -91.3600,
            BBOBFunction::StepEllipsoidal => 32.4900,
            BBOBFunction::RosenbrockFunction => 71.6000,
            BBOBFunction::RosenbrockFunctionRotated => -356.7000,
            BBOBFunction::Ellipsoidal => 51.0300,
            BBOBFunction::Discus => -96.6500,
            BBOBFunction::BentCigar => 553.3900,
            BBOBFunction::SharpRidge => 9.8800,
            BBOBFunction::DifferentPowers => 405.4700,
            BBOBFunction::RastriginMultiModal => 64.2500,
            BBOBFunction::Weierstrass => -43.2800,
            BBOBFunction::SchafferF7 => 227.5100,
            BBOBFunction::SchafferF7IllConditioned => 227.5100,
            BBOBFunction::CompositeGriewankRosenbrockF8F2 => 73.0600,
            BBOBFunction::Schwefel => -123.8100,
            BBOBFunction::GallagherGaussian101MePeaks => -44.4200,
            BBOBFunction::GallagherGaussian21HiPeaks => 222.1000,
            BBOBFunction::Katsuura => -1000.0000,
            BBOBFunction::LunacekBiRastrigin => -1.3300,
        }
    }
}
