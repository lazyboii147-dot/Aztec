/// Aztec Calendar module containing Xiuhpohualli and Tonalpohualli definitions.

pub enum CalendarSystem {
    Xiuhpohualli, // Solar calendar (365 days)
    Tonalpohualli, // Sacred calendar (260 days)
}

pub struct AztecCalendar {
    pub system: CalendarSystem,
    pub day_count: u32,
}
