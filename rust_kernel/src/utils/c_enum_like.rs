#[allow(unused_macros)]
macro_rules! r#abstract {
    ($ident: ident) => {
        macro_rules! $ident {
            () => {
                println!("test");
            };
        }
    };
}

#[allow(unused_macros)]
macro_rules! discard {
    ($discarded: tt) => {};
}

#[allow(unused_macros)]
macro_rules! replace_tt {
    ($discarded: tt, $by: tt) => {
        $by
    };
}

#[allow(unused_macros)]
macro_rules! replace_tt_cond {
    ($discarded: tt, $sub: tt) => {
        $sub
    };

    ($_t: tt $sub: tt, $discarded: tt) => {
        $sub
    };
}

#[macro_export]
macro_rules! enum_c_like_assoc {
    ($(#[$attribute:meta])* enum $name: ident {
        $($variant_name: ident ($content: ty) = $associated_number: expr ,)*
    }) => {

        #[derive(Copy, Clone, Debug)] // So yeah, the enum type must implement Copy/Clone and Debug...
        $(#[$attribute])* pub enum $name {
            $($variant_name($content),)*
        }

        impl From<&[u32]> for $name {
            fn from(value: &[u32]) -> Self {
                use $name::*;
                // *__fake_module__::_assoc_table.iter().find(|(variant, number)| *number == value)
                //     .map(|(variant, _)| variant)
                //     .expect(concat!("Could not find variant entry inside association table for: ", stringify!($name)))
                match value[0] {
                    $($associated_number => $name::$variant_name(value[1..].into()),)*
                        failing => panic!("Could not match value {} for a variant of {}", failing, stringify!($name)),
                }
            }
        }
    };

    ($(#[$attribute:meta])* enum $name: ident {
        $($variant_name: ident = $associated_number: expr ,)*
    }) => {

        #[derive(Copy, Clone, Debug)] // So yeah, the enum type must implement Copy/Clone...
        $(#[$attribute])* pub enum $name {
            $($variant_name,)*
        }

        impl From<&[u32]> for $name {
            fn from(value: &[u32]) -> Self {
                use $name::*;
                // *__fake_module__::_assoc_table.iter().find(|(variant, number)| *number == value)
                //     .map(|(variant, _)| variant)
                //     .expect(concat!("Could not find variant entry inside association table for: ", stringify!($name)))
                match value[0] {
                    $($associated_number => $name::$variant_name,)*
                        failing => panic!("Could not match value {} for a variant of {}", failing, stringify!($name)),
                }
            }
        }
    };
}

// macro_rules! enum_c_like_assoc {
//         ($(#[$attribute:meta])* enum $name: ident {
//             $($variant_name: ident $(($content: ty))* = $associated_number: expr ,)*
//         } $label: ident) => {

//             _enum_c_like_assoc! {
//                 $(#[$attribute])* enum $name {
//                     $($variant_name $(($content))* = $associated_number ,)*
//                 } $label (1..)
//             }
//         };

//         ($(#[$attribute:meta])* enum $name: ident {
//             $($variant_name: ident = $associated_number: expr ,)*
//         } $label: ident) => {

//             _enum_c_like_assoc! {
//                 $(#[$attribute])* enum $name {
//                     $($variant_name = $associated_number ,)*
//                 } $label (1)
//             }
//         };
//     }

#[cfg(test)]
mod test {
    enum_c_like_assoc! {
        enum SubA {
            A = 0x0,
            B = 0x1,
            C = 0x2,
        }
    }

    enum_c_like_assoc! {
        enum SubB {
            A = 0x0,
            B = 0x1,
            C = 0x2,
        }
    }

    enum_c_like_assoc! {
        enum SubC {
            A = 0x0,
            B = 0x1,
            C = 0x2,
            D = 0x3,
            E = 0x4,
            F = 0x5,
            G = 0x6,
            H = 0x7,
            I = 0x8,

        }
    }

    enum_c_like_assoc! {
        enum test {
            A(SubA) = 0x0,
            B(SubB) = 0x1,
            C(SubC) = 0x2,
        }
    }

    #[test]
    fn test_c_like_assoc() {
        for index in 0..3 {
            for sub_index in 0..3 {
                use core::array::FixedSizeArray;
                let test: test = [index, sub_index].as_ref().into();

                println!("({}:{}) => {:?}", index, sub_index, test);
            }
        }
    }
}
