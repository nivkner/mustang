use intrusive_collections::intrusive_adapter;
use intrusive_collections::rbtree::Entry;
use intrusive_collections::{KeyAdapter, RBTree, RBTreeLink, UnsafeRef};
use std::alloc::{GlobalAlloc, Layout};

static GLOBAL_ALLOCATOR: wee_alloc::WeeAlloc<'static> = wee_alloc::WeeAlloc::INIT;

struct IntrusivePair<K, V> {
    link: RBTreeLink,
    key: K,
    value: V,
}

intrusive_adapter!(IntrusiveAdapter<K, V> = UnsafeRef<IntrusivePair<K,V>>: IntrusivePair<K, V> { link: RBTreeLink });
impl<K, V> KeyAdapter<'_> for IntrusiveAdapter<K, V>
where
    K: Copy,
{
    type Key = K;
    fn get_key(&self, pair: &IntrusivePair<K, V>) -> K {
        pair.key
    }
}

pub struct IntrusiveMap<K, V> {
    tree: RBTree<IntrusiveAdapter<K, V>>,
}

impl<K, V> IntrusiveMap<K, V> {
    pub const fn new() -> Self {
        IntrusiveMap {
            tree: RBTree::new(IntrusiveAdapter::NEW),
        }
    }
}

impl<K, V> IntrusiveMap<K, V>
where
    K: Ord + Copy,
    V: Copy,
{
    pub fn insert(&mut self, key: K, value: V) {
        unsafe {
            let ptr = GLOBAL_ALLOCATOR
                .alloc(Layout::new::<IntrusivePair<K, V>>())
                .cast::<IntrusivePair<K, V>>();
            ptr.write(IntrusivePair {
                link: RBTreeLink::new(),
                key,
                value,
            });
            let unsaferef = UnsafeRef::from_raw(ptr);
            match self.tree.entry(&key) {
                Entry::Occupied(mut cursor) => {
                    cursor.insert_after(unsaferef);
                    if let Some(unsaferef) = cursor.remove() {
                        GLOBAL_ALLOCATOR.dealloc(
                            UnsafeRef::into_raw(unsaferef).cast::<_>(),
                            Layout::new::<IntrusivePair<K, V>>(),
                        );
                    }
                }
                Entry::Vacant(cursor) => {
                    cursor.insert(unsaferef);
                }
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        unsafe {
            let unsaferef = self.tree.find_mut(key).remove()?;
            let value = unsaferef.value;
            GLOBAL_ALLOCATOR.dealloc(
                UnsafeRef::into_raw(unsaferef).cast::<_>(),
                Layout::new::<IntrusivePair<K, V>>(),
            );
            Some(value)
        }
    }
}
