#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_capie: [u8; 0x02],
    _reserved_1_capifg: [u8; 0x02],
    _reserved_2_capiv: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - Captivate Interrupt Enable Register"]
    #[inline(always)]
    pub const fn capie(&self) -> &Capie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - Captivate Interrupt Flag Register"]
    #[inline(always)]
    pub const fn capifg(&self) -> &Capifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x04 - Captivate Interrupt Vector Register"]
    #[inline(always)]
    pub const fn capiv(&self) -> &Capiv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
}
#[doc = "CAPIE (rw) register accessor: Captivate Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`capie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capie`] module"]
#[doc(alias = "CAPIE")]
pub type Capie = crate::Reg<capie::CapieSpec>;
#[doc = "Captivate Interrupt Enable Register"]
pub mod capie;
#[doc = "CAPIFG (rw) register accessor: Captivate Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`capifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capifg`] module"]
#[doc(alias = "CAPIFG")]
pub type Capifg = crate::Reg<capifg::CapifgSpec>;
#[doc = "Captivate Interrupt Flag Register"]
pub mod capifg;
#[doc = "CAPIV (rw) register accessor: Captivate Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`capiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capiv`] module"]
#[doc(alias = "CAPIV")]
pub type Capiv = crate::Reg<capiv::CapivSpec>;
#[doc = "Captivate Interrupt Vector Register"]
pub mod capiv;
