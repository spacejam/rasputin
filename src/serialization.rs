// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct SetReq {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SetReq {
    pub fn new() -> SetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetReq {
        static mut instance: ::protobuf::lazy::Lazy<SetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetReq,
        };
        unsafe {
            instance.get(|| {
                SetReq {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SetReq {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetReq {
    fn new() -> SetReq {
        SetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    SetReq::has_key,
                    SetReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    SetReq::has_value,
                    SetReq::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetReq>(
                    "SetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetReq {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetReq {
    fn eq(&self, other: &SetReq) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetRes {
    // message fields
    success: ::std::option::Option<bool>,
    txid: ::std::option::Option<u64>,
    err: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SetRes {
    pub fn new() -> SetRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetRes {
        static mut instance: ::protobuf::lazy::Lazy<SetRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetRes,
        };
        unsafe {
            instance.get(|| {
                SetRes {
                    success: ::std::option::Option::None,
                    txid: ::std::option::Option::None,
                    err: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success<'a>(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // required uint64 txid = 2;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }

    // optional string err = 3;

    pub fn clear_err(&mut self) {
        self.err.clear();
    }

    pub fn has_err(&self) -> bool {
        self.err.is_some()
    }

    // Param is passed by value, moved
    pub fn set_err(&mut self, v: ::std::string::String) {
        self.err = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_err<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.err.is_none() {
            self.err.set_default();
        };
        self.err.as_mut().unwrap()
    }

    // Take field
    pub fn take_err(&mut self) -> ::std::string::String {
        self.err.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_err<'a>(&'a self) -> &'a str {
        match self.err.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SetRes {
    fn is_initialized(&self) -> bool {
        if self.success.is_none() {
            return false;
        };
        if self.txid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.err.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success.is_some() {
            my_size += 2;
        };
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.err.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.txid {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.err.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetRes {
    fn new() -> SetRes {
        SetRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    SetRes::has_success,
                    SetRes::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    SetRes::has_txid,
                    SetRes::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "err",
                    SetRes::has_err,
                    SetRes::get_err,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetRes>(
                    "SetRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetRes {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_txid();
        self.clear_err();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetRes {
    fn eq(&self, other: &SetRes) -> bool {
        self.success == other.success &&
        self.txid == other.txid &&
        self.err == other.err &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetReq {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetReq {
    pub fn new() -> GetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReq {
        static mut instance: ::protobuf::lazy::Lazy<GetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReq,
        };
        unsafe {
            instance.get(|| {
                GetReq {
                    key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for GetReq {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetReq {
    fn new() -> GetReq {
        GetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    GetReq::has_key,
                    GetReq::get_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReq>(
                    "GetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReq {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetReq {
    fn eq(&self, other: &GetReq) -> bool {
        self.key == other.key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetRes {
    // message fields
    success: ::std::option::Option<bool>,
    txid: ::std::option::Option<u64>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    err: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetRes {
    pub fn new() -> GetRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRes {
        static mut instance: ::protobuf::lazy::Lazy<GetRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRes,
        };
        unsafe {
            instance.get(|| {
                GetRes {
                    success: ::std::option::Option::None,
                    txid: ::std::option::Option::None,
                    value: ::protobuf::SingularField::none(),
                    err: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success<'a>(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // required uint64 txid = 2;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }

    // required bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string err = 4;

    pub fn clear_err(&mut self) {
        self.err.clear();
    }

    pub fn has_err(&self) -> bool {
        self.err.is_some()
    }

    // Param is passed by value, moved
    pub fn set_err(&mut self, v: ::std::string::String) {
        self.err = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_err<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.err.is_none() {
            self.err.set_default();
        };
        self.err.as_mut().unwrap()
    }

    // Take field
    pub fn take_err(&mut self) -> ::std::string::String {
        self.err.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_err<'a>(&'a self) -> &'a str {
        match self.err.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetRes {
    fn is_initialized(&self) -> bool {
        if self.success.is_none() {
            return false;
        };
        if self.txid.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.err.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success.is_some() {
            my_size += 2;
        };
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.err.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.txid {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.err.as_ref() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetRes {
    fn new() -> GetRes {
        GetRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    GetRes::has_success,
                    GetRes::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    GetRes::has_txid,
                    GetRes::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    GetRes::has_value,
                    GetRes::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "err",
                    GetRes::has_err,
                    GetRes::get_err,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRes>(
                    "GetRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRes {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_txid();
        self.clear_value();
        self.clear_err();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetRes {
    fn eq(&self, other: &GetRes) -> bool {
        self.success == other.success &&
        self.txid == other.txid &&
        self.value == other.value &&
        self.err == other.err &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CASReq {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    oldvalue: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CASReq {
    pub fn new() -> CASReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CASReq {
        static mut instance: ::protobuf::lazy::Lazy<CASReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CASReq,
        };
        unsafe {
            instance.get(|| {
                CASReq {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    oldvalue: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes oldvalue = 3;

    pub fn clear_oldvalue(&mut self) {
        self.oldvalue.clear();
    }

    pub fn has_oldvalue(&self) -> bool {
        self.oldvalue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oldvalue(&mut self, v: ::std::vec::Vec<u8>) {
        self.oldvalue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oldvalue<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.oldvalue.is_none() {
            self.oldvalue.set_default();
        };
        self.oldvalue.as_mut().unwrap()
    }

    // Take field
    pub fn take_oldvalue(&mut self) -> ::std::vec::Vec<u8> {
        self.oldvalue.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_oldvalue<'a>(&'a self) -> &'a [u8] {
        match self.oldvalue.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CASReq {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        if self.oldvalue.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.oldvalue.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.oldvalue.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.oldvalue.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CASReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CASReq {
    fn new() -> CASReq {
        CASReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<CASReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    CASReq::has_key,
                    CASReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    CASReq::has_value,
                    CASReq::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "oldvalue",
                    CASReq::has_oldvalue,
                    CASReq::get_oldvalue,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CASReq>(
                    "CASReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CASReq {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.clear_oldvalue();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CASReq {
    fn eq(&self, other: &CASReq) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.oldvalue == other.oldvalue &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CASReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CASRes {
    // message fields
    success: ::std::option::Option<bool>,
    txid: ::std::option::Option<u64>,
    err: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CASRes {
    pub fn new() -> CASRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CASRes {
        static mut instance: ::protobuf::lazy::Lazy<CASRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CASRes,
        };
        unsafe {
            instance.get(|| {
                CASRes {
                    success: ::std::option::Option::None,
                    txid: ::std::option::Option::None,
                    err: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success<'a>(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // required uint64 txid = 2;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }

    // optional string err = 3;

    pub fn clear_err(&mut self) {
        self.err.clear();
    }

    pub fn has_err(&self) -> bool {
        self.err.is_some()
    }

    // Param is passed by value, moved
    pub fn set_err(&mut self, v: ::std::string::String) {
        self.err = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_err<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.err.is_none() {
            self.err.set_default();
        };
        self.err.as_mut().unwrap()
    }

    // Take field
    pub fn take_err(&mut self) -> ::std::string::String {
        self.err.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_err<'a>(&'a self) -> &'a str {
        match self.err.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CASRes {
    fn is_initialized(&self) -> bool {
        if self.success.is_none() {
            return false;
        };
        if self.txid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.err.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success.is_some() {
            my_size += 2;
        };
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.err.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.txid {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.err.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CASRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CASRes {
    fn new() -> CASRes {
        CASRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<CASRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    CASRes::has_success,
                    CASRes::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    CASRes::has_txid,
                    CASRes::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "err",
                    CASRes::has_err,
                    CASRes::get_err,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CASRes>(
                    "CASRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CASRes {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_txid();
        self.clear_err();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CASRes {
    fn eq(&self, other: &CASRes) -> bool {
        self.success == other.success &&
        self.txid == other.txid &&
        self.err == other.err &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CASRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WatchReq {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lasttxid: ::std::option::Option<u64>,
    recursive: ::std::option::Option<bool>,
    historical: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WatchReq {
    pub fn new() -> WatchReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WatchReq {
        static mut instance: ::protobuf::lazy::Lazy<WatchReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WatchReq,
        };
        unsafe {
            instance.get(|| {
                WatchReq {
                    key: ::protobuf::SingularField::none(),
                    lasttxid: ::std::option::Option::None,
                    recursive: ::std::option::Option::None,
                    historical: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint64 lasttxid = 2;

    pub fn clear_lasttxid(&mut self) {
        self.lasttxid = ::std::option::Option::None;
    }

    pub fn has_lasttxid(&self) -> bool {
        self.lasttxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lasttxid(&mut self, v: u64) {
        self.lasttxid = ::std::option::Option::Some(v);
    }

    pub fn get_lasttxid<'a>(&self) -> u64 {
        self.lasttxid.unwrap_or(0)
    }

    // required bool recursive = 3;

    pub fn clear_recursive(&mut self) {
        self.recursive = ::std::option::Option::None;
    }

    pub fn has_recursive(&self) -> bool {
        self.recursive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_recursive(&mut self, v: bool) {
        self.recursive = ::std::option::Option::Some(v);
    }

    pub fn get_recursive<'a>(&self) -> bool {
        self.recursive.unwrap_or(false)
    }

    // required bool historical = 4;

    pub fn clear_historical(&mut self) {
        self.historical = ::std::option::Option::None;
    }

    pub fn has_historical(&self) -> bool {
        self.historical.is_some()
    }

    // Param is passed by value, moved
    pub fn set_historical(&mut self, v: bool) {
        self.historical = ::std::option::Option::Some(v);
    }

    pub fn get_historical<'a>(&self) -> bool {
        self.historical.unwrap_or(false)
    }
}

impl ::protobuf::Message for WatchReq {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.lasttxid.is_none() {
            return false;
        };
        if self.recursive.is_none() {
            return false;
        };
        if self.historical.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lasttxid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.recursive = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.historical = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.lasttxid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.recursive.is_some() {
            my_size += 2;
        };
        if self.historical.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.lasttxid {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.recursive {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.historical {
            try!(os.write_bool(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WatchReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WatchReq {
    fn new() -> WatchReq {
        WatchReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<WatchReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    WatchReq::has_key,
                    WatchReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lasttxid",
                    WatchReq::has_lasttxid,
                    WatchReq::get_lasttxid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "recursive",
                    WatchReq::has_recursive,
                    WatchReq::get_recursive,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "historical",
                    WatchReq::has_historical,
                    WatchReq::get_historical,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WatchReq>(
                    "WatchReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WatchReq {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_lasttxid();
        self.clear_recursive();
        self.clear_historical();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WatchReq {
    fn eq(&self, other: &WatchReq) -> bool {
        self.key == other.key &&
        self.lasttxid == other.lasttxid &&
        self.recursive == other.recursive &&
        self.historical == other.historical &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WatchReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VersionedKV {
    // message fields
    txid: ::std::option::Option<u64>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl VersionedKV {
    pub fn new() -> VersionedKV {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionedKV {
        static mut instance: ::protobuf::lazy::Lazy<VersionedKV> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionedKV,
        };
        unsafe {
            instance.get(|| {
                VersionedKV {
                    txid: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }

    // required bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for VersionedKV {
    fn is_initialized(&self) -> bool {
        if self.txid.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txid {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VersionedKV>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VersionedKV {
    fn new() -> VersionedKV {
        VersionedKV::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionedKV>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    VersionedKV::has_txid,
                    VersionedKV::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    VersionedKV::has_key,
                    VersionedKV::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    VersionedKV::has_value,
                    VersionedKV::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VersionedKV>(
                    "VersionedKV",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionedKV {
    fn clear(&mut self) {
        self.clear_txid();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VersionedKV {
    fn eq(&self, other: &VersionedKV) -> bool {
        self.txid == other.txid &&
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VersionedKV {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WatchRes {
    // message fields
    success: ::std::option::Option<bool>,
    history: ::protobuf::RepeatedField<VersionedKV>,
    err: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WatchRes {
    pub fn new() -> WatchRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WatchRes {
        static mut instance: ::protobuf::lazy::Lazy<WatchRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WatchRes,
        };
        unsafe {
            instance.get(|| {
                WatchRes {
                    success: ::std::option::Option::None,
                    history: ::protobuf::RepeatedField::new(),
                    err: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success<'a>(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // repeated .quall.VersionedKV history = 2;

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    // Param is passed by value, moved
    pub fn set_history(&mut self, v: ::protobuf::RepeatedField<VersionedKV>) {
        self.history = v;
    }

    // Mutable pointer to the field.
    pub fn mut_history<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<VersionedKV> {
        &mut self.history
    }

    // Take field
    pub fn take_history(&mut self) -> ::protobuf::RepeatedField<VersionedKV> {
        ::std::mem::replace(&mut self.history, ::protobuf::RepeatedField::new())
    }

    pub fn get_history<'a>(&'a self) -> &'a [VersionedKV] {
        &self.history
    }

    // optional string err = 3;

    pub fn clear_err(&mut self) {
        self.err.clear();
    }

    pub fn has_err(&self) -> bool {
        self.err.is_some()
    }

    // Param is passed by value, moved
    pub fn set_err(&mut self, v: ::std::string::String) {
        self.err = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_err<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.err.is_none() {
            self.err.set_default();
        };
        self.err.as_mut().unwrap()
    }

    // Take field
    pub fn take_err(&mut self) -> ::std::string::String {
        self.err.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_err<'a>(&'a self) -> &'a str {
        match self.err.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for WatchRes {
    fn is_initialized(&self) -> bool {
        if self.success.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.history));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.err.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success.is_some() {
            my_size += 2;
        };
        for value in self.history.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.err.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        for v in self.history.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.err.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WatchRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WatchRes {
    fn new() -> WatchRes {
        WatchRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<WatchRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    WatchRes::has_success,
                    WatchRes::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "history",
                    WatchRes::get_history,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "err",
                    WatchRes::has_err,
                    WatchRes::get_err,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WatchRes>(
                    "WatchRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WatchRes {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_history();
        self.clear_err();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WatchRes {
    fn eq(&self, other: &WatchRes) -> bool {
        self.success == other.success &&
        self.history == other.history &&
        self.err == other.err &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WatchRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CliReq {
    // message fields
    get: ::protobuf::SingularPtrField<GetReq>,
    set: ::protobuf::SingularPtrField<SetReq>,
    cas: ::protobuf::SingularPtrField<CASReq>,
    watch: ::protobuf::SingularPtrField<WatchReq>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CliReq {
    pub fn new() -> CliReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CliReq {
        static mut instance: ::protobuf::lazy::Lazy<CliReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CliReq,
        };
        unsafe {
            instance.get(|| {
                CliReq {
                    get: ::protobuf::SingularPtrField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    cas: ::protobuf::SingularPtrField::none(),
                    watch: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .quall.GetReq get = 1;

    pub fn clear_get(&mut self) {
        self.get.clear();
    }

    pub fn has_get(&self) -> bool {
        self.get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetReq) {
        self.get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get<'a>(&'a mut self) -> &'a mut GetReq {
        if self.get.is_none() {
            self.get.set_default();
        };
        self.get.as_mut().unwrap()
    }

    // Take field
    pub fn take_get(&mut self) -> GetReq {
        self.get.take().unwrap_or_else(|| GetReq::new())
    }

    pub fn get_get<'a>(&'a self) -> &'a GetReq {
        self.get.as_ref().unwrap_or_else(|| GetReq::default_instance())
    }

    // optional .quall.SetReq set = 2;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    pub fn has_set(&self) -> bool {
        self.set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: SetReq) {
        self.set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set<'a>(&'a mut self) -> &'a mut SetReq {
        if self.set.is_none() {
            self.set.set_default();
        };
        self.set.as_mut().unwrap()
    }

    // Take field
    pub fn take_set(&mut self) -> SetReq {
        self.set.take().unwrap_or_else(|| SetReq::new())
    }

    pub fn get_set<'a>(&'a self) -> &'a SetReq {
        self.set.as_ref().unwrap_or_else(|| SetReq::default_instance())
    }

    // optional .quall.CASReq cas = 3;

    pub fn clear_cas(&mut self) {
        self.cas.clear();
    }

    pub fn has_cas(&self) -> bool {
        self.cas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cas(&mut self, v: CASReq) {
        self.cas = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cas<'a>(&'a mut self) -> &'a mut CASReq {
        if self.cas.is_none() {
            self.cas.set_default();
        };
        self.cas.as_mut().unwrap()
    }

    // Take field
    pub fn take_cas(&mut self) -> CASReq {
        self.cas.take().unwrap_or_else(|| CASReq::new())
    }

    pub fn get_cas<'a>(&'a self) -> &'a CASReq {
        self.cas.as_ref().unwrap_or_else(|| CASReq::default_instance())
    }

    // optional .quall.WatchReq watch = 4;

    pub fn clear_watch(&mut self) {
        self.watch.clear();
    }

    pub fn has_watch(&self) -> bool {
        self.watch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_watch(&mut self, v: WatchReq) {
        self.watch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_watch<'a>(&'a mut self) -> &'a mut WatchReq {
        if self.watch.is_none() {
            self.watch.set_default();
        };
        self.watch.as_mut().unwrap()
    }

    // Take field
    pub fn take_watch(&mut self) -> WatchReq {
        self.watch.take().unwrap_or_else(|| WatchReq::new())
    }

    pub fn get_watch<'a>(&'a self) -> &'a WatchReq {
        self.watch.as_ref().unwrap_or_else(|| WatchReq::default_instance())
    }
}

impl ::protobuf::Message for CliReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.get.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.set.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cas.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.watch.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.get.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.set.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cas.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.watch.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.get.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.set.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cas.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.watch.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CliReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CliReq {
    fn new() -> CliReq {
        CliReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<CliReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get",
                    CliReq::has_get,
                    CliReq::get_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set",
                    CliReq::has_set,
                    CliReq::get_set,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cas",
                    CliReq::has_cas,
                    CliReq::get_cas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "watch",
                    CliReq::has_watch,
                    CliReq::get_watch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CliReq>(
                    "CliReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CliReq {
    fn clear(&mut self) {
        self.clear_get();
        self.clear_set();
        self.clear_cas();
        self.clear_watch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CliReq {
    fn eq(&self, other: &CliReq) -> bool {
        self.get == other.get &&
        self.set == other.set &&
        self.cas == other.cas &&
        self.watch == other.watch &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CliReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CliRes {
    // message fields
    get: ::protobuf::SingularPtrField<GetRes>,
    set: ::protobuf::SingularPtrField<SetRes>,
    cas: ::protobuf::SingularPtrField<CASRes>,
    watch: ::protobuf::SingularPtrField<WatchRes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CliRes {
    pub fn new() -> CliRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CliRes {
        static mut instance: ::protobuf::lazy::Lazy<CliRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CliRes,
        };
        unsafe {
            instance.get(|| {
                CliRes {
                    get: ::protobuf::SingularPtrField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    cas: ::protobuf::SingularPtrField::none(),
                    watch: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .quall.GetRes get = 1;

    pub fn clear_get(&mut self) {
        self.get.clear();
    }

    pub fn has_get(&self) -> bool {
        self.get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetRes) {
        self.get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get<'a>(&'a mut self) -> &'a mut GetRes {
        if self.get.is_none() {
            self.get.set_default();
        };
        self.get.as_mut().unwrap()
    }

    // Take field
    pub fn take_get(&mut self) -> GetRes {
        self.get.take().unwrap_or_else(|| GetRes::new())
    }

    pub fn get_get<'a>(&'a self) -> &'a GetRes {
        self.get.as_ref().unwrap_or_else(|| GetRes::default_instance())
    }

    // optional .quall.SetRes set = 2;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    pub fn has_set(&self) -> bool {
        self.set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: SetRes) {
        self.set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set<'a>(&'a mut self) -> &'a mut SetRes {
        if self.set.is_none() {
            self.set.set_default();
        };
        self.set.as_mut().unwrap()
    }

    // Take field
    pub fn take_set(&mut self) -> SetRes {
        self.set.take().unwrap_or_else(|| SetRes::new())
    }

    pub fn get_set<'a>(&'a self) -> &'a SetRes {
        self.set.as_ref().unwrap_or_else(|| SetRes::default_instance())
    }

    // optional .quall.CASRes cas = 3;

    pub fn clear_cas(&mut self) {
        self.cas.clear();
    }

    pub fn has_cas(&self) -> bool {
        self.cas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cas(&mut self, v: CASRes) {
        self.cas = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cas<'a>(&'a mut self) -> &'a mut CASRes {
        if self.cas.is_none() {
            self.cas.set_default();
        };
        self.cas.as_mut().unwrap()
    }

    // Take field
    pub fn take_cas(&mut self) -> CASRes {
        self.cas.take().unwrap_or_else(|| CASRes::new())
    }

    pub fn get_cas<'a>(&'a self) -> &'a CASRes {
        self.cas.as_ref().unwrap_or_else(|| CASRes::default_instance())
    }

    // optional .quall.WatchRes watch = 4;

    pub fn clear_watch(&mut self) {
        self.watch.clear();
    }

    pub fn has_watch(&self) -> bool {
        self.watch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_watch(&mut self, v: WatchRes) {
        self.watch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_watch<'a>(&'a mut self) -> &'a mut WatchRes {
        if self.watch.is_none() {
            self.watch.set_default();
        };
        self.watch.as_mut().unwrap()
    }

    // Take field
    pub fn take_watch(&mut self) -> WatchRes {
        self.watch.take().unwrap_or_else(|| WatchRes::new())
    }

    pub fn get_watch<'a>(&'a self) -> &'a WatchRes {
        self.watch.as_ref().unwrap_or_else(|| WatchRes::default_instance())
    }
}

impl ::protobuf::Message for CliRes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.get.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.set.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cas.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.watch.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.get.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.set.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cas.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.watch.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.get.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.set.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cas.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.watch.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CliRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CliRes {
    fn new() -> CliRes {
        CliRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<CliRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get",
                    CliRes::has_get,
                    CliRes::get_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "set",
                    CliRes::has_set,
                    CliRes::get_set,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cas",
                    CliRes::has_cas,
                    CliRes::get_cas,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "watch",
                    CliRes::has_watch,
                    CliRes::get_watch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CliRes>(
                    "CliRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CliRes {
    fn clear(&mut self) {
        self.clear_get();
        self.clear_set();
        self.clear_cas();
        self.clear_watch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CliRes {
    fn eq(&self, other: &CliRes) -> bool {
        self.get == other.get &&
        self.set == other.set &&
        self.cas == other.cas &&
        self.watch == other.watch &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CliRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Append {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Append {
    pub fn new() -> Append {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Append {
        static mut instance: ::protobuf::lazy::Lazy<Append> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Append,
        };
        unsafe {
            instance.get(|| {
                Append {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Append {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Append>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Append {
    fn new() -> Append {
        Append::new()
    }

    fn descriptor_static(_: ::std::option::Option<Append>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    Append::has_key,
                    Append::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    Append::has_value,
                    Append::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Append>(
                    "Append",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Append {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Append {
    fn eq(&self, other: &Append) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Append {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Learn {
    // message fields
    txid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Learn {
    pub fn new() -> Learn {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Learn {
        static mut instance: ::protobuf::lazy::Lazy<Learn> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Learn,
        };
        unsafe {
            instance.get(|| {
                Learn {
                    txid: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }
}

impl ::protobuf::Message for Learn {
    fn is_initialized(&self) -> bool {
        if self.txid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txid {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Learn>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Learn {
    fn new() -> Learn {
        Learn::new()
    }

    fn descriptor_static(_: ::std::option::Option<Learn>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    Learn::has_txid,
                    Learn::get_txid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Learn>(
                    "Learn",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Learn {
    fn clear(&mut self) {
        self.clear_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Learn {
    fn eq(&self, other: &Learn) -> bool {
        self.txid == other.txid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Learn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Propose {
    // message fields
    txid: ::std::option::Option<u64>,
    mutations: ::protobuf::RepeatedField<Append>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Propose {
    pub fn new() -> Propose {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Propose {
        static mut instance: ::protobuf::lazy::Lazy<Propose> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Propose,
        };
        unsafe {
            instance.get(|| {
                Propose {
                    txid: ::std::option::Option::None,
                    mutations: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }

    // repeated .quall.Append mutations = 2;

    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }

    // Param is passed by value, moved
    pub fn set_mutations(&mut self, v: ::protobuf::RepeatedField<Append>) {
        self.mutations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mutations<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Append> {
        &mut self.mutations
    }

    // Take field
    pub fn take_mutations(&mut self) -> ::protobuf::RepeatedField<Append> {
        ::std::mem::replace(&mut self.mutations, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutations<'a>(&'a self) -> &'a [Append] {
        &self.mutations
    }
}

impl ::protobuf::Message for Propose {
    fn is_initialized(&self) -> bool {
        if self.txid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutations));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.mutations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txid {
            try!(os.write_uint64(1, v));
        };
        for v in self.mutations.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Propose>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Propose {
    fn new() -> Propose {
        Propose::new()
    }

    fn descriptor_static(_: ::std::option::Option<Propose>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    Propose::has_txid,
                    Propose::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "mutations",
                    Propose::get_mutations,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Propose>(
                    "Propose",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Propose {
    fn clear(&mut self) {
        self.clear_txid();
        self.clear_mutations();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Propose {
    fn eq(&self, other: &Propose) -> bool {
        self.txid == other.txid &&
        self.mutations == other.mutations &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Propose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Accept {
    // message fields
    txid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Accept {
    pub fn new() -> Accept {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Accept {
        static mut instance: ::protobuf::lazy::Lazy<Accept> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Accept,
        };
        unsafe {
            instance.get(|| {
                Accept {
                    txid: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }
}

impl ::protobuf::Message for Accept {
    fn is_initialized(&self) -> bool {
        if self.txid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txid {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Accept>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Accept {
    fn new() -> Accept {
        Accept::new()
    }

    fn descriptor_static(_: ::std::option::Option<Accept>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    Accept::has_txid,
                    Accept::get_txid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Accept>(
                    "Accept",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Accept {
    fn clear(&mut self) {
        self.clear_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Accept {
    fn eq(&self, other: &Accept) -> bool {
        self.txid == other.txid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Accept {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Reject {
    // message fields
    txid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Reject {
    pub fn new() -> Reject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Reject {
        static mut instance: ::protobuf::lazy::Lazy<Reject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Reject,
        };
        unsafe {
            instance.get(|| {
                Reject {
                    txid: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> u64 {
        self.txid.unwrap_or(0)
    }
}

impl ::protobuf::Message for Reject {
    fn is_initialized(&self) -> bool {
        if self.txid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.txid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txid {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Reject>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Reject {
    fn new() -> Reject {
        Reject::new()
    }

    fn descriptor_static(_: ::std::option::Option<Reject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    Reject::has_txid,
                    Reject::get_txid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Reject>(
                    "Reject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Reject {
    fn clear(&mut self) {
        self.clear_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Reject {
    fn eq(&self, other: &Reject) -> bool {
        self.txid == other.txid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Reject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VoteReq {
    // message fields
    maxtxid: ::std::option::Option<u64>,
    attempt: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl VoteReq {
    pub fn new() -> VoteReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VoteReq {
        static mut instance: ::protobuf::lazy::Lazy<VoteReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VoteReq,
        };
        unsafe {
            instance.get(|| {
                VoteReq {
                    maxtxid: ::std::option::Option::None,
                    attempt: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 maxtxid = 1;

    pub fn clear_maxtxid(&mut self) {
        self.maxtxid = ::std::option::Option::None;
    }

    pub fn has_maxtxid(&self) -> bool {
        self.maxtxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxtxid(&mut self, v: u64) {
        self.maxtxid = ::std::option::Option::Some(v);
    }

    pub fn get_maxtxid<'a>(&self) -> u64 {
        self.maxtxid.unwrap_or(0)
    }

    // required uint64 attempt = 2;

    pub fn clear_attempt(&mut self) {
        self.attempt = ::std::option::Option::None;
    }

    pub fn has_attempt(&self) -> bool {
        self.attempt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attempt(&mut self, v: u64) {
        self.attempt = ::std::option::Option::Some(v);
    }

    pub fn get_attempt<'a>(&self) -> u64 {
        self.attempt.unwrap_or(0)
    }
}

impl ::protobuf::Message for VoteReq {
    fn is_initialized(&self) -> bool {
        if self.maxtxid.is_none() {
            return false;
        };
        if self.attempt.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.maxtxid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.attempt = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.maxtxid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.attempt.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.maxtxid {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.attempt {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VoteReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VoteReq {
    fn new() -> VoteReq {
        VoteReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<VoteReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "maxtxid",
                    VoteReq::has_maxtxid,
                    VoteReq::get_maxtxid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "attempt",
                    VoteReq::has_attempt,
                    VoteReq::get_attempt,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VoteReq>(
                    "VoteReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VoteReq {
    fn clear(&mut self) {
        self.clear_maxtxid();
        self.clear_attempt();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VoteReq {
    fn eq(&self, other: &VoteReq) -> bool {
        self.maxtxid == other.maxtxid &&
        self.attempt == other.attempt &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VoteReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VoteRes {
    // message fields
    success: ::std::option::Option<bool>,
    attempt: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl VoteRes {
    pub fn new() -> VoteRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VoteRes {
        static mut instance: ::protobuf::lazy::Lazy<VoteRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VoteRes,
        };
        unsafe {
            instance.get(|| {
                VoteRes {
                    success: ::std::option::Option::None,
                    attempt: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success<'a>(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // required uint64 attempt = 2;

    pub fn clear_attempt(&mut self) {
        self.attempt = ::std::option::Option::None;
    }

    pub fn has_attempt(&self) -> bool {
        self.attempt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attempt(&mut self, v: u64) {
        self.attempt = ::std::option::Option::Some(v);
    }

    pub fn get_attempt<'a>(&self) -> u64 {
        self.attempt.unwrap_or(0)
    }
}

impl ::protobuf::Message for VoteRes {
    fn is_initialized(&self) -> bool {
        if self.success.is_none() {
            return false;
        };
        if self.attempt.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.attempt = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success.is_some() {
            my_size += 2;
        };
        for value in self.attempt.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.attempt {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VoteRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VoteRes {
    fn new() -> VoteRes {
        VoteRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<VoteRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    VoteRes::has_success,
                    VoteRes::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "attempt",
                    VoteRes::has_attempt,
                    VoteRes::get_attempt,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VoteRes>(
                    "VoteRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VoteRes {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_attempt();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VoteRes {
    fn eq(&self, other: &VoteRes) -> bool {
        self.success == other.success &&
        self.attempt == other.attempt &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VoteRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BatchReq {
    // message fields
    fromtxid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BatchReq {
    pub fn new() -> BatchReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchReq {
        static mut instance: ::protobuf::lazy::Lazy<BatchReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchReq,
        };
        unsafe {
            instance.get(|| {
                BatchReq {
                    fromtxid: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 fromtxid = 1;

    pub fn clear_fromtxid(&mut self) {
        self.fromtxid = ::std::option::Option::None;
    }

    pub fn has_fromtxid(&self) -> bool {
        self.fromtxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromtxid(&mut self, v: u64) {
        self.fromtxid = ::std::option::Option::Some(v);
    }

    pub fn get_fromtxid<'a>(&self) -> u64 {
        self.fromtxid.unwrap_or(0)
    }
}

impl ::protobuf::Message for BatchReq {
    fn is_initialized(&self) -> bool {
        if self.fromtxid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.fromtxid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.fromtxid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fromtxid {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BatchReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BatchReq {
    fn new() -> BatchReq {
        BatchReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fromtxid",
                    BatchReq::has_fromtxid,
                    BatchReq::get_fromtxid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchReq>(
                    "BatchReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchReq {
    fn clear(&mut self) {
        self.clear_fromtxid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BatchReq {
    fn eq(&self, other: &BatchReq) -> bool {
        self.fromtxid == other.fromtxid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BatchReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BatchRes {
    // message fields
    history: ::protobuf::RepeatedField<VersionedKV>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BatchRes {
    pub fn new() -> BatchRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchRes {
        static mut instance: ::protobuf::lazy::Lazy<BatchRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchRes,
        };
        unsafe {
            instance.get(|| {
                BatchRes {
                    history: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .quall.VersionedKV history = 1;

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    // Param is passed by value, moved
    pub fn set_history(&mut self, v: ::protobuf::RepeatedField<VersionedKV>) {
        self.history = v;
    }

    // Mutable pointer to the field.
    pub fn mut_history<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<VersionedKV> {
        &mut self.history
    }

    // Take field
    pub fn take_history(&mut self) -> ::protobuf::RepeatedField<VersionedKV> {
        ::std::mem::replace(&mut self.history, ::protobuf::RepeatedField::new())
    }

    pub fn get_history<'a>(&'a self) -> &'a [VersionedKV] {
        &self.history
    }
}

impl ::protobuf::Message for BatchRes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.history));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.history.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.history.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BatchRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BatchRes {
    fn new() -> BatchRes {
        BatchRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "history",
                    BatchRes::get_history,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchRes>(
                    "BatchRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchRes {
    fn clear(&mut self) {
        self.clear_history();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BatchRes {
    fn eq(&self, other: &BatchRes) -> bool {
        self.history == other.history &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BatchRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SrvReq {
    // message fields
    srvid: ::std::option::Option<u64>,
    voteReq: ::protobuf::SingularPtrField<VoteReq>,
    batchReq: ::protobuf::SingularPtrField<BatchReq>,
    learn: ::protobuf::SingularPtrField<Learn>,
    propose: ::protobuf::SingularPtrField<Propose>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SrvReq {
    pub fn new() -> SrvReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SrvReq {
        static mut instance: ::protobuf::lazy::Lazy<SrvReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SrvReq,
        };
        unsafe {
            instance.get(|| {
                SrvReq {
                    srvid: ::std::option::Option::None,
                    voteReq: ::protobuf::SingularPtrField::none(),
                    batchReq: ::protobuf::SingularPtrField::none(),
                    learn: ::protobuf::SingularPtrField::none(),
                    propose: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 srvid = 1;

    pub fn clear_srvid(&mut self) {
        self.srvid = ::std::option::Option::None;
    }

    pub fn has_srvid(&self) -> bool {
        self.srvid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_srvid(&mut self, v: u64) {
        self.srvid = ::std::option::Option::Some(v);
    }

    pub fn get_srvid<'a>(&self) -> u64 {
        self.srvid.unwrap_or(0)
    }

    // optional .quall.VoteReq voteReq = 2;

    pub fn clear_voteReq(&mut self) {
        self.voteReq.clear();
    }

    pub fn has_voteReq(&self) -> bool {
        self.voteReq.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voteReq(&mut self, v: VoteReq) {
        self.voteReq = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voteReq<'a>(&'a mut self) -> &'a mut VoteReq {
        if self.voteReq.is_none() {
            self.voteReq.set_default();
        };
        self.voteReq.as_mut().unwrap()
    }

    // Take field
    pub fn take_voteReq(&mut self) -> VoteReq {
        self.voteReq.take().unwrap_or_else(|| VoteReq::new())
    }

    pub fn get_voteReq<'a>(&'a self) -> &'a VoteReq {
        self.voteReq.as_ref().unwrap_or_else(|| VoteReq::default_instance())
    }

    // optional .quall.BatchReq batchReq = 3;

    pub fn clear_batchReq(&mut self) {
        self.batchReq.clear();
    }

    pub fn has_batchReq(&self) -> bool {
        self.batchReq.is_some()
    }

    // Param is passed by value, moved
    pub fn set_batchReq(&mut self, v: BatchReq) {
        self.batchReq = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_batchReq<'a>(&'a mut self) -> &'a mut BatchReq {
        if self.batchReq.is_none() {
            self.batchReq.set_default();
        };
        self.batchReq.as_mut().unwrap()
    }

    // Take field
    pub fn take_batchReq(&mut self) -> BatchReq {
        self.batchReq.take().unwrap_or_else(|| BatchReq::new())
    }

    pub fn get_batchReq<'a>(&'a self) -> &'a BatchReq {
        self.batchReq.as_ref().unwrap_or_else(|| BatchReq::default_instance())
    }

    // optional .quall.Learn learn = 4;

    pub fn clear_learn(&mut self) {
        self.learn.clear();
    }

    pub fn has_learn(&self) -> bool {
        self.learn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_learn(&mut self, v: Learn) {
        self.learn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_learn<'a>(&'a mut self) -> &'a mut Learn {
        if self.learn.is_none() {
            self.learn.set_default();
        };
        self.learn.as_mut().unwrap()
    }

    // Take field
    pub fn take_learn(&mut self) -> Learn {
        self.learn.take().unwrap_or_else(|| Learn::new())
    }

    pub fn get_learn<'a>(&'a self) -> &'a Learn {
        self.learn.as_ref().unwrap_or_else(|| Learn::default_instance())
    }

    // optional .quall.Propose propose = 5;

    pub fn clear_propose(&mut self) {
        self.propose.clear();
    }

    pub fn has_propose(&self) -> bool {
        self.propose.is_some()
    }

    // Param is passed by value, moved
    pub fn set_propose(&mut self, v: Propose) {
        self.propose = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_propose<'a>(&'a mut self) -> &'a mut Propose {
        if self.propose.is_none() {
            self.propose.set_default();
        };
        self.propose.as_mut().unwrap()
    }

    // Take field
    pub fn take_propose(&mut self) -> Propose {
        self.propose.take().unwrap_or_else(|| Propose::new())
    }

    pub fn get_propose<'a>(&'a self) -> &'a Propose {
        self.propose.as_ref().unwrap_or_else(|| Propose::default_instance())
    }
}

impl ::protobuf::Message for SrvReq {
    fn is_initialized(&self) -> bool {
        if self.srvid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.srvid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.voteReq.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.batchReq.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.learn.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.propose.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.srvid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.voteReq.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.batchReq.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.learn.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.propose.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.srvid {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.voteReq.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.batchReq.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.learn.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.propose.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SrvReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SrvReq {
    fn new() -> SrvReq {
        SrvReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<SrvReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "srvid",
                    SrvReq::has_srvid,
                    SrvReq::get_srvid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "voteReq",
                    SrvReq::has_voteReq,
                    SrvReq::get_voteReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "batchReq",
                    SrvReq::has_batchReq,
                    SrvReq::get_batchReq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "learn",
                    SrvReq::has_learn,
                    SrvReq::get_learn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "propose",
                    SrvReq::has_propose,
                    SrvReq::get_propose,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SrvReq>(
                    "SrvReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SrvReq {
    fn clear(&mut self) {
        self.clear_srvid();
        self.clear_voteReq();
        self.clear_batchReq();
        self.clear_learn();
        self.clear_propose();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SrvReq {
    fn eq(&self, other: &SrvReq) -> bool {
        self.srvid == other.srvid &&
        self.voteReq == other.voteReq &&
        self.batchReq == other.batchReq &&
        self.learn == other.learn &&
        self.propose == other.propose &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SrvReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SrvRes {
    // message fields
    srvid: ::std::option::Option<u64>,
    voteRes: ::protobuf::SingularPtrField<VoteRes>,
    batchRes: ::protobuf::SingularPtrField<BatchRes>,
    accept: ::protobuf::SingularPtrField<Accept>,
    reject: ::protobuf::SingularPtrField<Reject>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SrvRes {
    pub fn new() -> SrvRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SrvRes {
        static mut instance: ::protobuf::lazy::Lazy<SrvRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SrvRes,
        };
        unsafe {
            instance.get(|| {
                SrvRes {
                    srvid: ::std::option::Option::None,
                    voteRes: ::protobuf::SingularPtrField::none(),
                    batchRes: ::protobuf::SingularPtrField::none(),
                    accept: ::protobuf::SingularPtrField::none(),
                    reject: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 srvid = 1;

    pub fn clear_srvid(&mut self) {
        self.srvid = ::std::option::Option::None;
    }

    pub fn has_srvid(&self) -> bool {
        self.srvid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_srvid(&mut self, v: u64) {
        self.srvid = ::std::option::Option::Some(v);
    }

    pub fn get_srvid<'a>(&self) -> u64 {
        self.srvid.unwrap_or(0)
    }

    // optional .quall.VoteRes voteRes = 2;

    pub fn clear_voteRes(&mut self) {
        self.voteRes.clear();
    }

    pub fn has_voteRes(&self) -> bool {
        self.voteRes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voteRes(&mut self, v: VoteRes) {
        self.voteRes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voteRes<'a>(&'a mut self) -> &'a mut VoteRes {
        if self.voteRes.is_none() {
            self.voteRes.set_default();
        };
        self.voteRes.as_mut().unwrap()
    }

    // Take field
    pub fn take_voteRes(&mut self) -> VoteRes {
        self.voteRes.take().unwrap_or_else(|| VoteRes::new())
    }

    pub fn get_voteRes<'a>(&'a self) -> &'a VoteRes {
        self.voteRes.as_ref().unwrap_or_else(|| VoteRes::default_instance())
    }

    // optional .quall.BatchRes batchRes = 3;

    pub fn clear_batchRes(&mut self) {
        self.batchRes.clear();
    }

    pub fn has_batchRes(&self) -> bool {
        self.batchRes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_batchRes(&mut self, v: BatchRes) {
        self.batchRes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_batchRes<'a>(&'a mut self) -> &'a mut BatchRes {
        if self.batchRes.is_none() {
            self.batchRes.set_default();
        };
        self.batchRes.as_mut().unwrap()
    }

    // Take field
    pub fn take_batchRes(&mut self) -> BatchRes {
        self.batchRes.take().unwrap_or_else(|| BatchRes::new())
    }

    pub fn get_batchRes<'a>(&'a self) -> &'a BatchRes {
        self.batchRes.as_ref().unwrap_or_else(|| BatchRes::default_instance())
    }

    // optional .quall.Accept accept = 4;

    pub fn clear_accept(&mut self) {
        self.accept.clear();
    }

    pub fn has_accept(&self) -> bool {
        self.accept.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accept(&mut self, v: Accept) {
        self.accept = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accept<'a>(&'a mut self) -> &'a mut Accept {
        if self.accept.is_none() {
            self.accept.set_default();
        };
        self.accept.as_mut().unwrap()
    }

    // Take field
    pub fn take_accept(&mut self) -> Accept {
        self.accept.take().unwrap_or_else(|| Accept::new())
    }

    pub fn get_accept<'a>(&'a self) -> &'a Accept {
        self.accept.as_ref().unwrap_or_else(|| Accept::default_instance())
    }

    // optional .quall.Reject reject = 5;

    pub fn clear_reject(&mut self) {
        self.reject.clear();
    }

    pub fn has_reject(&self) -> bool {
        self.reject.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reject(&mut self, v: Reject) {
        self.reject = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reject<'a>(&'a mut self) -> &'a mut Reject {
        if self.reject.is_none() {
            self.reject.set_default();
        };
        self.reject.as_mut().unwrap()
    }

    // Take field
    pub fn take_reject(&mut self) -> Reject {
        self.reject.take().unwrap_or_else(|| Reject::new())
    }

    pub fn get_reject<'a>(&'a self) -> &'a Reject {
        self.reject.as_ref().unwrap_or_else(|| Reject::default_instance())
    }
}

impl ::protobuf::Message for SrvRes {
    fn is_initialized(&self) -> bool {
        if self.srvid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.srvid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.voteRes.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.batchRes.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.accept.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.reject.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.srvid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.voteRes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.batchRes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.accept.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.reject.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.srvid {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.voteRes.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.batchRes.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.accept.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.reject.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SrvRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SrvRes {
    fn new() -> SrvRes {
        SrvRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<SrvRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "srvid",
                    SrvRes::has_srvid,
                    SrvRes::get_srvid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "voteRes",
                    SrvRes::has_voteRes,
                    SrvRes::get_voteRes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "batchRes",
                    SrvRes::has_batchRes,
                    SrvRes::get_batchRes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "accept",
                    SrvRes::has_accept,
                    SrvRes::get_accept,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "reject",
                    SrvRes::has_reject,
                    SrvRes::get_reject,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SrvRes>(
                    "SrvRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SrvRes {
    fn clear(&mut self) {
        self.clear_srvid();
        self.clear_voteRes();
        self.clear_batchRes();
        self.clear_accept();
        self.clear_reject();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SrvRes {
    fn eq(&self, other: &SrvRes) -> bool {
        self.srvid == other.srvid &&
        self.voteRes == other.voteRes &&
        self.batchRes == other.batchRes &&
        self.accept == other.accept &&
        self.reject == other.reject &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SrvRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x22, 0x24, 0x0a, 0x06,
    0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0c, 0x22, 0x34, 0x0a, 0x06, 0x53, 0x65, 0x74, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07,
    0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a,
    0x04, 0x74, 0x78, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0b, 0x0a, 0x03, 0x65,
    0x72, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x15, 0x0a, 0x06, 0x47, 0x65, 0x74, 0x52,
    0x65, 0x71, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x22,
    0x43, 0x0a, 0x06, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x22, 0x36, 0x0a, 0x06, 0x43, 0x41, 0x53, 0x52, 0x65, 0x71, 0x12, 0x0b,
    0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x6c,
    0x64, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x34, 0x0a, 0x06,
    0x43, 0x41, 0x53, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x22, 0x50, 0x0a, 0x08, 0x57, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x12, 0x0b,
    0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x6c,
    0x61, 0x73, 0x74, 0x74, 0x78, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x11, 0x0a,
    0x09, 0x72, 0x65, 0x63, 0x75, 0x72, 0x73, 0x69, 0x76, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08,
    0x12, 0x12, 0x0a, 0x0a, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x69, 0x63, 0x61, 0x6c, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x08, 0x22, 0x37, 0x0a, 0x0b, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65,
    0x64, 0x4b, 0x56, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x04, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0d,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x4d, 0x0a,
    0x08, 0x57, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x23, 0x0a, 0x07, 0x68, 0x69,
    0x73, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x71, 0x75,
    0x61, 0x6c, 0x6c, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x4b, 0x56, 0x12,
    0x0b, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x7c, 0x0a, 0x06,
    0x43, 0x6c, 0x69, 0x52, 0x65, 0x71, 0x12, 0x1a, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x47, 0x65, 0x74, 0x52,
    0x65, 0x71, 0x12, 0x1a, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0d, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x1a,
    0x0a, 0x03, 0x63, 0x61, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x75,
    0x61, 0x6c, 0x6c, 0x2e, 0x43, 0x41, 0x53, 0x52, 0x65, 0x71, 0x12, 0x1e, 0x0a, 0x05, 0x77, 0x61,
    0x74, 0x63, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x71, 0x75, 0x61, 0x6c,
    0x6c, 0x2e, 0x57, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x22, 0x7c, 0x0a, 0x06, 0x43, 0x6c,
    0x69, 0x52, 0x65, 0x73, 0x12, 0x1a, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73,
    0x12, 0x1a, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e,
    0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x65, 0x73, 0x12, 0x1a, 0x0a, 0x03,
    0x63, 0x61, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x75, 0x61, 0x6c,
    0x6c, 0x2e, 0x43, 0x41, 0x53, 0x52, 0x65, 0x73, 0x12, 0x1e, 0x0a, 0x05, 0x77, 0x61, 0x74, 0x63,
    0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e,
    0x57, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x22, 0x24, 0x0a, 0x06, 0x41, 0x70, 0x70, 0x65,
    0x6e, 0x64, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12,
    0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x15,
    0x0a, 0x05, 0x4c, 0x65, 0x61, 0x72, 0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x39, 0x0a, 0x07, 0x50, 0x72, 0x6f, 0x70, 0x6f, 0x73, 0x65,
    0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x20,
    0x0a, 0x09, 0x6d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64,
    0x22, 0x16, 0x0a, 0x06, 0x41, 0x63, 0x63, 0x65, 0x70, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x16, 0x0a, 0x06, 0x52, 0x65, 0x6a, 0x65,
    0x63, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04,
    0x22, 0x2b, 0x0a, 0x07, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x71, 0x12, 0x0f, 0x0a, 0x07, 0x6d,
    0x61, 0x78, 0x74, 0x78, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0f, 0x0a, 0x07,
    0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x2b, 0x0a,
    0x07, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63,
    0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0f, 0x0a, 0x07, 0x61, 0x74, 0x74,
    0x65, 0x6d, 0x70, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x1c, 0x0a, 0x08, 0x42, 0x61,
    0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x72, 0x6f, 0x6d, 0x74, 0x78,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x2f, 0x0a, 0x08, 0x42, 0x61, 0x74, 0x63,
    0x68, 0x52, 0x65, 0x73, 0x12, 0x23, 0x0a, 0x07, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x79, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x4b, 0x56, 0x22, 0x99, 0x01, 0x0a, 0x06, 0x53, 0x72,
    0x76, 0x52, 0x65, 0x71, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x72, 0x76, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x04, 0x12, 0x1f, 0x0a, 0x07, 0x76, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x71, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x56, 0x6f, 0x74,
    0x65, 0x52, 0x65, 0x71, 0x12, 0x21, 0x0a, 0x08, 0x62, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x42,
    0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x12, 0x1b, 0x0a, 0x05, 0x6c, 0x65, 0x61, 0x72, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x4c,
    0x65, 0x61, 0x72, 0x6e, 0x12, 0x1f, 0x0a, 0x07, 0x70, 0x72, 0x6f, 0x70, 0x6f, 0x73, 0x65, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x50, 0x72,
    0x6f, 0x70, 0x6f, 0x73, 0x65, 0x22, 0x99, 0x01, 0x0a, 0x06, 0x53, 0x72, 0x76, 0x52, 0x65, 0x73,
    0x12, 0x0d, 0x0a, 0x05, 0x73, 0x72, 0x76, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12,
    0x1f, 0x0a, 0x07, 0x76, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x73,
    0x12, 0x21, 0x0a, 0x08, 0x62, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x42, 0x61, 0x74, 0x63, 0x68,
    0x52, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x06, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x41, 0x63, 0x63, 0x65,
    0x70, 0x74, 0x12, 0x1d, 0x0a, 0x06, 0x72, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x2e, 0x52, 0x65, 0x6a, 0x65, 0x63,
    0x74, 0x4a, 0xd1, 0x25, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x8b, 0x01, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07,
    0x00, 0x0a, 0x01, 0x1a, 0x24, 0x0a, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x3c, 0x2d,
    0x3e, 0x20, 0x71, 0x75, 0x61, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08,
    0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x09, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x19, 0x1a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x10, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x0f, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x12, 0x00,
    0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x13, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x13, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x13, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x16, 0x00, 0x1b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x16, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x17, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17,
    0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x18, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x18, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x19,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x03, 0x12, 0x03, 0x1a, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1a, 0x18, 0x19, 0x0a,
    0x4d, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1e, 0x00, 0x22, 0x01, 0x1a, 0x41, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x43, 0x41, 0x53, 0x20, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x20, 0x68, 0x61, 0x70, 0x70,
    0x65, 0x6e, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x61, 0x67, 0x61, 0x74,
    0x65, 0x73, 0x20, 0x76, 0x69, 0x61, 0x20, 0x70, 0x61, 0x78, 0x6f, 0x73, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x1f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x11,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x20, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x20, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x20, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x21, 0x02,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x24, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x24, 0x08,
    0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x25, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x25, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03,
    0x26, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x26, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x02, 0x12, 0x03, 0x27, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x27, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27,
    0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x18, 0x19,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x2a, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x06, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00,
    0x12, 0x03, 0x2b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x11, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x17, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x2c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c,
    0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x2d, 0x02, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x2d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12,
    0x03, 0x2e, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x10, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2e, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x07, 0x12, 0x04, 0x31, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
    0x03, 0x31, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x32, 0x02,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x01, 0x12, 0x03, 0x33, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x33,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33, 0x11, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x34, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x34, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x34, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x34, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x37, 0x00, 0x3b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x38, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38,
    0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x39, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x39, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x39, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x3a,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3a, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x18, 0x19, 0x0a, 0x21, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x04, 0x3e, 0x00, 0x43, 0x01, 0x1a, 0x15, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x74, 0x6f, 0x70, 0x2d, 0x6c, 0x65, 0x76, 0x6c, 0x20, 0x41, 0x50, 0x49, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x03, 0x3f, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3f,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x40, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x40, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x40, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x40, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x41, 0x02, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x06, 0x12, 0x03, 0x41, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x41, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x41, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03,
    0x12, 0x03, 0x42, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x06, 0x12, 0x03, 0x42, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x42, 0x14, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x42, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0a, 0x12, 0x04, 0x45, 0x00, 0x4a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x03, 0x45, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x46,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x46, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x46, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x01, 0x12, 0x03, 0x47, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x47, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x47, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x47, 0x18, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x48, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x48, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x48, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x48, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x03, 0x12, 0x03, 0x49, 0x02,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x49, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x06, 0x12, 0x03, 0x49, 0x0b, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x49, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x49, 0x1c, 0x1d, 0x0a, 0x1b, 0x0a, 0x02, 0x04, 0x0b, 0x12,
    0x04, 0x4f, 0x00, 0x52, 0x01, 0x1a, 0x0f, 0x0a, 0x20, 0x50, 0x72, 0x6f, 0x70, 0x61, 0x67, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x4f,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x50, 0x02, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x50, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x50, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12,
    0x03, 0x51, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x51,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x51, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x51, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x51, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0c, 0x12, 0x04, 0x54, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12,
    0x03, 0x54, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x55, 0x02,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x55, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12,
    0x04, 0x58, 0x00, 0x5c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x58, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x59, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x59, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x59, 0x19, 0x1a, 0x0a, 0x5c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03,
    0x5b, 0x02, 0x20, 0x1a, 0x4f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20,
    0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x6c,
    0x79, 0x20, 0x67, 0x65, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x78, 0x69, 0x64, 0x20, 0x2b, 0x20, 0x31, 0x2c, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x78, 0x69, 0x64, 0x20, 0x2b, 0x20, 0x32, 0x20, 0x65, 0x74, 0x63,
    0x2e, 0x2e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5b, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x12, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5b, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0e, 0x12, 0x04, 0x5e, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12,
    0x03, 0x5e, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x02,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5f, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12,
    0x04, 0x62, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x62, 0x08,
    0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x63, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x63, 0x19, 0x1a, 0x0a, 0x1a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x69, 0x00,
    0x6c, 0x01, 0x1a, 0x0e, 0x0a, 0x20, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70,
    0x0a, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x69, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x6a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6a, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x6a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x02, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x6b, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x04,
    0x6d, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x03, 0x6d, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x03, 0x6e, 0x02, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x6e, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x6e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x03, 0x6f,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6f, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6f, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6f, 0x1c, 0x1d, 0x0a, 0x1c, 0x0a, 0x02, 0x04, 0x12,
    0x12, 0x04, 0x75, 0x00, 0x77, 0x01, 0x1a, 0x10, 0x0a, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x43, 0x61,
    0x74, 0x63, 0x68, 0x2d, 0x55, 0x70, 0x0a, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12,
    0x03, 0x75, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x03, 0x76, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x03, 0x76, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x03, 0x76, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x13, 0x12,
    0x04, 0x78, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x03, 0x78, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x03, 0x79, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x03, 0x79, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x79, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x79, 0x21, 0x22, 0x0a, 0x2c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x05, 0x7d, 0x00,
    0x83, 0x01, 0x01, 0x1a, 0x1f, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x3c, 0x2d, 0x3e, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x70, 0x2d, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20,
    0x61, 0x70, 0x69, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x03, 0x7d, 0x08, 0x0e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x03, 0x7e, 0x02, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x7e, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x7e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x03, 0x7f,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x06, 0x12, 0x03, 0x7f, 0x0b, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7f, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7f, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x02, 0x12, 0x04, 0x80, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x80, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x06,
    0x12, 0x04, 0x80, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x80, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x80, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04, 0x81, 0x01,
    0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0x81, 0x01, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0x81, 0x01, 0x19, 0x1a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x04, 0x06, 0x12, 0x04, 0x82, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x82, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x82, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06,
    0x85, 0x01, 0x00, 0x8b, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0x85,
    0x01, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x02,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x12, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0x87, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x87, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x87, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x87, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12,
    0x04, 0x88, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x88, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x06, 0x12, 0x04, 0x88,
    0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01,
    0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0x88, 0x01, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0x89, 0x01, 0x02, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0x89, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06, 0x12, 0x04, 0x89, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0x89, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0x89, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x8a, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x8a, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x8a, 0x01, 0x1b, 0x1c,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

#[cfg(test)]
mod tests {
    use serialization::CliReq;

    #[test]
    fn test_serialize() {
        let cr = CliReq::new();
    }
}
