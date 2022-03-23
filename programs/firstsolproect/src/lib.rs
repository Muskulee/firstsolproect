use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod firstsolproect {
  use super::*;
  pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> Result<()> {
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff {}