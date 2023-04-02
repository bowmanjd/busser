pub const DATE_FORMATS: [&str; 14] = [
    "%Y-%m-%d",
    "%Y%m%d",
    "%m/%d/%Y",
    "%m-%d-%Y",
    "%Y.%m.%d",
    "%Y/%m/%d",
    "%m.%d.%Y",
    "%B %d %Y",
    "%B %d, %Y",
    "%d %B %Y",
    "%d %B, %Y",
    "%d %Y %B",
    "%Y %B %d",
    "%Y %d %B",
];

pub const TIME_FORMATS: [&str; 6] = ["%T%.f", "%I:%M:%S%.f %p", "%T", "%H:%M", "%r", "%I:%M %p"];

pub const DATETIME_FORMATS: [&str; 85] = [
    "%Y-%m-%dT%H:%M:%S%.f",
    "%Y-%m-%d %T%.f",
    "%Y-%m-%d %I:%M:%S%.f %p",
    "%Y-%m-%d %T",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%d %r",
    "%Y-%m-%d %I:%M %p",
    "%Y%m%d %T%.f",
    "%Y%m%d %I:%M:%S%.f %p",
    "%Y%m%d %T",
    "%Y%m%d %H:%M",
    "%Y%m%d %r",
    "%Y%m%d %I:%M %p",
    "%m/%d/%Y %T%.f",
    "%m/%d/%Y %I:%M:%S%.f %p",
    "%m/%d/%Y %T",
    "%m/%d/%Y %H:%M",
    "%m/%d/%Y %r",
    "%m/%d/%Y %I:%M %p",
    "%m-%d-%Y %T%.f",
    "%m-%d-%Y %I:%M:%S%.f %p",
    "%m-%d-%Y %T",
    "%m-%d-%Y %H:%M",
    "%m-%d-%Y %r",
    "%m-%d-%Y %I:%M %p",
    "%Y.%m.%d %T%.f",
    "%Y.%m.%d %I:%M:%S%.f %p",
    "%Y.%m.%d %T",
    "%Y.%m.%d %H:%M",
    "%Y.%m.%d %r",
    "%Y.%m.%d %I:%M %p",
    "%Y/%m/%d %T%.f",
    "%Y/%m/%d %I:%M:%S%.f %p",
    "%Y/%m/%d %T",
    "%Y/%m/%d %H:%M",
    "%Y/%m/%d %r",
    "%Y/%m/%d %I:%M %p",
    "%m.%d.%Y %T%.f",
    "%m.%d.%Y %I:%M:%S%.f %p",
    "%m.%d.%Y %T",
    "%m.%d.%Y %H:%M",
    "%m.%d.%Y %r",
    "%m.%d.%Y %I:%M %p",
    "%B %d %Y %T%.f",
    "%B %d %Y %I:%M:%S%.f %p",
    "%B %d %Y %T",
    "%B %d %Y %H:%M",
    "%B %d %Y %r",
    "%B %d %Y %I:%M %p",
    "%B %d, %Y %T%.f",
    "%B %d, %Y %I:%M:%S%.f %p",
    "%B %d, %Y %T",
    "%B %d, %Y %H:%M",
    "%B %d, %Y %r",
    "%B %d, %Y %I:%M %p",
    "%d %B %Y %T%.f",
    "%d %B %Y %I:%M:%S%.f %p",
    "%d %B %Y %T",
    "%d %B %Y %H:%M",
    "%d %B %Y %r",
    "%d %B %Y %I:%M %p",
    "%d %B, %Y %T%.f",
    "%d %B, %Y %I:%M:%S%.f %p",
    "%d %B, %Y %T",
    "%d %B, %Y %H:%M",
    "%d %B, %Y %r",
    "%d %B, %Y %I:%M %p",
    "%d %Y %B %T%.f",
    "%d %Y %B %I:%M:%S%.f %p",
    "%d %Y %B %T",
    "%d %Y %B %H:%M",
    "%d %Y %B %r",
    "%d %Y %B %I:%M %p",
    "%Y %B %d %T%.f",
    "%Y %B %d %I:%M:%S%.f %p",
    "%Y %B %d %T",
    "%Y %B %d %H:%M",
    "%Y %B %d %r",
    "%Y %B %d %I:%M %p",
    "%Y %d %B %T%.f",
    "%Y %d %B %I:%M:%S%.f %p",
    "%Y %d %B %T",
    "%Y %d %B %H:%M",
    "%Y %d %B %r",
    "%Y %d %B %I:%M %p",
];

pub const DATETIMEOFFSET_FORMATS: [&str; 86] = [
    "%+",
    "%Y-%m-%dT%H:%M:%S%.f %:z",
    "%Y-%m-%d %T%.f %:z",
    "%Y-%m-%d %I:%M:%S%.f %p %:z",
    "%Y-%m-%d %T %:z",
    "%Y-%m-%d %H:%M %:z",
    "%Y-%m-%d %r %:z",
    "%Y-%m-%d %I:%M %p %:z",
    "%Y%m%d %T%.f %:z",
    "%Y%m%d %I:%M:%S%.f %p %:z",
    "%Y%m%d %T %:z",
    "%Y%m%d %H:%M %:z",
    "%Y%m%d %r %:z",
    "%Y%m%d %I:%M %p %:z",
    "%m/%d/%Y %T%.f %:z",
    "%m/%d/%Y %I:%M:%S%.f %p %:z",
    "%m/%d/%Y %T %:z",
    "%m/%d/%Y %H:%M %:z",
    "%m/%d/%Y %r %:z",
    "%m/%d/%Y %I:%M %p %:z",
    "%m-%d-%Y %T%.f %:z",
    "%m-%d-%Y %I:%M:%S%.f %p %:z",
    "%m-%d-%Y %T %:z",
    "%m-%d-%Y %H:%M %:z",
    "%m-%d-%Y %r %:z",
    "%m-%d-%Y %I:%M %p %:z",
    "%Y.%m.%d %T%.f %:z",
    "%Y.%m.%d %I:%M:%S%.f %p %:z",
    "%Y.%m.%d %T %:z",
    "%Y.%m.%d %H:%M %:z",
    "%Y.%m.%d %r %:z",
    "%Y.%m.%d %I:%M %p %:z",
    "%Y/%m/%d %T%.f %:z",
    "%Y/%m/%d %I:%M:%S%.f %p %:z",
    "%Y/%m/%d %T %:z",
    "%Y/%m/%d %H:%M %:z",
    "%Y/%m/%d %r %:z",
    "%Y/%m/%d %I:%M %p %:z",
    "%m.%d.%Y %T%.f %:z",
    "%m.%d.%Y %I:%M:%S%.f %p %:z",
    "%m.%d.%Y %T %:z",
    "%m.%d.%Y %H:%M %:z",
    "%m.%d.%Y %r %:z",
    "%m.%d.%Y %I:%M %p %:z",
    "%B %d %Y %T%.f %:z",
    "%B %d %Y %I:%M:%S%.f %p %:z",
    "%B %d %Y %T %:z",
    "%B %d %Y %H:%M %:z",
    "%B %d %Y %r %:z",
    "%B %d %Y %I:%M %p %:z",
    "%B %d, %Y %T%.f %:z",
    "%B %d, %Y %I:%M:%S%.f %p %:z",
    "%B %d, %Y %T %:z",
    "%B %d, %Y %H:%M %:z",
    "%B %d, %Y %r %:z",
    "%B %d, %Y %I:%M %p %:z",
    "%d %B %Y %T%.f %:z",
    "%d %B %Y %I:%M:%S%.f %p %:z",
    "%d %B %Y %T %:z",
    "%d %B %Y %H:%M %:z",
    "%d %B %Y %r %:z",
    "%d %B %Y %I:%M %p %:z",
    "%d %B, %Y %T%.f %:z",
    "%d %B, %Y %I:%M:%S%.f %p %:z",
    "%d %B, %Y %T %:z",
    "%d %B, %Y %H:%M %:z",
    "%d %B, %Y %r %:z",
    "%d %B, %Y %I:%M %p %:z",
    "%d %Y %B %T%.f %:z",
    "%d %Y %B %I:%M:%S%.f %p %:z",
    "%d %Y %B %T %:z",
    "%d %Y %B %H:%M %:z",
    "%d %Y %B %r %:z",
    "%d %Y %B %I:%M %p %:z",
    "%Y %B %d %T%.f %:z",
    "%Y %B %d %I:%M:%S%.f %p %:z",
    "%Y %B %d %T %:z",
    "%Y %B %d %H:%M %:z",
    "%Y %B %d %r %:z",
    "%Y %B %d %I:%M %p %:z",
    "%Y %d %B %T%.f %:z",
    "%Y %d %B %I:%M:%S%.f %p %:z",
    "%Y %d %B %T %:z",
    "%Y %d %B %H:%M %:z",
    "%Y %d %B %r %:z",
    "%Y %d %B %I:%M %p %:z",
];
