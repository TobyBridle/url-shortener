// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc 27.0-rc1
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `shortened-url-response.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ShortenedResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ShortenedResponse {
    // message fields
    // @@protoc_insertion_point(field:ShortenedResponse.slug)
    pub slug: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:ShortenedResponse.error_message)
    pub error_message: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:ShortenedResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ShortenedResponse {
    fn default() -> &'a ShortenedResponse {
        <ShortenedResponse as ::protobuf::Message>::default_instance()
    }
}

impl ShortenedResponse {
    pub fn new() -> ShortenedResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "slug",
            |m: &ShortenedResponse| { &m.slug },
            |m: &mut ShortenedResponse| { &mut m.slug },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "error_message",
            |m: &ShortenedResponse| { &m.error_message },
            |m: &mut ShortenedResponse| { &mut m.error_message },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ShortenedResponse>(
            "ShortenedResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ShortenedResponse {
    const NAME: &'static str = "ShortenedResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.slug = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    self.error_message = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.slug.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.error_message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.slug.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ShortenedResponse {
        ShortenedResponse::new()
    }

    fn clear(&mut self) {
        self.slug = ::std::option::Option::None;
        self.error_message = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ShortenedResponse {
        static instance: ShortenedResponse = ShortenedResponse {
            slug: ::std::option::Option::None,
            error_message: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ShortenedResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ShortenedResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ShortenedResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortenedResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cshortened-url-response.proto\"q\n\x11ShortenedResponse\x12\x17\n\
    \x04slug\x18\x01\x20\x01(\tH\0R\x04slug\x88\x01\x01\x12(\n\rerror_messag\
    e\x18\x02\x20\x01(\tH\x01R\x0cerrorMessage\x88\x01\x01B\x07\n\x05_slugB\
    \x10\n\x0e_error_messageb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ShortenedResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
