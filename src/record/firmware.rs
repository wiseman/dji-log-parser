use binrw::binread;

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct Firmware {
    pub sender_type: u8,
    pub sub_sender_type: u8,
    #[br(map = |x: [u8; 4]| format!("{}.{}.{}", x[0], x[1], x[2]))]
    pub version: String,
}
