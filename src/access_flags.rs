
// class access flags
pub const CLASS_ACC_FLAGS : [(u16, &'static str); 8] = [(0x0001,     "ACC_PUBLIC"),
                                                        (0x0010,      "ACC_FINAL"),
                                                        (0x0020,      "ACC_SUPER"),
                                                        (0x0200,  "ACC_INTERFACE"),
                                                        (0x0400,   "ACC_ABSTRACT"),
                                                        (0x1000,  "ACC_SYNTHETIC"),
                                                        (0x2000, "ACC_ANNOTATION"),
                                                        (0x4000,       "ACC_ENUM")];

// fields access flags
pub const FIELD_ACC_FLAGS : [(u16, &'static str); 9] = [(0x0001,     "ACC_PUBLIC"),
                                                        (0x0002,    "ACC_PRIVATE"),
                                                        (0x0004,  "ACC_PROTECTED"),
                                                        (0x0008,     "ACC_STATIC"),
                                                        (0x0010,      "ACC_FINAL"),
                                                        (0x0040,   "ACC_VOLATILE"),
                                                        (0x0080,  "ACC_TRANSIENT"),
                                                        (0x1000,  "ACC_SYNTHETIC"),
                                                        (0x4000,       "ACC_ENUM")];

pub fn read_access_flags(mask: u16, flags_list: &[(u16, &'static str)]) -> Vec<&'static str> {
    let mut flags = Vec::new();
    for &(m, f) in flags_list {
        if (mask & m) == m {
            flags.push(f);
        }
    }
    return flags;
}
