use std::{collections::HashSet, marker::PhantomData};

use crate::{ecs::{component::components::Components, system_param::{query::{query_filter::QueryFilter, query_parameter::QueryParamter}, system_param::SystemParameter}}, prelude::*};

#[derive(Clone)]
pub struct Query<
    'a,
    T,
    F = (),
> where
    T: QueryParamter + 'static,
    F: QueryFilter
{
    pub(crate) value: Vec<T::Item<'a>>,
    _a: PhantomData<(&'a T, F)>,
}

impl<
    'res,
    T: QueryParamter + 'static,
    F: QueryFilter
> SystemParameter for Query<'res, T, F> {
    type Item<'new> = Query<'new, T, F>;
    fn retrieve<'r>(components: &'r Components) -> Self::Item<'r> {
        Query {
            value: components
                .iter()
                .map(|((e,_),_)|*e)
                .collect::<HashSet<EntityId>>()
                .into_iter()
                .filter(|e|F::filter(*e, components))
                .filter_map(|e|T::retrieve(e, components).map(|r|r))
                .collect(),
            _a: PhantomData,
        }
    }
}