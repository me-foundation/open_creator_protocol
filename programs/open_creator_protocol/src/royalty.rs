use crate::errors::OCPErrorCode;
use anchor_lang::prelude::Result;
use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use serde::{Deserialize, Serialize};

pub const DYNAMIC_ROYALTY_KIND_PRICE_LINEAR: u8 = 0;

#[derive(Default, Clone, Serialize, Deserialize, AnchorSerialize, AnchorDeserialize)]
pub struct DynamicRoyaltyPriceLinear {
    // size: 33 + 8 + 8 + 2 + 2 = 53
    pub price_mint: Option<Pubkey>,
    pub start_price: u64,
    pub end_price: u64,
    pub start_multiplier_bp: u16,
    pub end_multiplier_bp: u16,
}

impl DynamicRoyaltyPriceLinear {
    pub fn valid(&self) -> Result<()> {
        if self.start_price > self.end_price {
            msg!("start_price must be less than or equal to end_price");
            return Err(OCPErrorCode::InvalidDynamicRoyalty.into());
        }
        Ok(())
    }

    pub fn get_royalty_bp(&self, price: u64, royalty_bp: u16) -> Result<u16> {
        if price <= self.start_price {
            return Ok(DynamicRoyalty::safe_mul_bp(self.start_multiplier_bp, royalty_bp));
        }
        if price >= self.end_price {
            return Ok(DynamicRoyalty::safe_mul_bp(self.end_multiplier_bp, royalty_bp));
        }

        // (p - x1) / (x2 - x1) = (multiplier_bp - y1) / (y2 - y1)
        // thus, multiplier_bp = y1 + (y2 - y1) * (p - x1) / (x2 - x1)
        //       multiplier_bp = y1 + y * d / x

        let x1 = self.start_price as i128;
        let x2 = self.end_price as i128;
        let y1 = self.start_multiplier_bp as i128;
        let y2 = self.end_multiplier_bp as i128;
        let p = price as i128;

        let y = y2.checked_sub(y1).ok_or(OCPErrorCode::NumericalOverflow)?;
        let d = p.checked_sub(x1).ok_or(OCPErrorCode::NumericalOverflow)?;
        let x = x2.checked_sub(x1).ok_or(OCPErrorCode::NumericalOverflow)?;

        let multiplier_bp = y1
            .checked_add(
                y.checked_mul(d)
                    .ok_or(OCPErrorCode::NumericalOverflow)?
                    .checked_div(x)
                    .ok_or(OCPErrorCode::NumericalOverflow)?,
            )
            .ok_or(OCPErrorCode::NumericalOverflow)?;

        Ok(DynamicRoyalty::safe_mul_bp(
            match multiplier_bp {
                x if x < 0 => 0,
                x if x > u16::MAX as i128 => u16::MAX,
                x => x as u16,
            },
            royalty_bp,
        ))
    }
}

#[derive(Default, Clone, Serialize, Deserialize, AnchorSerialize, AnchorDeserialize)]
pub struct DynamicRoyalty {
    // size: 1 + 1 + 3 + 53 + 32 * 4 = 186
    pub version: u8,
    pub kind: u8,
    pub override_royalty_bp: Option<u16>, // if not set, we should use the one from metadata
    pub kind_price_linear: Option<DynamicRoyaltyPriceLinear>,
    pub _reserved_0: [u8; 32],
    pub _reserved_1: [u8; 32],
    pub _reserved_2: [u8; 32],
    pub _reserved_3: [u8; 32],
}

impl DynamicRoyalty {
    #[inline(always)]
    fn safe_mul_bp(multiplier_bp: u16, bp: u16) -> u16 {
        let ret = (multiplier_bp as u32 * bp as u32 / 10000) as u16;
        if ret > 10000 {
            10000
        } else {
            ret
        }
    }

    pub fn valid(&self) -> Result<()> {
        if self.override_royalty_bp.is_some() && self.override_royalty_bp.unwrap() > 10000 {
            msg!("base_royalty_bp must be less than or equal to 10000");
            return Err(OCPErrorCode::InvalidDynamicRoyalty.into());
        }
        match self.kind {
            DYNAMIC_ROYALTY_KIND_PRICE_LINEAR => {
                if self.kind_price_linear.is_none() {
                    msg!("kind_price_linear must be set for DYNAMIC_ROYALTY_KIND_PRICE_LINEAR");
                    return Err(OCPErrorCode::InvalidDynamicRoyalty.into());
                }
                self.kind_price_linear.as_ref().unwrap().valid()?;
            }
            _ => {
                msg!("Invalid DynamicRoyalty kind");
                return Err(OCPErrorCode::InvalidDynamicRoyalty.into());
            }
        }
        Ok(())
    }

    pub fn get_royalty_bp(&self, price: u64, metadata_royalty_bp: u16) -> u16 {
        let royalty_bp = self.override_royalty_bp.unwrap_or(metadata_royalty_bp);

        match self.kind {
            DYNAMIC_ROYALTY_KIND_PRICE_LINEAR => self
                .kind_price_linear
                .as_ref()
                .expect("kind_price_linear should not be empty")
                .get_royalty_bp(price, royalty_bp)
                .unwrap_or(royalty_bp),
            _ => royalty_bp,
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_program::native_token::LAMPORTS_PER_SOL;

    use super::*;

    #[test]
    fn test_price_linear_function() {
        // happy code path
        {
            let price_linear = DynamicRoyaltyPriceLinear {
                price_mint: None,
                start_price: 100,
                end_price: 1000,
                start_multiplier_bp: 10000, // start from 100%
                end_multiplier_bp: 100,     // end at 1%
            };
            let metadat_roaylty_bp = 1000;
            assert_eq!(price_linear.get_royalty_bp(0, metadat_roaylty_bp).unwrap(), 1000);
            assert_eq!(price_linear.get_royalty_bp(100, metadat_roaylty_bp).unwrap(), 1000);
            assert_eq!(price_linear.get_royalty_bp(1000, metadat_roaylty_bp).unwrap(), 10);
            assert_eq!(price_linear.get_royalty_bp(10000, metadat_roaylty_bp).unwrap(), 10);
            assert_eq!(price_linear.get_royalty_bp(500, metadat_roaylty_bp).unwrap(), 560);
        }

        // happy code path - 2
        {
            let price_linear = DynamicRoyaltyPriceLinear {
                price_mint: None,
                start_price: LAMPORTS_PER_SOL,
                end_price: 3 * LAMPORTS_PER_SOL,
                start_multiplier_bp: 10000, // start from 100%
                end_multiplier_bp: 5000,    // end at 50%
            };
            let metadat_roaylty_bp = 500;
            assert_eq!(price_linear.get_royalty_bp(0, metadat_roaylty_bp).unwrap(), 500);
            assert_eq!(price_linear.get_royalty_bp(1 * LAMPORTS_PER_SOL, metadat_roaylty_bp).unwrap(), 500);
            assert_eq!(price_linear.get_royalty_bp(2 * LAMPORTS_PER_SOL, metadat_roaylty_bp).unwrap(), 375);
            assert_eq!(price_linear.get_royalty_bp(3 * LAMPORTS_PER_SOL, metadat_roaylty_bp).unwrap(), 250);
            assert_eq!(price_linear.get_royalty_bp(100 * LAMPORTS_PER_SOL, metadat_roaylty_bp).unwrap(), 250);
        }

        // cannot exceed the 10000 from get_royalty_bp
        {
            let price_linear = DynamicRoyaltyPriceLinear {
                price_mint: None,
                start_price: 0,
                end_price: 100,
                start_multiplier_bp: 20000, // start from 200%
                end_multiplier_bp: 10000,   // end at 100%
            };
            let metadat_roaylty_bp = 7500;
            assert_eq!(price_linear.get_royalty_bp(0, metadat_roaylty_bp).unwrap(), 10000);
            assert_eq!(price_linear.get_royalty_bp(50, metadat_roaylty_bp).unwrap(), 10000);
            assert_eq!(price_linear.get_royalty_bp(90, metadat_roaylty_bp).unwrap(), 8250);
            assert_eq!(price_linear.get_royalty_bp(100, metadat_roaylty_bp).unwrap(), 7500);
            assert_eq!(price_linear.get_royalty_bp(200, metadat_roaylty_bp).unwrap(), 7500);
        }
    }

    #[test]
    fn test_dynamic_royalty() {
        // price linear desc
        {
            let price_linear = DynamicRoyaltyPriceLinear {
                price_mint: None,
                start_price: 100,
                end_price: 1000,
                start_multiplier_bp: 10000, // start from 100%
                end_multiplier_bp: 100,     // end at 1%
            };
            let dynamic_royalty = DynamicRoyalty {
                version: 1,
                kind: DYNAMIC_ROYALTY_KIND_PRICE_LINEAR,
                override_royalty_bp: None,
                kind_price_linear: Some(price_linear),
                _reserved_0: [0; 32],
                _reserved_1: [0; 32],
                _reserved_2: [0; 32],
                _reserved_3: [0; 32],
            };
            let metadat_roaylty_bp = 1000;

            assert_eq!(dynamic_royalty.get_royalty_bp(0, metadat_roaylty_bp), 1000);
            assert_eq!(dynamic_royalty.get_royalty_bp(100, metadat_roaylty_bp), 1000);
            assert_eq!(dynamic_royalty.get_royalty_bp(1000, metadat_roaylty_bp), 10);
            assert_eq!(dynamic_royalty.get_royalty_bp(10000, metadat_roaylty_bp), 10);
            assert_eq!(dynamic_royalty.get_royalty_bp(500, metadat_roaylty_bp), 560);
        }

        // price linear asc
        {
            let price_linear = DynamicRoyaltyPriceLinear {
                price_mint: None,
                start_price: 100,
                end_price: 1000,
                start_multiplier_bp: 5000, // start from 50%
                end_multiplier_bp: 20000,  // end at 200%
            };
            let dynamic_royalty = DynamicRoyalty {
                version: 1,
                kind: DYNAMIC_ROYALTY_KIND_PRICE_LINEAR,
                override_royalty_bp: None,
                kind_price_linear: Some(price_linear),
                _reserved_0: [0; 32],
                _reserved_1: [0; 32],
                _reserved_2: [0; 32],
                _reserved_3: [0; 32],
            };
            let metadat_roaylty_bp = 1000;

            assert_eq!(dynamic_royalty.get_royalty_bp(0, metadat_roaylty_bp), 500);
            assert_eq!(dynamic_royalty.get_royalty_bp(100, metadat_roaylty_bp), 500);
            assert_eq!(dynamic_royalty.get_royalty_bp(1000, metadat_roaylty_bp), 2000);
            assert_eq!(dynamic_royalty.get_royalty_bp(10000, metadat_roaylty_bp), 2000);
            assert_eq!(dynamic_royalty.get_royalty_bp(500, metadat_roaylty_bp), 1166);
        }

        // override royalty_bp
        {
            // with 0 override
            let price_linear = DynamicRoyaltyPriceLinear {
                price_mint: None,
                start_price: 100,
                end_price: 1000,
                start_multiplier_bp: 5000, // start from 50%
                end_multiplier_bp: 20000,  // end at 200%
            };
            let dynamic_royalty = DynamicRoyalty {
                version: 1,
                kind: DYNAMIC_ROYALTY_KIND_PRICE_LINEAR,
                override_royalty_bp: Some(0),
                kind_price_linear: Some(price_linear),
                _reserved_0: [0; 32],
                _reserved_1: [0; 32],
                _reserved_2: [0; 32],
                _reserved_3: [0; 32],
            };
            let metadat_roaylty_bp = 1000; // not used

            assert_eq!(dynamic_royalty.get_royalty_bp(0, metadat_roaylty_bp), 0);
            assert_eq!(dynamic_royalty.get_royalty_bp(100, metadat_roaylty_bp), 0);
            assert_eq!(dynamic_royalty.get_royalty_bp(1000, metadat_roaylty_bp), 0);
            assert_eq!(dynamic_royalty.get_royalty_bp(10000, metadat_roaylty_bp), 0);
            assert_eq!(dynamic_royalty.get_royalty_bp(500, metadat_roaylty_bp), 0);

            // with normal override
            let price_linear = DynamicRoyaltyPriceLinear {
                price_mint: None,
                start_price: 100,
                end_price: 1000,
                start_multiplier_bp: 5000, // start from 50%
                end_multiplier_bp: 20000,  // end at 200%
            };
            let dynamic_royalty = DynamicRoyalty {
                version: 1,
                kind: DYNAMIC_ROYALTY_KIND_PRICE_LINEAR,
                override_royalty_bp: Some(2000),
                kind_price_linear: Some(price_linear),
                _reserved_0: [0; 32],
                _reserved_1: [0; 32],
                _reserved_2: [0; 32],
                _reserved_3: [0; 32],
            };
            let metadat_roaylty_bp = 1000; // not used

            assert_eq!(dynamic_royalty.get_royalty_bp(0, metadat_roaylty_bp), 1000);
            assert_eq!(dynamic_royalty.get_royalty_bp(100, metadat_roaylty_bp), 1000);
            assert_eq!(dynamic_royalty.get_royalty_bp(1000, metadat_roaylty_bp), 4000);
            assert_eq!(dynamic_royalty.get_royalty_bp(10000, metadat_roaylty_bp), 4000);
            assert_eq!(dynamic_royalty.get_royalty_bp(500, metadat_roaylty_bp), 2333);
        }
    }
}
