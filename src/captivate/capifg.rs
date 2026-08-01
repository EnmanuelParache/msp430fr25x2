#[doc = "Register `CAPIFG` reader"]
pub type R = crate::R<CapifgSpec>;
#[doc = "Register `CAPIFG` writer"]
pub type W = crate::W<CapifgSpec>;
#[doc = "End of conversion interrupt flag This bit is set by hardware when each of the enabled CRx channels has finished converting and its results are ready. This bit is cleared by hardware when a conversion is launched (when CIPF becomes 1) or when CAPPWR = 0. If EOCITEN = 1, the Captivate interrupt occurs when EOCIFG transitions to 1. EOCIFG must be cleared by software before exiting the interrupt service routine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocifg {
    #[doc = "0: No end of conversion has occurred"]
    Eocifg0 = 0,
    #[doc = "1: End of conversion has occurred"]
    Eocifg1 = 1,
}
impl From<Eocifg> for bool {
    #[inline(always)]
    fn from(variant: Eocifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIFG` reader - End of conversion interrupt flag This bit is set by hardware when each of the enabled CRx channels has finished converting and its results are ready. This bit is cleared by hardware when a conversion is launched (when CIPF becomes 1) or when CAPPWR = 0. If EOCITEN = 1, the Captivate interrupt occurs when EOCIFG transitions to 1. EOCIFG must be cleared by software before exiting the interrupt service routine."]
pub type EocifgR = crate::BitReader<Eocifg>;
impl EocifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocifg {
        match self.bits {
            false => Eocifg::Eocifg0,
            true => Eocifg::Eocifg1,
        }
    }
    #[doc = "No end of conversion has occurred"]
    #[inline(always)]
    pub fn is_eocifg_0(&self) -> bool {
        *self == Eocifg::Eocifg0
    }
    #[doc = "End of conversion has occurred"]
    #[inline(always)]
    pub fn is_eocifg_1(&self) -> bool {
        *self == Eocifg::Eocifg1
    }
}
#[doc = "Field `EOCIFG` writer - End of conversion interrupt flag This bit is set by hardware when each of the enabled CRx channels has finished converting and its results are ready. This bit is cleared by hardware when a conversion is launched (when CIPF becomes 1) or when CAPPWR = 0. If EOCITEN = 1, the Captivate interrupt occurs when EOCIFG transitions to 1. EOCIFG must be cleared by software before exiting the interrupt service routine."]
pub type EocifgW<'a, REG> = crate::BitWriter<'a, REG, Eocifg>;
impl<'a, REG> EocifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No end of conversion has occurred"]
    #[inline(always)]
    pub fn eocifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eocifg::Eocifg0)
    }
    #[doc = "End of conversion has occurred"]
    #[inline(always)]
    pub fn eocifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eocifg::Eocifg1)
    }
}
#[doc = "Captivate detection interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capdtctifg {
    #[doc = "0: No interrupt pending"]
    Capdtctifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Capdtctifg1 = 1,
}
impl From<Capdtctifg> for bool {
    #[inline(always)]
    fn from(variant: Capdtctifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPDTCTIFG` reader - Captivate detection interrupt flag"]
pub type CapdtctifgR = crate::BitReader<Capdtctifg>;
impl CapdtctifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capdtctifg {
        match self.bits {
            false => Capdtctifg::Capdtctifg0,
            true => Capdtctifg::Capdtctifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_capdtctifg_0(&self) -> bool {
        *self == Capdtctifg::Capdtctifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_capdtctifg_1(&self) -> bool {
        *self == Capdtctifg::Capdtctifg1
    }
}
#[doc = "Field `CAPDTCTIFG` writer - Captivate detection interrupt flag"]
pub type CapdtctifgW<'a, REG> = crate::BitWriter<'a, REG, Capdtctifg>;
impl<'a, REG> CapdtctifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn capdtctifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Capdtctifg::Capdtctifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn capdtctifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Capdtctifg::Capdtctifg1)
    }
}
#[doc = "Captivate timer interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captifg {
    #[doc = "0: No interrupt pending"]
    Captifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Captifg1 = 1,
}
impl From<Captifg> for bool {
    #[inline(always)]
    fn from(variant: Captifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTIFG` reader - Captivate timer interrupt flag"]
pub type CaptifgR = crate::BitReader<Captifg>;
impl CaptifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captifg {
        match self.bits {
            false => Captifg::Captifg0,
            true => Captifg::Captifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_captifg_0(&self) -> bool {
        *self == Captifg::Captifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_captifg_1(&self) -> bool {
        *self == Captifg::Captifg1
    }
}
#[doc = "Field `CAPTIFG` writer - Captivate timer interrupt flag"]
pub type CaptifgW<'a, REG> = crate::BitWriter<'a, REG, Captifg>;
impl<'a, REG> CaptifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn captifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Captifg::Captifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn captifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Captifg::Captifg1)
    }
}
#[doc = "specified number of conversion have been reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capcntrifg {
    #[doc = "0: No interrupt pending"]
    Capcntrifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Capcntrifg1 = 1,
}
impl From<Capcntrifg> for bool {
    #[inline(always)]
    fn from(variant: Capcntrifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPCNTRIFG` reader - specified number of conversion have been reached"]
pub type CapcntrifgR = crate::BitReader<Capcntrifg>;
impl CapcntrifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capcntrifg {
        match self.bits {
            false => Capcntrifg::Capcntrifg0,
            true => Capcntrifg::Capcntrifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_capcntrifg_0(&self) -> bool {
        *self == Capcntrifg::Capcntrifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_capcntrifg_1(&self) -> bool {
        *self == Capcntrifg::Capcntrifg1
    }
}
#[doc = "Field `CAPCNTRIFG` writer - specified number of conversion have been reached"]
pub type CapcntrifgW<'a, REG> = crate::BitWriter<'a, REG, Capcntrifg>;
impl<'a, REG> CapcntrifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn capcntrifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Capcntrifg::Capcntrifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn capcntrifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Capcntrifg::Capcntrifg1)
    }
}
#[doc = "Captivate maximum count interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capmaxifg {
    #[doc = "0: Maximum count not reached"]
    Capmaxifg0 = 0,
    #[doc = "1: Maximum count reached"]
    Capmaxifg1 = 1,
}
impl From<Capmaxifg> for bool {
    #[inline(always)]
    fn from(variant: Capmaxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPMAXIFG` reader - Captivate maximum count interrupt flag"]
pub type CapmaxifgR = crate::BitReader<Capmaxifg>;
impl CapmaxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capmaxifg {
        match self.bits {
            false => Capmaxifg::Capmaxifg0,
            true => Capmaxifg::Capmaxifg1,
        }
    }
    #[doc = "Maximum count not reached"]
    #[inline(always)]
    pub fn is_capmaxifg_0(&self) -> bool {
        *self == Capmaxifg::Capmaxifg0
    }
    #[doc = "Maximum count reached"]
    #[inline(always)]
    pub fn is_capmaxifg_1(&self) -> bool {
        *self == Capmaxifg::Capmaxifg1
    }
}
#[doc = "Field `CAPMAXIFG` writer - Captivate maximum count interrupt flag"]
pub type CapmaxifgW<'a, REG> = crate::BitWriter<'a, REG, Capmaxifg>;
impl<'a, REG> CapmaxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Maximum count not reached"]
    #[inline(always)]
    pub fn capmaxifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Capmaxifg::Capmaxifg0)
    }
    #[doc = "Maximum count reached"]
    #[inline(always)]
    pub fn capmaxifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Capmaxifg::Capmaxifg1)
    }
}
impl R {
    #[doc = "Bit 0 - End of conversion interrupt flag This bit is set by hardware when each of the enabled CRx channels has finished converting and its results are ready. This bit is cleared by hardware when a conversion is launched (when CIPF becomes 1) or when CAPPWR = 0. If EOCITEN = 1, the Captivate interrupt occurs when EOCIFG transitions to 1. EOCIFG must be cleared by software before exiting the interrupt service routine."]
    #[inline(always)]
    pub fn eocifg(&self) -> EocifgR {
        EocifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Captivate detection interrupt flag"]
    #[inline(always)]
    pub fn capdtctifg(&self) -> CapdtctifgR {
        CapdtctifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Captivate timer interrupt flag"]
    #[inline(always)]
    pub fn captifg(&self) -> CaptifgR {
        CaptifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - specified number of conversion have been reached"]
    #[inline(always)]
    pub fn capcntrifg(&self) -> CapcntrifgR {
        CapcntrifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Captivate maximum count interrupt flag"]
    #[inline(always)]
    pub fn capmaxifg(&self) -> CapmaxifgR {
        CapmaxifgR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of conversion interrupt flag This bit is set by hardware when each of the enabled CRx channels has finished converting and its results are ready. This bit is cleared by hardware when a conversion is launched (when CIPF becomes 1) or when CAPPWR = 0. If EOCITEN = 1, the Captivate interrupt occurs when EOCIFG transitions to 1. EOCIFG must be cleared by software before exiting the interrupt service routine."]
    #[inline(always)]
    pub fn eocifg(&mut self) -> EocifgW<'_, CapifgSpec> {
        EocifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Captivate detection interrupt flag"]
    #[inline(always)]
    pub fn capdtctifg(&mut self) -> CapdtctifgW<'_, CapifgSpec> {
        CapdtctifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Captivate timer interrupt flag"]
    #[inline(always)]
    pub fn captifg(&mut self) -> CaptifgW<'_, CapifgSpec> {
        CaptifgW::new(self, 2)
    }
    #[doc = "Bit 3 - specified number of conversion have been reached"]
    #[inline(always)]
    pub fn capcntrifg(&mut self) -> CapcntrifgW<'_, CapifgSpec> {
        CapcntrifgW::new(self, 3)
    }
    #[doc = "Bit 8 - Captivate maximum count interrupt flag"]
    #[inline(always)]
    pub fn capmaxifg(&mut self) -> CapmaxifgW<'_, CapifgSpec> {
        CapmaxifgW::new(self, 8)
    }
}
#[doc = "Captivate Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`capifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapifgSpec;
impl crate::RegisterSpec for CapifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`capifg::R`](R) reader structure"]
impl crate::Readable for CapifgSpec {}
#[doc = "`write(|w| ..)` method takes [`capifg::W`](W) writer structure"]
impl crate::Writable for CapifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAPIFG to value 0"]
impl crate::Resettable for CapifgSpec {}
