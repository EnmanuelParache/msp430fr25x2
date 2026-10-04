#[doc = "Register `SYSCFG3` reader"]
pub type R = crate::R<Syscfg3Spec>;
#[doc = "Register `SYSCFG3` writer"]
pub type W = crate::W<Syscfg3Spec>;
#[doc = "eUSCIA remapping source selection, please refer to device specific for details\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscia0rmp {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    Uscia0rmp0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    Uscia0rmp1 = 1,
}
impl From<Uscia0rmp> for bool {
    #[inline(always)]
    fn from(variant: Uscia0rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USCIA0RMP` reader - eUSCIA remapping source selection, please refer to device specific for details"]
pub type Uscia0rmpR = crate::BitReader<Uscia0rmp>;
impl Uscia0rmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uscia0rmp {
        match self.bits {
            false => Uscia0rmp::Uscia0rmp0,
            true => Uscia0rmp::Uscia0rmp1,
        }
    }
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscia0rmp_0(&self) -> bool {
        *self == Uscia0rmp::Uscia0rmp0
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscia0rmp_1(&self) -> bool {
        *self == Uscia0rmp::Uscia0rmp1
    }
}
#[doc = "Field `USCIA0RMP` writer - eUSCIA remapping source selection, please refer to device specific for details"]
pub type Uscia0rmpW<'a, REG> = crate::BitWriter<'a, REG, Uscia0rmp>;
impl<'a, REG> Uscia0rmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscia0rmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscia0rmp::Uscia0rmp0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscia0rmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscia0rmp::Uscia0rmp1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscia0rmp(&self) -> Uscia0rmpR {
        Uscia0rmpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn uscia0rmp(&mut self) -> Uscia0rmpW<'_, Syscfg3Spec> {
        Uscia0rmpW::new(self, 0)
    }
}
#[doc = "System Configuration Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg3Spec;
impl crate::RegisterSpec for Syscfg3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg3::R`](R) reader structure"]
impl crate::Readable for Syscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg3::W`](W) writer structure"]
impl crate::Writable for Syscfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG3 to value 0"]
impl crate::Resettable for Syscfg3Spec {}
