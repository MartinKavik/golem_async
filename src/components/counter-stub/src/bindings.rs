#[allow(dead_code)]
pub mod golem {
    #[allow(dead_code)]
    pub mod rpc {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type NodeIndex = i32;
            #[derive(Clone)]
            pub struct Uri {
                pub value: _rt::String,
            }
            impl ::core::fmt::Debug for Uri {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Uri").field("value", &self.value).finish()
                }
            }
            #[derive(Clone)]
            pub enum WitNode {
                RecordValue(_rt::Vec<NodeIndex>),
                VariantValue((u32, Option<NodeIndex>)),
                EnumValue(u32),
                FlagsValue(_rt::Vec<bool>),
                TupleValue(_rt::Vec<NodeIndex>),
                ListValue(_rt::Vec<NodeIndex>),
                OptionValue(Option<NodeIndex>),
                ResultValue(Result<Option<NodeIndex>, Option<NodeIndex>>),
                PrimU8(u8),
                PrimU16(u16),
                PrimU32(u32),
                PrimU64(u64),
                PrimS8(i8),
                PrimS16(i16),
                PrimS32(i32),
                PrimS64(i64),
                PrimFloat32(f32),
                PrimFloat64(f64),
                PrimChar(char),
                PrimBool(bool),
                PrimString(_rt::String),
                Handle((Uri, u64)),
            }
            impl ::core::fmt::Debug for WitNode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        WitNode::RecordValue(e) => {
                            f.debug_tuple("WitNode::RecordValue").field(e).finish()
                        }
                        WitNode::VariantValue(e) => {
                            f.debug_tuple("WitNode::VariantValue").field(e).finish()
                        }
                        WitNode::EnumValue(e) => {
                            f.debug_tuple("WitNode::EnumValue").field(e).finish()
                        }
                        WitNode::FlagsValue(e) => {
                            f.debug_tuple("WitNode::FlagsValue").field(e).finish()
                        }
                        WitNode::TupleValue(e) => {
                            f.debug_tuple("WitNode::TupleValue").field(e).finish()
                        }
                        WitNode::ListValue(e) => {
                            f.debug_tuple("WitNode::ListValue").field(e).finish()
                        }
                        WitNode::OptionValue(e) => {
                            f.debug_tuple("WitNode::OptionValue").field(e).finish()
                        }
                        WitNode::ResultValue(e) => {
                            f.debug_tuple("WitNode::ResultValue").field(e).finish()
                        }
                        WitNode::PrimU8(e) => {
                            f.debug_tuple("WitNode::PrimU8").field(e).finish()
                        }
                        WitNode::PrimU16(e) => {
                            f.debug_tuple("WitNode::PrimU16").field(e).finish()
                        }
                        WitNode::PrimU32(e) => {
                            f.debug_tuple("WitNode::PrimU32").field(e).finish()
                        }
                        WitNode::PrimU64(e) => {
                            f.debug_tuple("WitNode::PrimU64").field(e).finish()
                        }
                        WitNode::PrimS8(e) => {
                            f.debug_tuple("WitNode::PrimS8").field(e).finish()
                        }
                        WitNode::PrimS16(e) => {
                            f.debug_tuple("WitNode::PrimS16").field(e).finish()
                        }
                        WitNode::PrimS32(e) => {
                            f.debug_tuple("WitNode::PrimS32").field(e).finish()
                        }
                        WitNode::PrimS64(e) => {
                            f.debug_tuple("WitNode::PrimS64").field(e).finish()
                        }
                        WitNode::PrimFloat32(e) => {
                            f.debug_tuple("WitNode::PrimFloat32").field(e).finish()
                        }
                        WitNode::PrimFloat64(e) => {
                            f.debug_tuple("WitNode::PrimFloat64").field(e).finish()
                        }
                        WitNode::PrimChar(e) => {
                            f.debug_tuple("WitNode::PrimChar").field(e).finish()
                        }
                        WitNode::PrimBool(e) => {
                            f.debug_tuple("WitNode::PrimBool").field(e).finish()
                        }
                        WitNode::PrimString(e) => {
                            f.debug_tuple("WitNode::PrimString").field(e).finish()
                        }
                        WitNode::Handle(e) => {
                            f.debug_tuple("WitNode::Handle").field(e).finish()
                        }
                    }
                }
            }
            #[derive(Clone)]
            pub struct WitValue {
                pub nodes: _rt::Vec<WitNode>,
            }
            impl ::core::fmt::Debug for WitValue {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("WitValue").field("nodes", &self.nodes).finish()
                }
            }
            #[derive(Clone)]
            pub enum RpcError {
                ProtocolError(_rt::String),
                Denied(_rt::String),
                NotFound(_rt::String),
                RemoteInternalError(_rt::String),
            }
            impl ::core::fmt::Debug for RpcError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        RpcError::ProtocolError(e) => {
                            f.debug_tuple("RpcError::ProtocolError").field(e).finish()
                        }
                        RpcError::Denied(e) => {
                            f.debug_tuple("RpcError::Denied").field(e).finish()
                        }
                        RpcError::NotFound(e) => {
                            f.debug_tuple("RpcError::NotFound").field(e).finish()
                        }
                        RpcError::RemoteInternalError(e) => {
                            f.debug_tuple("RpcError::RemoteInternalError")
                                .field(e)
                                .finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for RpcError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for RpcError {}
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct WasmRpc {
                handle: _rt::Resource<WasmRpc>,
            }
            impl WasmRpc {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for WasmRpc {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]wasm-rpc"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct FutureInvokeResult {
                handle: _rt::Resource<FutureInvokeResult>,
            }
            impl FutureInvokeResult {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for FutureInvokeResult {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]future-invoke-result"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl WasmRpc {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(location: &Uri) -> Self {
                    unsafe {
                        let Uri { value: value0 } = location;
                        let vec1 = value0;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[constructor]wasm-rpc"]
                            fn wit_import(_: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(ptr1.cast_mut(), len1);
                        WasmRpc::from_handle(ret as u32)
                    }
                }
            }
            impl WasmRpc {
                #[allow(unused_unsafe, clippy::all)]
                pub fn invoke_and_await(
                    &self,
                    function_name: &str,
                    function_params: &[WitValue],
                ) -> Result<WitValue, RpcError> {
                    unsafe {
                        let mut cleanup_list = _rt::Vec::new();
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = function_name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec12 = function_params;
                        let len12 = vec12.len();
                        let layout12 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec12.len() * 8,
                            4,
                        );
                        let result12 = if layout12.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout12).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout12);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec12.into_iter().enumerate() {
                            let base = result12.add(i * 8);
                            {
                                let WitValue { nodes: nodes1 } = e;
                                let vec11 = nodes1;
                                let len11 = vec11.len();
                                let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                                    vec11.len() * 24,
                                    8,
                                );
                                let result11 = if layout11.size() != 0 {
                                    let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                    if ptr.is_null() {
                                        _rt::alloc::handle_alloc_error(layout11);
                                    }
                                    ptr
                                } else {
                                    ::core::ptr::null_mut()
                                };
                                for (i, e) in vec11.into_iter().enumerate() {
                                    let base = result11.add(i * 24);
                                    {
                                        match e {
                                            WitNode::RecordValue(e) => {
                                                *base.add(0).cast::<u8>() = (0i32) as u8;
                                                let vec2 = e;
                                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                                let len2 = vec2.len();
                                                *base.add(12).cast::<usize>() = len2;
                                                *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                                            }
                                            WitNode::VariantValue(e) => {
                                                *base.add(0).cast::<u8>() = (1i32) as u8;
                                                let (t3_0, t3_1) = e;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(t3_0);
                                                match t3_1 {
                                                    Some(e) => {
                                                        *base.add(12).cast::<u8>() = (1i32) as u8;
                                                        *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                    }
                                                    None => {
                                                        *base.add(12).cast::<u8>() = (0i32) as u8;
                                                    }
                                                };
                                            }
                                            WitNode::EnumValue(e) => {
                                                *base.add(0).cast::<u8>() = (2i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::FlagsValue(e) => {
                                                *base.add(0).cast::<u8>() = (3i32) as u8;
                                                let vec4 = e;
                                                let len4 = vec4.len();
                                                let layout4 = _rt::alloc::Layout::from_size_align_unchecked(
                                                    vec4.len() * 1,
                                                    1,
                                                );
                                                let result4 = if layout4.size() != 0 {
                                                    let ptr = _rt::alloc::alloc(layout4).cast::<u8>();
                                                    if ptr.is_null() {
                                                        _rt::alloc::handle_alloc_error(layout4);
                                                    }
                                                    ptr
                                                } else {
                                                    ::core::ptr::null_mut()
                                                };
                                                for (i, e) in vec4.into_iter().enumerate() {
                                                    let base = result4.add(i * 1);
                                                    {
                                                        *base.add(0).cast::<u8>() = (match e {
                                                            true => 1,
                                                            false => 0,
                                                        }) as u8;
                                                    }
                                                }
                                                *base.add(12).cast::<usize>() = len4;
                                                *base.add(8).cast::<*mut u8>() = result4;
                                                cleanup_list.extend_from_slice(&[(result4, layout4)]);
                                            }
                                            WitNode::TupleValue(e) => {
                                                *base.add(0).cast::<u8>() = (4i32) as u8;
                                                let vec5 = e;
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                *base.add(12).cast::<usize>() = len5;
                                                *base.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            WitNode::ListValue(e) => {
                                                *base.add(0).cast::<u8>() = (5i32) as u8;
                                                let vec6 = e;
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                *base.add(12).cast::<usize>() = len6;
                                                *base.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            WitNode::OptionValue(e) => {
                                                *base.add(0).cast::<u8>() = (6i32) as u8;
                                                match e {
                                                    Some(e) => {
                                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                                        *base.add(12).cast::<i32>() = _rt::as_i32(e);
                                                    }
                                                    None => {
                                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                                    }
                                                };
                                            }
                                            WitNode::ResultValue(e) => {
                                                *base.add(0).cast::<u8>() = (7i32) as u8;
                                                match e {
                                                    Ok(e) => {
                                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                                        match e {
                                                            Some(e) => {
                                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                                *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                            }
                                                            None => {
                                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                            }
                                                        };
                                                    }
                                                    Err(e) => {
                                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                                        match e {
                                                            Some(e) => {
                                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                                *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                            }
                                                            None => {
                                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                            }
                                                        };
                                                    }
                                                };
                                            }
                                            WitNode::PrimU8(e) => {
                                                *base.add(0).cast::<u8>() = (8i32) as u8;
                                                *base.add(8).cast::<u8>() = (_rt::as_i32(e)) as u8;
                                            }
                                            WitNode::PrimU16(e) => {
                                                *base.add(0).cast::<u8>() = (9i32) as u8;
                                                *base.add(8).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                            }
                                            WitNode::PrimU32(e) => {
                                                *base.add(0).cast::<u8>() = (10i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimU64(e) => {
                                                *base.add(0).cast::<u8>() = (11i32) as u8;
                                                *base.add(8).cast::<i64>() = _rt::as_i64(e);
                                            }
                                            WitNode::PrimS8(e) => {
                                                *base.add(0).cast::<u8>() = (12i32) as u8;
                                                *base.add(8).cast::<u8>() = (_rt::as_i32(e)) as u8;
                                            }
                                            WitNode::PrimS16(e) => {
                                                *base.add(0).cast::<u8>() = (13i32) as u8;
                                                *base.add(8).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                            }
                                            WitNode::PrimS32(e) => {
                                                *base.add(0).cast::<u8>() = (14i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimS64(e) => {
                                                *base.add(0).cast::<u8>() = (15i32) as u8;
                                                *base.add(8).cast::<i64>() = _rt::as_i64(e);
                                            }
                                            WitNode::PrimFloat32(e) => {
                                                *base.add(0).cast::<u8>() = (16i32) as u8;
                                                *base.add(8).cast::<f32>() = _rt::as_f32(e);
                                            }
                                            WitNode::PrimFloat64(e) => {
                                                *base.add(0).cast::<u8>() = (17i32) as u8;
                                                *base.add(8).cast::<f64>() = _rt::as_f64(e);
                                            }
                                            WitNode::PrimChar(e) => {
                                                *base.add(0).cast::<u8>() = (18i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimBool(e) => {
                                                *base.add(0).cast::<u8>() = (19i32) as u8;
                                                *base.add(8).cast::<u8>() = (match e {
                                                    true => 1,
                                                    false => 0,
                                                }) as u8;
                                            }
                                            WitNode::PrimString(e) => {
                                                *base.add(0).cast::<u8>() = (20i32) as u8;
                                                let vec7 = e;
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                *base.add(12).cast::<usize>() = len7;
                                                *base.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                            WitNode::Handle(e) => {
                                                *base.add(0).cast::<u8>() = (21i32) as u8;
                                                let (t8_0, t8_1) = e;
                                                let Uri { value: value9 } = t8_0;
                                                let vec10 = value9;
                                                let ptr10 = vec10.as_ptr().cast::<u8>();
                                                let len10 = vec10.len();
                                                *base.add(12).cast::<usize>() = len10;
                                                *base.add(8).cast::<*mut u8>() = ptr10.cast_mut();
                                                *base.add(16).cast::<i64>() = _rt::as_i64(t8_1);
                                            }
                                        }
                                    }
                                }
                                *base.add(4).cast::<usize>() = len11;
                                *base.add(0).cast::<*mut u8>() = result11;
                                cleanup_list.extend_from_slice(&[(result11, layout11)]);
                            }
                        }
                        let ptr13 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]wasm-rpc.invoke-and-await"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            result12,
                            len12,
                            ptr13,
                        );
                        let l14 = i32::from(*ptr13.add(0).cast::<u8>());
                        if layout12.size() != 0 {
                            _rt::alloc::dealloc(result12.cast(), layout12);
                        }
                        for (ptr, layout) in cleanup_list {
                            if layout.size() != 0 {
                                _rt::alloc::dealloc(ptr.cast(), layout);
                            }
                        }
                        match l14 {
                            0 => {
                                let e = {
                                    let l15 = *ptr13.add(4).cast::<*mut u8>();
                                    let l16 = *ptr13.add(8).cast::<usize>();
                                    let base62 = l15;
                                    let len62 = l16;
                                    let mut result62 = _rt::Vec::with_capacity(len62);
                                    for i in 0..len62 {
                                        let base = base62.add(i * 24);
                                        let e62 = {
                                            let l17 = i32::from(*base.add(0).cast::<u8>());
                                            let v61 = match l17 {
                                                0 => {
                                                    let e61 = {
                                                        let l18 = *base.add(8).cast::<*mut u8>();
                                                        let l19 = *base.add(12).cast::<usize>();
                                                        let len20 = l19;
                                                        _rt::Vec::from_raw_parts(l18.cast(), len20, len20)
                                                    };
                                                    WitNode::RecordValue(e61)
                                                }
                                                1 => {
                                                    let e61 = {
                                                        let l21 = *base.add(8).cast::<i32>();
                                                        let l22 = i32::from(*base.add(12).cast::<u8>());
                                                        (
                                                            l21 as u32,
                                                            match l22 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l23 = *base.add(16).cast::<i32>();
                                                                        l23
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        )
                                                    };
                                                    WitNode::VariantValue(e61)
                                                }
                                                2 => {
                                                    let e61 = {
                                                        let l24 = *base.add(8).cast::<i32>();
                                                        l24 as u32
                                                    };
                                                    WitNode::EnumValue(e61)
                                                }
                                                3 => {
                                                    let e61 = {
                                                        let l25 = *base.add(8).cast::<*mut u8>();
                                                        let l26 = *base.add(12).cast::<usize>();
                                                        let base28 = l25;
                                                        let len28 = l26;
                                                        let mut result28 = _rt::Vec::with_capacity(len28);
                                                        for i in 0..len28 {
                                                            let base = base28.add(i * 1);
                                                            let e28 = {
                                                                let l27 = i32::from(*base.add(0).cast::<u8>());
                                                                _rt::bool_lift(l27 as u8)
                                                            };
                                                            result28.push(e28);
                                                        }
                                                        _rt::cabi_dealloc(base28, len28 * 1, 1);
                                                        result28
                                                    };
                                                    WitNode::FlagsValue(e61)
                                                }
                                                4 => {
                                                    let e61 = {
                                                        let l29 = *base.add(8).cast::<*mut u8>();
                                                        let l30 = *base.add(12).cast::<usize>();
                                                        let len31 = l30;
                                                        _rt::Vec::from_raw_parts(l29.cast(), len31, len31)
                                                    };
                                                    WitNode::TupleValue(e61)
                                                }
                                                5 => {
                                                    let e61 = {
                                                        let l32 = *base.add(8).cast::<*mut u8>();
                                                        let l33 = *base.add(12).cast::<usize>();
                                                        let len34 = l33;
                                                        _rt::Vec::from_raw_parts(l32.cast(), len34, len34)
                                                    };
                                                    WitNode::ListValue(e61)
                                                }
                                                6 => {
                                                    let e61 = {
                                                        let l35 = i32::from(*base.add(8).cast::<u8>());
                                                        match l35 {
                                                            0 => None,
                                                            1 => {
                                                                let e = {
                                                                    let l36 = *base.add(12).cast::<i32>();
                                                                    l36
                                                                };
                                                                Some(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    WitNode::OptionValue(e61)
                                                }
                                                7 => {
                                                    let e61 = {
                                                        let l37 = i32::from(*base.add(8).cast::<u8>());
                                                        match l37 {
                                                            0 => {
                                                                let e = {
                                                                    let l38 = i32::from(*base.add(12).cast::<u8>());
                                                                    match l38 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l39 = *base.add(16).cast::<i32>();
                                                                                l39
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    }
                                                                };
                                                                Ok(e)
                                                            }
                                                            1 => {
                                                                let e = {
                                                                    let l40 = i32::from(*base.add(12).cast::<u8>());
                                                                    match l40 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l41 = *base.add(16).cast::<i32>();
                                                                                l41
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    }
                                                                };
                                                                Err(e)
                                                            }
                                                            _ => _rt::invalid_enum_discriminant(),
                                                        }
                                                    };
                                                    WitNode::ResultValue(e61)
                                                }
                                                8 => {
                                                    let e61 = {
                                                        let l42 = i32::from(*base.add(8).cast::<u8>());
                                                        l42 as u8
                                                    };
                                                    WitNode::PrimU8(e61)
                                                }
                                                9 => {
                                                    let e61 = {
                                                        let l43 = i32::from(*base.add(8).cast::<u16>());
                                                        l43 as u16
                                                    };
                                                    WitNode::PrimU16(e61)
                                                }
                                                10 => {
                                                    let e61 = {
                                                        let l44 = *base.add(8).cast::<i32>();
                                                        l44 as u32
                                                    };
                                                    WitNode::PrimU32(e61)
                                                }
                                                11 => {
                                                    let e61 = {
                                                        let l45 = *base.add(8).cast::<i64>();
                                                        l45 as u64
                                                    };
                                                    WitNode::PrimU64(e61)
                                                }
                                                12 => {
                                                    let e61 = {
                                                        let l46 = i32::from(*base.add(8).cast::<i8>());
                                                        l46 as i8
                                                    };
                                                    WitNode::PrimS8(e61)
                                                }
                                                13 => {
                                                    let e61 = {
                                                        let l47 = i32::from(*base.add(8).cast::<i16>());
                                                        l47 as i16
                                                    };
                                                    WitNode::PrimS16(e61)
                                                }
                                                14 => {
                                                    let e61 = {
                                                        let l48 = *base.add(8).cast::<i32>();
                                                        l48
                                                    };
                                                    WitNode::PrimS32(e61)
                                                }
                                                15 => {
                                                    let e61 = {
                                                        let l49 = *base.add(8).cast::<i64>();
                                                        l49
                                                    };
                                                    WitNode::PrimS64(e61)
                                                }
                                                16 => {
                                                    let e61 = {
                                                        let l50 = *base.add(8).cast::<f32>();
                                                        l50
                                                    };
                                                    WitNode::PrimFloat32(e61)
                                                }
                                                17 => {
                                                    let e61 = {
                                                        let l51 = *base.add(8).cast::<f64>();
                                                        l51
                                                    };
                                                    WitNode::PrimFloat64(e61)
                                                }
                                                18 => {
                                                    let e61 = {
                                                        let l52 = *base.add(8).cast::<i32>();
                                                        _rt::char_lift(l52 as u32)
                                                    };
                                                    WitNode::PrimChar(e61)
                                                }
                                                19 => {
                                                    let e61 = {
                                                        let l53 = i32::from(*base.add(8).cast::<u8>());
                                                        _rt::bool_lift(l53 as u8)
                                                    };
                                                    WitNode::PrimBool(e61)
                                                }
                                                20 => {
                                                    let e61 = {
                                                        let l54 = *base.add(8).cast::<*mut u8>();
                                                        let l55 = *base.add(12).cast::<usize>();
                                                        let len56 = l55;
                                                        let bytes56 = _rt::Vec::from_raw_parts(
                                                            l54.cast(),
                                                            len56,
                                                            len56,
                                                        );
                                                        _rt::string_lift(bytes56)
                                                    };
                                                    WitNode::PrimString(e61)
                                                }
                                                n => {
                                                    debug_assert_eq!(n, 21, "invalid enum discriminant");
                                                    let e61 = {
                                                        let l57 = *base.add(8).cast::<*mut u8>();
                                                        let l58 = *base.add(12).cast::<usize>();
                                                        let len59 = l58;
                                                        let bytes59 = _rt::Vec::from_raw_parts(
                                                            l57.cast(),
                                                            len59,
                                                            len59,
                                                        );
                                                        let l60 = *base.add(16).cast::<i64>();
                                                        (
                                                            Uri {
                                                                value: _rt::string_lift(bytes59),
                                                            },
                                                            l60 as u64,
                                                        )
                                                    };
                                                    WitNode::Handle(e61)
                                                }
                                            };
                                            v61
                                        };
                                        result62.push(e62);
                                    }
                                    _rt::cabi_dealloc(base62, len62 * 24, 8);
                                    WitValue { nodes: result62 }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l63 = i32::from(*ptr13.add(4).cast::<u8>());
                                    let v76 = match l63 {
                                        0 => {
                                            let e76 = {
                                                let l64 = *ptr13.add(8).cast::<*mut u8>();
                                                let l65 = *ptr13.add(12).cast::<usize>();
                                                let len66 = l65;
                                                let bytes66 = _rt::Vec::from_raw_parts(
                                                    l64.cast(),
                                                    len66,
                                                    len66,
                                                );
                                                _rt::string_lift(bytes66)
                                            };
                                            RpcError::ProtocolError(e76)
                                        }
                                        1 => {
                                            let e76 = {
                                                let l67 = *ptr13.add(8).cast::<*mut u8>();
                                                let l68 = *ptr13.add(12).cast::<usize>();
                                                let len69 = l68;
                                                let bytes69 = _rt::Vec::from_raw_parts(
                                                    l67.cast(),
                                                    len69,
                                                    len69,
                                                );
                                                _rt::string_lift(bytes69)
                                            };
                                            RpcError::Denied(e76)
                                        }
                                        2 => {
                                            let e76 = {
                                                let l70 = *ptr13.add(8).cast::<*mut u8>();
                                                let l71 = *ptr13.add(12).cast::<usize>();
                                                let len72 = l71;
                                                let bytes72 = _rt::Vec::from_raw_parts(
                                                    l70.cast(),
                                                    len72,
                                                    len72,
                                                );
                                                _rt::string_lift(bytes72)
                                            };
                                            RpcError::NotFound(e76)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 3, "invalid enum discriminant");
                                            let e76 = {
                                                let l73 = *ptr13.add(8).cast::<*mut u8>();
                                                let l74 = *ptr13.add(12).cast::<usize>();
                                                let len75 = l74;
                                                let bytes75 = _rt::Vec::from_raw_parts(
                                                    l73.cast(),
                                                    len75,
                                                    len75,
                                                );
                                                _rt::string_lift(bytes75)
                                            };
                                            RpcError::RemoteInternalError(e76)
                                        }
                                    };
                                    v76
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl WasmRpc {
                #[allow(unused_unsafe, clippy::all)]
                pub fn invoke(
                    &self,
                    function_name: &str,
                    function_params: &[WitValue],
                ) -> Result<(), RpcError> {
                    unsafe {
                        let mut cleanup_list = _rt::Vec::new();
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = function_name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec12 = function_params;
                        let len12 = vec12.len();
                        let layout12 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec12.len() * 8,
                            4,
                        );
                        let result12 = if layout12.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout12).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout12);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec12.into_iter().enumerate() {
                            let base = result12.add(i * 8);
                            {
                                let WitValue { nodes: nodes1 } = e;
                                let vec11 = nodes1;
                                let len11 = vec11.len();
                                let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                                    vec11.len() * 24,
                                    8,
                                );
                                let result11 = if layout11.size() != 0 {
                                    let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                    if ptr.is_null() {
                                        _rt::alloc::handle_alloc_error(layout11);
                                    }
                                    ptr
                                } else {
                                    ::core::ptr::null_mut()
                                };
                                for (i, e) in vec11.into_iter().enumerate() {
                                    let base = result11.add(i * 24);
                                    {
                                        match e {
                                            WitNode::RecordValue(e) => {
                                                *base.add(0).cast::<u8>() = (0i32) as u8;
                                                let vec2 = e;
                                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                                let len2 = vec2.len();
                                                *base.add(12).cast::<usize>() = len2;
                                                *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                                            }
                                            WitNode::VariantValue(e) => {
                                                *base.add(0).cast::<u8>() = (1i32) as u8;
                                                let (t3_0, t3_1) = e;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(t3_0);
                                                match t3_1 {
                                                    Some(e) => {
                                                        *base.add(12).cast::<u8>() = (1i32) as u8;
                                                        *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                    }
                                                    None => {
                                                        *base.add(12).cast::<u8>() = (0i32) as u8;
                                                    }
                                                };
                                            }
                                            WitNode::EnumValue(e) => {
                                                *base.add(0).cast::<u8>() = (2i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::FlagsValue(e) => {
                                                *base.add(0).cast::<u8>() = (3i32) as u8;
                                                let vec4 = e;
                                                let len4 = vec4.len();
                                                let layout4 = _rt::alloc::Layout::from_size_align_unchecked(
                                                    vec4.len() * 1,
                                                    1,
                                                );
                                                let result4 = if layout4.size() != 0 {
                                                    let ptr = _rt::alloc::alloc(layout4).cast::<u8>();
                                                    if ptr.is_null() {
                                                        _rt::alloc::handle_alloc_error(layout4);
                                                    }
                                                    ptr
                                                } else {
                                                    ::core::ptr::null_mut()
                                                };
                                                for (i, e) in vec4.into_iter().enumerate() {
                                                    let base = result4.add(i * 1);
                                                    {
                                                        *base.add(0).cast::<u8>() = (match e {
                                                            true => 1,
                                                            false => 0,
                                                        }) as u8;
                                                    }
                                                }
                                                *base.add(12).cast::<usize>() = len4;
                                                *base.add(8).cast::<*mut u8>() = result4;
                                                cleanup_list.extend_from_slice(&[(result4, layout4)]);
                                            }
                                            WitNode::TupleValue(e) => {
                                                *base.add(0).cast::<u8>() = (4i32) as u8;
                                                let vec5 = e;
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                *base.add(12).cast::<usize>() = len5;
                                                *base.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            WitNode::ListValue(e) => {
                                                *base.add(0).cast::<u8>() = (5i32) as u8;
                                                let vec6 = e;
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                *base.add(12).cast::<usize>() = len6;
                                                *base.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            WitNode::OptionValue(e) => {
                                                *base.add(0).cast::<u8>() = (6i32) as u8;
                                                match e {
                                                    Some(e) => {
                                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                                        *base.add(12).cast::<i32>() = _rt::as_i32(e);
                                                    }
                                                    None => {
                                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                                    }
                                                };
                                            }
                                            WitNode::ResultValue(e) => {
                                                *base.add(0).cast::<u8>() = (7i32) as u8;
                                                match e {
                                                    Ok(e) => {
                                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                                        match e {
                                                            Some(e) => {
                                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                                *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                            }
                                                            None => {
                                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                            }
                                                        };
                                                    }
                                                    Err(e) => {
                                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                                        match e {
                                                            Some(e) => {
                                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                                *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                            }
                                                            None => {
                                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                            }
                                                        };
                                                    }
                                                };
                                            }
                                            WitNode::PrimU8(e) => {
                                                *base.add(0).cast::<u8>() = (8i32) as u8;
                                                *base.add(8).cast::<u8>() = (_rt::as_i32(e)) as u8;
                                            }
                                            WitNode::PrimU16(e) => {
                                                *base.add(0).cast::<u8>() = (9i32) as u8;
                                                *base.add(8).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                            }
                                            WitNode::PrimU32(e) => {
                                                *base.add(0).cast::<u8>() = (10i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimU64(e) => {
                                                *base.add(0).cast::<u8>() = (11i32) as u8;
                                                *base.add(8).cast::<i64>() = _rt::as_i64(e);
                                            }
                                            WitNode::PrimS8(e) => {
                                                *base.add(0).cast::<u8>() = (12i32) as u8;
                                                *base.add(8).cast::<u8>() = (_rt::as_i32(e)) as u8;
                                            }
                                            WitNode::PrimS16(e) => {
                                                *base.add(0).cast::<u8>() = (13i32) as u8;
                                                *base.add(8).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                            }
                                            WitNode::PrimS32(e) => {
                                                *base.add(0).cast::<u8>() = (14i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimS64(e) => {
                                                *base.add(0).cast::<u8>() = (15i32) as u8;
                                                *base.add(8).cast::<i64>() = _rt::as_i64(e);
                                            }
                                            WitNode::PrimFloat32(e) => {
                                                *base.add(0).cast::<u8>() = (16i32) as u8;
                                                *base.add(8).cast::<f32>() = _rt::as_f32(e);
                                            }
                                            WitNode::PrimFloat64(e) => {
                                                *base.add(0).cast::<u8>() = (17i32) as u8;
                                                *base.add(8).cast::<f64>() = _rt::as_f64(e);
                                            }
                                            WitNode::PrimChar(e) => {
                                                *base.add(0).cast::<u8>() = (18i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimBool(e) => {
                                                *base.add(0).cast::<u8>() = (19i32) as u8;
                                                *base.add(8).cast::<u8>() = (match e {
                                                    true => 1,
                                                    false => 0,
                                                }) as u8;
                                            }
                                            WitNode::PrimString(e) => {
                                                *base.add(0).cast::<u8>() = (20i32) as u8;
                                                let vec7 = e;
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                *base.add(12).cast::<usize>() = len7;
                                                *base.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                            WitNode::Handle(e) => {
                                                *base.add(0).cast::<u8>() = (21i32) as u8;
                                                let (t8_0, t8_1) = e;
                                                let Uri { value: value9 } = t8_0;
                                                let vec10 = value9;
                                                let ptr10 = vec10.as_ptr().cast::<u8>();
                                                let len10 = vec10.len();
                                                *base.add(12).cast::<usize>() = len10;
                                                *base.add(8).cast::<*mut u8>() = ptr10.cast_mut();
                                                *base.add(16).cast::<i64>() = _rt::as_i64(t8_1);
                                            }
                                        }
                                    }
                                }
                                *base.add(4).cast::<usize>() = len11;
                                *base.add(0).cast::<*mut u8>() = result11;
                                cleanup_list.extend_from_slice(&[(result11, layout11)]);
                            }
                        }
                        let ptr13 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]wasm-rpc.invoke"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            result12,
                            len12,
                            ptr13,
                        );
                        let l14 = i32::from(*ptr13.add(0).cast::<u8>());
                        if layout12.size() != 0 {
                            _rt::alloc::dealloc(result12.cast(), layout12);
                        }
                        for (ptr, layout) in cleanup_list {
                            if layout.size() != 0 {
                                _rt::alloc::dealloc(ptr.cast(), layout);
                            }
                        }
                        match l14 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l15 = i32::from(*ptr13.add(4).cast::<u8>());
                                    let v28 = match l15 {
                                        0 => {
                                            let e28 = {
                                                let l16 = *ptr13.add(8).cast::<*mut u8>();
                                                let l17 = *ptr13.add(12).cast::<usize>();
                                                let len18 = l17;
                                                let bytes18 = _rt::Vec::from_raw_parts(
                                                    l16.cast(),
                                                    len18,
                                                    len18,
                                                );
                                                _rt::string_lift(bytes18)
                                            };
                                            RpcError::ProtocolError(e28)
                                        }
                                        1 => {
                                            let e28 = {
                                                let l19 = *ptr13.add(8).cast::<*mut u8>();
                                                let l20 = *ptr13.add(12).cast::<usize>();
                                                let len21 = l20;
                                                let bytes21 = _rt::Vec::from_raw_parts(
                                                    l19.cast(),
                                                    len21,
                                                    len21,
                                                );
                                                _rt::string_lift(bytes21)
                                            };
                                            RpcError::Denied(e28)
                                        }
                                        2 => {
                                            let e28 = {
                                                let l22 = *ptr13.add(8).cast::<*mut u8>();
                                                let l23 = *ptr13.add(12).cast::<usize>();
                                                let len24 = l23;
                                                let bytes24 = _rt::Vec::from_raw_parts(
                                                    l22.cast(),
                                                    len24,
                                                    len24,
                                                );
                                                _rt::string_lift(bytes24)
                                            };
                                            RpcError::NotFound(e28)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 3, "invalid enum discriminant");
                                            let e28 = {
                                                let l25 = *ptr13.add(8).cast::<*mut u8>();
                                                let l26 = *ptr13.add(12).cast::<usize>();
                                                let len27 = l26;
                                                let bytes27 = _rt::Vec::from_raw_parts(
                                                    l25.cast(),
                                                    len27,
                                                    len27,
                                                );
                                                _rt::string_lift(bytes27)
                                            };
                                            RpcError::RemoteInternalError(e28)
                                        }
                                    };
                                    v28
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl WasmRpc {
                #[allow(unused_unsafe, clippy::all)]
                pub fn async_invoke_and_await(
                    &self,
                    function_name: &str,
                    function_params: &[WitValue],
                ) -> FutureInvokeResult {
                    unsafe {
                        let mut cleanup_list = _rt::Vec::new();
                        let vec0 = function_name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec12 = function_params;
                        let len12 = vec12.len();
                        let layout12 = _rt::alloc::Layout::from_size_align_unchecked(
                            vec12.len() * 8,
                            4,
                        );
                        let result12 = if layout12.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout12).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout12);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec12.into_iter().enumerate() {
                            let base = result12.add(i * 8);
                            {
                                let WitValue { nodes: nodes1 } = e;
                                let vec11 = nodes1;
                                let len11 = vec11.len();
                                let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                                    vec11.len() * 24,
                                    8,
                                );
                                let result11 = if layout11.size() != 0 {
                                    let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                    if ptr.is_null() {
                                        _rt::alloc::handle_alloc_error(layout11);
                                    }
                                    ptr
                                } else {
                                    ::core::ptr::null_mut()
                                };
                                for (i, e) in vec11.into_iter().enumerate() {
                                    let base = result11.add(i * 24);
                                    {
                                        match e {
                                            WitNode::RecordValue(e) => {
                                                *base.add(0).cast::<u8>() = (0i32) as u8;
                                                let vec2 = e;
                                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                                let len2 = vec2.len();
                                                *base.add(12).cast::<usize>() = len2;
                                                *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                                            }
                                            WitNode::VariantValue(e) => {
                                                *base.add(0).cast::<u8>() = (1i32) as u8;
                                                let (t3_0, t3_1) = e;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(t3_0);
                                                match t3_1 {
                                                    Some(e) => {
                                                        *base.add(12).cast::<u8>() = (1i32) as u8;
                                                        *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                    }
                                                    None => {
                                                        *base.add(12).cast::<u8>() = (0i32) as u8;
                                                    }
                                                };
                                            }
                                            WitNode::EnumValue(e) => {
                                                *base.add(0).cast::<u8>() = (2i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::FlagsValue(e) => {
                                                *base.add(0).cast::<u8>() = (3i32) as u8;
                                                let vec4 = e;
                                                let len4 = vec4.len();
                                                let layout4 = _rt::alloc::Layout::from_size_align_unchecked(
                                                    vec4.len() * 1,
                                                    1,
                                                );
                                                let result4 = if layout4.size() != 0 {
                                                    let ptr = _rt::alloc::alloc(layout4).cast::<u8>();
                                                    if ptr.is_null() {
                                                        _rt::alloc::handle_alloc_error(layout4);
                                                    }
                                                    ptr
                                                } else {
                                                    ::core::ptr::null_mut()
                                                };
                                                for (i, e) in vec4.into_iter().enumerate() {
                                                    let base = result4.add(i * 1);
                                                    {
                                                        *base.add(0).cast::<u8>() = (match e {
                                                            true => 1,
                                                            false => 0,
                                                        }) as u8;
                                                    }
                                                }
                                                *base.add(12).cast::<usize>() = len4;
                                                *base.add(8).cast::<*mut u8>() = result4;
                                                cleanup_list.extend_from_slice(&[(result4, layout4)]);
                                            }
                                            WitNode::TupleValue(e) => {
                                                *base.add(0).cast::<u8>() = (4i32) as u8;
                                                let vec5 = e;
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                *base.add(12).cast::<usize>() = len5;
                                                *base.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            WitNode::ListValue(e) => {
                                                *base.add(0).cast::<u8>() = (5i32) as u8;
                                                let vec6 = e;
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                *base.add(12).cast::<usize>() = len6;
                                                *base.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            WitNode::OptionValue(e) => {
                                                *base.add(0).cast::<u8>() = (6i32) as u8;
                                                match e {
                                                    Some(e) => {
                                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                                        *base.add(12).cast::<i32>() = _rt::as_i32(e);
                                                    }
                                                    None => {
                                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                                    }
                                                };
                                            }
                                            WitNode::ResultValue(e) => {
                                                *base.add(0).cast::<u8>() = (7i32) as u8;
                                                match e {
                                                    Ok(e) => {
                                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                                        match e {
                                                            Some(e) => {
                                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                                *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                            }
                                                            None => {
                                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                            }
                                                        };
                                                    }
                                                    Err(e) => {
                                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                                        match e {
                                                            Some(e) => {
                                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                                *base.add(16).cast::<i32>() = _rt::as_i32(e);
                                                            }
                                                            None => {
                                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                            }
                                                        };
                                                    }
                                                };
                                            }
                                            WitNode::PrimU8(e) => {
                                                *base.add(0).cast::<u8>() = (8i32) as u8;
                                                *base.add(8).cast::<u8>() = (_rt::as_i32(e)) as u8;
                                            }
                                            WitNode::PrimU16(e) => {
                                                *base.add(0).cast::<u8>() = (9i32) as u8;
                                                *base.add(8).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                            }
                                            WitNode::PrimU32(e) => {
                                                *base.add(0).cast::<u8>() = (10i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimU64(e) => {
                                                *base.add(0).cast::<u8>() = (11i32) as u8;
                                                *base.add(8).cast::<i64>() = _rt::as_i64(e);
                                            }
                                            WitNode::PrimS8(e) => {
                                                *base.add(0).cast::<u8>() = (12i32) as u8;
                                                *base.add(8).cast::<u8>() = (_rt::as_i32(e)) as u8;
                                            }
                                            WitNode::PrimS16(e) => {
                                                *base.add(0).cast::<u8>() = (13i32) as u8;
                                                *base.add(8).cast::<u16>() = (_rt::as_i32(e)) as u16;
                                            }
                                            WitNode::PrimS32(e) => {
                                                *base.add(0).cast::<u8>() = (14i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimS64(e) => {
                                                *base.add(0).cast::<u8>() = (15i32) as u8;
                                                *base.add(8).cast::<i64>() = _rt::as_i64(e);
                                            }
                                            WitNode::PrimFloat32(e) => {
                                                *base.add(0).cast::<u8>() = (16i32) as u8;
                                                *base.add(8).cast::<f32>() = _rt::as_f32(e);
                                            }
                                            WitNode::PrimFloat64(e) => {
                                                *base.add(0).cast::<u8>() = (17i32) as u8;
                                                *base.add(8).cast::<f64>() = _rt::as_f64(e);
                                            }
                                            WitNode::PrimChar(e) => {
                                                *base.add(0).cast::<u8>() = (18i32) as u8;
                                                *base.add(8).cast::<i32>() = _rt::as_i32(e);
                                            }
                                            WitNode::PrimBool(e) => {
                                                *base.add(0).cast::<u8>() = (19i32) as u8;
                                                *base.add(8).cast::<u8>() = (match e {
                                                    true => 1,
                                                    false => 0,
                                                }) as u8;
                                            }
                                            WitNode::PrimString(e) => {
                                                *base.add(0).cast::<u8>() = (20i32) as u8;
                                                let vec7 = e;
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                *base.add(12).cast::<usize>() = len7;
                                                *base.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                            WitNode::Handle(e) => {
                                                *base.add(0).cast::<u8>() = (21i32) as u8;
                                                let (t8_0, t8_1) = e;
                                                let Uri { value: value9 } = t8_0;
                                                let vec10 = value9;
                                                let ptr10 = vec10.as_ptr().cast::<u8>();
                                                let len10 = vec10.len();
                                                *base.add(12).cast::<usize>() = len10;
                                                *base.add(8).cast::<*mut u8>() = ptr10.cast_mut();
                                                *base.add(16).cast::<i64>() = _rt::as_i64(t8_1);
                                            }
                                        }
                                    }
                                }
                                *base.add(4).cast::<usize>() = len11;
                                *base.add(0).cast::<*mut u8>() = result11;
                                cleanup_list.extend_from_slice(&[(result11, layout11)]);
                            }
                        }
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]wasm-rpc.async-invoke-and-await"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                            ) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                        ) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            result12,
                            len12,
                        );
                        if layout12.size() != 0 {
                            _rt::alloc::dealloc(result12.cast(), layout12);
                        }
                        for (ptr, layout) in cleanup_list {
                            if layout.size() != 0 {
                                _rt::alloc::dealloc(ptr.cast(), layout);
                            }
                        }
                        FutureInvokeResult::from_handle(ret as u32)
                    }
                }
            }
            impl FutureInvokeResult {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]future-invoke-result.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl FutureInvokeResult {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self) -> Option<Result<WitValue, RpcError>> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 20]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 20],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "golem:rpc/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]future-invoke-result.get"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    match l2 {
                                        0 => {
                                            let e = {
                                                let l3 = *ptr0.add(8).cast::<*mut u8>();
                                                let l4 = *ptr0.add(12).cast::<usize>();
                                                let base50 = l3;
                                                let len50 = l4;
                                                let mut result50 = _rt::Vec::with_capacity(len50);
                                                for i in 0..len50 {
                                                    let base = base50.add(i * 24);
                                                    let e50 = {
                                                        let l5 = i32::from(*base.add(0).cast::<u8>());
                                                        let v49 = match l5 {
                                                            0 => {
                                                                let e49 = {
                                                                    let l6 = *base.add(8).cast::<*mut u8>();
                                                                    let l7 = *base.add(12).cast::<usize>();
                                                                    let len8 = l7;
                                                                    _rt::Vec::from_raw_parts(l6.cast(), len8, len8)
                                                                };
                                                                WitNode::RecordValue(e49)
                                                            }
                                                            1 => {
                                                                let e49 = {
                                                                    let l9 = *base.add(8).cast::<i32>();
                                                                    let l10 = i32::from(*base.add(12).cast::<u8>());
                                                                    (
                                                                        l9 as u32,
                                                                        match l10 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l11 = *base.add(16).cast::<i32>();
                                                                                    l11
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        },
                                                                    )
                                                                };
                                                                WitNode::VariantValue(e49)
                                                            }
                                                            2 => {
                                                                let e49 = {
                                                                    let l12 = *base.add(8).cast::<i32>();
                                                                    l12 as u32
                                                                };
                                                                WitNode::EnumValue(e49)
                                                            }
                                                            3 => {
                                                                let e49 = {
                                                                    let l13 = *base.add(8).cast::<*mut u8>();
                                                                    let l14 = *base.add(12).cast::<usize>();
                                                                    let base16 = l13;
                                                                    let len16 = l14;
                                                                    let mut result16 = _rt::Vec::with_capacity(len16);
                                                                    for i in 0..len16 {
                                                                        let base = base16.add(i * 1);
                                                                        let e16 = {
                                                                            let l15 = i32::from(*base.add(0).cast::<u8>());
                                                                            _rt::bool_lift(l15 as u8)
                                                                        };
                                                                        result16.push(e16);
                                                                    }
                                                                    _rt::cabi_dealloc(base16, len16 * 1, 1);
                                                                    result16
                                                                };
                                                                WitNode::FlagsValue(e49)
                                                            }
                                                            4 => {
                                                                let e49 = {
                                                                    let l17 = *base.add(8).cast::<*mut u8>();
                                                                    let l18 = *base.add(12).cast::<usize>();
                                                                    let len19 = l18;
                                                                    _rt::Vec::from_raw_parts(l17.cast(), len19, len19)
                                                                };
                                                                WitNode::TupleValue(e49)
                                                            }
                                                            5 => {
                                                                let e49 = {
                                                                    let l20 = *base.add(8).cast::<*mut u8>();
                                                                    let l21 = *base.add(12).cast::<usize>();
                                                                    let len22 = l21;
                                                                    _rt::Vec::from_raw_parts(l20.cast(), len22, len22)
                                                                };
                                                                WitNode::ListValue(e49)
                                                            }
                                                            6 => {
                                                                let e49 = {
                                                                    let l23 = i32::from(*base.add(8).cast::<u8>());
                                                                    match l23 {
                                                                        0 => None,
                                                                        1 => {
                                                                            let e = {
                                                                                let l24 = *base.add(12).cast::<i32>();
                                                                                l24
                                                                            };
                                                                            Some(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    }
                                                                };
                                                                WitNode::OptionValue(e49)
                                                            }
                                                            7 => {
                                                                let e49 = {
                                                                    let l25 = i32::from(*base.add(8).cast::<u8>());
                                                                    match l25 {
                                                                        0 => {
                                                                            let e = {
                                                                                let l26 = i32::from(*base.add(12).cast::<u8>());
                                                                                match l26 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l27 = *base.add(16).cast::<i32>();
                                                                                            l27
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            Ok(e)
                                                                        }
                                                                        1 => {
                                                                            let e = {
                                                                                let l28 = i32::from(*base.add(12).cast::<u8>());
                                                                                match l28 {
                                                                                    0 => None,
                                                                                    1 => {
                                                                                        let e = {
                                                                                            let l29 = *base.add(16).cast::<i32>();
                                                                                            l29
                                                                                        };
                                                                                        Some(e)
                                                                                    }
                                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                                }
                                                                            };
                                                                            Err(e)
                                                                        }
                                                                        _ => _rt::invalid_enum_discriminant(),
                                                                    }
                                                                };
                                                                WitNode::ResultValue(e49)
                                                            }
                                                            8 => {
                                                                let e49 = {
                                                                    let l30 = i32::from(*base.add(8).cast::<u8>());
                                                                    l30 as u8
                                                                };
                                                                WitNode::PrimU8(e49)
                                                            }
                                                            9 => {
                                                                let e49 = {
                                                                    let l31 = i32::from(*base.add(8).cast::<u16>());
                                                                    l31 as u16
                                                                };
                                                                WitNode::PrimU16(e49)
                                                            }
                                                            10 => {
                                                                let e49 = {
                                                                    let l32 = *base.add(8).cast::<i32>();
                                                                    l32 as u32
                                                                };
                                                                WitNode::PrimU32(e49)
                                                            }
                                                            11 => {
                                                                let e49 = {
                                                                    let l33 = *base.add(8).cast::<i64>();
                                                                    l33 as u64
                                                                };
                                                                WitNode::PrimU64(e49)
                                                            }
                                                            12 => {
                                                                let e49 = {
                                                                    let l34 = i32::from(*base.add(8).cast::<i8>());
                                                                    l34 as i8
                                                                };
                                                                WitNode::PrimS8(e49)
                                                            }
                                                            13 => {
                                                                let e49 = {
                                                                    let l35 = i32::from(*base.add(8).cast::<i16>());
                                                                    l35 as i16
                                                                };
                                                                WitNode::PrimS16(e49)
                                                            }
                                                            14 => {
                                                                let e49 = {
                                                                    let l36 = *base.add(8).cast::<i32>();
                                                                    l36
                                                                };
                                                                WitNode::PrimS32(e49)
                                                            }
                                                            15 => {
                                                                let e49 = {
                                                                    let l37 = *base.add(8).cast::<i64>();
                                                                    l37
                                                                };
                                                                WitNode::PrimS64(e49)
                                                            }
                                                            16 => {
                                                                let e49 = {
                                                                    let l38 = *base.add(8).cast::<f32>();
                                                                    l38
                                                                };
                                                                WitNode::PrimFloat32(e49)
                                                            }
                                                            17 => {
                                                                let e49 = {
                                                                    let l39 = *base.add(8).cast::<f64>();
                                                                    l39
                                                                };
                                                                WitNode::PrimFloat64(e49)
                                                            }
                                                            18 => {
                                                                let e49 = {
                                                                    let l40 = *base.add(8).cast::<i32>();
                                                                    _rt::char_lift(l40 as u32)
                                                                };
                                                                WitNode::PrimChar(e49)
                                                            }
                                                            19 => {
                                                                let e49 = {
                                                                    let l41 = i32::from(*base.add(8).cast::<u8>());
                                                                    _rt::bool_lift(l41 as u8)
                                                                };
                                                                WitNode::PrimBool(e49)
                                                            }
                                                            20 => {
                                                                let e49 = {
                                                                    let l42 = *base.add(8).cast::<*mut u8>();
                                                                    let l43 = *base.add(12).cast::<usize>();
                                                                    let len44 = l43;
                                                                    let bytes44 = _rt::Vec::from_raw_parts(
                                                                        l42.cast(),
                                                                        len44,
                                                                        len44,
                                                                    );
                                                                    _rt::string_lift(bytes44)
                                                                };
                                                                WitNode::PrimString(e49)
                                                            }
                                                            n => {
                                                                debug_assert_eq!(n, 21, "invalid enum discriminant");
                                                                let e49 = {
                                                                    let l45 = *base.add(8).cast::<*mut u8>();
                                                                    let l46 = *base.add(12).cast::<usize>();
                                                                    let len47 = l46;
                                                                    let bytes47 = _rt::Vec::from_raw_parts(
                                                                        l45.cast(),
                                                                        len47,
                                                                        len47,
                                                                    );
                                                                    let l48 = *base.add(16).cast::<i64>();
                                                                    (
                                                                        Uri {
                                                                            value: _rt::string_lift(bytes47),
                                                                        },
                                                                        l48 as u64,
                                                                    )
                                                                };
                                                                WitNode::Handle(e49)
                                                            }
                                                        };
                                                        v49
                                                    };
                                                    result50.push(e50);
                                                }
                                                _rt::cabi_dealloc(base50, len50 * 24, 8);
                                                WitValue { nodes: result50 }
                                            };
                                            Ok(e)
                                        }
                                        1 => {
                                            let e = {
                                                let l51 = i32::from(*ptr0.add(8).cast::<u8>());
                                                let v64 = match l51 {
                                                    0 => {
                                                        let e64 = {
                                                            let l52 = *ptr0.add(12).cast::<*mut u8>();
                                                            let l53 = *ptr0.add(16).cast::<usize>();
                                                            let len54 = l53;
                                                            let bytes54 = _rt::Vec::from_raw_parts(
                                                                l52.cast(),
                                                                len54,
                                                                len54,
                                                            );
                                                            _rt::string_lift(bytes54)
                                                        };
                                                        RpcError::ProtocolError(e64)
                                                    }
                                                    1 => {
                                                        let e64 = {
                                                            let l55 = *ptr0.add(12).cast::<*mut u8>();
                                                            let l56 = *ptr0.add(16).cast::<usize>();
                                                            let len57 = l56;
                                                            let bytes57 = _rt::Vec::from_raw_parts(
                                                                l55.cast(),
                                                                len57,
                                                                len57,
                                                            );
                                                            _rt::string_lift(bytes57)
                                                        };
                                                        RpcError::Denied(e64)
                                                    }
                                                    2 => {
                                                        let e64 = {
                                                            let l58 = *ptr0.add(12).cast::<*mut u8>();
                                                            let l59 = *ptr0.add(16).cast::<usize>();
                                                            let len60 = l59;
                                                            let bytes60 = _rt::Vec::from_raw_parts(
                                                                l58.cast(),
                                                                len60,
                                                                len60,
                                                            );
                                                            _rt::string_lift(bytes60)
                                                        };
                                                        RpcError::NotFound(e64)
                                                    }
                                                    n => {
                                                        debug_assert_eq!(n, 3, "invalid enum discriminant");
                                                        let e64 = {
                                                            let l61 = *ptr0.add(12).cast::<*mut u8>();
                                                            let l62 = *ptr0.add(16).cast::<usize>();
                                                            let len63 = l62;
                                                            let bytes63 = _rt::Vec::from_raw_parts(
                                                                l61.cast(),
                                                                len63,
                                                                len63,
                                                            );
                                                            _rt::string_lift(bytes63)
                                                        };
                                                        RpcError::RemoteInternalError(e64)
                                                    }
                                                };
                                                v64
                                            };
                                            Err(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod io {
        #[allow(dead_code, clippy::all)]
        pub mod poll {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// `pollable` epresents a single I/O event which may be ready, or not.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Pollable {
                handle: _rt::Resource<Pollable>,
            }
            impl Pollable {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Pollable {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]pollable"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the readiness of a pollable. This function never blocks.
                ///
                /// Returns `true` when the pollable is ready, and `false` otherwise.
                pub fn ready(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.ready"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// `block` returns immediately if the pollable is ready, and otherwise
                /// blocks until ready.
                ///
                /// This function is equivalent to calling `poll.poll` on a list
                /// containing only this pollable.
                pub fn block(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.block"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Poll for completion on a set of pollables.
            ///
            /// This function takes a list of pollables, which identify I/O sources of
            /// interest, and waits until one or more of the events is ready for I/O.
            ///
            /// The result `list<u32>` contains one or more indices of handles in the
            /// argument list that is ready for I/O.
            ///
            /// If the list contains more elements than can be indexed with a `u32`
            /// value, this function traps.
            ///
            /// A timeout can be implemented by adding a pollable from the
            /// wasi-clocks API to the list.
            ///
            /// This function does not return a `result`; polling in itself does not
            /// do any I/O so it doesn't fail. If any of the I/O sources identified by
            /// the pollables has an error, it is indicated by marking the source as
            /// being reaedy for I/O.
            pub fn poll(in_: &[&Pollable]) -> _rt::Vec<u32> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = in_;
                    let len0 = vec0.len();
                    let layout0 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec0.len() * 4,
                        4,
                    );
                    let result0 = if layout0.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout0);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = (e).handle() as i32;
                        }
                    }
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                    extern "C" {
                        #[link_name = "poll"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(result0, len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    if layout0.size() != 0 {
                        _rt::alloc::dealloc(result0.cast(), layout0);
                    }
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod golem {
        #[allow(dead_code)]
        pub mod component_counter_stub {
            #[allow(dead_code, clippy::all)]
            pub mod stub_counter {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type GolemRpcUri = super::super::super::super::golem::rpc::types::Uri;
                pub type WasiIoPollable = super::super::super::super::wasi::io::poll::Pollable;
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct FutureGetResult {
                    handle: _rt::Resource<FutureGetResult>,
                }
                type _FutureGetResultRep<T> = Option<T>;
                impl FutureGetResult {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `FutureGetResult`.
                    pub fn new<T: GuestFutureGetResult>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _FutureGetResultRep<T> = Some(val);
                        let ptr: *mut _FutureGetResultRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestFutureGetResult>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestFutureGetResult>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestFutureGetResult>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(
                            handle as *mut _FutureGetResultRep<T>,
                        );
                    }
                    fn as_ptr<T: GuestFutureGetResult>(
                        &self,
                    ) -> *mut _FutureGetResultRep<T> {
                        FutureGetResult::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`FutureGetResult`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct FutureGetResultBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a FutureGetResult>,
                }
                impl<'a> FutureGetResultBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestFutureGetResult>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _FutureGetResultRep<T> {
                        FutureGetResult::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for FutureGetResult {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]golem:component-counter-stub/stub-counter"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]future-get-result"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ApiCounter {
                    handle: _rt::Resource<ApiCounter>,
                }
                type _ApiCounterRep<T> = Option<T>;
                impl ApiCounter {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `ApiCounter`.
                    pub fn new<T: GuestApiCounter>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ApiCounterRep<T> = Some(val);
                        let ptr: *mut _ApiCounterRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestApiCounter>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestApiCounter>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestApiCounter>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _ApiCounterRep<T>);
                    }
                    fn as_ptr<T: GuestApiCounter>(&self) -> *mut _ApiCounterRep<T> {
                        ApiCounter::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`ApiCounter`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ApiCounterBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a ApiCounter>,
                }
                impl<'a> ApiCounterBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestApiCounter>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ApiCounterRep<T> {
                        ApiCounter::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for ApiCounter {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]golem:component-counter-stub/stub-counter"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]api-counter"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_future_get_result_subscribe_cabi<
                    T: GuestFutureGetResult,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::subscribe(
                        FutureGetResultBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_future_get_result_get_cabi<
                    T: GuestFutureGetResult,
                >(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get(
                        FutureGetResultBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(8).cast::<i64>() = _rt::as_i64(e);
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_api_counter_cabi<T: GuestApiCounter>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = ApiCounter::new(
                        T::new(super::super::super::super::golem::rpc::types::Uri {
                            value: _rt::string_lift(bytes0),
                        }),
                    );
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_api_counter_blocking_add_cabi<
                    T: GuestApiCounter,
                >(arg0: *mut u8, arg1: i64) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::blocking_add(
                        ApiCounterBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_api_counter_add_cabi<T: GuestApiCounter>(
                    arg0: *mut u8,
                    arg1: i64,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::add(
                        ApiCounterBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_api_counter_blocking_get_cabi<
                    T: GuestApiCounter,
                >(arg0: *mut u8) -> i64 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::blocking_get(
                        ApiCounterBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i64(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_api_counter_get_cabi<T: GuestApiCounter>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get(
                        ApiCounterBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_api_counter_blocking_run_all_tasks_cabi<
                    T: GuestApiCounter,
                >(arg0: *mut u8) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::blocking_run_all_tasks(
                        ApiCounterBorrow::lift(arg0 as u32 as usize).get(),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_api_counter_run_all_tasks_cabi<
                    T: GuestApiCounter,
                >(arg0: *mut u8) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::run_all_tasks(ApiCounterBorrow::lift(arg0 as u32 as usize).get());
                }
                pub trait Guest {
                    type FutureGetResult: GuestFutureGetResult;
                    type ApiCounter: GuestApiCounter;
                }
                pub trait GuestFutureGetResult: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]golem:component-counter-stub/stub-counter"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]future-get-result"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]golem:component-counter-stub/stub-counter"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]future-get-result"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn subscribe(&self) -> WasiIoPollable;
                    fn get(&self) -> Option<u64>;
                }
                pub trait GuestApiCounter: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]golem:component-counter-stub/stub-counter"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]api-counter"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]golem:component-counter-stub/stub-counter"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]api-counter"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(location: GolemRpcUri) -> Self;
                    fn blocking_add(&self, value: u64);
                    fn add(&self, value: u64);
                    fn blocking_get(&self) -> u64;
                    fn get(&self) -> FutureGetResult;
                    fn blocking_run_all_tasks(&self);
                    fn run_all_tasks(&self);
                }
                #[doc(hidden)]
                macro_rules! __export_golem_component_counter_stub_stub_counter_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "golem:component-counter-stub/stub-counter#[method]future-get-result.subscribe"]
                        unsafe extern "C" fn
                        export_method_future_get_result_subscribe(arg0 : * mut u8,) ->
                        i32 { $($path_to_types)*::
                        _export_method_future_get_result_subscribe_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FutureGetResult > (arg0) }
                        #[export_name =
                        "golem:component-counter-stub/stub-counter#[method]future-get-result.get"]
                        unsafe extern "C" fn export_method_future_get_result_get(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_future_get_result_get_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::FutureGetResult > (arg0) }
                        #[export_name =
                        "golem:component-counter-stub/stub-counter#[constructor]api-counter"]
                        unsafe extern "C" fn export_constructor_api_counter(arg0 : * mut
                        u8, arg1 : usize,) -> i32 { $($path_to_types)*::
                        _export_constructor_api_counter_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (arg0, arg1) }
                        #[export_name =
                        "golem:component-counter-stub/stub-counter#[method]api-counter.blocking-add"]
                        unsafe extern "C" fn export_method_api_counter_blocking_add(arg0
                        : * mut u8, arg1 : i64,) { $($path_to_types)*::
                        _export_method_api_counter_blocking_add_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (arg0, arg1) }
                        #[export_name =
                        "golem:component-counter-stub/stub-counter#[method]api-counter.add"]
                        unsafe extern "C" fn export_method_api_counter_add(arg0 : * mut
                        u8, arg1 : i64,) { $($path_to_types)*::
                        _export_method_api_counter_add_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (arg0, arg1) }
                        #[export_name =
                        "golem:component-counter-stub/stub-counter#[method]api-counter.blocking-get"]
                        unsafe extern "C" fn export_method_api_counter_blocking_get(arg0
                        : * mut u8,) -> i64 { $($path_to_types)*::
                        _export_method_api_counter_blocking_get_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (arg0) } #[export_name
                        =
                        "golem:component-counter-stub/stub-counter#[method]api-counter.get"]
                        unsafe extern "C" fn export_method_api_counter_get(arg0 : * mut
                        u8,) -> i32 { $($path_to_types)*::
                        _export_method_api_counter_get_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (arg0) } #[export_name
                        =
                        "golem:component-counter-stub/stub-counter#[method]api-counter.blocking-run-all-tasks"]
                        unsafe extern "C" fn
                        export_method_api_counter_blocking_run_all_tasks(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        _export_method_api_counter_blocking_run_all_tasks_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (arg0) } #[export_name
                        =
                        "golem:component-counter-stub/stub-counter#[method]api-counter.run-all-tasks"]
                        unsafe extern "C" fn export_method_api_counter_run_all_tasks(arg0
                        : * mut u8,) { $($path_to_types)*::
                        _export_method_api_counter_run_all_tasks_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (arg0) } const _ : ()
                        = { #[doc(hidden)] #[export_name =
                        "golem:component-counter-stub/stub-counter#[dtor]future-get-result"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: FutureGetResult::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::FutureGetResult > (rep) } }; const
                        _ : () = { #[doc(hidden)] #[export_name =
                        "golem:component-counter-stub/stub-counter#[dtor]api-counter"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: ApiCounter::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::ApiCounter > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_golem_component_counter_stub_stub_counter_cabi;
                #[repr(align(8))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::alloc;
    pub use alloc_crate::string::String;
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub fn as_f64<T: AsF64>(t: T) -> f64 {
        t.as_f64()
    }
    pub trait AsF64 {
        fn as_f64(self) -> f64;
    }
    impl<'a, T: Copy + AsF64> AsF64 for &'a T {
        fn as_f64(self) -> f64 {
            (*self).as_f64()
        }
    }
    impl AsF64 for f64 {
        #[inline]
        fn as_f64(self) -> f64 {
            self as f64
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub unsafe fn char_lift(val: u32) -> char {
        if cfg!(debug_assertions) {
            core::char::from_u32(val).unwrap()
        } else {
            core::char::from_u32_unchecked(val)
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_wasm_rpc_stub_counter_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::golem::component_counter_stub::stub_counter::__export_golem_component_counter_stub_stub_counter_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::golem::component_counter_stub::stub_counter);
    };
}
#[doc(inline)]
pub(crate) use __export_wasm_rpc_stub_counter_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:golem:component-counter-stub:wasm-rpc-stub-counter:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1917] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf1\x0d\x01A\x02\x01\
A\x08\x01B\x0a\x04\0\x08pollable\x03\x01\x01h\0\x01@\x01\x04self\x01\0\x7f\x04\0\
\x16[method]pollable.ready\x01\x02\x01@\x01\x04self\x01\x01\0\x04\0\x16[method]p\
ollable.block\x01\x03\x01p\x01\x01py\x01@\x01\x02in\x04\0\x05\x04\0\x04poll\x01\x06\
\x03\x01\x12wasi:io/poll@0.2.0\x05\0\x02\x03\0\0\x08pollable\x01B*\x02\x03\x02\x01\
\x01\x04\0\x08pollable\x03\0\0\x01z\x04\0\x0anode-index\x03\0\x02\x01r\x01\x05va\
lues\x04\0\x03uri\x03\0\x04\x01p\x03\x01k\x03\x01o\x02y\x07\x01p\x7f\x01j\x01\x07\
\x01\x07\x01o\x02\x05w\x01q\x16\x0crecord-value\x01\x06\0\x0dvariant-value\x01\x08\
\0\x0aenum-value\x01y\0\x0bflags-value\x01\x09\0\x0btuple-value\x01\x06\0\x0alis\
t-value\x01\x06\0\x0coption-value\x01\x07\0\x0cresult-value\x01\x0a\0\x07prim-u8\
\x01}\0\x08prim-u16\x01{\0\x08prim-u32\x01y\0\x08prim-u64\x01w\0\x07prim-s8\x01~\
\0\x08prim-s16\x01|\0\x08prim-s32\x01z\0\x08prim-s64\x01x\0\x0cprim-float32\x01v\
\0\x0cprim-float64\x01u\0\x09prim-char\x01t\0\x09prim-bool\x01\x7f\0\x0bprim-str\
ing\x01s\0\x06handle\x01\x0b\0\x04\0\x08wit-node\x03\0\x0c\x01p\x0d\x01r\x01\x05\
nodes\x0e\x04\0\x09wit-value\x03\0\x0f\x01q\x04\x0eprotocol-error\x01s\0\x06deni\
ed\x01s\0\x09not-found\x01s\0\x15remote-internal-error\x01s\0\x04\0\x09rpc-error\
\x03\0\x11\x04\0\x08wasm-rpc\x03\x01\x04\0\x14future-invoke-result\x03\x01\x01i\x13\
\x01@\x01\x08location\x05\0\x15\x04\0\x15[constructor]wasm-rpc\x01\x16\x01h\x13\x01\
p\x10\x01j\x01\x10\x01\x12\x01@\x03\x04self\x17\x0dfunction-names\x0ffunction-pa\
rams\x18\0\x19\x04\0![method]wasm-rpc.invoke-and-await\x01\x1a\x01j\0\x01\x12\x01\
@\x03\x04self\x17\x0dfunction-names\x0ffunction-params\x18\0\x1b\x04\0\x17[metho\
d]wasm-rpc.invoke\x01\x1c\x01i\x14\x01@\x03\x04self\x17\x0dfunction-names\x0ffun\
ction-params\x18\0\x1d\x04\0'[method]wasm-rpc.async-invoke-and-await\x01\x1e\x01\
h\x14\x01i\x01\x01@\x01\x04self\x1f\0\x20\x04\0&[method]future-invoke-result.sub\
scribe\x01!\x01k\x19\x01@\x01\x04self\x1f\0\"\x04\0\x20[method]future-invoke-res\
ult.get\x01#\x03\x01\x15golem:rpc/types@0.1.0\x05\x02\x02\x03\0\x01\x03uri\x01B\x1c\
\x02\x03\x02\x01\x03\x04\0\x0dgolem-rpc-uri\x03\0\0\x02\x03\x02\x01\x01\x04\0\x10\
wasi-io-pollable\x03\0\x02\x04\0\x11future-get-result\x03\x01\x04\0\x0bapi-count\
er\x03\x01\x01h\x04\x01i\x03\x01@\x01\x04self\x06\0\x07\x04\0#[method]future-get\
-result.subscribe\x01\x08\x01kw\x01@\x01\x04self\x06\0\x09\x04\0\x1d[method]futu\
re-get-result.get\x01\x0a\x01i\x05\x01@\x01\x08location\x01\0\x0b\x04\0\x18[cons\
tructor]api-counter\x01\x0c\x01h\x05\x01@\x02\x04self\x0d\x05valuew\x01\0\x04\0\x20\
[method]api-counter.blocking-add\x01\x0e\x04\0\x17[method]api-counter.add\x01\x0e\
\x01@\x01\x04self\x0d\0w\x04\0\x20[method]api-counter.blocking-get\x01\x0f\x01i\x04\
\x01@\x01\x04self\x0d\0\x10\x04\0\x17[method]api-counter.get\x01\x11\x01@\x01\x04\
self\x0d\x01\0\x04\0*[method]api-counter.blocking-run-all-tasks\x01\x12\x04\0![m\
ethod]api-counter.run-all-tasks\x01\x12\x04\x01)golem:component-counter-stub/stu\
b-counter\x05\x04\x04\x012golem:component-counter-stub/wasm-rpc-stub-counter\x04\
\0\x0b\x1b\x01\0\x15wasm-rpc-stub-counter\x03\0\0\0G\x09producers\x01\x0cprocess\
ed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
