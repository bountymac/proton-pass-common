// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc 28.2
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `google_authenticator.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MigrationPayload)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MigrationPayload {
    // message fields
    // @@protoc_insertion_point(field:MigrationPayload.otp_parameters)
    pub otp_parameters: ::std::vec::Vec<migration_payload::OtpParameters>,
    // @@protoc_insertion_point(field:MigrationPayload.version)
    pub version: i32,
    // @@protoc_insertion_point(field:MigrationPayload.batch_size)
    pub batch_size: i32,
    // @@protoc_insertion_point(field:MigrationPayload.batch_index)
    pub batch_index: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:MigrationPayload.batch_id)
    pub batch_id: i32,
    // special fields
    // @@protoc_insertion_point(special_field:MigrationPayload.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MigrationPayload {
    fn default() -> &'a MigrationPayload {
        <MigrationPayload as ::protobuf::Message>::default_instance()
    }
}

impl MigrationPayload {
    pub fn new() -> MigrationPayload {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "otp_parameters",
            |m: &MigrationPayload| { &m.otp_parameters },
            |m: &mut MigrationPayload| { &mut m.otp_parameters },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "version",
            |m: &MigrationPayload| { &m.version },
            |m: &mut MigrationPayload| { &mut m.version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "batch_size",
            |m: &MigrationPayload| { &m.batch_size },
            |m: &mut MigrationPayload| { &mut m.batch_size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "batch_index",
            |m: &MigrationPayload| { &m.batch_index },
            |m: &mut MigrationPayload| { &mut m.batch_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "batch_id",
            |m: &MigrationPayload| { &m.batch_id },
            |m: &mut MigrationPayload| { &mut m.batch_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MigrationPayload>(
            "MigrationPayload",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MigrationPayload {
    const NAME: &'static str = "MigrationPayload";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.otp_parameters.push(is.read_message()?);
                },
                16 => {
                    self.version = is.read_int32()?;
                },
                24 => {
                    self.batch_size = is.read_int32()?;
                },
                32 => {
                    self.batch_index = ::std::option::Option::Some(is.read_int32()?);
                },
                40 => {
                    self.batch_id = is.read_int32()?;
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
        for value in &self.otp_parameters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.version != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.version);
        }
        if self.batch_size != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.batch_size);
        }
        if let Some(v) = self.batch_index {
            my_size += ::protobuf::rt::int32_size(4, v);
        }
        if self.batch_id != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.batch_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.otp_parameters {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.version != 0 {
            os.write_int32(2, self.version)?;
        }
        if self.batch_size != 0 {
            os.write_int32(3, self.batch_size)?;
        }
        if let Some(v) = self.batch_index {
            os.write_int32(4, v)?;
        }
        if self.batch_id != 0 {
            os.write_int32(5, self.batch_id)?;
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

    fn new() -> MigrationPayload {
        MigrationPayload::new()
    }

    fn clear(&mut self) {
        self.otp_parameters.clear();
        self.version = 0;
        self.batch_size = 0;
        self.batch_index = ::std::option::Option::None;
        self.batch_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MigrationPayload {
        static instance: MigrationPayload = MigrationPayload {
            otp_parameters: ::std::vec::Vec::new(),
            version: 0,
            batch_size: 0,
            batch_index: ::std::option::Option::None,
            batch_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MigrationPayload {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MigrationPayload").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MigrationPayload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MigrationPayload {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `MigrationPayload`
pub mod migration_payload {
    // @@protoc_insertion_point(message:MigrationPayload.OtpParameters)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct OtpParameters {
        // message fields
        // @@protoc_insertion_point(field:MigrationPayload.OtpParameters.secret)
        pub secret: ::std::vec::Vec<u8>,
        // @@protoc_insertion_point(field:MigrationPayload.OtpParameters.name)
        pub name: ::std::string::String,
        // @@protoc_insertion_point(field:MigrationPayload.OtpParameters.issuer)
        pub issuer: ::std::string::String,
        // @@protoc_insertion_point(field:MigrationPayload.OtpParameters.algorithm)
        pub algorithm: ::protobuf::EnumOrUnknown<Algorithm>,
        // @@protoc_insertion_point(field:MigrationPayload.OtpParameters.digits)
        pub digits: ::protobuf::EnumOrUnknown<DigitCount>,
        // @@protoc_insertion_point(field:MigrationPayload.OtpParameters.type)
        pub type_: ::protobuf::EnumOrUnknown<OtpType>,
        // @@protoc_insertion_point(field:MigrationPayload.OtpParameters.counter)
        pub counter: ::std::option::Option<i64>,
        // special fields
        // @@protoc_insertion_point(special_field:MigrationPayload.OtpParameters.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a OtpParameters {
        fn default() -> &'a OtpParameters {
            <OtpParameters as ::protobuf::Message>::default_instance()
        }
    }

    impl OtpParameters {
        pub fn new() -> OtpParameters {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(7);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "secret",
                |m: &OtpParameters| { &m.secret },
                |m: &mut OtpParameters| { &mut m.secret },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "name",
                |m: &OtpParameters| { &m.name },
                |m: &mut OtpParameters| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "issuer",
                |m: &OtpParameters| { &m.issuer },
                |m: &mut OtpParameters| { &mut m.issuer },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "algorithm",
                |m: &OtpParameters| { &m.algorithm },
                |m: &mut OtpParameters| { &mut m.algorithm },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "digits",
                |m: &OtpParameters| { &m.digits },
                |m: &mut OtpParameters| { &mut m.digits },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "type",
                |m: &OtpParameters| { &m.type_ },
                |m: &mut OtpParameters| { &mut m.type_ },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
                "counter",
                |m: &OtpParameters| { &m.counter },
                |m: &mut OtpParameters| { &mut m.counter },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OtpParameters>(
                "MigrationPayload.OtpParameters",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for OtpParameters {
        const NAME: &'static str = "OtpParameters";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.secret = is.read_bytes()?;
                    },
                    18 => {
                        self.name = is.read_string()?;
                    },
                    26 => {
                        self.issuer = is.read_string()?;
                    },
                    32 => {
                        self.algorithm = is.read_enum_or_unknown()?;
                    },
                    40 => {
                        self.digits = is.read_enum_or_unknown()?;
                    },
                    48 => {
                        self.type_ = is.read_enum_or_unknown()?;
                    },
                    56 => {
                        self.counter = ::std::option::Option::Some(is.read_int64()?);
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
            if !self.secret.is_empty() {
                my_size += ::protobuf::rt::bytes_size(1, &self.secret);
            }
            if !self.name.is_empty() {
                my_size += ::protobuf::rt::string_size(2, &self.name);
            }
            if !self.issuer.is_empty() {
                my_size += ::protobuf::rt::string_size(3, &self.issuer);
            }
            if self.algorithm != ::protobuf::EnumOrUnknown::new(Algorithm::ALGORITHM_UNSPECIFIED) {
                my_size += ::protobuf::rt::int32_size(4, self.algorithm.value());
            }
            if self.digits != ::protobuf::EnumOrUnknown::new(DigitCount::DIGIT_COUNT_UNSPECIFIED) {
                my_size += ::protobuf::rt::int32_size(5, self.digits.value());
            }
            if self.type_ != ::protobuf::EnumOrUnknown::new(OtpType::OTP_TYPE_UNSPECIFIED) {
                my_size += ::protobuf::rt::int32_size(6, self.type_.value());
            }
            if let Some(v) = self.counter {
                my_size += ::protobuf::rt::int64_size(7, v);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.secret.is_empty() {
                os.write_bytes(1, &self.secret)?;
            }
            if !self.name.is_empty() {
                os.write_string(2, &self.name)?;
            }
            if !self.issuer.is_empty() {
                os.write_string(3, &self.issuer)?;
            }
            if self.algorithm != ::protobuf::EnumOrUnknown::new(Algorithm::ALGORITHM_UNSPECIFIED) {
                os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.algorithm))?;
            }
            if self.digits != ::protobuf::EnumOrUnknown::new(DigitCount::DIGIT_COUNT_UNSPECIFIED) {
                os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.digits))?;
            }
            if self.type_ != ::protobuf::EnumOrUnknown::new(OtpType::OTP_TYPE_UNSPECIFIED) {
                os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.type_))?;
            }
            if let Some(v) = self.counter {
                os.write_int64(7, v)?;
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

        fn new() -> OtpParameters {
            OtpParameters::new()
        }

        fn clear(&mut self) {
            self.secret.clear();
            self.name.clear();
            self.issuer.clear();
            self.algorithm = ::protobuf::EnumOrUnknown::new(Algorithm::ALGORITHM_UNSPECIFIED);
            self.digits = ::protobuf::EnumOrUnknown::new(DigitCount::DIGIT_COUNT_UNSPECIFIED);
            self.type_ = ::protobuf::EnumOrUnknown::new(OtpType::OTP_TYPE_UNSPECIFIED);
            self.counter = ::std::option::Option::None;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static OtpParameters {
            static instance: OtpParameters = OtpParameters {
                secret: ::std::vec::Vec::new(),
                name: ::std::string::String::new(),
                issuer: ::std::string::String::new(),
                algorithm: ::protobuf::EnumOrUnknown::from_i32(0),
                digits: ::protobuf::EnumOrUnknown::from_i32(0),
                type_: ::protobuf::EnumOrUnknown::from_i32(0),
                counter: ::std::option::Option::None,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for OtpParameters {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("MigrationPayload.OtpParameters").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for OtpParameters {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for OtpParameters {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:MigrationPayload.Algorithm)
    pub enum Algorithm {
        // @@protoc_insertion_point(enum_value:MigrationPayload.Algorithm.ALGORITHM_UNSPECIFIED)
        ALGORITHM_UNSPECIFIED = 0,
        // @@protoc_insertion_point(enum_value:MigrationPayload.Algorithm.ALGORITHM_SHA1)
        ALGORITHM_SHA1 = 1,
        // @@protoc_insertion_point(enum_value:MigrationPayload.Algorithm.ALGORITHM_SHA256)
        ALGORITHM_SHA256 = 2,
        // @@protoc_insertion_point(enum_value:MigrationPayload.Algorithm.ALGORITHM_SHA512)
        ALGORITHM_SHA512 = 3,
        // @@protoc_insertion_point(enum_value:MigrationPayload.Algorithm.ALGORITHM_MD5)
        ALGORITHM_MD5 = 4,
    }

    impl ::protobuf::Enum for Algorithm {
        const NAME: &'static str = "Algorithm";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Algorithm> {
            match value {
                0 => ::std::option::Option::Some(Algorithm::ALGORITHM_UNSPECIFIED),
                1 => ::std::option::Option::Some(Algorithm::ALGORITHM_SHA1),
                2 => ::std::option::Option::Some(Algorithm::ALGORITHM_SHA256),
                3 => ::std::option::Option::Some(Algorithm::ALGORITHM_SHA512),
                4 => ::std::option::Option::Some(Algorithm::ALGORITHM_MD5),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Algorithm> {
            match str {
                "ALGORITHM_UNSPECIFIED" => ::std::option::Option::Some(Algorithm::ALGORITHM_UNSPECIFIED),
                "ALGORITHM_SHA1" => ::std::option::Option::Some(Algorithm::ALGORITHM_SHA1),
                "ALGORITHM_SHA256" => ::std::option::Option::Some(Algorithm::ALGORITHM_SHA256),
                "ALGORITHM_SHA512" => ::std::option::Option::Some(Algorithm::ALGORITHM_SHA512),
                "ALGORITHM_MD5" => ::std::option::Option::Some(Algorithm::ALGORITHM_MD5),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Algorithm] = &[
            Algorithm::ALGORITHM_UNSPECIFIED,
            Algorithm::ALGORITHM_SHA1,
            Algorithm::ALGORITHM_SHA256,
            Algorithm::ALGORITHM_SHA512,
            Algorithm::ALGORITHM_MD5,
        ];
    }

    impl ::protobuf::EnumFull for Algorithm {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("MigrationPayload.Algorithm").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Algorithm {
        fn default() -> Self {
            Algorithm::ALGORITHM_UNSPECIFIED
        }
    }

    impl Algorithm {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Algorithm>("MigrationPayload.Algorithm")
        }
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:MigrationPayload.DigitCount)
    pub enum DigitCount {
        // @@protoc_insertion_point(enum_value:MigrationPayload.DigitCount.DIGIT_COUNT_UNSPECIFIED)
        DIGIT_COUNT_UNSPECIFIED = 0,
        // @@protoc_insertion_point(enum_value:MigrationPayload.DigitCount.DIGIT_COUNT_SIX)
        DIGIT_COUNT_SIX = 1,
        // @@protoc_insertion_point(enum_value:MigrationPayload.DigitCount.DIGIT_COUNT_EIGHT)
        DIGIT_COUNT_EIGHT = 2,
    }

    impl ::protobuf::Enum for DigitCount {
        const NAME: &'static str = "DigitCount";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<DigitCount> {
            match value {
                0 => ::std::option::Option::Some(DigitCount::DIGIT_COUNT_UNSPECIFIED),
                1 => ::std::option::Option::Some(DigitCount::DIGIT_COUNT_SIX),
                2 => ::std::option::Option::Some(DigitCount::DIGIT_COUNT_EIGHT),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<DigitCount> {
            match str {
                "DIGIT_COUNT_UNSPECIFIED" => ::std::option::Option::Some(DigitCount::DIGIT_COUNT_UNSPECIFIED),
                "DIGIT_COUNT_SIX" => ::std::option::Option::Some(DigitCount::DIGIT_COUNT_SIX),
                "DIGIT_COUNT_EIGHT" => ::std::option::Option::Some(DigitCount::DIGIT_COUNT_EIGHT),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [DigitCount] = &[
            DigitCount::DIGIT_COUNT_UNSPECIFIED,
            DigitCount::DIGIT_COUNT_SIX,
            DigitCount::DIGIT_COUNT_EIGHT,
        ];
    }

    impl ::protobuf::EnumFull for DigitCount {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("MigrationPayload.DigitCount").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for DigitCount {
        fn default() -> Self {
            DigitCount::DIGIT_COUNT_UNSPECIFIED
        }
    }

    impl DigitCount {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DigitCount>("MigrationPayload.DigitCount")
        }
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:MigrationPayload.OtpType)
    pub enum OtpType {
        // @@protoc_insertion_point(enum_value:MigrationPayload.OtpType.OTP_TYPE_UNSPECIFIED)
        OTP_TYPE_UNSPECIFIED = 0,
        // @@protoc_insertion_point(enum_value:MigrationPayload.OtpType.OTP_TYPE_HOTP)
        OTP_TYPE_HOTP = 1,
        // @@protoc_insertion_point(enum_value:MigrationPayload.OtpType.OTP_TYPE_TOTP)
        OTP_TYPE_TOTP = 2,
    }

    impl ::protobuf::Enum for OtpType {
        const NAME: &'static str = "OtpType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<OtpType> {
            match value {
                0 => ::std::option::Option::Some(OtpType::OTP_TYPE_UNSPECIFIED),
                1 => ::std::option::Option::Some(OtpType::OTP_TYPE_HOTP),
                2 => ::std::option::Option::Some(OtpType::OTP_TYPE_TOTP),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<OtpType> {
            match str {
                "OTP_TYPE_UNSPECIFIED" => ::std::option::Option::Some(OtpType::OTP_TYPE_UNSPECIFIED),
                "OTP_TYPE_HOTP" => ::std::option::Option::Some(OtpType::OTP_TYPE_HOTP),
                "OTP_TYPE_TOTP" => ::std::option::Option::Some(OtpType::OTP_TYPE_TOTP),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [OtpType] = &[
            OtpType::OTP_TYPE_UNSPECIFIED,
            OtpType::OTP_TYPE_HOTP,
            OtpType::OTP_TYPE_TOTP,
        ];
    }

    impl ::protobuf::EnumFull for OtpType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("MigrationPayload.OtpType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for OtpType {
        fn default() -> Self {
            OtpType::OTP_TYPE_UNSPECIFIED
        }
    }

    impl OtpType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<OtpType>("MigrationPayload.OtpType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1agoogle_authenticator.proto\"\xa2\x06\n\x10MigrationPayload\x12F\n\
    \x0eotp_parameters\x18\x01\x20\x03(\x0b2\x1f.MigrationPayload.OtpParamet\
    ersR\rotpParameters\x12\x18\n\x07version\x18\x02\x20\x01(\x05R\x07versio\
    n\x12\x1d\n\nbatch_size\x18\x03\x20\x01(\x05R\tbatchSize\x12$\n\x0bbatch\
    _index\x18\x04\x20\x01(\x05H\0R\nbatchIndex\x88\x01\x01\x12\x19\n\x08bat\
    ch_id\x18\x05\x20\x01(\x05R\x07batchId\x1a\x9e\x02\n\rOtpParameters\x12\
    \x16\n\x06secret\x18\x01\x20\x01(\x0cR\x06secret\x12\x12\n\x04name\x18\
    \x02\x20\x01(\tR\x04name\x12\x16\n\x06issuer\x18\x03\x20\x01(\tR\x06issu\
    er\x129\n\talgorithm\x18\x04\x20\x01(\x0e2\x1b.MigrationPayload.Algorith\
    mR\talgorithm\x124\n\x06digits\x18\x05\x20\x01(\x0e2\x1c.MigrationPayloa\
    d.DigitCountR\x06digits\x12-\n\x04type\x18\x06\x20\x01(\x0e2\x19.Migrati\
    onPayload.OtpTypeR\x04type\x12\x1d\n\x07counter\x18\x07\x20\x01(\x03H\0R\
    \x07counter\x88\x01\x01B\n\n\x08_counter\"y\n\tAlgorithm\x12\x19\n\x15AL\
    GORITHM_UNSPECIFIED\x10\0\x12\x12\n\x0eALGORITHM_SHA1\x10\x01\x12\x14\n\
    \x10ALGORITHM_SHA256\x10\x02\x12\x14\n\x10ALGORITHM_SHA512\x10\x03\x12\
    \x11\n\rALGORITHM_MD5\x10\x04\"U\n\nDigitCount\x12\x1b\n\x17DIGIT_COUNT_\
    UNSPECIFIED\x10\0\x12\x13\n\x0fDIGIT_COUNT_SIX\x10\x01\x12\x15\n\x11DIGI\
    T_COUNT_EIGHT\x10\x02\"I\n\x07OtpType\x12\x18\n\x14OTP_TYPE_UNSPECIFIED\
    \x10\0\x12\x11\n\rOTP_TYPE_HOTP\x10\x01\x12\x11\n\rOTP_TYPE_TOTP\x10\x02\
    B\x0e\n\x0c_batch_indexb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(MigrationPayload::generated_message_descriptor_data());
            messages.push(migration_payload::OtpParameters::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(3);
            enums.push(migration_payload::Algorithm::generated_enum_descriptor_data());
            enums.push(migration_payload::DigitCount::generated_enum_descriptor_data());
            enums.push(migration_payload::OtpType::generated_enum_descriptor_data());
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
