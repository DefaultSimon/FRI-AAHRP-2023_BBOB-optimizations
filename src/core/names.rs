/// See http://numbbo.github.io/coco/testsuites/bbob for more info.
#[derive(Copy, Clone)]
pub enum BBOBFunctionName {
    Sphere,
    SeparableEllipsoidal,
    Rastrigin,
    BucheRastrigin,
    LinearSlope,
    AttractiveSector,
    StepEllipsoidal,
    RosenbrockFunction,
    RosenbrockFunctionRotated,
    Ellipsoidal,
    Discus,
    BentCigar,
    SharpRidge,
    DifferentPowers,
    RastriginMultiModal,
    Weierstrass,
    SchafferF7,
    SchafferF7IllConditioned,
    CompositeGriewankRosenbrockF8F2,
    Schwefel,
    GallagherGaussian101MePeaks,
    GallagherGaussian21HiPeaks,
    Katsuura,
    LunacekBiRastrigin,
}

pub const ALL_BBOB_FUNCTION_NAMES: [BBOBFunctionName; 24] = [
    BBOBFunctionName::Sphere,
    BBOBFunctionName::SeparableEllipsoidal,
    BBOBFunctionName::Rastrigin,
    BBOBFunctionName::BucheRastrigin,
    BBOBFunctionName::LinearSlope,
    BBOBFunctionName::AttractiveSector,
    BBOBFunctionName::StepEllipsoidal,
    BBOBFunctionName::RosenbrockFunction,
    BBOBFunctionName::RosenbrockFunctionRotated,
    BBOBFunctionName::Ellipsoidal,
    BBOBFunctionName::Discus,
    BBOBFunctionName::BentCigar,
    BBOBFunctionName::SharpRidge,
    BBOBFunctionName::DifferentPowers,
    BBOBFunctionName::RastriginMultiModal,
    BBOBFunctionName::Weierstrass,
    BBOBFunctionName::SchafferF7,
    BBOBFunctionName::SchafferF7IllConditioned,
    BBOBFunctionName::CompositeGriewankRosenbrockF8F2,
    BBOBFunctionName::Schwefel,
    BBOBFunctionName::GallagherGaussian101MePeaks,
    BBOBFunctionName::GallagherGaussian21HiPeaks,
    BBOBFunctionName::Katsuura,
    BBOBFunctionName::LunacekBiRastrigin,
];

impl BBOBFunctionName {
    /// See http://numbbo.github.io/coco/testsuites/bbob for more info.
    #[allow(dead_code)]
    pub fn from_function_index(index: usize) -> Option<Self> {
        match index {
            1 => Some(BBOBFunctionName::Sphere),
            2 => Some(BBOBFunctionName::SeparableEllipsoidal),
            3 => Some(BBOBFunctionName::Rastrigin),
            4 => Some(BBOBFunctionName::BucheRastrigin),
            5 => Some(BBOBFunctionName::LinearSlope),
            6 => Some(BBOBFunctionName::AttractiveSector),
            7 => Some(BBOBFunctionName::StepEllipsoidal),
            8 => Some(BBOBFunctionName::RosenbrockFunction),
            9 => Some(BBOBFunctionName::RosenbrockFunctionRotated),
            10 => Some(BBOBFunctionName::Ellipsoidal),
            11 => Some(BBOBFunctionName::Discus),
            12 => Some(BBOBFunctionName::BentCigar),
            13 => Some(BBOBFunctionName::SharpRidge),
            14 => Some(BBOBFunctionName::DifferentPowers),
            15 => Some(BBOBFunctionName::RastriginMultiModal),
            16 => Some(BBOBFunctionName::Weierstrass),
            17 => Some(BBOBFunctionName::SchafferF7),
            18 => Some(BBOBFunctionName::SchafferF7IllConditioned),
            19 => Some(BBOBFunctionName::CompositeGriewankRosenbrockF8F2),
            20 => Some(BBOBFunctionName::Schwefel),
            21 => Some(BBOBFunctionName::GallagherGaussian101MePeaks),
            22 => Some(BBOBFunctionName::GallagherGaussian21HiPeaks),
            23 => Some(BBOBFunctionName::Katsuura),
            24 => Some(BBOBFunctionName::LunacekBiRastrigin),
            _ => None,
        }
    }

    /// See http://numbbo.github.io/coco/testsuites/bbob for more info.
    pub fn to_function_index(&self) -> usize {
        match self {
            BBOBFunctionName::Sphere => 1,
            BBOBFunctionName::SeparableEllipsoidal => 2,
            BBOBFunctionName::Rastrigin => 3,
            BBOBFunctionName::BucheRastrigin => 4,
            BBOBFunctionName::LinearSlope => 5,
            BBOBFunctionName::AttractiveSector => 6,
            BBOBFunctionName::StepEllipsoidal => 7,
            BBOBFunctionName::RosenbrockFunction => 8,
            BBOBFunctionName::RosenbrockFunctionRotated => 9,
            BBOBFunctionName::Ellipsoidal => 10,
            BBOBFunctionName::Discus => 11,
            BBOBFunctionName::BentCigar => 12,
            BBOBFunctionName::SharpRidge => 13,
            BBOBFunctionName::DifferentPowers => 14,
            BBOBFunctionName::RastriginMultiModal => 15,
            BBOBFunctionName::Weierstrass => 16,
            BBOBFunctionName::SchafferF7 => 17,
            BBOBFunctionName::SchafferF7IllConditioned => 18,
            BBOBFunctionName::CompositeGriewankRosenbrockF8F2 => 19,
            BBOBFunctionName::Schwefel => 20,
            BBOBFunctionName::GallagherGaussian101MePeaks => 21,
            BBOBFunctionName::GallagherGaussian21HiPeaks => 22,
            BBOBFunctionName::Katsuura => 23,
            BBOBFunctionName::LunacekBiRastrigin => 24,
        }
    }

    pub fn to_function_name(&self) -> String {
        match self {
            BBOBFunctionName::Sphere => "Sphere",
            BBOBFunctionName::SeparableEllipsoidal => "SeparableEllipsoidal",
            BBOBFunctionName::Rastrigin => "Rastrigin",
            BBOBFunctionName::BucheRastrigin => "BucheRastrigin",
            BBOBFunctionName::LinearSlope => "LinearSlope",
            BBOBFunctionName::AttractiveSector => "AttractiveSector",
            BBOBFunctionName::StepEllipsoidal => "StepEllipsoidal",
            BBOBFunctionName::RosenbrockFunction => "RosenbrockFunction",
            BBOBFunctionName::RosenbrockFunctionRotated => {
                "RosenbrockFunctionRotated"
            }
            BBOBFunctionName::Ellipsoidal => "Ellipsoidal",
            BBOBFunctionName::Discus => "Discus",
            BBOBFunctionName::BentCigar => "BentCigar",
            BBOBFunctionName::SharpRidge => "SharpRidge",
            BBOBFunctionName::DifferentPowers => "DifferentPowers",
            BBOBFunctionName::RastriginMultiModal => "RastriginMultiModal",
            BBOBFunctionName::Weierstrass => "Weierstrass",
            BBOBFunctionName::SchafferF7 => "SchafferF7",
            BBOBFunctionName::SchafferF7IllConditioned => {
                "SchafferF7IllConditioned"
            }
            BBOBFunctionName::CompositeGriewankRosenbrockF8F2 => {
                "CompositeGriewankRosenbrockF8F2"
            }
            BBOBFunctionName::Schwefel => "Schwefel",
            BBOBFunctionName::GallagherGaussian101MePeaks => {
                "GallagherGaussian101MePeaks"
            }
            BBOBFunctionName::GallagherGaussian21HiPeaks => {
                "GallagherGaussian21HiPeaks"
            }
            BBOBFunctionName::Katsuura => "Katsuura",
            BBOBFunctionName::LunacekBiRastrigin => "LunacekBiRastrigin",
        }
        .to_string()
    }
}
