pub trait CubeId: PartialEq + Eq + std::hash::Hash + Clone + Copy + Send + Sync + 'static {}

impl<T> CubeId for T where T: PartialEq + Eq + std::hash::Hash + Clone + Copy + Send + Sync + 'static {}
