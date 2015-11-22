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
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes value = 1;

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
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value.as_ref() {
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
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetReq {
    fn eq(&self, other: &SetReq) -> bool {
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
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
                let fields = ::std::vec::Vec::new();
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetReq {
    fn eq(&self, other: &GetReq) -> bool {
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

    // optional bytes value = 3;

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
    new_value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    old_value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    new_value: ::protobuf::SingularField::none(),
                    old_value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes new_value = 1;

    pub fn clear_new_value(&mut self) {
        self.new_value.clear();
    }

    pub fn has_new_value(&self) -> bool {
        self.new_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.new_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.new_value.is_none() {
            self.new_value.set_default();
        };
        self.new_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_value(&mut self) -> ::std::vec::Vec<u8> {
        self.new_value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_new_value<'a>(&'a self) -> &'a [u8] {
        match self.new_value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes old_value = 2;

    pub fn clear_old_value(&mut self) {
        self.old_value.clear();
    }

    pub fn has_old_value(&self) -> bool {
        self.old_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.old_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.old_value.is_none() {
            self.old_value.set_default();
        };
        self.old_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_value(&mut self) -> ::std::vec::Vec<u8> {
        self.old_value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_old_value<'a>(&'a self) -> &'a [u8] {
        match self.old_value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CASReq {
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
                    let tmp = self.new_value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.old_value.set_default();
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
        for value in self.new_value.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.old_value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_value.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.old_value.as_ref() {
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
                    "new_value",
                    CASReq::has_new_value,
                    CASReq::get_new_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "old_value",
                    CASReq::has_old_value,
                    CASReq::get_old_value,
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
        self.clear_new_value();
        self.clear_old_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CASReq {
    fn eq(&self, other: &CASReq) -> bool {
        self.new_value == other.new_value &&
        self.old_value == other.old_value &&
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
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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

    // optional bytes value = 3;

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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    CASRes::has_value,
                    CASRes::get_value,
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
        self.clear_value();
        self.clear_err();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CASRes {
    fn eq(&self, other: &CASRes) -> bool {
        self.success == other.success &&
        self.txid == other.txid &&
        self.value == other.value &&
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
pub struct DelReq {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DelReq {
    pub fn new() -> DelReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DelReq {
        static mut instance: ::protobuf::lazy::Lazy<DelReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DelReq,
        };
        unsafe {
            instance.get(|| {
                DelReq {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for DelReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<DelReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DelReq {
    fn new() -> DelReq {
        DelReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<DelReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DelReq>(
                    "DelReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DelReq {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DelReq {
    fn eq(&self, other: &DelReq) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DelReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DelRes {
    // message fields
    success: ::std::option::Option<bool>,
    txid: ::std::option::Option<u64>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    err: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DelRes {
    pub fn new() -> DelRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DelRes {
        static mut instance: ::protobuf::lazy::Lazy<DelRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DelRes,
        };
        unsafe {
            instance.get(|| {
                DelRes {
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

impl ::protobuf::Message for DelRes {
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
        ::std::any::TypeId::of::<DelRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DelRes {
    fn new() -> DelRes {
        DelRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<DelRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    DelRes::has_success,
                    DelRes::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    DelRes::has_txid,
                    DelRes::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    DelRes::has_value,
                    DelRes::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "err",
                    DelRes::has_err,
                    DelRes::get_err,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DelRes>(
                    "DelRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DelRes {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_txid();
        self.clear_value();
        self.clear_err();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DelRes {
    fn eq(&self, other: &DelRes) -> bool {
        self.success == other.success &&
        self.txid == other.txid &&
        self.value == other.value &&
        self.err == other.err &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DelRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WatchReq {
    // message fields
    last_txid: ::std::option::Option<u64>,
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
                    last_txid: ::std::option::Option::None,
                    recursive: ::std::option::Option::None,
                    historical: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 last_txid = 1;

    pub fn clear_last_txid(&mut self) {
        self.last_txid = ::std::option::Option::None;
    }

    pub fn has_last_txid(&self) -> bool {
        self.last_txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_txid(&mut self, v: u64) {
        self.last_txid = ::std::option::Option::Some(v);
    }

    pub fn get_last_txid<'a>(&self) -> u64 {
        self.last_txid.unwrap_or(0)
    }

    // required bool recursive = 2;

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

    // required bool historical = 3;

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
        if self.last_txid.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_txid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.recursive = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        for value in self.last_txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.last_txid {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.recursive {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.historical {
            try!(os.write_bool(3, v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_txid",
                    WatchReq::has_last_txid,
                    WatchReq::get_last_txid,
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
        self.clear_last_txid();
        self.clear_recursive();
        self.clear_historical();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WatchReq {
    fn eq(&self, other: &WatchReq) -> bool {
        self.last_txid == other.last_txid &&
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
pub struct WatchRes {
    // message fields
    success: ::std::option::Option<bool>,
    history: ::protobuf::RepeatedField<Mutation>,
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

    // repeated .rasputin.Mutation history = 2;

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    // Param is passed by value, moved
    pub fn set_history(&mut self, v: ::protobuf::RepeatedField<Mutation>) {
        self.history = v;
    }

    // Mutable pointer to the field.
    pub fn mut_history<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Mutation> {
        &mut self.history
    }

    // Take field
    pub fn take_history(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.history, ::protobuf::RepeatedField::new())
    }

    pub fn get_history<'a>(&'a self) -> &'a [Mutation] {
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
pub struct RedirectRes {
    // message fields
    success: ::std::option::Option<bool>,
    address: ::protobuf::SingularField<::std::string::String>,
    err: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RedirectRes {
    pub fn new() -> RedirectRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RedirectRes {
        static mut instance: ::protobuf::lazy::Lazy<RedirectRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RedirectRes,
        };
        unsafe {
            instance.get(|| {
                RedirectRes {
                    success: ::std::option::Option::None,
                    address: ::protobuf::SingularField::none(),
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

    // optional string address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address<'a>(&'a self) -> &'a str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
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

impl ::protobuf::Message for RedirectRes {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.address.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.address.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
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
        if let Some(v) = self.address.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<RedirectRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RedirectRes {
    fn new() -> RedirectRes {
        RedirectRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<RedirectRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    RedirectRes::has_success,
                    RedirectRes::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "address",
                    RedirectRes::has_address,
                    RedirectRes::get_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "err",
                    RedirectRes::has_err,
                    RedirectRes::get_err,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RedirectRes>(
                    "RedirectRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RedirectRes {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_address();
        self.clear_err();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RedirectRes {
    fn eq(&self, other: &RedirectRes) -> bool {
        self.success == other.success &&
        self.address == other.address &&
        self.err == other.err &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RedirectRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Mutation {
    // message fields
    field_type: ::std::option::Option<MutationType>,
    version: ::protobuf::SingularPtrField<Version>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    old_value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Mutation {
    pub fn new() -> Mutation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mutation {
        static mut instance: ::protobuf::lazy::Lazy<Mutation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mutation,
        };
        unsafe {
            instance.get(|| {
                Mutation {
                    field_type: ::std::option::Option::None,
                    version: ::protobuf::SingularPtrField::none(),
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    old_value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .rasputin.MutationType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MutationType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> MutationType {
        self.field_type.unwrap_or(MutationType::KVSET)
    }

    // required .rasputin.Version version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: Version) {
        self.version = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version<'a>(&'a mut self) -> &'a mut Version {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> Version {
        self.version.take().unwrap_or_else(|| Version::new())
    }

    pub fn get_version<'a>(&'a self) -> &'a Version {
        self.version.as_ref().unwrap_or_else(|| Version::default_instance())
    }

    // required bytes key = 3;

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

    // optional bytes value = 4;

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

    // optional bytes old_value = 5;

    pub fn clear_old_value(&mut self) {
        self.old_value.clear();
    }

    pub fn has_old_value(&self) -> bool {
        self.old_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.old_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.old_value.is_none() {
            self.old_value.set_default();
        };
        self.old_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_value(&mut self) -> ::std::vec::Vec<u8> {
        self.old_value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_old_value<'a>(&'a self) -> &'a [u8] {
        match self.old_value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Mutation {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.version.is_none() {
            return false;
        };
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.version.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.old_value.set_default();
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
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.version.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.old_value.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.version.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.old_value.as_ref() {
            try!(os.write_bytes(5, &v));
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
        ::std::any::TypeId::of::<Mutation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mutation {
    fn new() -> Mutation {
        Mutation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mutation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    Mutation::has_field_type,
                    Mutation::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "version",
                    Mutation::has_version,
                    Mutation::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    Mutation::has_key,
                    Mutation::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    Mutation::has_value,
                    Mutation::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "old_value",
                    Mutation::has_old_value,
                    Mutation::get_old_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mutation>(
                    "Mutation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_version();
        self.clear_key();
        self.clear_value();
        self.clear_old_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Mutation {
    fn eq(&self, other: &Mutation) -> bool {
        self.field_type == other.field_type &&
        self.version == other.version &&
        self.key == other.key &&
        self.value == other.value &&
        self.old_value == other.old_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Version {
    // message fields
    txid: ::std::option::Option<u64>,
    term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Version {
    pub fn new() -> Version {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Version {
        static mut instance: ::protobuf::lazy::Lazy<Version> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Version,
        };
        unsafe {
            instance.get(|| {
                Version {
                    txid: ::std::option::Option::None,
                    term: ::std::option::Option::None,
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

    // required uint64 term = 2;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term<'a>(&self) -> u64 {
        self.term.unwrap_or(0)
    }
}

impl ::protobuf::Message for Version {
    fn is_initialized(&self) -> bool {
        if self.txid.is_none() {
            return false;
        };
        if self.term.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
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
        for value in self.term.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txid {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.term {
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
        ::std::any::TypeId::of::<Version>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Version {
    fn new() -> Version {
        Version::new()
    }

    fn descriptor_static(_: ::std::option::Option<Version>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "txid",
                    Version::has_txid,
                    Version::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    Version::has_term,
                    Version::get_term,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Version>(
                    "Version",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Version {
    fn clear(&mut self) {
        self.clear_txid();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.txid == other.txid &&
        self.term == other.term &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Version {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CliReq {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    req_id: ::std::option::Option<u64>,
    get: ::protobuf::SingularPtrField<GetReq>,
    set: ::protobuf::SingularPtrField<SetReq>,
    cas: ::protobuf::SingularPtrField<CASReq>,
    del: ::protobuf::SingularPtrField<DelReq>,
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
                    key: ::protobuf::SingularField::none(),
                    req_id: ::std::option::Option::None,
                    get: ::protobuf::SingularPtrField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    cas: ::protobuf::SingularPtrField::none(),
                    del: ::protobuf::SingularPtrField::none(),
                    watch: ::protobuf::SingularPtrField::none(),
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

    // required uint64 req_id = 2;

    pub fn clear_req_id(&mut self) {
        self.req_id = ::std::option::Option::None;
    }

    pub fn has_req_id(&self) -> bool {
        self.req_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_req_id(&mut self, v: u64) {
        self.req_id = ::std::option::Option::Some(v);
    }

    pub fn get_req_id<'a>(&self) -> u64 {
        self.req_id.unwrap_or(0)
    }

    // optional .rasputin.GetReq get = 3;

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

    // optional .rasputin.SetReq set = 4;

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

    // optional .rasputin.CASReq cas = 5;

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

    // optional .rasputin.DelReq del = 6;

    pub fn clear_del(&mut self) {
        self.del.clear();
    }

    pub fn has_del(&self) -> bool {
        self.del.is_some()
    }

    // Param is passed by value, moved
    pub fn set_del(&mut self, v: DelReq) {
        self.del = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_del<'a>(&'a mut self) -> &'a mut DelReq {
        if self.del.is_none() {
            self.del.set_default();
        };
        self.del.as_mut().unwrap()
    }

    // Take field
    pub fn take_del(&mut self) -> DelReq {
        self.del.take().unwrap_or_else(|| DelReq::new())
    }

    pub fn get_del<'a>(&'a self) -> &'a DelReq {
        self.del.as_ref().unwrap_or_else(|| DelReq::default_instance())
    }

    // optional .rasputin.WatchReq watch = 7;

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
        if self.key.is_none() {
            return false;
        };
        if self.req_id.is_none() {
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
                    self.req_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.get.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.set.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cas.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.del.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.req_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
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
        for value in self.del.iter() {
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
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.req_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.get.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.set.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cas.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.del.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.watch.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    CliReq::has_key,
                    CliReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "req_id",
                    CliReq::has_req_id,
                    CliReq::get_req_id,
                ));
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
                    "del",
                    CliReq::has_del,
                    CliReq::get_del,
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
        self.clear_key();
        self.clear_req_id();
        self.clear_get();
        self.clear_set();
        self.clear_cas();
        self.clear_del();
        self.clear_watch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CliReq {
    fn eq(&self, other: &CliReq) -> bool {
        self.key == other.key &&
        self.req_id == other.req_id &&
        self.get == other.get &&
        self.set == other.set &&
        self.cas == other.cas &&
        self.del == other.del &&
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
    req_id: ::std::option::Option<u64>,
    get: ::protobuf::SingularPtrField<GetRes>,
    set: ::protobuf::SingularPtrField<SetRes>,
    cas: ::protobuf::SingularPtrField<CASRes>,
    del: ::protobuf::SingularPtrField<DelRes>,
    watch: ::protobuf::SingularPtrField<WatchRes>,
    redirect: ::protobuf::SingularPtrField<RedirectRes>,
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
                    req_id: ::std::option::Option::None,
                    get: ::protobuf::SingularPtrField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    cas: ::protobuf::SingularPtrField::none(),
                    del: ::protobuf::SingularPtrField::none(),
                    watch: ::protobuf::SingularPtrField::none(),
                    redirect: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 req_id = 1;

    pub fn clear_req_id(&mut self) {
        self.req_id = ::std::option::Option::None;
    }

    pub fn has_req_id(&self) -> bool {
        self.req_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_req_id(&mut self, v: u64) {
        self.req_id = ::std::option::Option::Some(v);
    }

    pub fn get_req_id<'a>(&self) -> u64 {
        self.req_id.unwrap_or(0)
    }

    // optional .rasputin.GetRes get = 2;

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

    // optional .rasputin.SetRes set = 3;

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

    // optional .rasputin.CASRes cas = 4;

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

    // optional .rasputin.DelRes del = 5;

    pub fn clear_del(&mut self) {
        self.del.clear();
    }

    pub fn has_del(&self) -> bool {
        self.del.is_some()
    }

    // Param is passed by value, moved
    pub fn set_del(&mut self, v: DelRes) {
        self.del = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_del<'a>(&'a mut self) -> &'a mut DelRes {
        if self.del.is_none() {
            self.del.set_default();
        };
        self.del.as_mut().unwrap()
    }

    // Take field
    pub fn take_del(&mut self) -> DelRes {
        self.del.take().unwrap_or_else(|| DelRes::new())
    }

    pub fn get_del<'a>(&'a self) -> &'a DelRes {
        self.del.as_ref().unwrap_or_else(|| DelRes::default_instance())
    }

    // optional .rasputin.WatchRes watch = 6;

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

    // optional .rasputin.RedirectRes redirect = 7;

    pub fn clear_redirect(&mut self) {
        self.redirect.clear();
    }

    pub fn has_redirect(&self) -> bool {
        self.redirect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redirect(&mut self, v: RedirectRes) {
        self.redirect = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect<'a>(&'a mut self) -> &'a mut RedirectRes {
        if self.redirect.is_none() {
            self.redirect.set_default();
        };
        self.redirect.as_mut().unwrap()
    }

    // Take field
    pub fn take_redirect(&mut self) -> RedirectRes {
        self.redirect.take().unwrap_or_else(|| RedirectRes::new())
    }

    pub fn get_redirect<'a>(&'a self) -> &'a RedirectRes {
        self.redirect.as_ref().unwrap_or_else(|| RedirectRes::default_instance())
    }
}

impl ::protobuf::Message for CliRes {
    fn is_initialized(&self) -> bool {
        if self.req_id.is_none() {
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
                    self.req_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.get.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.set.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cas.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.del.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.watch.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.redirect.set_default();
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
        for value in self.req_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
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
        for value in self.del.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.watch.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.redirect.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.req_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.get.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.set.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cas.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.del.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.watch.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.redirect.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "req_id",
                    CliRes::has_req_id,
                    CliRes::get_req_id,
                ));
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
                    "del",
                    CliRes::has_del,
                    CliRes::get_del,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "watch",
                    CliRes::has_watch,
                    CliRes::get_watch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "redirect",
                    CliRes::has_redirect,
                    CliRes::get_redirect,
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
        self.clear_req_id();
        self.clear_get();
        self.clear_set();
        self.clear_cas();
        self.clear_del();
        self.clear_watch();
        self.clear_redirect();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CliRes {
    fn eq(&self, other: &CliRes) -> bool {
        self.req_id == other.req_id &&
        self.get == other.get &&
        self.set == other.set &&
        self.cas == other.cas &&
        self.del == other.del &&
        self.watch == other.watch &&
        self.redirect == other.redirect &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CliRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VoteReq {
    // message fields
    term: ::std::option::Option<u64>,
    last_learned_term: ::std::option::Option<u64>,
    last_learned_txid: ::std::option::Option<u64>,
    last_accepted_term: ::std::option::Option<u64>,
    last_accepted_txid: ::std::option::Option<u64>,
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
                    term: ::std::option::Option::None,
                    last_learned_term: ::std::option::Option::None,
                    last_learned_txid: ::std::option::Option::None,
                    last_accepted_term: ::std::option::Option::None,
                    last_accepted_txid: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term<'a>(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    // required uint64 last_learned_term = 2;

    pub fn clear_last_learned_term(&mut self) {
        self.last_learned_term = ::std::option::Option::None;
    }

    pub fn has_last_learned_term(&self) -> bool {
        self.last_learned_term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_learned_term(&mut self, v: u64) {
        self.last_learned_term = ::std::option::Option::Some(v);
    }

    pub fn get_last_learned_term<'a>(&self) -> u64 {
        self.last_learned_term.unwrap_or(0)
    }

    // required uint64 last_learned_txid = 3;

    pub fn clear_last_learned_txid(&mut self) {
        self.last_learned_txid = ::std::option::Option::None;
    }

    pub fn has_last_learned_txid(&self) -> bool {
        self.last_learned_txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_learned_txid(&mut self, v: u64) {
        self.last_learned_txid = ::std::option::Option::Some(v);
    }

    pub fn get_last_learned_txid<'a>(&self) -> u64 {
        self.last_learned_txid.unwrap_or(0)
    }

    // required uint64 last_accepted_term = 4;

    pub fn clear_last_accepted_term(&mut self) {
        self.last_accepted_term = ::std::option::Option::None;
    }

    pub fn has_last_accepted_term(&self) -> bool {
        self.last_accepted_term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_accepted_term(&mut self, v: u64) {
        self.last_accepted_term = ::std::option::Option::Some(v);
    }

    pub fn get_last_accepted_term<'a>(&self) -> u64 {
        self.last_accepted_term.unwrap_or(0)
    }

    // required uint64 last_accepted_txid = 5;

    pub fn clear_last_accepted_txid(&mut self) {
        self.last_accepted_txid = ::std::option::Option::None;
    }

    pub fn has_last_accepted_txid(&self) -> bool {
        self.last_accepted_txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_accepted_txid(&mut self, v: u64) {
        self.last_accepted_txid = ::std::option::Option::Some(v);
    }

    pub fn get_last_accepted_txid<'a>(&self) -> u64 {
        self.last_accepted_txid.unwrap_or(0)
    }
}

impl ::protobuf::Message for VoteReq {
    fn is_initialized(&self) -> bool {
        if self.term.is_none() {
            return false;
        };
        if self.last_learned_term.is_none() {
            return false;
        };
        if self.last_learned_txid.is_none() {
            return false;
        };
        if self.last_accepted_term.is_none() {
            return false;
        };
        if self.last_accepted_txid.is_none() {
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
                    self.term = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_learned_term = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_learned_txid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_accepted_term = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_accepted_txid = ::std::option::Option::Some(tmp);
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
        for value in self.term.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.last_learned_term.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.last_learned_txid.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.last_accepted_term.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.last_accepted_txid.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.term {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.last_learned_term {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.last_learned_txid {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.last_accepted_term {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.last_accepted_txid {
            try!(os.write_uint64(5, v));
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
                    "term",
                    VoteReq::has_term,
                    VoteReq::get_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_learned_term",
                    VoteReq::has_last_learned_term,
                    VoteReq::get_last_learned_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_learned_txid",
                    VoteReq::has_last_learned_txid,
                    VoteReq::get_last_learned_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_accepted_term",
                    VoteReq::has_last_accepted_term,
                    VoteReq::get_last_accepted_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_accepted_txid",
                    VoteReq::has_last_accepted_txid,
                    VoteReq::get_last_accepted_txid,
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
        self.clear_term();
        self.clear_last_learned_term();
        self.clear_last_learned_txid();
        self.clear_last_accepted_term();
        self.clear_last_accepted_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VoteReq {
    fn eq(&self, other: &VoteReq) -> bool {
        self.term == other.term &&
        self.last_learned_term == other.last_learned_term &&
        self.last_learned_txid == other.last_learned_txid &&
        self.last_accepted_term == other.last_accepted_term &&
        self.last_accepted_txid == other.last_accepted_txid &&
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
    term: ::std::option::Option<u64>,
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
                    term: ::std::option::Option::None,
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

    // required uint64 term = 2;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term<'a>(&self) -> u64 {
        self.term.unwrap_or(0)
    }
}

impl ::protobuf::Message for VoteRes {
    fn is_initialized(&self) -> bool {
        if self.success.is_none() {
            return false;
        };
        if self.term.is_none() {
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
                    self.term = ::std::option::Option::Some(tmp);
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
        for value in self.term.iter() {
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
        if let Some(v) = self.term {
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
                    "term",
                    VoteRes::has_term,
                    VoteRes::get_term,
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
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VoteRes {
    fn eq(&self, other: &VoteRes) -> bool {
        self.success == other.success &&
        self.term == other.term &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VoteRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Append {
    // message fields
    from_txid: ::std::option::Option<u64>,
    from_term: ::std::option::Option<u64>,
    batch: ::protobuf::RepeatedField<Mutation>,
    last_learned_txid: ::std::option::Option<u64>,
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
                    from_txid: ::std::option::Option::None,
                    from_term: ::std::option::Option::None,
                    batch: ::protobuf::RepeatedField::new(),
                    last_learned_txid: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 from_txid = 1;

    pub fn clear_from_txid(&mut self) {
        self.from_txid = ::std::option::Option::None;
    }

    pub fn has_from_txid(&self) -> bool {
        self.from_txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_txid(&mut self, v: u64) {
        self.from_txid = ::std::option::Option::Some(v);
    }

    pub fn get_from_txid<'a>(&self) -> u64 {
        self.from_txid.unwrap_or(0)
    }

    // required uint64 from_term = 2;

    pub fn clear_from_term(&mut self) {
        self.from_term = ::std::option::Option::None;
    }

    pub fn has_from_term(&self) -> bool {
        self.from_term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_term(&mut self, v: u64) {
        self.from_term = ::std::option::Option::Some(v);
    }

    pub fn get_from_term<'a>(&self) -> u64 {
        self.from_term.unwrap_or(0)
    }

    // repeated .rasputin.Mutation batch = 3;

    pub fn clear_batch(&mut self) {
        self.batch.clear();
    }

    // Param is passed by value, moved
    pub fn set_batch(&mut self, v: ::protobuf::RepeatedField<Mutation>) {
        self.batch = v;
    }

    // Mutable pointer to the field.
    pub fn mut_batch<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Mutation> {
        &mut self.batch
    }

    // Take field
    pub fn take_batch(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.batch, ::protobuf::RepeatedField::new())
    }

    pub fn get_batch<'a>(&'a self) -> &'a [Mutation] {
        &self.batch
    }

    // required uint64 last_learned_txid = 4;

    pub fn clear_last_learned_txid(&mut self) {
        self.last_learned_txid = ::std::option::Option::None;
    }

    pub fn has_last_learned_txid(&self) -> bool {
        self.last_learned_txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_learned_txid(&mut self, v: u64) {
        self.last_learned_txid = ::std::option::Option::Some(v);
    }

    pub fn get_last_learned_txid<'a>(&self) -> u64 {
        self.last_learned_txid.unwrap_or(0)
    }
}

impl ::protobuf::Message for Append {
    fn is_initialized(&self) -> bool {
        if self.from_txid.is_none() {
            return false;
        };
        if self.from_term.is_none() {
            return false;
        };
        if self.last_learned_txid.is_none() {
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
                    self.from_txid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.from_term = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.batch));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_learned_txid = ::std::option::Option::Some(tmp);
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
        for value in self.from_txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.from_term.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.batch.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.last_learned_txid.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.from_txid {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.from_term {
            try!(os.write_uint64(2, v));
        };
        for v in self.batch.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.last_learned_txid {
            try!(os.write_uint64(4, v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "from_txid",
                    Append::has_from_txid,
                    Append::get_from_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "from_term",
                    Append::has_from_term,
                    Append::get_from_term,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "batch",
                    Append::get_batch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_learned_txid",
                    Append::has_last_learned_txid,
                    Append::get_last_learned_txid,
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
        self.clear_from_txid();
        self.clear_from_term();
        self.clear_batch();
        self.clear_last_learned_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Append {
    fn eq(&self, other: &Append) -> bool {
        self.from_txid == other.from_txid &&
        self.from_term == other.from_term &&
        self.batch == other.batch &&
        self.last_learned_txid == other.last_learned_txid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Append {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppendRes {
    // message fields
    accepted: ::std::option::Option<bool>,
    last_accepted_txid: ::std::option::Option<u64>,
    last_accepted_term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AppendRes {
    pub fn new() -> AppendRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendRes {
        static mut instance: ::protobuf::lazy::Lazy<AppendRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendRes,
        };
        unsafe {
            instance.get(|| {
                AppendRes {
                    accepted: ::std::option::Option::None,
                    last_accepted_txid: ::std::option::Option::None,
                    last_accepted_term: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool accepted = 1;

    pub fn clear_accepted(&mut self) {
        self.accepted = ::std::option::Option::None;
    }

    pub fn has_accepted(&self) -> bool {
        self.accepted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accepted(&mut self, v: bool) {
        self.accepted = ::std::option::Option::Some(v);
    }

    pub fn get_accepted<'a>(&self) -> bool {
        self.accepted.unwrap_or(false)
    }

    // optional uint64 last_accepted_txid = 2;

    pub fn clear_last_accepted_txid(&mut self) {
        self.last_accepted_txid = ::std::option::Option::None;
    }

    pub fn has_last_accepted_txid(&self) -> bool {
        self.last_accepted_txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_accepted_txid(&mut self, v: u64) {
        self.last_accepted_txid = ::std::option::Option::Some(v);
    }

    pub fn get_last_accepted_txid<'a>(&self) -> u64 {
        self.last_accepted_txid.unwrap_or(0)
    }

    // optional uint64 last_accepted_term = 3;

    pub fn clear_last_accepted_term(&mut self) {
        self.last_accepted_term = ::std::option::Option::None;
    }

    pub fn has_last_accepted_term(&self) -> bool {
        self.last_accepted_term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_accepted_term(&mut self, v: u64) {
        self.last_accepted_term = ::std::option::Option::Some(v);
    }

    pub fn get_last_accepted_term<'a>(&self) -> u64 {
        self.last_accepted_term.unwrap_or(0)
    }
}

impl ::protobuf::Message for AppendRes {
    fn is_initialized(&self) -> bool {
        if self.accepted.is_none() {
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
                    self.accepted = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_accepted_txid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.last_accepted_term = ::std::option::Option::Some(tmp);
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
        if self.accepted.is_some() {
            my_size += 2;
        };
        for value in self.last_accepted_txid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.last_accepted_term.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.accepted {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.last_accepted_txid {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.last_accepted_term {
            try!(os.write_uint64(3, v));
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
        ::std::any::TypeId::of::<AppendRes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendRes {
    fn new() -> AppendRes {
        AppendRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "accepted",
                    AppendRes::has_accepted,
                    AppendRes::get_accepted,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_accepted_txid",
                    AppendRes::has_last_accepted_txid,
                    AppendRes::get_last_accepted_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "last_accepted_term",
                    AppendRes::has_last_accepted_term,
                    AppendRes::get_last_accepted_term,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendRes>(
                    "AppendRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendRes {
    fn clear(&mut self) {
        self.clear_accepted();
        self.clear_last_accepted_txid();
        self.clear_last_accepted_term();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppendRes {
    fn eq(&self, other: &AppendRes) -> bool {
        self.accepted == other.accepted &&
        self.last_accepted_txid == other.last_accepted_txid &&
        self.last_accepted_term == other.last_accepted_term &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppendRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PeerMsg {
    // message fields
    range_prefix: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    srvid: ::protobuf::SingularField<::std::string::String>,
    vote_req: ::protobuf::SingularPtrField<VoteReq>,
    vote_res: ::protobuf::SingularPtrField<VoteRes>,
    append: ::protobuf::SingularPtrField<Append>,
    append_res: ::protobuf::SingularPtrField<AppendRes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PeerMsg {
    pub fn new() -> PeerMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerMsg {
        static mut instance: ::protobuf::lazy::Lazy<PeerMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerMsg,
        };
        unsafe {
            instance.get(|| {
                PeerMsg {
                    range_prefix: ::protobuf::SingularField::none(),
                    srvid: ::protobuf::SingularField::none(),
                    vote_req: ::protobuf::SingularPtrField::none(),
                    vote_res: ::protobuf::SingularPtrField::none(),
                    append: ::protobuf::SingularPtrField::none(),
                    append_res: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes range_prefix = 6;

    pub fn clear_range_prefix(&mut self) {
        self.range_prefix.clear();
    }

    pub fn has_range_prefix(&self) -> bool {
        self.range_prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_prefix(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_prefix<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.range_prefix.is_none() {
            self.range_prefix.set_default();
        };
        self.range_prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_range_prefix(&mut self) -> ::std::vec::Vec<u8> {
        self.range_prefix.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_range_prefix<'a>(&'a self) -> &'a [u8] {
        match self.range_prefix.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required string srvid = 1;

    pub fn clear_srvid(&mut self) {
        self.srvid.clear();
    }

    pub fn has_srvid(&self) -> bool {
        self.srvid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_srvid(&mut self, v: ::std::string::String) {
        self.srvid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_srvid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.srvid.is_none() {
            self.srvid.set_default();
        };
        self.srvid.as_mut().unwrap()
    }

    // Take field
    pub fn take_srvid(&mut self) -> ::std::string::String {
        self.srvid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_srvid<'a>(&'a self) -> &'a str {
        match self.srvid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .rasputin.VoteReq vote_req = 2;

    pub fn clear_vote_req(&mut self) {
        self.vote_req.clear();
    }

    pub fn has_vote_req(&self) -> bool {
        self.vote_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vote_req(&mut self, v: VoteReq) {
        self.vote_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vote_req<'a>(&'a mut self) -> &'a mut VoteReq {
        if self.vote_req.is_none() {
            self.vote_req.set_default();
        };
        self.vote_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_vote_req(&mut self) -> VoteReq {
        self.vote_req.take().unwrap_or_else(|| VoteReq::new())
    }

    pub fn get_vote_req<'a>(&'a self) -> &'a VoteReq {
        self.vote_req.as_ref().unwrap_or_else(|| VoteReq::default_instance())
    }

    // optional .rasputin.VoteRes vote_res = 3;

    pub fn clear_vote_res(&mut self) {
        self.vote_res.clear();
    }

    pub fn has_vote_res(&self) -> bool {
        self.vote_res.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vote_res(&mut self, v: VoteRes) {
        self.vote_res = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vote_res<'a>(&'a mut self) -> &'a mut VoteRes {
        if self.vote_res.is_none() {
            self.vote_res.set_default();
        };
        self.vote_res.as_mut().unwrap()
    }

    // Take field
    pub fn take_vote_res(&mut self) -> VoteRes {
        self.vote_res.take().unwrap_or_else(|| VoteRes::new())
    }

    pub fn get_vote_res<'a>(&'a self) -> &'a VoteRes {
        self.vote_res.as_ref().unwrap_or_else(|| VoteRes::default_instance())
    }

    // optional .rasputin.Append append = 4;

    pub fn clear_append(&mut self) {
        self.append.clear();
    }

    pub fn has_append(&self) -> bool {
        self.append.is_some()
    }

    // Param is passed by value, moved
    pub fn set_append(&mut self, v: Append) {
        self.append = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_append<'a>(&'a mut self) -> &'a mut Append {
        if self.append.is_none() {
            self.append.set_default();
        };
        self.append.as_mut().unwrap()
    }

    // Take field
    pub fn take_append(&mut self) -> Append {
        self.append.take().unwrap_or_else(|| Append::new())
    }

    pub fn get_append<'a>(&'a self) -> &'a Append {
        self.append.as_ref().unwrap_or_else(|| Append::default_instance())
    }

    // optional .rasputin.AppendRes append_res = 5;

    pub fn clear_append_res(&mut self) {
        self.append_res.clear();
    }

    pub fn has_append_res(&self) -> bool {
        self.append_res.is_some()
    }

    // Param is passed by value, moved
    pub fn set_append_res(&mut self, v: AppendRes) {
        self.append_res = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_append_res<'a>(&'a mut self) -> &'a mut AppendRes {
        if self.append_res.is_none() {
            self.append_res.set_default();
        };
        self.append_res.as_mut().unwrap()
    }

    // Take field
    pub fn take_append_res(&mut self) -> AppendRes {
        self.append_res.take().unwrap_or_else(|| AppendRes::new())
    }

    pub fn get_append_res<'a>(&'a self) -> &'a AppendRes {
        self.append_res.as_ref().unwrap_or_else(|| AppendRes::default_instance())
    }
}

impl ::protobuf::Message for PeerMsg {
    fn is_initialized(&self) -> bool {
        if self.range_prefix.is_none() {
            return false;
        };
        if self.srvid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.range_prefix.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.srvid.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.vote_req.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.vote_res.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.append.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.append_res.set_default();
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
        for value in self.range_prefix.iter() {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        for value in self.srvid.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.vote_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.vote_res.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.append.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.append_res.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.range_prefix.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.srvid.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.vote_req.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.vote_res.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.append.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.append_res.as_ref() {
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
        ::std::any::TypeId::of::<PeerMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PeerMsg {
    fn new() -> PeerMsg {
        PeerMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "range_prefix",
                    PeerMsg::has_range_prefix,
                    PeerMsg::get_range_prefix,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "srvid",
                    PeerMsg::has_srvid,
                    PeerMsg::get_srvid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "vote_req",
                    PeerMsg::has_vote_req,
                    PeerMsg::get_vote_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "vote_res",
                    PeerMsg::has_vote_res,
                    PeerMsg::get_vote_res,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "append",
                    PeerMsg::has_append,
                    PeerMsg::get_append,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "append_res",
                    PeerMsg::has_append_res,
                    PeerMsg::get_append_res,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerMsg>(
                    "PeerMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerMsg {
    fn clear(&mut self) {
        self.clear_range_prefix();
        self.clear_srvid();
        self.clear_vote_req();
        self.clear_vote_res();
        self.clear_append();
        self.clear_append_res();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PeerMsg {
    fn eq(&self, other: &PeerMsg) -> bool {
        self.range_prefix == other.range_prefix &&
        self.srvid == other.srvid &&
        self.vote_req == other.vote_req &&
        self.vote_res == other.vote_res &&
        self.append == other.append &&
        self.append_res == other.append_res &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PeerMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RetentionPolicy {
    // message fields
    hours: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RetentionPolicy {
    pub fn new() -> RetentionPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RetentionPolicy {
        static mut instance: ::protobuf::lazy::Lazy<RetentionPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RetentionPolicy,
        };
        unsafe {
            instance.get(|| {
                RetentionPolicy {
                    hours: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 hours = 1;

    pub fn clear_hours(&mut self) {
        self.hours = ::std::option::Option::None;
    }

    pub fn has_hours(&self) -> bool {
        self.hours.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hours(&mut self, v: u64) {
        self.hours = ::std::option::Option::Some(v);
    }

    pub fn get_hours<'a>(&self) -> u64 {
        self.hours.unwrap_or(0)
    }
}

impl ::protobuf::Message for RetentionPolicy {
    fn is_initialized(&self) -> bool {
        if self.hours.is_none() {
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
                    self.hours = ::std::option::Option::Some(tmp);
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
        for value in self.hours.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hours {
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
        ::std::any::TypeId::of::<RetentionPolicy>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RetentionPolicy {
    fn new() -> RetentionPolicy {
        RetentionPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<RetentionPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "hours",
                    RetentionPolicy::has_hours,
                    RetentionPolicy::get_hours,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RetentionPolicy>(
                    "RetentionPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RetentionPolicy {
    fn clear(&mut self) {
        self.clear_hours();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RetentionPolicy {
    fn eq(&self, other: &RetentionPolicy) -> bool {
        self.hours == other.hours &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RetentionPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Collection {
    // message fields
    prefix: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::std::option::Option<CollectionType>,
    name: ::protobuf::SingularField<::std::string::String>,
    retention_policy: ::protobuf::SingularPtrField<RetentionPolicy>,
    replication_factor: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Collection {
    pub fn new() -> Collection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Collection {
        static mut instance: ::protobuf::lazy::Lazy<Collection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Collection,
        };
        unsafe {
            instance.get(|| {
                Collection {
                    prefix: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    retention_policy: ::protobuf::SingularPtrField::none(),
                    replication_factor: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes prefix = 1;

    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::vec::Vec<u8>) {
        self.prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.prefix.is_none() {
            self.prefix.set_default();
        };
        self.prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::vec::Vec<u8> {
        self.prefix.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_prefix<'a>(&'a self) -> &'a [u8] {
        match self.prefix.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required .rasputin.CollectionType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: CollectionType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> CollectionType {
        self.field_type.unwrap_or(CollectionType::KV)
    }

    // required string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .rasputin.RetentionPolicy retention_policy = 4;

    pub fn clear_retention_policy(&mut self) {
        self.retention_policy.clear();
    }

    pub fn has_retention_policy(&self) -> bool {
        self.retention_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retention_policy(&mut self, v: RetentionPolicy) {
        self.retention_policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_retention_policy<'a>(&'a mut self) -> &'a mut RetentionPolicy {
        if self.retention_policy.is_none() {
            self.retention_policy.set_default();
        };
        self.retention_policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_retention_policy(&mut self) -> RetentionPolicy {
        self.retention_policy.take().unwrap_or_else(|| RetentionPolicy::new())
    }

    pub fn get_retention_policy<'a>(&'a self) -> &'a RetentionPolicy {
        self.retention_policy.as_ref().unwrap_or_else(|| RetentionPolicy::default_instance())
    }

    // optional uint32 replication_factor = 5;

    pub fn clear_replication_factor(&mut self) {
        self.replication_factor = ::std::option::Option::None;
    }

    pub fn has_replication_factor(&self) -> bool {
        self.replication_factor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication_factor(&mut self, v: u32) {
        self.replication_factor = ::std::option::Option::Some(v);
    }

    pub fn get_replication_factor<'a>(&self) -> u32 {
        self.replication_factor.unwrap_or(0)
    }
}

impl ::protobuf::Message for Collection {
    fn is_initialized(&self) -> bool {
        if self.prefix.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        if self.name.is_none() {
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
                    let tmp = self.prefix.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.retention_policy.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.replication_factor = ::std::option::Option::Some(tmp);
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
        for value in self.prefix.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.retention_policy.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.replication_factor.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.prefix.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.retention_policy.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.replication_factor {
            try!(os.write_uint32(5, v));
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
        ::std::any::TypeId::of::<Collection>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Collection {
    fn new() -> Collection {
        Collection::new()
    }

    fn descriptor_static(_: ::std::option::Option<Collection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "prefix",
                    Collection::has_prefix,
                    Collection::get_prefix,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    Collection::has_field_type,
                    Collection::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Collection::has_name,
                    Collection::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "retention_policy",
                    Collection::has_retention_policy,
                    Collection::get_retention_policy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "replication_factor",
                    Collection::has_replication_factor,
                    Collection::get_replication_factor,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Collection>(
                    "Collection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Collection {
    fn clear(&mut self) {
        self.clear_prefix();
        self.clear_field_type();
        self.clear_name();
        self.clear_retention_policy();
        self.clear_replication_factor();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Collection {
    fn eq(&self, other: &Collection) -> bool {
        self.prefix == other.prefix &&
        self.field_type == other.field_type &&
        self.name == other.name &&
        self.retention_policy == other.retention_policy &&
        self.replication_factor == other.replication_factor &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Collection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Replica {
    // message fields
    address: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Replica {
    pub fn new() -> Replica {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Replica {
        static mut instance: ::protobuf::lazy::Lazy<Replica> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Replica,
        };
        unsafe {
            instance.get(|| {
                Replica {
                    address: ::protobuf::SingularField::none(),
                    id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address<'a>(&'a self) -> &'a str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required bytes id = 2;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id<'a>(&'a self) -> &'a [u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Replica {
    fn is_initialized(&self) -> bool {
        if self.address.is_none() {
            return false;
        };
        if self.id.is_none() {
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
                    let tmp = self.address.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
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
        for value in self.address.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.id.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.address.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.id.as_ref() {
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
        ::std::any::TypeId::of::<Replica>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Replica {
    fn new() -> Replica {
        Replica::new()
    }

    fn descriptor_static(_: ::std::option::Option<Replica>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "address",
                    Replica::has_address,
                    Replica::get_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "id",
                    Replica::has_id,
                    Replica::get_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Replica>(
                    "Replica",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Replica {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Replica {
    fn eq(&self, other: &Replica) -> bool {
        self.address == other.address &&
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Replica {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Range {
    // message fields
    lower: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    upper: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::std::option::Option<CollectionType>,
    name: ::protobuf::SingularField<::std::string::String>,
    replicas: ::protobuf::RepeatedField<Replica>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Range {
    pub fn new() -> Range {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Range {
        static mut instance: ::protobuf::lazy::Lazy<Range> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Range,
        };
        unsafe {
            instance.get(|| {
                Range {
                    lower: ::protobuf::SingularField::none(),
                    upper: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    replicas: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes lower = 1;

    pub fn clear_lower(&mut self) {
        self.lower.clear();
    }

    pub fn has_lower(&self) -> bool {
        self.lower.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lower(&mut self, v: ::std::vec::Vec<u8>) {
        self.lower = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lower<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.lower.is_none() {
            self.lower.set_default();
        };
        self.lower.as_mut().unwrap()
    }

    // Take field
    pub fn take_lower(&mut self) -> ::std::vec::Vec<u8> {
        self.lower.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_lower<'a>(&'a self) -> &'a [u8] {
        match self.lower.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes upper = 2;

    pub fn clear_upper(&mut self) {
        self.upper.clear();
    }

    pub fn has_upper(&self) -> bool {
        self.upper.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upper(&mut self, v: ::std::vec::Vec<u8>) {
        self.upper = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upper<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.upper.is_none() {
            self.upper.set_default();
        };
        self.upper.as_mut().unwrap()
    }

    // Take field
    pub fn take_upper(&mut self) -> ::std::vec::Vec<u8> {
        self.upper.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_upper<'a>(&'a self) -> &'a [u8] {
        match self.upper.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required .rasputin.CollectionType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: CollectionType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> CollectionType {
        self.field_type.unwrap_or(CollectionType::KV)
    }

    // required string name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .rasputin.Replica replicas = 5;

    pub fn clear_replicas(&mut self) {
        self.replicas.clear();
    }

    // Param is passed by value, moved
    pub fn set_replicas(&mut self, v: ::protobuf::RepeatedField<Replica>) {
        self.replicas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_replicas<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Replica> {
        &mut self.replicas
    }

    // Take field
    pub fn take_replicas(&mut self) -> ::protobuf::RepeatedField<Replica> {
        ::std::mem::replace(&mut self.replicas, ::protobuf::RepeatedField::new())
    }

    pub fn get_replicas<'a>(&'a self) -> &'a [Replica] {
        &self.replicas
    }
}

impl ::protobuf::Message for Range {
    fn is_initialized(&self) -> bool {
        if self.lower.is_none() {
            return false;
        };
        if self.upper.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        if self.name.is_none() {
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
                    let tmp = self.lower.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.upper.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.replicas));
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
        for value in self.lower.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.upper.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.replicas.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lower.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.upper.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(4, &v));
        };
        for v in self.replicas.iter() {
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
        ::std::any::TypeId::of::<Range>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Range {
    fn new() -> Range {
        Range::new()
    }

    fn descriptor_static(_: ::std::option::Option<Range>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "lower",
                    Range::has_lower,
                    Range::get_lower,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "upper",
                    Range::has_upper,
                    Range::get_upper,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    Range::has_field_type,
                    Range::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Range::has_name,
                    Range::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "replicas",
                    Range::get_replicas,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Range>(
                    "Range",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Range {
    fn clear(&mut self) {
        self.clear_lower();
        self.clear_upper();
        self.clear_field_type();
        self.clear_name();
        self.clear_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Range {
    fn eq(&self, other: &Range) -> bool {
        self.lower == other.lower &&
        self.upper == other.upper &&
        self.field_type == other.field_type &&
        self.name == other.name &&
        self.replicas == other.replicas &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Range {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Meta {
    // message fields
    collections: ::protobuf::RepeatedField<Collection>,
    ranges: ::protobuf::RepeatedField<Range>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Meta {
    pub fn new() -> Meta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Meta {
        static mut instance: ::protobuf::lazy::Lazy<Meta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Meta,
        };
        unsafe {
            instance.get(|| {
                Meta {
                    collections: ::protobuf::RepeatedField::new(),
                    ranges: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .rasputin.Collection collections = 1;

    pub fn clear_collections(&mut self) {
        self.collections.clear();
    }

    // Param is passed by value, moved
    pub fn set_collections(&mut self, v: ::protobuf::RepeatedField<Collection>) {
        self.collections = v;
    }

    // Mutable pointer to the field.
    pub fn mut_collections<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Collection> {
        &mut self.collections
    }

    // Take field
    pub fn take_collections(&mut self) -> ::protobuf::RepeatedField<Collection> {
        ::std::mem::replace(&mut self.collections, ::protobuf::RepeatedField::new())
    }

    pub fn get_collections<'a>(&'a self) -> &'a [Collection] {
        &self.collections
    }

    // repeated .rasputin.Range ranges = 2;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<Range>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Range> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<Range> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges<'a>(&'a self) -> &'a [Range] {
        &self.ranges
    }
}

impl ::protobuf::Message for Meta {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.collections));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges));
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
        for value in self.collections.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ranges.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.collections.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.ranges.iter() {
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
        ::std::any::TypeId::of::<Meta>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Meta {
    fn new() -> Meta {
        Meta::new()
    }

    fn descriptor_static(_: ::std::option::Option<Meta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "collections",
                    Meta::get_collections,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ranges",
                    Meta::get_ranges,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Meta>(
                    "Meta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Meta {
    fn clear(&mut self) {
        self.clear_collections();
        self.clear_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Meta {
    fn eq(&self, other: &Meta) -> bool {
        self.collections == other.collections &&
        self.ranges == other.ranges &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Meta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MutationType {
    KVSET = 1,
    KVCAS = 2,
    KVDEL = 3,
}

impl ::protobuf::ProtobufEnum for MutationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MutationType> {
        match value {
            1 => ::std::option::Option::Some(MutationType::KVSET),
            2 => ::std::option::Option::Some(MutationType::KVCAS),
            3 => ::std::option::Option::Some(MutationType::KVDEL),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<MutationType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MutationType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MutationType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CollectionType {
    KV = 1,
    LOG = 2,
    OBJECT = 3,
    TIMESERIES = 4,
}

impl ::protobuf::ProtobufEnum for CollectionType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CollectionType> {
        match value {
            1 => ::std::option::Option::Some(CollectionType::KV),
            2 => ::std::option::Option::Some(CollectionType::LOG),
            3 => ::std::option::Option::Some(CollectionType::OBJECT),
            4 => ::std::option::Option::Some(CollectionType::TIMESERIES),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<CollectionType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CollectionType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CollectionType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1b, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x2f, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x72,
    0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x22, 0x17, 0x0a, 0x06, 0x53, 0x65, 0x74, 0x52, 0x65,
    0x71, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x22, 0x34, 0x0a, 0x06, 0x53, 0x65, 0x74, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x78, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x72, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x08, 0x0a, 0x06, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71,
    0x22, 0x43, 0x0a, 0x06, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x78, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x2e, 0x0a, 0x06, 0x43, 0x41, 0x53, 0x52, 0x65, 0x71, 0x12,
    0x11, 0x0a, 0x09, 0x6e, 0x65, 0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x6c, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x43, 0x0a, 0x06, 0x43, 0x41, 0x53, 0x52, 0x65, 0x73, 0x12,
    0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08,
    0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0b, 0x0a,
    0x03, 0x65, 0x72, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x08, 0x0a, 0x06, 0x44, 0x65,
    0x6c, 0x52, 0x65, 0x71, 0x22, 0x43, 0x0a, 0x06, 0x44, 0x65, 0x6c, 0x52, 0x65, 0x73, 0x12, 0x0f,
    0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0d, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03,
    0x65, 0x72, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x44, 0x0a, 0x08, 0x57, 0x61, 0x74,
    0x63, 0x68, 0x52, 0x65, 0x71, 0x12, 0x11, 0x0a, 0x09, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x74, 0x78,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x65, 0x63, 0x75,
    0x72, 0x73, 0x69, 0x76, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x68,
    0x69, 0x73, 0x74, 0x6f, 0x72, 0x69, 0x63, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x22,
    0x4d, 0x0a, 0x08, 0x57, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73,
    0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x23, 0x0a, 0x07,
    0x68, 0x69, 0x73, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e,
    0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x3c,
    0x0a, 0x0b, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a,
    0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0f,
    0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x0b, 0x0a, 0x03, 0x65, 0x72, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x83, 0x01, 0x0a,
    0x08, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74,
    0x69, 0x6e, 0x2e, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x22, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x11, 0x0a, 0x09, 0x6f, 0x6c, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x25, 0x0a, 0x07, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x0c, 0x0a,
    0x04, 0x74, 0x78, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x65, 0x72, 0x6d, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0xc4, 0x01, 0x0a, 0x06, 0x43, 0x6c,
    0x69, 0x52, 0x65, 0x71, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x65, 0x71, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x04, 0x12, 0x1d, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71,
    0x12, 0x1d, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12,
    0x1d, 0x0a, 0x03, 0x63, 0x61, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x72,
    0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x43, 0x41, 0x53, 0x52, 0x65, 0x71, 0x12, 0x1d,
    0x0a, 0x03, 0x64, 0x65, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x72, 0x61,
    0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x44, 0x65, 0x6c, 0x52, 0x65, 0x71, 0x12, 0x21, 0x0a,
    0x05, 0x77, 0x61, 0x74, 0x63, 0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x72,
    0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x57, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71,
    0x22, 0xe0, 0x01, 0x0a, 0x06, 0x43, 0x6c, 0x69, 0x52, 0x65, 0x73, 0x12, 0x0e, 0x0a, 0x06, 0x72,
    0x65, 0x71, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1d, 0x0a, 0x03, 0x67,
    0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75,
    0x74, 0x69, 0x6e, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x03, 0x73, 0x65,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74,
    0x69, 0x6e, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x03, 0x63, 0x61, 0x73,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69,
    0x6e, 0x2e, 0x43, 0x41, 0x53, 0x52, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x03, 0x64, 0x65, 0x6c, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e,
    0x2e, 0x44, 0x65, 0x6c, 0x52, 0x65, 0x73, 0x12, 0x21, 0x0a, 0x05, 0x77, 0x61, 0x74, 0x63, 0x68,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69,
    0x6e, 0x2e, 0x57, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x12, 0x27, 0x0a, 0x08, 0x72, 0x65,
    0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x72,
    0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x52, 0x65, 0x73, 0x22, 0x85, 0x01, 0x0a, 0x07, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x71, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x19, 0x0a,
    0x11, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x6c, 0x65, 0x61, 0x72, 0x6e, 0x65, 0x64, 0x5f, 0x74, 0x65,
    0x72, 0x6d, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x19, 0x0a, 0x11, 0x6c, 0x61, 0x73, 0x74,
    0x5f, 0x6c, 0x65, 0x61, 0x72, 0x6e, 0x65, 0x64, 0x5f, 0x74, 0x78, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x04, 0x12, 0x1a, 0x0a, 0x12, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x65,
    0x70, 0x74, 0x65, 0x64, 0x5f, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x04, 0x20, 0x02, 0x28, 0x04, 0x12,
    0x1a, 0x0a, 0x12, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x65, 0x64,
    0x5f, 0x74, 0x78, 0x69, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x04, 0x22, 0x28, 0x0a, 0x07, 0x56,
    0x6f, 0x74, 0x65, 0x52, 0x65, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x6c, 0x0a, 0x06, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x12,
    0x11, 0x0a, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x74, 0x78, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x74, 0x65, 0x72, 0x6d, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x21, 0x0a, 0x05, 0x62, 0x61, 0x74, 0x63, 0x68, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e,
    0x4d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x19, 0x0a, 0x11, 0x6c, 0x61, 0x73, 0x74,
    0x5f, 0x6c, 0x65, 0x61, 0x72, 0x6e, 0x65, 0x64, 0x5f, 0x74, 0x78, 0x69, 0x64, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x04, 0x22, 0x55, 0x0a, 0x09, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x52, 0x65, 0x73,
    0x12, 0x10, 0x0a, 0x08, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x08, 0x12, 0x1a, 0x0a, 0x12, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x65, 0x70,
    0x74, 0x65, 0x64, 0x5f, 0x74, 0x78, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1a,
    0x0a, 0x12, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x65, 0x64, 0x5f,
    0x74, 0x65, 0x72, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x22, 0xc3, 0x01, 0x0a, 0x07, 0x50,
    0x65, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x12, 0x14, 0x0a, 0x0c, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f,
    0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05,
    0x73, 0x72, 0x76, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x23, 0x0a, 0x08, 0x76,
    0x6f, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x71,
    0x12, 0x23, 0x0a, 0x08, 0x76, 0x6f, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x56, 0x6f,
    0x74, 0x65, 0x52, 0x65, 0x73, 0x12, 0x20, 0x0a, 0x06, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e,
    0x2e, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x12, 0x27, 0x0a, 0x0a, 0x61, 0x70, 0x70, 0x65, 0x6e,
    0x64, 0x5f, 0x72, 0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x72, 0x61,
    0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x52, 0x65, 0x73,
    0x22, 0x20, 0x0a, 0x0f, 0x52, 0x65, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x6f, 0x6c,
    0x69, 0x63, 0x79, 0x12, 0x0d, 0x0a, 0x05, 0x68, 0x6f, 0x75, 0x72, 0x73, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x04, 0x22, 0xa3, 0x01, 0x0a, 0x0a, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x0e, 0x0a, 0x06, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0c, 0x12, 0x26, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x18, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x43, 0x6f, 0x6c, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x33, 0x0a, 0x10, 0x72, 0x65, 0x74, 0x65, 0x6e,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x19, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x52, 0x65, 0x74,
    0x65, 0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x12, 0x1a, 0x0a, 0x12,
    0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x66, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x26, 0x0a, 0x07, 0x52, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x12, 0x0f, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c,
    0x22, 0x80, 0x01, 0x0a, 0x05, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x6c, 0x6f,
    0x77, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x75, 0x70, 0x70,
    0x65, 0x72, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x26, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69,
    0x6e, 0x2e, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x23,
    0x0a, 0x08, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x52, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x22, 0x52, 0x0a, 0x04, 0x4d, 0x65, 0x74, 0x61, 0x12, 0x29, 0x0a, 0x0b, 0x63,
    0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e, 0x2e, 0x43, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x06, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69,
    0x6e, 0x2e, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x2a, 0x2f, 0x0a, 0x0c, 0x4d, 0x75, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x4b, 0x56, 0x53, 0x45, 0x54,
    0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x4b, 0x56, 0x43, 0x41, 0x53, 0x10, 0x02, 0x12, 0x09, 0x0a,
    0x05, 0x4b, 0x56, 0x44, 0x45, 0x4c, 0x10, 0x03, 0x2a, 0x3d, 0x0a, 0x0e, 0x43, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x06, 0x0a, 0x02, 0x4b, 0x56,
    0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x4c, 0x4f, 0x47, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x4f,
    0x42, 0x4a, 0x45, 0x43, 0x54, 0x10, 0x03, 0x12, 0x0e, 0x0a, 0x0a, 0x54, 0x49, 0x4d, 0x45, 0x53,
    0x45, 0x52, 0x49, 0x45, 0x53, 0x10, 0x04, 0x4a, 0xd9, 0x36, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00,
    0xbd, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x10, 0x0a, 0x33, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x1a, 0x27, 0x0a, 0x20, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x20, 0x3c, 0x2d, 0x3e, 0x20, 0x72, 0x61, 0x73, 0x70, 0x75, 0x74, 0x69, 0x6e,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x0a, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x08, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x08, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00, 0x0f, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0d, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0e,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x11, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x11,
    0x08, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x14, 0x00, 0x19, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x15, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x10,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x1a, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x16, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x16, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x16, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x17, 0x02,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x03, 0x12, 0x03, 0x18, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x18,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x18, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x18, 0x18, 0x19, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1b, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x1b, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x1c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x11, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d,
    0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x1d, 0x1e,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x20, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x20, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x21, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x10, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x22, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x23, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x23, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12,
    0x03, 0x24, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x24,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x12, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x27, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x27, 0x08, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x2a, 0x00, 0x2f, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x2b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2b, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x2c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03,
    0x2d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2e,
    0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2e, 0x18, 0x19,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x31, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x08, 0x01, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00,
    0x12, 0x03, 0x32, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x12, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x33, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x33, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x33, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33,
    0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x34, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x34, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x34, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x34, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x37,
    0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x38, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x38, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x03, 0x39, 0x0b, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x02, 0x12, 0x03, 0x3a, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3a,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x12, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x18, 0x19, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x3d, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x03, 0x3d, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03,
    0x3e, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3e, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3e, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x10, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x3f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f,
    0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3f, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x40, 0x02, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x02, 0x05, 0x12, 0x03, 0x40, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x40, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x40, 0x18, 0x19, 0x0a, 0x17, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x44, 0x00, 0x48,
    0x01, 0x1a, 0x0b, 0x20, 0x64, 0x61, 0x74, 0x61, 0x74, 0x79, 0x70, 0x65, 0x73, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x44, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x45, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x45, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x45, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x46, 0x02, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x02, 0x07, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x46, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x47, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x47, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x47, 0x0a, 0x0b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x4a, 0x00, 0x50,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x4b, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4b, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4c, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4c, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x4c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12,
    0x03, 0x4d, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4d, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x11, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4d, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x4e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x4e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4e, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x03, 0x4f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x4f, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x4f, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x52, 0x00,
    0x55, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x52, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x53, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x53, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x53, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x53, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x54, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x54, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x19, 0x1a, 0x0a, 0x21, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04,
    0x58, 0x00, 0x60, 0x01, 0x1a, 0x15, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f,
    0x70, 0x2d, 0x6c, 0x65, 0x76, 0x6c, 0x20, 0x41, 0x50, 0x49, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0d, 0x01, 0x12, 0x03, 0x58, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12,
    0x03, 0x59, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x59,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x59, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x11, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x59, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x5a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x5a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x5a, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5a, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x03, 0x5b, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x02, 0x06, 0x12, 0x03, 0x5b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x5b, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x03,
    0x5c, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x03, 0x5c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x06, 0x12, 0x03, 0x5c, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5c, 0x12, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5c, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x5d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5d,
    0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x03, 0x5d, 0x18, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x03, 0x5e, 0x02, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x05, 0x04, 0x12, 0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x05, 0x06, 0x12, 0x03, 0x5e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x5e, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x5e, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x06, 0x12, 0x03, 0x5f,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x06, 0x12, 0x03, 0x5f, 0x0b, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x01, 0x12, 0x03, 0x5f, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x06, 0x03, 0x12, 0x03, 0x5f, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e,
    0x12, 0x04, 0x62, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x62,
    0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x63, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x63, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12,
    0x03, 0x64, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x64,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x06, 0x12, 0x03, 0x64, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x64, 0x12, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x02, 0x12, 0x03, 0x65, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x65, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x65, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x65, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x03, 0x65, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x03, 0x66, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x03, 0x66, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x03, 0x06, 0x12, 0x03, 0x66, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x66, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x66, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04, 0x12, 0x03,
    0x67, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x04, 0x12, 0x03, 0x67, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x06, 0x12, 0x03, 0x67, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x01, 0x12, 0x03, 0x67, 0x12, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12, 0x03, 0x67, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x05, 0x12, 0x03, 0x68, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x68, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x68, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x01, 0x12, 0x03, 0x68,
    0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x03, 0x12, 0x03, 0x68, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x06, 0x12, 0x03, 0x69, 0x02, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x06, 0x04, 0x12, 0x03, 0x69, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x06, 0x06, 0x12, 0x03, 0x69, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x69, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x69, 0x22, 0x23, 0x0a, 0x1a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x6f, 0x00, 0x75,
    0x01, 0x1a, 0x0e, 0x0a, 0x20, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x0a,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x70, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x70, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x70, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x71, 0x02, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x03, 0x71, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x71, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x12, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x71, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12,
    0x03, 0x72, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x03, 0x72,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x03, 0x72, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x03, 0x72, 0x12, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x03, 0x72, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0f, 0x02, 0x03, 0x12, 0x03, 0x73, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x73, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x73, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x03, 0x73, 0x27,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x04, 0x12, 0x03, 0x74, 0x02, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12, 0x03, 0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x04, 0x05, 0x12, 0x03, 0x74, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x74, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x74, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x77, 0x00,
    0x7a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x77, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x78, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x04, 0x12, 0x03, 0x78, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x78, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x78, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x78, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x79, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x79, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x03, 0x79, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x79, 0x19, 0x1a, 0x0a, 0x1c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x05,
    0x7f, 0x00, 0x84, 0x01, 0x01, 0x1a, 0x0f, 0x0a, 0x20, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x03, 0x7f,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0x80, 0x01, 0x02, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0x80, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x81, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x81, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04,
    0x82, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0x82,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x06, 0x12, 0x04, 0x82, 0x01,
    0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x14,
    0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0x82, 0x01, 0x1c, 0x1d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x04, 0x83, 0x01, 0x02, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x04, 0x83, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x03, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x03, 0x01, 0x12, 0x04, 0x83, 0x01, 0x12, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x03, 0x03, 0x12, 0x04, 0x83, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12,
    0x12, 0x06, 0x86, 0x01, 0x00, 0x8a, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12,
    0x04, 0x86, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x87,
    0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x01, 0x0b,
    0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x10, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01, 0x1b, 0x1c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0x88, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0x88, 0x01, 0x12, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x88, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02,
    0x02, 0x12, 0x04, 0x89, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x89, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05, 0x12,
    0x04, 0x89, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x89, 0x01, 0x12, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0x89,
    0x01, 0x27, 0x28, 0x0a, 0x2d, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0x8d, 0x01, 0x00, 0x94, 0x01,
    0x01, 0x1a, 0x1f, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x3c, 0x2d, 0x3e, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x70, 0x2d, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x61, 0x70,
    0x69, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x0f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x01, 0x02, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8e, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x8f, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x8f, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8f,
    0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0x90, 0x01, 0x02,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0x90, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x06, 0x12, 0x04, 0x90, 0x01, 0x0b, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0x90, 0x01, 0x13, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0x90, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0x91, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x03, 0x04, 0x12, 0x04, 0x91, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x03, 0x06, 0x12, 0x04, 0x91, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x91, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x91, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x04, 0x12,
    0x04, 0x92, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x92, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x06, 0x12, 0x04, 0x92,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0x92, 0x01,
    0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x03, 0x12, 0x04, 0x92, 0x01, 0x1b,
    0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x05, 0x12, 0x04, 0x93, 0x01, 0x02, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x04, 0x12, 0x04, 0x93, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x06, 0x12, 0x04, 0x93, 0x01, 0x0b, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x05, 0x01, 0x12, 0x04, 0x93, 0x01, 0x15, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x05, 0x03, 0x12, 0x04, 0x93, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x05,
    0x01, 0x12, 0x06, 0x9a, 0x01, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x01, 0x01,
    0x12, 0x04, 0x9a, 0x01, 0x05, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x04,
    0x9b, 0x01, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9b,
    0x01, 0x02, 0x04, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x04, 0x9b, 0x01,
    0x07, 0x08, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x02, 0x05, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x09, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x0d, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x02, 0x08, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x02, 0x02, 0x12, 0x04, 0x9d, 0x01, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02,
    0x12, 0x04, 0x9e, 0x01, 0x0f, 0x10, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xa1, 0x01,
    0x00, 0xa3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x01, 0x02, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x15, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xab, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01,
    0x12, 0x04, 0xa5, 0x01, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04,
    0xa6, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa6,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x1a, 0x1b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa7, 0x01, 0x0b, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x02, 0x12, 0x04, 0xa8, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xa8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xa8, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xa8, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xa8, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x01,
    0x02, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa9, 0x01, 0x0b, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x1b, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x2e, 0x2f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x04, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x04, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x12, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x04, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06,
    0xad, 0x01, 0x00, 0xb0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xad,
    0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xae, 0x01, 0x02,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0xae, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xae, 0x01, 0x12, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xae, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x11, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xaf, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xb2,
    0x01, 0x00, 0xb8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xb2, 0x01,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb3, 0x01, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xb4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xb4, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xb4, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xb4, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04,
    0xb5, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb5,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x06, 0x12, 0x04, 0xb5, 0x01,
    0x0b, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x1a,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb5, 0x01, 0x21, 0x22,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x02, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x03, 0x05, 0x12, 0x04, 0xb6, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x06,
    0x12, 0x04, 0xb7, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xb7, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xb7, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xba, 0x01, 0x00, 0xbd,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x0c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xbb, 0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02,
    0x01, 0x12, 0x04, 0xbc, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xbc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xbc, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xbc, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbc,
    0x01, 0x1a, 0x1b,
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
