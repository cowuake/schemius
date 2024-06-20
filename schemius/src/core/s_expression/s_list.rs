pub trait SList<T>
where
    T: Clone,
    Self: Sized,
{
    fn s_append(lists: &[Self]) -> Self;
    fn s_car(&self) -> Option<&T>;
    fn s_cdr(&self) -> Box<dyn Iterator<Item = &T> + '_>;
    fn s_len(&self) -> usize;
    fn s_reverse(&self) -> Self;
}

impl<T> SList<T> for Vec<T>
where
    T: Clone,
{
    fn s_append(lists: &[Self]) -> Self {
        lists.iter().flat_map(|list| list.iter().cloned()).collect()
    }

    fn s_car(&self) -> Option<&T> {
        self.first()
    }

    fn s_cdr(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter().skip(1))
    }

    fn s_len(&self) -> usize {
        self.len()
    }

    fn s_reverse(&self) -> Self {
        self.iter().rev().cloned().collect()
    }
}

#[cfg(test)]
pub mod tests_slist_vector {
    use super::*;

    #[test]
    fn test_slist_vector_append() {
        let list1 = vec![1, 2, 3];
        let list2 = vec![4, 5, 6];
        let list3 = vec![7, 8, 9];
        let lists = &[list1, list2, list3];
        let appended = SList::s_append(lists);
        assert_eq!(appended, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_slist_vector_car() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(list.s_car(), Some(&1));
    }

    #[test]
    fn test_slist_vector_cdr() {
        let list = vec![1, 2, 3, 4, 5];
        let cdr: Vec<i32> = list.s_cdr().cloned().collect();
        assert_eq!(cdr, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_slist_vector_len() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(list.s_len(), 5);
    }

    #[test]
    fn test_slist_reverse() {
        let list = vec![1, 2, 3, 4, 5];
        let reversed = list.s_reverse();
        assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
    }
}
