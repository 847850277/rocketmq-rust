/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(sync_unsafe_cell)]
#![allow(unused_variables)]

use std::borrow::Borrow;
use std::cell::SyncUnsafeCell;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Weak;

pub use crate::common::attribute::topic_attributes as TopicAttributes;
pub use crate::common::message::message_accessor as MessageAccessor;
pub use crate::common::message::message_decoder as MessageDecoder;
use crate::error::Error;
pub use crate::thread_pool::FuturesExecutorService;
pub use crate::thread_pool::FuturesExecutorServiceBuilder;
pub use crate::thread_pool::ScheduledExecutorService;
pub use crate::thread_pool::TokioExecutorService;
pub use crate::utils::cleanup_policy_utils as CleanupPolicyUtils;
pub use crate::utils::crc32_utils as CRC32Utils;
pub use crate::utils::env_utils as EnvUtils;
pub use crate::utils::file_utils as FileUtils;
pub use crate::utils::message_utils as MessageUtils;
pub use crate::utils::parse_config_file as ParseConfigFile;
pub use crate::utils::time_utils as TimeUtils;
pub use crate::utils::util_all as UtilAll;

pub mod common;
pub mod error;
pub mod log;
mod thread_pool;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;

pub struct WeakCellWrapper<T: ?Sized> {
    inner: Weak<SyncUnsafeCell<T>>,
}

// Implementation of PartialEq for WeakCellWrapper<T>
impl<T: PartialEq> PartialEq for WeakCellWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        // Upgrade the Weak references to Arc, then compare the inner values
        if let (Some(self_arc), Some(other_arc)) = (self.inner.upgrade(), other.inner.upgrade()) {
            unsafe { *self_arc.get() == *other_arc.get() }
        } else {
            false
        }
    }
}

// Implementation of Eq for WeakCellWrapper<T>
impl<T: PartialEq> Eq for WeakCellWrapper<T> {}

// Implementation of Hash for WeakCellWrapper<T>
impl<T: Hash> Hash for WeakCellWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(arc) = self.inner.upgrade() {
            unsafe { (*arc.get()).hash(state) }
        }
    }
}

impl<T: ?Sized> Clone for WeakCellWrapper<T> {
    fn clone(&self) -> Self {
        WeakCellWrapper {
            inner: self.inner.clone(),
        }
    }
}

impl<T> WeakCellWrapper<T> {
    pub fn upgrade(&self) -> Option<ArcRefCellWrapper<T>> {
        self.inner
            .upgrade()
            .map(|value| ArcRefCellWrapper { inner: value })
    }
}

#[derive(Default)]
pub struct ArcRefCellWrapper<T: ?Sized> {
    inner: Arc<SyncUnsafeCell<T>>,
}

// Implementation of PartialEq for ArcRefCellWrapper<T>
impl<T: PartialEq> PartialEq for ArcRefCellWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        // Compare the inner values by borrowing them unsafely
        unsafe { *self.inner.get() == *other.inner.get() }
    }
}

impl<T: Hash> Hash for ArcRefCellWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Compute the hash of the inner value
        unsafe { (*self.inner.get()).hash(state) }
    }
}

// Implementation of Eq for ArcRefCellWrapper<T>
// Eq implies PartialEq, so we don't need to add any methods here
impl<T: PartialEq> Eq for ArcRefCellWrapper<T> {}

impl<T> ArcRefCellWrapper<T> {
    #[allow(clippy::mut_from_ref)]
    pub fn mut_from_ref(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn downgrade(this: &Self) -> WeakCellWrapper<T> {
        WeakCellWrapper {
            inner: Arc::downgrade(&this.inner),
        }
    }

    pub fn get_inner(&self) -> &Arc<SyncUnsafeCell<T>> {
        &self.inner
    }
}

impl<T> ArcRefCellWrapper<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(SyncUnsafeCell::new(value)),
        }
    }
}

impl<T: ?Sized> Clone for ArcRefCellWrapper<T> {
    fn clone(&self) -> Self {
        ArcRefCellWrapper {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> AsRef<T> for ArcRefCellWrapper<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.inner.get() }
    }
}

impl<T> AsMut<T> for ArcRefCellWrapper<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}

impl<T> Deref for ArcRefCellWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ArcRefCellWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

pub struct SyncUnsafeCellWrapper<T: ?Sized> {
    inner: SyncUnsafeCell<T>,
}

impl<T> SyncUnsafeCellWrapper<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            inner: SyncUnsafeCell::new(value),
        }
    }
}

impl<T> SyncUnsafeCellWrapper<T> {
    #[allow(clippy::mut_from_ref)]
    pub fn mut_from_ref(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}

impl<T> AsRef<T> for SyncUnsafeCellWrapper<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.inner.get() }
    }
}

impl<T> AsMut<T> for SyncUnsafeCellWrapper<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut *self.inner.get_mut()
    }
}

impl<T> Deref for SyncUnsafeCellWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for SyncUnsafeCellWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[cfg(test)]
mod arc_cell_wrapper_tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn new_creates_arc_cell_wrapper_with_provided_value() {
        let wrapper = ArcRefCellWrapper::new(10);
        assert_eq!(*wrapper.as_ref(), 10);
    }

    #[test]
    fn clone_creates_a_new_instance_with_same_value() {
        let wrapper = ArcRefCellWrapper::new(20);
        let cloned_wrapper = wrapper.clone();
        assert_eq!(*cloned_wrapper.as_ref(), 20);
    }

    #[test]
    fn as_ref_returns_immutable_reference_to_value() {
        let wrapper = ArcRefCellWrapper::new(30);
        assert_eq!(*wrapper.as_ref(), 30);
    }

    #[test]
    fn as_mut_returns_mutable_reference_to_value() {
        let mut wrapper = ArcRefCellWrapper::new(40);
        *wrapper.as_mut() = 50;
        assert_eq!(*wrapper.as_ref(), 50);
    }

    #[test]
    fn deref_returns_reference_to_inner_value() {
        let wrapper = ArcRefCellWrapper::new(60);
        assert_eq!(*wrapper, 60);
    }

    #[test]
    fn deref_mut_allows_modification_of_inner_value() {
        let mut wrapper = ArcRefCellWrapper::new(70);
        *wrapper = 80;
        assert_eq!(*wrapper, 80);
    }

    #[test]
    fn multiple_clones_share_the_same_underlying_data() {
        let wrapper = ArcRefCellWrapper::new(Arc::new(90));
        let cloned_wrapper1 = wrapper.clone();
        let cloned_wrapper2 = wrapper.clone();

        assert_eq!(Arc::strong_count(wrapper.as_ref()), 1);
        assert_eq!(Arc::strong_count(cloned_wrapper1.as_ref()), 1);
        assert_eq!(Arc::strong_count(cloned_wrapper2.as_ref()), 1);
    }
}
