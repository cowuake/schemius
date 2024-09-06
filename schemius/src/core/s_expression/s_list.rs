use std::collections::LinkedList;

pub trait SList<T>
where
    T: Clone,
    Self: Sized + IntoIterator<Item = T> + FromIterator<T>,
{
    fn new() -> impl SList<T> {
        Self::from_iter(std::iter::empty())
    }
    fn s_append(lists: &[Self]) -> impl SList<T>;
    fn s_car(&self) -> Option<&T> {
        self.s_ref(0)
    }
    fn s_cadr(&self) -> Option<&T>;
    fn s_cdr(&self) -> Option<impl SList<T>>;
    fn s_len(&self) -> usize;
    fn s_ref(&self, index: usize) -> Option<&T>;
    fn s_splice(&self, insert: Self, start: usize, end: usize) -> impl SList<T>;
    fn s_tail(&self, k: usize) -> impl SList<T>;
    fn s_reverse(&self) -> impl SList<T>;
    fn set_car(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
    fn last(&self) -> Option<&T> {
        self.s_ref(self.s_len() - 1)
    }
    fn extract_range(self, start: usize, end: usize) -> impl SList<T> {
        let mut result = Self::new();
        self.into_iter().skip(start).take(end - start).for_each(|item| {
            result.push(item);
        });

        result
    }
}

impl<T> SList<T> for Vec<T>
where
    T: Clone,
{
    fn s_append(lists: &[Self]) -> impl SList<T> {
        lists.iter().flat_map(|list| list.iter().cloned()).collect::<Vec<T>>()
    }

    fn s_car(&self) -> Option<&T> {
        self.first()
    }

    fn s_cadr(&self) -> Option<&T> {
        self.get(1)
    }

    fn s_cdr(&self) -> Option<impl SList<T>> {
        if self.is_empty() {
            return None;
        }
        Some(self.iter().skip(1).cloned().collect::<Vec<T>>())
    }

    fn s_len(&self) -> usize {
        self.len()
    }

    fn s_ref(&self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn s_splice(&self, insert: Self, start: usize, end: usize) -> impl SList<T> {
        let mut result = self.clone();
        result.splice(start..end, insert);
        result
    }

    fn s_tail(&self, k: usize) -> impl SList<T> {
        self.iter().skip(k).cloned().collect::<Vec<T>>()
    }

    fn s_reverse(&self) -> impl SList<T> {
        self.iter().rev().cloned().collect::<Vec<T>>()
    }

    fn set_car(&mut self, value: T) {
        if let Some(first) = self.first_mut() {
            *first = value;
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn push(&mut self, value: T) {
        self.push(value);
    }
}

impl<T> SList<T> for LinkedList<T>
where
    T: Clone,
{
    fn s_append(lists: &[Self]) -> impl SList<T> {
        lists.iter().flat_map(|list| list.iter().cloned()).collect::<LinkedList<T>>()
    }

    fn s_car(&self) -> Option<&T> {
        self.front()
    }

    fn s_cadr(&self) -> Option<&T> {
        self.iter().nth(1)
    }

    fn s_cdr(&self) -> Option<impl SList<T>> {
        if self.is_empty() {
            return None;
        }
        let mut cdr = self.clone();
        cdr.pop_front();
        Some(cdr)
    }

    fn s_len(&self) -> usize {
        self.len()
    }

    fn s_ref(&self, index: usize) -> Option<&T> {
        self.iter().nth(index)
    }

    fn s_splice(&self, insert: Self, start: usize, end: usize) -> impl SList<T> {
        let mut head = self.clone();
        let mut tail = head.split_off(start);
        let tail_end = tail.split_off(end - start);

        head.extend(insert);
        head.extend(tail_end);

        head
    }

    fn s_tail(&self, k: usize) -> impl SList<T> {
        self.iter().skip(k).cloned().collect::<LinkedList<T>>()
    }

    fn s_reverse(&self) -> impl SList<T> {
        self.iter().rev().cloned().collect::<LinkedList<T>>()
    }

    fn set_car(&mut self, value: T) {
        if let Some(first) = self.front_mut() {
            *first = value;
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }

    fn push(&mut self, value: T) {
        self.push_back(value);
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
        let lists = &[list1, list2, list3].into_iter();
        let appended = SList::s_append(lists).into_iter();
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
    fn test_slist_vector_splice() {
        let list = vec![1, 2, 3, 4, 5];
        let insert1 = vec![10, 11, 12];
        let insert2 = vec![20, 21, 22];

        let spliced1 = list.s_splice(insert1, 2, 2).into_iter();
        let spliced2 = list.s_splice(insert2, 1, 4).into_iter();

        assert_eq!(spliced1, vec![1, 2, 10, 11, 12, 3, 4, 5].into_iter());
        assert_eq!(spliced2, vec![1, 20, 21, 22, 5].into_iter());
    }

    #[test]
    fn test_slist_vector_tail() {
        let list = vec![1, 2, 3, 4, 5];
        let tail = list.s_tail(2);
        assert_eq!(tail, vec![3, 4, 5]);
    }

    #[test]
    fn test_slist_vector_reverse() {
        let list = vec![1, 2, 3, 4, 5];
        let reversed = list.s_reverse();
        assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_slist_vector_set_car() {
        let mut list = vec![1, 2, 3, 4, 5];
        list.set_car(10);
        assert_eq!(list, vec![10, 2, 3, 4, 5]);
    }

    #[test]
    fn test_slist_vector_pop() {
        let mut list = vec![1, 2, 3, 4, 5];
        let popped = list.pop();
        assert_eq!(list.len(), 4);
        assert_eq!(popped, Some(5));
    }

    #[test]
    fn test_slist_vector_push() {
        let mut list = vec![1, 2, 3, 4, 5];
        list.push(10);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 10]);
    }

    #[test]
    fn test_slist_vector_last() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(list.last(), Some(&5));
    }

    #[test]
    fn test_slist_vector_extract_range() {
        let list = vec![1, 2, 3, 4, 5];
        let extracted = list.extract_range(1, 4);
        assert_eq!(extracted, &[2, 3, 4]);
    }
}

#[cfg(test)]
pub mod test_slist_linked_list {
    use super::*;

    #[test]
    fn test_slist_linked_list_append() {
        let mut list1 = LinkedList::new();
        list1.push_back(1);
        list1.push_back(2);
        list1.push_back(3);

        let mut list2 = LinkedList::new();
        list2.push_back(4);
        list2.push_back(5);
        list2.push_back(6);

        let mut list3 = LinkedList::new();
        list3.push_back(7);
        list3.push_back(8);
        list3.push_back(9);

        let lists = &[list1, list2, list3];
        let appended = SList::s_append(lists);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(appended.iter().cloned().collect::<Vec<i32>>(), expected);
    }

    #[test]
    fn test_slist_linked_list_car() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.s_car(), Some(&1));
    }

    #[test]
    fn test_slist_linked_list_cadr() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.s_cadr(), Some(&2));
    }

    #[test]
    fn test_slist_linked_list_cdr() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let cdr = list.s_cdr().unwrap();
        let expected = vec![2, 3];
        assert_eq!(cdr.iter().cloned().collect::<Vec<i32>>(), expected);
    }

    #[test]
    fn test_slist_linked_list_len() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.s_len(), 3);
    }

    #[test]
    fn test_slist_linked_list_ref() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.s_ref(2), Some(&3));
    }

    #[test]
    fn test_slist_linked_list_splice() {
        let list = LinkedList::from_iter([1, 2, 3, 4, 5]);
        let insert1 = LinkedList::from_iter([10, 11, 12]);
        let insert2 = LinkedList::from_iter([20, 21, 22]);

        let spliced1 = list.s_splice(insert1, 2, 2);
        let spliced2 = list.s_splice(insert2, 1, 4);

        assert_eq!(spliced1, LinkedList::from_iter([1, 2, 10, 11, 12, 3, 4, 5]));
        assert_eq!(spliced2, LinkedList::from_iter([1, 20, 21, 22, 5]));
    }

    #[test]
    fn test_slist_linked_list_tail() {
        let list = LinkedList::from_iter([1, 2, 3, 4, 5]);
        let tail = list.s_tail(2);
        assert_eq!(tail, LinkedList::from_iter([3, 4, 5]));
    }

    #[test]
    fn test_slist_linked_list_pop() {
        let mut list = LinkedList::from_iter([1, 2, 3, 4, 5]);
        let popped = list.pop();
        assert_eq!(list.len(), 4);
        assert_eq!(popped, Some(1));
    }

    #[test]
    fn test_slist_linked_list_push() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push(4);
        let expected = vec![1, 2, 3, 4];
        assert_eq!(list.iter().cloned().collect::<Vec<i32>>(), expected);
    }

    #[test]
    fn test_slist_linked_list_last() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.last(), Some(&3));
    }

    #[test]
    fn test_slist_linked_list_extract_range() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        let extracted = list.extract_range(1, 4);
        let expected = vec![2, 3, 4];
        assert_eq!(extracted.iter().cloned().collect::<Vec<i32>>(), expected);
    }
}
