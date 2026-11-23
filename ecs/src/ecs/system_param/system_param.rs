use super::super::component::components::Components;

pub(crate) trait SystemParameter {
    type Item<'new>;
    fn retrieve<'r>(resources: &'r Components) -> Self::Item<'r>;
}

impl<T1: SystemParameter, T2: SystemParameter> SystemParameter for (T1, T2) {
    type Item<'new> = (T1::Item<'new>, T2::Item<'new>);

    fn retrieve<'r>(resources: &'r Components) -> Self::Item<'r> {
        (
            T1::retrieve(resources),
            T2::retrieve(resources),
        )
    }
}