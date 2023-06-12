use time::format_description::FormatItem;
use time::macros::format_description;

pub const DATE_FORMATS: [&[FormatItem]; 16] = [
    format_description!(version = 2, "[year][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year]"),
    format_description!(version = 2, "[year][month padding:zero][day padding:zero]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year]"),
    format_description!(version = 2, "[day padding:none] [year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]]"),
    format_description!(version = 2, "[year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none]"),
    format_description!(version = 2, "[year] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]]"),
    format_description!(version = 2, "[year repr:last_two][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none]"),
    format_description!(version = 2, "[year repr:last_two][month padding:zero][day padding:zero]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year repr:last_two]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year repr:last_two]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year repr:last_two]"),
    format_description!(version = 2, "[day padding:none] [year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]]"),
    format_description!(version = 2, "[year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none]"),
    format_description!(version = 2, "[year repr:last_two] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]]"),
];

pub const TIME_FORMATS: [&[FormatItem]; 2] = [
    format_description!(version = 2, "[hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]]"),
    format_description!(version = 2, "[hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false]"),
];

pub const DATETIME_FORMATS: [&[FormatItem]; 33] = [
    format_description!(version = 2, "[year]-[month padding:none]-[day padding:none]T[hour repr:24]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year][month padding:zero][day padding:zero] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two][month padding:zero][day padding:zero] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year repr:last_two] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year repr:last_two] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year repr:last_two] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [Z]]"),
    format_description!(version = 2, "[year][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[year][month padding:zero][day padding:zero] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[year] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two][month padding:zero][day padding:zero] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year repr:last_two] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year repr:last_two] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year repr:last_two] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[day padding:none] [year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
    format_description!(version = 2, "[year repr:last_two] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [Z]]"),
];

pub const DATETIMEOFFSET_FORMATS: [&[FormatItem]; 33] = [
    format_description!(version = 2, "[year]-[month padding:none]-[day padding:none]T[hour repr:24]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year][month padding:zero][day padding:zero] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two][month padding:zero][day padding:zero] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year repr:last_two] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year repr:last_two] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year repr:last_two] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:24 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year][month padding:zero][day padding:zero] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two][first [-] [/] [.]][month padding:none][first [-] [/] [.]][day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two][month padding:zero][day padding:zero] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[month padding:none][first [-] [/] [.]][day padding:none][first [-] [/] [.]][year repr:last_two] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none][optional [,]] [year repr:last_two] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]][optional [,]] [year repr:last_two] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[day padding:none] [year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [day padding:none] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
    format_description!(version = 2, "[year repr:last_two] [day padding:none] [first [month case_sensitive:false repr:long] [month case_sensitive:false repr:short]] [hour repr:12 padding:none]:[minute][optional [:[second]]][optional [.[subsecond]]][optional [ ]][period case_sensitive:false][optional [ ]][offset_hour]:[offset_minute]"),
];
