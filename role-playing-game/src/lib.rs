// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn chk_mana(&self) -> Option<u32> {
        if self.level < 10 {
            None
        } else {
            Some(100)
        }
    }
    pub fn revive(&self) -> Option<Player> {
        //unimplemented!("Revive this player")
        if self.health > 0 {
            return None;
        }
        Some(Player {
            health: 100,
            //mana: if self.level < 10 { None } else { Some(100) },
            mana: self.chk_mana(),
            level: self.level,
        })
    }

    /* fn chk_cost(&mut self, mana_cost: u32) -> u32 {
        if self.mana < mana_cost {
            return 0;
        }
        self.mana = Some(self.mana - mana_cost);
        2 * mana_cost
    } */
    fn chk_health(&mut self, mana_cost: u32) -> u32 {
        self.health
            - if self.health < mana_cost {
                self.health
            } else {
                mana_cost
            }
        //0
    }
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        //unimplemented!("Cast a spell of cost {}", mana_cost)
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    return 0;
                }
                self.mana = Some(mana - mana_cost);
                2 * mana_cost
            }
            None => {
                self.health = self.chk_health(mana_cost);
                /* self.health
                - if self.health < mana_cost {
                    self.health
                } else {
                    mana_cost
                }; */
                0
            }
        }
    }
}
