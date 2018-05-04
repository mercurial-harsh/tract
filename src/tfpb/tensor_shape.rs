// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702



use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct TensorShapeProto {
    // message fields
    pub dim: ::protobuf::RepeatedField<TensorShapeProto_Dim>,
    pub unknown_rank: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorShapeProto {}

impl TensorShapeProto {
    pub fn new() -> TensorShapeProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorShapeProto {
        static mut instance: ::protobuf::lazy::Lazy<TensorShapeProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorShapeProto,
        };
        unsafe {
            instance.get(TensorShapeProto::new)
        }
    }

    // repeated .tensorflow.TensorShapeProto.Dim dim = 2;

    pub fn clear_dim(&mut self) {
        self.dim.clear();
    }

    // Param is passed by value, moved
    pub fn set_dim(&mut self, v: ::protobuf::RepeatedField<TensorShapeProto_Dim>) {
        self.dim = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dim(&mut self) -> &mut ::protobuf::RepeatedField<TensorShapeProto_Dim> {
        &mut self.dim
    }

    // Take field
    pub fn take_dim(&mut self) -> ::protobuf::RepeatedField<TensorShapeProto_Dim> {
        ::std::mem::replace(&mut self.dim, ::protobuf::RepeatedField::new())
    }

    pub fn get_dim(&self) -> &[TensorShapeProto_Dim] {
        &self.dim
    }

    fn get_dim_for_reflect(&self) -> &::protobuf::RepeatedField<TensorShapeProto_Dim> {
        &self.dim
    }

    fn mut_dim_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TensorShapeProto_Dim> {
        &mut self.dim
    }

    // bool unknown_rank = 3;

    pub fn clear_unknown_rank(&mut self) {
        self.unknown_rank = false;
    }

    // Param is passed by value, moved
    pub fn set_unknown_rank(&mut self, v: bool) {
        self.unknown_rank = v;
    }

    pub fn get_unknown_rank(&self) -> bool {
        self.unknown_rank
    }

    fn get_unknown_rank_for_reflect(&self) -> &bool {
        &self.unknown_rank
    }

    fn mut_unknown_rank_for_reflect(&mut self) -> &mut bool {
        &mut self.unknown_rank
    }
}

impl ::protobuf::Message for TensorShapeProto {
    fn is_initialized(&self) -> bool {
        for v in &self.dim {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.dim)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.unknown_rank = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.dim {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.unknown_rank != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.dim {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.unknown_rank != false {
            os.write_bool(3, self.unknown_rank)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TensorShapeProto {
    fn new() -> TensorShapeProto {
        TensorShapeProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorShapeProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TensorShapeProto_Dim>>(
                    "dim",
                    TensorShapeProto::get_dim_for_reflect,
                    TensorShapeProto::mut_dim_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "unknown_rank",
                    TensorShapeProto::get_unknown_rank_for_reflect,
                    TensorShapeProto::mut_unknown_rank_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorShapeProto>(
                    "TensorShapeProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorShapeProto {
    fn clear(&mut self) {
        self.clear_dim();
        self.clear_unknown_rank();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorShapeProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorShapeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TensorShapeProto_Dim {
    // message fields
    pub size: i64,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorShapeProto_Dim {}

impl TensorShapeProto_Dim {
    pub fn new() -> TensorShapeProto_Dim {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorShapeProto_Dim {
        static mut instance: ::protobuf::lazy::Lazy<TensorShapeProto_Dim> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorShapeProto_Dim,
        };
        unsafe {
            instance.get(TensorShapeProto_Dim::new)
        }
    }

    // int64 size = 1;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i64) {
        self.size = v;
    }

    pub fn get_size(&self) -> i64 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &i64 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.size
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for TensorShapeProto_Dim {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.size = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.size != 0 {
            os.write_int64(1, self.size)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TensorShapeProto_Dim {
    fn new() -> TensorShapeProto_Dim {
        TensorShapeProto_Dim::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorShapeProto_Dim>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "size",
                    TensorShapeProto_Dim::get_size_for_reflect,
                    TensorShapeProto_Dim::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    TensorShapeProto_Dim::get_name_for_reflect,
                    TensorShapeProto_Dim::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorShapeProto_Dim>(
                    "TensorShapeProto_Dim",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorShapeProto_Dim {
    fn clear(&mut self) {
        self.clear_size();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorShapeProto_Dim {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorShapeProto_Dim {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,tensorflow/core/framework/tensor_shape.proto\x12\ntensorflow\"\x98\
    \x01\n\x10TensorShapeProto\x122\n\x03dim\x18\x02\x20\x03(\x0b2\x20.tenso\
    rflow.TensorShapeProto.DimR\x03dim\x12!\n\x0cunknown_rank\x18\x03\x20\
    \x01(\x08R\x0bunknownRank\x1a-\n\x03Dim\x12\x12\n\x04size\x18\x01\x20\
    \x01(\x03R\x04size\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04nameB2\n\x18\
    org.tensorflow.frameworkB\x11TensorShapeProtosP\x01\xf8\x01\x01b\x06prot\
    o3\
";

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