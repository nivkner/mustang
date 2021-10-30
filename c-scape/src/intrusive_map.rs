use intrusive_collections::intrusive_adapter;
use intrusive_collections::rbtree::Entry;
use intrusive_collections::{KeyAdapter, RBTree, RBTreeLink, UnsafeRef};
use std::alloc::{GlobalAlloc, Layout};

static GLOBAL_ALLOCATOR: wee_alloc::WeeAlloc<'static> = wee_alloc::WeeAlloc::INIT;

struct BYOAPair<K, V> {
    link: RBTreeLink,
    key: K,
    value: V,
}

intrusive_adapter!(BYOAAdapter<K, V> = UnsafeRef<BYOAPair<K,V>>: BYOAPair<K, V> { link: RBTreeLink });
impl<K, V> KeyAdapter<'_> for BYOAAdapter<K, V>
where
    K: Copy,
{
    type Key = K;
    fn get_key(&self, pair: &BYOAPair<K, V>) -> K {
        pair.key
    }
}

pub struct BYOAMap<K, V> {
    tree: RBTree<BYOAAdapter<K, V>>,
}

impl<K, V> BYOAMap<K, V> {
    pub const fn new() -> Self {
        BYOAMap {
            tree: RBTree::new(BYOAAdapter::NEW),
        }
    }
}

impl<K, V> BYOAMap<K, V>
where
    K: Ord + Copy,
    V: Copy,
{
    pub fn insert(&mut self, key: K, value: V) {
        unsafe {
            let ptr = GLOBAL_ALLOCATOR
                .alloc(Layout::new::<BYOAPair<K, V>>())
                .cast::<BYOAPair<K, V>>();
            ptr.write(BYOAPair {
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
                            Layout::new::<BYOAPair<K, V>>(),
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
                Layout::new::<BYOAPair<K, V>>(),
            );
            Some(value)
        }
    }
}
