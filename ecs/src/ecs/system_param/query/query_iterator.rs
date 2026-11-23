use std::{collections::VecDeque, marker::PhantomData};

use crate::ecs::system_param::query::{query::Query, query_filter::QueryFilter, query_parameter::QueryParamter};

impl<'a, T: QueryParamter + 'static, F: QueryFilter> IntoIterator for Query<'a, T, F> {
    type Item = T::Item<'a>;
    type IntoIter = QueryIterator<'a, T::Item<'a>>;
    fn into_iter(self) -> Self::IntoIter {
        QueryIterator{
            value: self.value.into_iter().collect(),
            _marker: PhantomData,
        }
    }
}
impl<'a, T: QueryParamter, F: QueryFilter> Query<'a, T, F> {
    pub fn iter<'b>(&'b self) -> QueryIterator<'b, &'b T::Item<'a>> {
        QueryIterator{
            value: self.value.iter().collect(),
            _marker: PhantomData,
        }
    }
}
pub struct QueryIterator<'a, T>{
    value: VecDeque<T>,
    _marker: PhantomData<&'a T>
}
impl<'a, T> Iterator for QueryIterator<'a, T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.value.pop_front()
    }
}