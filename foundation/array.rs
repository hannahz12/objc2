use std::cmp::Ordering;
use std::marker::ContravariantLifetime;
use std::ops::Index;

use objc::runtime::Object;
use objc::{Id, IdSlice, IntoIdVector, Owned, Ownership, Shared, ShareId};

use {INSCopying, INSMutableCopying, INSObject};

#[repr(isize)]
#[derive(Copy)]
pub enum NSComparisonResult {
    Ascending  = -1,
    Same       = 0,
    Descending = 1,
}

impl NSComparisonResult {
    pub fn from_ordering(order: Ordering) -> NSComparisonResult {
        match order {
            Ordering::Less => NSComparisonResult::Ascending,
            Ordering::Equal => NSComparisonResult::Same,
            Ordering::Greater => NSComparisonResult::Descending,
        }
    }

    pub fn as_ordering(&self) -> Ordering {
        match *self {
            NSComparisonResult::Ascending => Ordering::Less,
            NSComparisonResult::Same => Ordering::Equal,
            NSComparisonResult::Descending => Ordering::Greater,
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct NSRange {
    pub location: usize,
    pub length: usize,
}

pub struct NSEnumerator<'a, T> {
    id: Id<Object>,
    marker: ContravariantLifetime<'a>,
}

impl<'a, T: INSObject> NSEnumerator<'a, T> {
    pub unsafe fn from_ptr(ptr: *mut Object) -> NSEnumerator<'a, T> {
        NSEnumerator { id: Id::from_ptr(ptr), marker: ContravariantLifetime }
    }
}

impl<'a, T: INSObject> Iterator for NSEnumerator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            let obj: *mut T = msg_send![self.id, nextObject];
            obj.as_ref()
        }
    }
}

pub trait INSArray : INSObject {
    type Item: INSObject;
    type Own: Ownership;

    fn count(&self) -> usize {
        unsafe {
            msg_send![self, count]
        }
    }

    fn object_at(&self, index: usize) -> &Self::Item {
        unsafe {
            let obj: *const Self::Item = msg_send![self, objectAtIndex:index];
            &*obj
        }
    }

    fn first_object(&self) -> Option<&Self::Item> {
        unsafe {
            let obj: *const Self::Item = msg_send![self, firstObject];
            obj.as_ref()
        }
    }

    fn last_object(&self) -> Option<&Self::Item> {
        unsafe {
            let obj: *const Self::Item = msg_send![self, lastObject];
            obj.as_ref()
        }
    }

    fn object_enumerator(&self) -> NSEnumerator<Self::Item> {
        unsafe {
            let result: *mut Object = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    unsafe fn from_refs(refs: &[&Self::Item]) -> Id<Self> {
        let cls = <Self as INSObject>::class();
        let obj: *mut Self = msg_send![cls, alloc];
        let obj: *mut Self = msg_send![obj, initWithObjects:refs.as_ptr()
                                                      count:refs.len()];
        Id::from_retained_ptr(obj)
    }

    fn from_vec(vec: Vec<Id<Self::Item, Self::Own>>) -> Id<Self> {
        let refs = vec.as_refs_slice();
        unsafe {
            INSArray::from_refs(refs)
        }
    }

    fn objects_in_range(&self, start: usize, len: usize) -> Vec<&Self::Item> {
        let mut vec: Vec<&Self::Item> = Vec::with_capacity(len);
        let range = NSRange { location: start, length: len };
        unsafe {
            let _: () = msg_send![self, getObjects:vec.as_ptr() range:range];
            vec.set_len(len);
        }
        vec
    }

    fn to_vec(&self) -> Vec<&Self::Item> {
        self.objects_in_range(0, self.count())
    }

    fn into_vec(array: Id<Self>) -> Vec<Id<Self::Item, Self::Own>> {
        let vec = array.to_vec();
        unsafe {
            vec.into_id_vec()
        }
    }
}

pub trait INSOwnedArray : INSArray<Own=Owned> {
    fn mut_object_at(&mut self, index: usize) -> &mut Self::Item {
        unsafe {
            let result: *mut Self::Item = msg_send![self, objectAtIndex:index];
            &mut *result
        }
    }
}

pub trait INSSharedArray : INSArray<Own=Shared> {
    fn shared_object_at(&self, index: usize) -> ShareId<Self::Item> {
        let obj = self.object_at(index);
        unsafe {
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        }
    }

    fn from_slice(slice: &[ShareId<Self::Item>]) -> Id<Self> {
        let refs = slice.as_refs_slice();
        unsafe {
            INSArray::from_refs(refs)
        }
    }

    fn to_shared_vec(&self) -> Vec<ShareId<Self::Item>> {
        let vec = self.to_vec();
        unsafe {
            vec.into_id_vec()
        }
    }
}

#[allow(missing_copy_implementations)]
pub enum NSArray<T, O = Owned> { }

object_impl!(NSArray<T, O>);

impl<T: INSObject, O: Ownership> INSArray for NSArray<T, O> {
    type Item = T;
    type Own = O;
}

impl<T: INSObject> INSOwnedArray for NSArray<T, Owned> { }

impl<T: INSObject> INSSharedArray for NSArray<T, Shared> { }

impl<T> INSCopying for NSArray<T, Shared> {
    type Output = NSSharedArray<T>;
}

impl<T> INSMutableCopying for NSArray<T, Shared> {
    type Output = NSMutableSharedArray<T>;
}

impl<T: INSObject, O: Ownership> Index<usize> for NSArray<T, O> {
    type Output = T;

    fn index(&self, index: &usize) -> &T {
        self.object_at(*index)
    }
}

pub type NSSharedArray<T> = NSArray<T, Shared>;

pub trait INSMutableArray : INSArray {
    fn add_object(&mut self, obj: Id<Self::Item, Self::Own>) {
        unsafe {
            let _: () = msg_send![self, addObject:obj];
        }
    }

    fn insert_object_at(&mut self, index: usize, obj: Id<Self::Item, Self::Own>) {
        unsafe {
            let _: () = msg_send![self, insertObject:obj atIndex:index];
        }
    }

    fn replace_object_at(&mut self, index: usize, obj: Id<Self::Item, Self::Own>) ->
            Id<Self::Item, Self::Own> {
        let old_obj = unsafe {
            let obj = self.object_at(index);
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        };
        unsafe {
            let _: () = msg_send![self, replaceObjectAtIndex:index
                                                  withObject:obj];
        }
        old_obj
    }

    fn remove_object_at(&mut self, index: usize) -> Id<Self::Item, Self::Own> {
        let obj = unsafe {
            let obj = self.object_at(index);
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        };
        unsafe {
            let _: () = msg_send![self, removeObjectAtIndex:index];
        }
        obj
    }

    fn remove_last_object(&mut self) -> Id<Self::Item, Self::Own> {
        let obj = self.last_object().map(|obj| unsafe {
            Id::from_ptr(obj as *const _ as *mut Self::Item)
        });
        unsafe {
            let _: () = msg_send![self, removeLastObject];
        }
        // removeLastObject would have failed if the array is empty,
        // so we know this won't be None
        obj.unwrap()
    }

    fn remove_all_objects(&mut self) {
        unsafe {
            let _: () = msg_send![self, removeAllObjects];
        }
    }

    fn sort_by<F: FnMut(&Self::Item, &Self::Item) -> Ordering>(&mut self, compare: F) {
        extern fn compare_with_closure<T, F: FnMut(&T, &T) -> Ordering>(
                obj1: &T, obj2: &T, compare: &mut F) -> NSComparisonResult {
            NSComparisonResult::from_ordering((*compare)(obj1, obj2))
        }

        let mut closure = compare;
        unsafe {
            let _: () = msg_send![self, sortUsingFunction:compare_with_closure::<Self::Item, F>
                                                  context:&mut closure];
        }
    }
}

#[allow(missing_copy_implementations)]
pub enum NSMutableArray<T, O = Owned> { }

object_impl!(NSMutableArray<T, O>);

impl<T: INSObject, O: Ownership> INSArray for NSMutableArray<T, O> {
    type Item = T;
    type Own = O;
}

impl<T: INSObject> INSOwnedArray for NSMutableArray<T, Owned> { }

impl<T: INSObject> INSSharedArray for NSMutableArray<T, Shared> { }

impl<T: INSObject, O: Ownership> INSMutableArray for NSMutableArray<T, O> { }

impl<T> INSCopying for NSMutableArray<T, Shared> {
    type Output = NSSharedArray<T>;
}

impl<T> INSMutableCopying for NSMutableArray<T, Shared> {
    type Output = NSMutableSharedArray<T>;
}

impl<T: INSObject, O: Ownership> Index<usize> for NSMutableArray<T, O> {
    type Output = T;

    fn index(&self, index: &usize) -> &T {
        self.object_at(*index)
    }
}

pub type NSMutableSharedArray<T> = NSMutableArray<T, Shared>;

#[cfg(test)]
mod tests {
    use objc::{Id};
    use {INSObject, INSString, NSObject, NSString};
    use super::{INSArray, INSMutableArray, NSArray, NSMutableArray};

    fn sample_array(len: usize) -> Id<NSArray<NSObject>> {
        let mut vec: Vec<Id<NSObject>> = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(INSObject::new());
        }
        INSArray::from_vec(vec)
    }

    #[test]
    fn test_count() {
        let empty_array: Id<NSArray<NSObject>> = INSObject::new();
        assert!(empty_array.count() == 0);

        let array = sample_array(4);
        assert!(array.count() == 4);
    }

    #[test]
    fn test_object_at() {
        let array = sample_array(4);
        assert!(array.object_at(0) != array.object_at(3));
        assert!(array.first_object().unwrap() == array.object_at(0));
        assert!(array.last_object().unwrap() == array.object_at(3));

        let empty_array: Id<NSArray<NSObject>> = INSObject::new();
        assert!(empty_array.first_object().is_none());
        assert!(empty_array.last_object().is_none());
    }

    #[test]
    fn test_object_enumerator() {
        let array = sample_array(4);

        assert!(array.object_enumerator().count() == 4);
        assert!(array.object_enumerator()
                     .enumerate()
                     .all(|(i, obj)| obj == array.object_at(i)));
    }

    #[test]
    fn test_objects_in_range() {
        let array = sample_array(4);

        let middle_objs = array.objects_in_range(1, 2);
        assert!(middle_objs.len() == 2);
        assert!(middle_objs[0] == array.object_at(1));
        assert!(middle_objs[1] == array.object_at(2));

        let empty_objs = array.objects_in_range(1, 0);
        assert!(empty_objs.len() == 0);

        let all_objs = array.objects_in_range(0, 4);
        assert!(all_objs.len() == 4);
    }

    #[test]
    fn test_into_vec() {
        let array = sample_array(4);

        let vec = INSArray::into_vec(array);
        assert!(vec.len() == 4);
    }

    #[test]
    fn test_add_object() {
        let mut array: Id<NSMutableArray<NSObject>> = INSObject::new();
        let obj: Id<NSObject> = INSObject::new();
        array.add_object(obj);

        assert!(array.count() == 1);
        assert!(array.object_at(0) == array.object_at(0));

        let obj: Id<NSObject> = INSObject::new();
        array.insert_object_at(0, obj);
        assert!(array.count() == 2);
    }

    #[test]
    fn test_replace_object() {
        let mut array: Id<NSMutableArray<NSObject>> = INSObject::new();
        let obj: Id<NSObject> = INSObject::new();
        array.add_object(obj);

        let obj: Id<NSObject> = INSObject::new();
        let old_obj = array.replace_object_at(0, obj);
        assert!(&*old_obj != array.object_at(0));
    }

    #[test]
    fn test_remove_object() {
        let mut array: Id<NSMutableArray<NSObject>> = INSObject::new();
        for _ in 0..4 {
            let obj: Id<NSObject> = INSObject::new();
            array.add_object(obj);
        }

        array.remove_object_at(1);
        assert!(array.count() == 3);

        array.remove_last_object();
        assert!(array.count() == 2);

        array.remove_all_objects();
        assert!(array.count() == 0);
    }

    #[test]
    fn test_sort() {
        let strings: Vec<Id<NSString>> = vec![
            INSString::from_str("hello"),
            INSString::from_str("hi"),
        ];
        let mut strings: Id<NSMutableArray<_>> = INSArray::from_vec(strings);

        strings.sort_by(|s1, s2| s1.as_str().len().cmp(&s2.as_str().len()));
        assert!(strings[0].as_str() == "hi");
        assert!(strings[1].as_str() == "hello");
    }
}
