/// This module contains useful macros for anywhere in the project (hopefully)

/// Ok this is horrifying, rust does not currently support creation of new identifier in fonction name position. (in a stable way).
#[macro_export]
macro_rules! gen_builder_pattern_bitfields_methods {
    (#[$doc_setter: meta], #[$doc_getter: meta], $field: ident, $setter_name: ident, $bit: expr, $member: ident) => {
        #[$doc_setter]
        #[allow(dead_code)]
        pub fn $setter_name(&mut self, value: bool) -> &mut Self {
            use bit_field::BitField;

            self.$member.set_bit($bit, value);
            self
        }

        #[allow(dead_code)]
        #[$doc_getter]
        pub fn $field(&self) -> bool {
            use bit_field::BitField;

            self.$member.get_bit($bit)
        }
    };
}

/// Ok this is horrifying, rust does not currently support creation of new identifier in fonction name position. (in a stable way).
#[macro_export]
macro_rules! gen_builder_pattern_bitfields_range_methods {
    ($field: ident, $setter_name: ident, $bits: expr, $member: ident, $type: ty) => {
        #[allow(dead_code)]
        pub fn $setter_name(&mut self, value: $type ) -> &mut Self {
            use bit_field::BitField;

            self.$member.set_bits($bits, value.into());
            self
        }

        #[allow(dead_code)]
        pub fn $field(&self) -> $type {
            use bit_field::BitField;

            self.$member.get_bits($bits)
        }
    }
}

#[cfg(test)]
mod test {
    struct Bitfield {
        pub data: u32,
    }
    impl Bitfield {
        gen_builder_pattern_bitfields_methods!(#[doc="getbi2"], #[doc="setbit2"], bit2, set_bit2, 2, data);
    }

    #[test]
    fn test_gen_builder_pattern_bitfields_methods() {
        let mut a = Bitfield { data: 0 };
        assert_eq!(a.bit2(), false);
        a.set_bit2(true);
        assert_eq!(a.bit2(), true);
    }
}
