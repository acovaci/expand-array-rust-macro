use super::flags::BitifyFlags;

pub const fn bitify_func<const SIZE: usize>(bytes: &[u8]) -> [u8; SIZE] {
    let mut new_bytes = [0u8; SIZE];
    let mut i = 0;
    let mut offset = 0;
    let size = bytes.len();

    let mut flags = BitifyFlags::new();

    while i < size && i - offset < SIZE {
        if flags.is_on(BitifyFlags::ESCAPING) {
            flags = flags.toggle(BitifyFlags::ESCAPING);

            new_bytes[i - offset] = bytes[i];
            i += 1;
            continue;
        }

        if bytes[i] == b'\\' {
            flags = flags.toggle(BitifyFlags::ESCAPING);

            offset += 1;
            i += 1;
            continue;
        }

        if bytes[i] == b'c' || bytes[i] == b'b' {
            if flags.is_off(BitifyFlags::START) && i != 0 {
                panic!("Invalid string literal")
            }

            if flags.is_on(BitifyFlags::START) {
                new_bytes[i - offset] = bytes[i];
                i += 1;
                continue;
            }

            offset += 1;
            i += 1;
            continue;
        }

        if bytes[i] == b'"' {
            if flags.is_on(BitifyFlags::END) {
                panic!("Invalid string literal")
            }

            if flags.is_on(BitifyFlags::START) {
                flags = flags.set_on(BitifyFlags::END);
            }

            flags = flags.set_on(BitifyFlags::START);

            offset += 1;
            i += 1;
            continue;
        }

        new_bytes[i - offset] = bytes[i];
        i += 1;
    }

    new_bytes
}
