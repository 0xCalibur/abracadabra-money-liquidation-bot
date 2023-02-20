use ethers::types::U256;

use crate::bindings::cauldronliquidator::Rebase;

impl Rebase {
    pub fn to_elastic(&self, base: U256, round_up: bool) -> U256 {
        if self.base == 0 {
            return base
        } else {
            let elastic = base.checked_mul(U256::from(self.elastic)).unwrap() / self.base;
            if round_up && elastic.checked_mul(U256::from(self.base)).unwrap() / self.elastic < base
            {
                elastic.checked_add(U256::from(1));
            }

            return elastic
        }
    }

    pub fn to_base(&self, elastic: U256, round_up: bool) -> U256 {
        if self.elastic == 0 {
            return elastic
        } else {
            let base = elastic.checked_mul(U256::from(self.base)).unwrap() / self.elastic;
            if round_up && base.checked_mul(U256::from(self.elastic)).unwrap() / self.base < elastic
            {
                base.checked_add(U256::from(1));
            }

            return base
        }
    }
}
