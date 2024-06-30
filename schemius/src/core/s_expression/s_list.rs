pub trait SList<T>
where
    T: Clone,
    Self: Sized,
{
    fn s_append(lists: &[Self]) -> Self;
    fn s_car(&self) -> Option<&T>;
    fn s_cadr(&self) -> Option<&T>;
    fn s_cdr(&self) -> Option<Self>;
    fn s_len(&self) -> usize;
    fn s_ref(&self, index: usize) -> Option<&T>;
    fn s_tail(&self, k: usize) -> Self;
    fn s_reverse(&self) -> Self;
    fn set_car(&mut self, value: T);
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

    fn s_cadr(&self) -> Option<&T> {
        self.get(1)
    }

    fn s_cdr(&self) -> Option<Self> {
        if self.is_empty() {
            return None;
        }
        Some(self.iter().skip(1).cloned().collect())
    }

    fn s_len(&self) -> usize {
        self.len()
    }

    fn s_ref(&self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn s_tail(&self, k: usize) -> Self {
        self.iter().skip(k).cloned().collect()
    }

    fn s_reverse(&self) -> Self {
        self.iter().rev().cloned().collect()
    }

    fn set_car(&mut self, value: T) {
        if let Some(first) = self.first_mut() {
            *first = value;
        }
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
    fn test_slist_vector_cadr() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(list.s_cadr(), Some(&2));
    }

    #[test]
    fn test_slist_vector_cdr() {
        let list = vec![1, 2, 3, 4, 5];
        let cdr = list.s_cdr().unwrap();
        assert_eq!(cdr, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_slist_vector_len() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(list.s_len(), 5);
    }

    #[test]
    fn test_slist_vector_ref() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(list.s_ref(2), Some(&3));
    }

    #[test]
    fn test_slist_vector_tail() {
        let list = vec![1, 2, 3, 4, 5];
        let tail = list.s_tail(2);
        assert_eq!(tail, vec![3, 4, 5]);
    }

    #[test]
    fn test_slist_reverse() {
        let list = vec![1, 2, 3, 4, 5];
        let reversed = list.s_reverse();
        assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_slist_set_car() {
        let mut list = vec![1, 2, 3, 4, 5];
        list.set_car(10);
        assert_eq!(list, vec![10, 2, 3, 4, 5]);
    }
}
