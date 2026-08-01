#[doc = "Register `CAPIE` reader"]
pub type R = crate::R<CapieSpec>;
#[doc = "Register `CAPIE` writer"]
pub type W = crate::W<CapieSpec>;
#[doc = "End of conversion interrupt enable When enabled, an interrupt is called when EOCIFG = 1; that is, at the end of each conversion. EOCIFG must be cleared during the interrupt service routine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocien {
    #[doc = "0: Interrupt disabled"]
    Eocien0 = 0,
    #[doc = "1: Interrupt enabled"]
    Eocien1 = 1,
}
impl From<Eocien> for bool {
    #[inline(always)]
    fn from(variant: Eocien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIEN` reader - End of conversion interrupt enable When enabled, an interrupt is called when EOCIFG = 1; that is, at the end of each conversion. EOCIFG must be cleared during the interrupt service routine."]
pub type EocienR = crate::BitReader<Eocien>;
impl EocienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocien {
        match self.bits {
            false => Eocien::Eocien0,
            true => Eocien::Eocien1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_eocien_0(&self) -> bool {
        *self == Eocien::Eocien0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_eocien_1(&self) -> bool {
        *self == Eocien::Eocien1
    }
}
#[doc = "Field `EOCIEN` writer - End of conversion interrupt enable When enabled, an interrupt is called when EOCIFG = 1; that is, at the end of each conversion. EOCIFG must be cleared during the interrupt service routine."]
pub type EocienW<'a, REG> = crate::BitWriter<'a, REG, Eocien>;
impl<'a, REG> EocienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn eocien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Eocien::Eocien0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn eocien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Eocien::Eocien1)
    }
}
#[doc = "Captivate detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capdtctien {
    #[doc = "0: Interrupt disabled"]
    Capdtctien0 = 0,
    #[doc = "1: Interrupt enabled"]
    Capdtctien1 = 1,
}
impl From<Capdtctien> for bool {
    #[inline(always)]
    fn from(variant: Capdtctien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPDTCTIEN` reader - Captivate detection interrupt enable"]
pub type CapdtctienR = crate::BitReader<Capdtctien>;
impl CapdtctienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capdtctien {
        match self.bits {
            false => Capdtctien::Capdtctien0,
            true => Capdtctien::Capdtctien1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_capdtctien_0(&self) -> bool {
        *self == Capdtctien::Capdtctien0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_capdtctien_1(&self) -> bool {
        *self == Capdtctien::Capdtctien1
    }
}
#[doc = "Field `CAPDTCTIEN` writer - Captivate detection interrupt enable"]
pub type CapdtctienW<'a, REG> = crate::BitWriter<'a, REG, Capdtctien>;
impl<'a, REG> CapdtctienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn capdtctien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Capdtctien::Capdtctien0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn capdtctien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Capdtctien::Capdtctien1)
    }
}
#[doc = "Captivate Timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captien {
    #[doc = "0: Interrupt disabled"]
    Captien0 = 0,
    #[doc = "1: Interrupt enabled"]
    Captien1 = 1,
}
impl From<Captien> for bool {
    #[inline(always)]
    fn from(variant: Captien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTIEN` reader - Captivate Timer interrupt enable"]
pub type CaptienR = crate::BitReader<Captien>;
impl CaptienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captien {
        match self.bits {
            false => Captien::Captien0,
            true => Captien::Captien1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_captien_0(&self) -> bool {
        *self == Captien::Captien0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_captien_1(&self) -> bool {
        *self == Captien::Captien1
    }
}
#[doc = "Field `CAPTIEN` writer - Captivate Timer interrupt enable"]
pub type CaptienW<'a, REG> = crate::BitWriter<'a, REG, Captien>;
impl<'a, REG> CaptienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn captien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Captien::Captien0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn captien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Captien::Captien1)
    }
}
#[doc = "Captivate Conversion Counter interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capcntrien {
    #[doc = "0: Interrupt disabled"]
    Capcntrien0 = 0,
    #[doc = "1: Interrupt enabled"]
    Capcntrien1 = 1,
}
impl From<Capcntrien> for bool {
    #[inline(always)]
    fn from(variant: Capcntrien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPCNTRIEN` reader - Captivate Conversion Counter interrupt enable"]
pub type CapcntrienR = crate::BitReader<Capcntrien>;
impl CapcntrienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capcntrien {
        match self.bits {
            false => Capcntrien::Capcntrien0,
            true => Capcntrien::Capcntrien1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_capcntrien_0(&self) -> bool {
        *self == Capcntrien::Capcntrien0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_capcntrien_1(&self) -> bool {
        *self == Capcntrien::Capcntrien1
    }
}
#[doc = "Field `CAPCNTRIEN` writer - Captivate Conversion Counter interrupt enable"]
pub type CapcntrienW<'a, REG> = crate::BitWriter<'a, REG, Capcntrien>;
impl<'a, REG> CapcntrienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn capcntrien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Capcntrien::Capcntrien0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn capcntrien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Capcntrien::Capcntrien1)
    }
}
#[doc = "Captivate maximum count interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capmaxien {
    #[doc = "0: Interrupt disabled"]
    Capmaxien0 = 0,
    #[doc = "1: Interrupt enabled"]
    Capmaxien1 = 1,
}
impl From<Capmaxien> for bool {
    #[inline(always)]
    fn from(variant: Capmaxien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPMAXIEN` reader - Captivate maximum count interrupt enable"]
pub type CapmaxienR = crate::BitReader<Capmaxien>;
impl CapmaxienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Capmaxien {
        match self.bits {
            false => Capmaxien::Capmaxien0,
            true => Capmaxien::Capmaxien1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_capmaxien_0(&self) -> bool {
        *self == Capmaxien::Capmaxien0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_capmaxien_1(&self) -> bool {
        *self == Capmaxien::Capmaxien1
    }
}
#[doc = "Field `CAPMAXIEN` writer - Captivate maximum count interrupt enable"]
pub type CapmaxienW<'a, REG> = crate::BitWriter<'a, REG, Capmaxien>;
impl<'a, REG> CapmaxienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn capmaxien_0(self) -> &'a mut crate::W<REG> {
        self.variant(Capmaxien::Capmaxien0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn capmaxien_1(self) -> &'a mut crate::W<REG> {
        self.variant(Capmaxien::Capmaxien1)
    }
}
impl R {
    #[doc = "Bit 0 - End of conversion interrupt enable When enabled, an interrupt is called when EOCIFG = 1; that is, at the end of each conversion. EOCIFG must be cleared during the interrupt service routine."]
    #[inline(always)]
    pub fn eocien(&self) -> EocienR {
        EocienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Captivate detection interrupt enable"]
    #[inline(always)]
    pub fn capdtctien(&self) -> CapdtctienR {
        CapdtctienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Captivate Timer interrupt enable"]
    #[inline(always)]
    pub fn captien(&self) -> CaptienR {
        CaptienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Captivate Conversion Counter interrupt enable"]
    #[inline(always)]
    pub fn capcntrien(&self) -> CapcntrienR {
        CapcntrienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Captivate maximum count interrupt enable"]
    #[inline(always)]
    pub fn capmaxien(&self) -> CapmaxienR {
        CapmaxienR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of conversion interrupt enable When enabled, an interrupt is called when EOCIFG = 1; that is, at the end of each conversion. EOCIFG must be cleared during the interrupt service routine."]
    #[inline(always)]
    pub fn eocien(&mut self) -> EocienW<'_, CapieSpec> {
        EocienW::new(self, 0)
    }
    #[doc = "Bit 1 - Captivate detection interrupt enable"]
    #[inline(always)]
    pub fn capdtctien(&mut self) -> CapdtctienW<'_, CapieSpec> {
        CapdtctienW::new(self, 1)
    }
    #[doc = "Bit 2 - Captivate Timer interrupt enable"]
    #[inline(always)]
    pub fn captien(&mut self) -> CaptienW<'_, CapieSpec> {
        CaptienW::new(self, 2)
    }
    #[doc = "Bit 3 - Captivate Conversion Counter interrupt enable"]
    #[inline(always)]
    pub fn capcntrien(&mut self) -> CapcntrienW<'_, CapieSpec> {
        CapcntrienW::new(self, 3)
    }
    #[doc = "Bit 8 - Captivate maximum count interrupt enable"]
    #[inline(always)]
    pub fn capmaxien(&mut self) -> CapmaxienW<'_, CapieSpec> {
        CapmaxienW::new(self, 8)
    }
}
#[doc = "Captivate Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`capie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapieSpec;
impl crate::RegisterSpec for CapieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`capie::R`](R) reader structure"]
impl crate::Readable for CapieSpec {}
#[doc = "`write(|w| ..)` method takes [`capie::W`](W) writer structure"]
impl crate::Writable for CapieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAPIE to value 0"]
impl crate::Resettable for CapieSpec {}
