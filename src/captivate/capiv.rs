#[doc = "Register `CAPIV` reader"]
pub type R = crate::R<CapivSpec>;
#[doc = "Register `CAPIV` writer"]
pub type W = crate::W<CapivSpec>;
#[doc = "Captivate Interrupt vector value. It generates an value that can be used as address offset for fast interrupt service routine handling. 000Ch to FFFEh = Reserved Read will clear highest priority interrupt. Write will clear all pending interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Capiv {
    #[doc = "0: No interrupt pending"]
    Capiv0 = 0,
    #[doc = "2: Interrupt source: End of conversion interrupt, Flag = EOCIFG"]
    Capiv2 = 2,
    #[doc = "4: Interrupt source: Detection interrupt, Flag = CAPDTCTIFG"]
    Capiv4 = 4,
    #[doc = "6: Interrupt source: Captivate Timer interrupt, Flag = CAPTIFG"]
    Capiv6 = 6,
    #[doc = "8: Interrupt source: Captivate Counter interrupt, Flag = CAPCNTRIFG"]
    Capiv8 = 8,
    #[doc = "10: Interrupt source: max count value reached, Flag = CAPMAXIFG"]
    Capiv10 = 10,
}
impl From<Capiv> for u16 {
    #[inline(always)]
    fn from(variant: Capiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Capiv {
    type Ux = u16;
}
impl crate::IsEnum for Capiv {}
#[doc = "Field `CAPIV` reader - Captivate Interrupt vector value. It generates an value that can be used as address offset for fast interrupt service routine handling. 000Ch to FFFEh = Reserved Read will clear highest priority interrupt. Write will clear all pending interrupts."]
pub type CapivR = crate::FieldReader<Capiv>;
impl CapivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Capiv> {
        match self.bits {
            0 => Some(Capiv::Capiv0),
            2 => Some(Capiv::Capiv2),
            4 => Some(Capiv::Capiv4),
            6 => Some(Capiv::Capiv6),
            8 => Some(Capiv::Capiv8),
            10 => Some(Capiv::Capiv10),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_capiv_0(&self) -> bool {
        *self == Capiv::Capiv0
    }
    #[doc = "Interrupt source: End of conversion interrupt, Flag = EOCIFG"]
    #[inline(always)]
    pub fn is_capiv_2(&self) -> bool {
        *self == Capiv::Capiv2
    }
    #[doc = "Interrupt source: Detection interrupt, Flag = CAPDTCTIFG"]
    #[inline(always)]
    pub fn is_capiv_4(&self) -> bool {
        *self == Capiv::Capiv4
    }
    #[doc = "Interrupt source: Captivate Timer interrupt, Flag = CAPTIFG"]
    #[inline(always)]
    pub fn is_capiv_6(&self) -> bool {
        *self == Capiv::Capiv6
    }
    #[doc = "Interrupt source: Captivate Counter interrupt, Flag = CAPCNTRIFG"]
    #[inline(always)]
    pub fn is_capiv_8(&self) -> bool {
        *self == Capiv::Capiv8
    }
    #[doc = "Interrupt source: max count value reached, Flag = CAPMAXIFG"]
    #[inline(always)]
    pub fn is_capiv_10(&self) -> bool {
        *self == Capiv::Capiv10
    }
}
impl R {
    #[doc = "Bits 0:15 - Captivate Interrupt vector value. It generates an value that can be used as address offset for fast interrupt service routine handling. 000Ch to FFFEh = Reserved Read will clear highest priority interrupt. Write will clear all pending interrupts."]
    #[inline(always)]
    pub fn capiv(&self) -> CapivR {
        CapivR::new(self.bits)
    }
}
impl W {}
#[doc = "Captivate Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`capiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapivSpec;
impl crate::RegisterSpec for CapivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`capiv::R`](R) reader structure"]
impl crate::Readable for CapivSpec {}
#[doc = "`write(|w| ..)` method takes [`capiv::W`](W) writer structure"]
impl crate::Writable for CapivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAPIV to value 0"]
impl crate::Resettable for CapivSpec {}
