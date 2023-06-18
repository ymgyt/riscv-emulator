macro_rules! add_imm_signed {
    ($r:expr, $imm:expr) => {
        (($r as i64) + ($imm as i64)) as u32
    };
}

pub(crate) use add_imm_signed;
