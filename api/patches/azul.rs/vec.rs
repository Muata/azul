    use std::fmt;
    use crate::gl::{
        GLint as AzGLint,
        GLuint as AzGLuint,
    };

    macro_rules! impl_vec {($struct_type:ident, $struct_name:ident) => (

        impl $struct_name {

            #[inline]
            pub fn iter(&self) -> std::slice::Iter<$struct_type> {
                self.as_ref().iter()
            }

            #[inline]
            pub fn into_iter(self) -> std::vec::IntoIter<$struct_type> {
                let v1: Vec<$struct_type> = self.into();
                v1.into_iter()
            }

            #[inline]
            pub fn iter_mut(&mut self) -> std::slice::IterMut<$struct_type> {
                self.as_mut().iter_mut()
            }

            #[inline]
            pub fn ptr_as_usize(&self) -> usize {
                self.ptr as usize
            }

            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut $struct_type {
                self.ptr
            }

            #[inline]
            pub fn len(&self) -> usize {
                self.len
            }

            #[inline]
            pub fn capacity(&self) -> usize {
                self.cap
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            pub fn get(&self, index: usize) -> Option<&$struct_type> {
                self.as_ref().get(index)
            }

            #[inline]
            pub unsafe fn get_unchecked(&self, index: usize) -> &$struct_type {
                self.as_ref().get_unchecked(index)
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl AsRef<[$struct_type]> for $struct_name {
            fn as_ref(&self) -> &[$struct_type] {
                unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
            }
        }

        impl AsMut<[$struct_type]> for $struct_name {
            fn as_mut(&mut self) -> &mut [$struct_type] {
                unsafe { std::slice::from_raw_parts_mut (self.ptr, self.len) }
            }
        }

        impl std::iter::FromIterator<$struct_type> for $struct_name {
            fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = $struct_type> {
                let v: Vec<$struct_type> = Vec::from_iter(iter);
                v.into()
            }
        }

        impl From<Vec<$struct_type>> for $struct_name {
            fn from(input: Vec<$struct_type>) -> $struct_name {
                let s: &[$struct_type] = input.as_ref();
                s.into()
            }
        }

        impl From<&[$struct_type]> for $struct_name {
            fn from(input: &[$struct_type]) -> $struct_name {
                Self::copy_from(input.as_ptr(), input.len())
            }
        }

        impl From<$struct_name> for Vec<$struct_type> {
            fn from(mut input: $struct_name) -> Vec<$struct_type> {
                unsafe { std::slice::from_raw_parts(input.as_mut_ptr(), input.len()) }.to_vec()
            }
        }

        // Drop, Debug + Clone already implemented by default
    )}

    macro_rules! impl_vec_partialord {($struct_type:ident, $struct_name:ident) => (
        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                self.as_ref().partial_cmp(rhs.as_ref())
            }
        }
    )}

    macro_rules! impl_vec_ord {($struct_type:ident, $struct_name:ident) => (
        impl Ord for $struct_name {
            fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                self.as_ref().cmp(rhs.as_ref())
            }
        }
    )}

    macro_rules! impl_vec_partialeq {($struct_type:ident, $struct_name:ident) => (
        impl PartialEq for $struct_name {
            fn eq(&self, rhs: &Self) -> bool {
                self.as_ref().eq(rhs.as_ref())
            }
        }
    )}

    macro_rules! impl_vec_eq {($struct_type:ident, $struct_name:ident) => (
        impl Eq for $struct_name { }
    )}

    macro_rules! impl_vec_hash {($struct_type:ident, $struct_name:ident) => (
        impl std::hash::Hash for $struct_name {
            fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
                self.as_ref().hash(state);
            }
        }
    )}

    impl_vec!(u8, AzU8Vec);
    impl_vec_partialord!(u8, AzU8Vec);
    impl_vec_ord!(u8, AzU8Vec);
    impl_vec_partialeq!(u8, AzU8Vec);
    impl_vec_eq!(u8, AzU8Vec);
    impl_vec_hash!(u8, AzU8Vec);

    impl_vec!(AzCallbackData, AzCallbackDataVec);
    impl_vec_partialord!(AzCallbackData, AzCallbackDataVec);
    impl_vec_ord!(AzCallbackData, AzCallbackDataVec);
    impl_vec_partialeq!(AzCallbackData, AzCallbackDataVec);
    impl_vec_eq!(AzCallbackData, AzCallbackDataVec);
    impl_vec_hash!(AzCallbackData, AzCallbackDataVec);

    impl_vec!(AzCssProperty, AzCssPropertyVec);
    impl_vec_partialord!(AzCssProperty, AzCssPropertyVec);
    impl_vec_ord!(AzCssProperty, AzCssPropertyVec);
    impl_vec_partialeq!(AzCssProperty, AzCssPropertyVec);
    impl_vec_eq!(AzCssProperty, AzCssPropertyVec);
    impl_vec_hash!(AzCssProperty, AzCssPropertyVec);

    impl_vec!(AzDom, AzDomVec);
    impl_vec_partialord!(AzDom, AzDomVec);
    impl_vec_ord!(AzDom, AzDomVec);
    impl_vec_partialeq!(AzDom, AzDomVec);
    impl_vec_eq!(AzDom, AzDomVec);
    impl_vec_hash!(AzDom, AzDomVec);

    impl_vec!(AzString, AzStringVec);
    impl_vec_partialord!(AzString, AzStringVec);
    impl_vec_ord!(AzString, AzStringVec);
    impl_vec_partialeq!(AzString, AzStringVec);
    impl_vec_eq!(AzString, AzStringVec);
    impl_vec_hash!(AzString, AzStringVec);

    impl_vec!(AzGradientStopPre, AzGradientStopPreVec);
    impl_vec_partialord!(AzGradientStopPre, AzGradientStopPreVec);
    impl_vec_ord!(AzGradientStopPre, AzGradientStopPreVec);
    impl_vec_partialeq!(AzGradientStopPre, AzGradientStopPreVec);
    impl_vec_eq!(AzGradientStopPre, AzGradientStopPreVec);
    impl_vec_hash!(AzGradientStopPre, AzGradientStopPreVec);

    impl_vec!(AzDebugMessage, AzDebugMessageVec);
    impl_vec_partialord!(AzDebugMessage, AzDebugMessageVec);
    impl_vec_ord!(AzDebugMessage, AzDebugMessageVec);
    impl_vec_partialeq!(AzDebugMessage, AzDebugMessageVec);
    impl_vec_eq!(AzDebugMessage, AzDebugMessageVec);
    impl_vec_hash!(AzDebugMessage, AzDebugMessageVec);

    impl_vec!(AzGLint, AzGLintVec);
    impl_vec_partialord!(AzGLint, AzGLintVec);
    impl_vec_ord!(AzGLint, AzGLintVec);
    impl_vec_partialeq!(AzGLint, AzGLintVec);
    impl_vec_eq!(AzGLint, AzGLintVec);
    impl_vec_hash!(AzGLint, AzGLintVec);

    impl_vec!(AzGLuint, AzGLuintVec);
    impl_vec_partialord!(AzGLuint, AzGLuintVec);
    impl_vec_ord!(AzGLuint, AzGLuintVec);
    impl_vec_partialeq!(AzGLuint, AzGLuintVec);
    impl_vec_eq!(AzGLuint, AzGLuintVec);
    impl_vec_hash!(AzGLuint, AzGLuintVec);

    impl From<std::vec::Vec<std::string::String>> for crate::vec::StringVec {
        fn from(v: std::vec::Vec<std::string::String>) -> crate::vec::StringVec {
            let mut vec: Vec<AzString> = v.into_iter().map(Into::into).collect();
            (crate::dll::get_azul_dll().az_string_vec_copy_from)(vec.as_mut_ptr(), vec.len())
        }
    }

    impl From<crate::vec::StringVec> for std::vec::Vec<std::string::String> {
        fn from(v: crate::vec::StringVec) -> std::vec::Vec<std::string::String> {
            v
            .as_ref()
            .iter()
            .cloned()
            .map(Into::into)
            .collect()

            // delete() not necessary because StringVec is stack-allocated
        }
    }