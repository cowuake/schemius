pub trait SList<T>
where
    T: Clone,
{
    fn s_car(&self) -> Option<&T>;
    fn s_cdr(&self) -> Box<dyn Iterator<Item = &T> + '_>;
    fn s_len(&self) -> usize;
    fn s_reverse(&self) -> Self;
}

impl<T> SList<T> for Vec<T>
where
    T: Clone,
{
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
