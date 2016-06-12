// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default,Debug)]
pub struct compressed_image {
    // message fields
    width: ::std::option::Option<u32>,
    height: ::std::option::Option<u32>,
    red: ::std::vec::Vec<i32>,
    green: ::std::vec::Vec<i32>,
    blue: ::std::vec::Vec<i32>,
    alpha: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for compressed_image {}

impl compressed_image {
    pub fn new() -> compressed_image {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static compressed_image {
        static mut instance: ::protobuf::lazy::Lazy<compressed_image> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const compressed_image,
        };
        unsafe {
            instance.get(|| {
                compressed_image {
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    red: ::std::vec::Vec::new(),
                    green: ::std::vec::Vec::new(),
                    blue: ::std::vec::Vec::new(),
                    alpha: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 width = 2;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: u32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> u32 {
        self.width.unwrap_or(0)
    }

    // optional uint32 height = 3;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> u32 {
        self.height.unwrap_or(0)
    }

    // repeated sint32 red = 6;

    pub fn clear_red(&mut self) {
        self.red.clear();
    }

    // Param is passed by value, moved
    pub fn set_red(&mut self, v: ::std::vec::Vec<i32>) {
        self.red = v;
    }

    // Mutable pointer to the field.
    pub fn mut_red(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.red
    }

    // Take field
    pub fn take_red(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.red, ::std::vec::Vec::new())
    }

    pub fn get_red(&self) -> &[i32] {
        &self.red
    }

    // repeated sint32 green = 7;

    pub fn clear_green(&mut self) {
        self.green.clear();
    }

    // Param is passed by value, moved
    pub fn set_green(&mut self, v: ::std::vec::Vec<i32>) {
        self.green = v;
    }

    // Mutable pointer to the field.
    pub fn mut_green(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.green
    }

    // Take field
    pub fn take_green(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.green, ::std::vec::Vec::new())
    }

    pub fn get_green(&self) -> &[i32] {
        &self.green
    }

    // repeated sint32 blue = 8;

    pub fn clear_blue(&mut self) {
        self.blue.clear();
    }

    // Param is passed by value, moved
    pub fn set_blue(&mut self, v: ::std::vec::Vec<i32>) {
        self.blue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blue(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.blue
    }

    // Take field
    pub fn take_blue(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.blue, ::std::vec::Vec::new())
    }

    pub fn get_blue(&self) -> &[i32] {
        &self.blue
    }

    // repeated sint32 alpha = 9;

    pub fn clear_alpha(&mut self) {
        self.alpha.clear();
    }

    // Param is passed by value, moved
    pub fn set_alpha(&mut self, v: ::std::vec::Vec<i32>) {
        self.alpha = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alpha(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.alpha
    }

    // Take field
    pub fn take_alpha(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.alpha, ::std::vec::Vec::new())
    }

    pub fn get_alpha(&self) -> &[i32] {
        &self.alpha
    }
}

impl ::protobuf::Message for compressed_image {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.width = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.red));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.green));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.blue));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.alpha));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.width.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.height.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.red.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(6, &self.red);
        };
        if !self.green.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(7, &self.green);
        };
        if !self.blue.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(8, &self.blue);
        };
        if !self.alpha.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(9, &self.alpha);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.height {
            try!(os.write_uint32(3, v));
        };
        if !self.red.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.red)));
            for v in self.red.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.green.is_empty() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.green)));
            for v in self.green.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.blue.is_empty() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.blue)));
            for v in self.blue.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.alpha.is_empty() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.alpha)));
            for v in self.alpha.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<compressed_image>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for compressed_image {
    fn new() -> compressed_image {
        compressed_image::new()
    }
}

impl ::protobuf::Clear for compressed_image {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_height();
        self.clear_red();
        self.clear_green();
        self.clear_blue();
        self.clear_alpha();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for compressed_image {
    fn eq(&self, other: &compressed_image) -> bool {
        self.width == other.width &&
        self.height == other.height &&
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue &&
        self.alpha == other.alpha &&
        self.unknown_fields == other.unknown_fields
    }
}
