use anchor_lang::prelude::*;

declare_id!("CaGtvV7d6nuih4gFisrJ9FJoY9kXeC7x4nWhdDzsrxRa");

#[program]
pub mod solana_of {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.users = [].to_vec(); 
        Ok(())
    }
 
    pub fn add_user(ctx: Context<AddUser>) -> Result<()> {
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;
   
      let user = User {
        user_address: *user.to_account_info().key,
        image: "".to_string(),
        name: "".to_string(),
        bio: "".to_string(), 
        month_price: 1,
        total: 0,
        creator: false,
        subscriptions: [].to_vec(),
        contents: [].to_vec(),
        followers: [].to_vec(),
      };
          
      base_account.users.push(user);
      Ok(())
    }

    pub fn update_user_info(ctx: Context<UpdateUserInfo>, name:String, image: String, bio: String, month_price: u32) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
   
        if base_account.users.iter().any(|u| u.user_address == *user.to_account_info().key) {
            let index = base_account.users.iter().position(|u| u.user_address == *user.to_account_info().key).unwrap();
             
            base_account.users[index].name = name.to_string();
            base_account.users[index].image = image.to_string();
            base_account.users[index].bio = bio.to_string();
            base_account.users[index].month_price = month_price as u64; 
        }
        
        Ok(())
    }

    pub fn become_creator(ctx: Context<BecomeCreator>, name: String, image: String, bio: String, month_price: u32) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        if !base_account.users.iter().any(|u| u.user_address == *user.to_account_info().key) {
            let user = User {
                user_address: *user.to_account_info().key,
                image: "".to_string(),
                name: "".to_string(),
                bio: "".to_string(), 
                month_price: 1,
                total: 0,
                creator: false,
                subscriptions: [].to_vec(),
                contents: [].to_vec(),
                followers: [].to_vec(),
              };
                  
              base_account.users.push(user);
        }
   
        if base_account.users.iter().any(|u| u.user_address == *user.to_account_info().key) {
            let index = base_account.users.iter().position(|u| u.user_address == *user.to_account_info().key).unwrap();
            
            base_account.users[index].name = name.to_string();
            base_account.users[index].image = image.to_string();
            base_account.users[index].bio = bio.to_string();
            base_account.users[index].month_price = month_price as u64;
            base_account.users[index].creator = true;
        }
        
        Ok(())
    }

    pub fn add_content(ctx: Context<AddContent>, link: String, title: String, description: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
   
        if base_account.users.iter().any(|u| u.user_address == *user.to_account_info().key) {
            let index = base_account.users.iter().position(|u| u.user_address == *user.to_account_info().key).unwrap();

            let content = Content {
                user_address: *user.to_account_info().key,
                link: link.to_string(),
                title: title.to_string(),
                description: description.to_string(),
                votes: 0,
                user_votes: [].to_vec(),
                date: Clock::get().unwrap().unix_timestamp,
            };
            
            base_account.users[index].contents.push(content);
        }
        
        Ok(())
    }

    pub fn add_subscription(ctx: Context<AddSubscription>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let subscriber = &mut ctx.accounts.subscriber;
        let creator = &mut ctx.accounts.creator;

        if !base_account.users.iter().any(|u| u.user_address == *subscriber.to_account_info().key) {
            let user = User {
                user_address: *subscriber.to_account_info().key,
                image: "".to_string(),
                name: "".to_string(),
                bio: "".to_string(), 
                month_price: 1,
                total: 0,
                creator: false,
                subscriptions: [].to_vec(),
                contents: [].to_vec(),
                followers: [].to_vec(),
              };
                  
              base_account.users.push(user);
        }
   
        if base_account.users.iter().any(|u| u.user_address == *subscriber.to_account_info().key) {
            let index = base_account.users.iter().position(|u| u.user_address == *subscriber.to_account_info().key).unwrap();
            
            let subscription = Subscription {
                user_address: creator.key(),
                subscription_end: Clock::get().unwrap().unix_timestamp + 30 * 86400,
            };

            base_account.users[index].subscriptions.push(subscription);
            
            if base_account.users.iter().any(|u| u.user_address == *creator.to_account_info().key) {
                

                let creator_index = base_account.users.iter().position(|u| u.user_address == *creator.to_account_info().key).unwrap();
    

                let follower_subscription = Subscription {
                    user_address: subscriber.key(),
                    subscription_end: Clock::get().unwrap().unix_timestamp + 30 * 86400, 
                };

                base_account.users[creator_index].total = base_account.users[index].total + base_account.users[index].month_price;
                base_account.users[creator_index].followers.push(follower_subscription);
            }
        }

        Ok(())
    }

    pub fn remove_subscription(ctx: Context<RemoveSubscription>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let subscriber = &mut ctx.accounts.subscriber;
        let creator = &mut ctx.accounts.creator;
   
        if base_account.users.iter().any(|u| u.user_address == *subscriber.to_account_info().key) {
            let user_index = base_account.users.iter().position(|u| u.user_address == *subscriber.to_account_info().key).unwrap();
            let subscription_index = base_account.users[user_index].subscriptions.iter().position(|s| s.user_address == *creator.to_account_info().key).unwrap();
            base_account.users[user_index].subscriptions.remove(subscription_index);
            
            if base_account.users.iter().any(|u| u.user_address == *creator.to_account_info().key) {
                let creator_index = base_account.users.iter().position(|u| u.user_address == *creator.to_account_info().key).unwrap();
                let subscription_index = base_account.users[creator_index].followers.iter().position(|s| s.user_address == *subscriber.to_account_info().key).unwrap();
                base_account.users[creator_index].followers.remove(subscription_index);
            }
        }
        
        Ok(())
    }

    pub fn up_vote(ctx: Context<UpVote>, link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let voter = &mut ctx.accounts.voter;
        
        if base_account.users.iter().any(|u| u.user_address == *user.to_account_info().key) {
            let user_index = base_account.users.iter().position(|u| u.user_address == *user.to_account_info().key).unwrap();
            let content_index = base_account.users[user_index].contents.iter().position(|c| c.link == link).unwrap();

            base_account.users[user_index].contents[content_index].votes += 1;
            base_account.users[user_index].contents[content_index].user_votes.push(*voter.to_account_info().key);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddUser<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateUserInfo<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct BecomeCreator<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddContent<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddSubscription<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub subscriber: Signer<'info>,
  #[account(mut)]
  pub creator: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpVote<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: AccountInfo<'info>,
  #[account(mut)]
  pub voter: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveSubscription<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub subscriber: Signer<'info>,
  #[account(mut)]
  pub creator: AccountInfo<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Content {
    pub user_address: Pubkey,
    pub link: String,
    pub title: String,
    pub description: String,
    pub votes: u64,
    pub user_votes: Vec<Pubkey>,
    pub date: i64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Subscription {
    pub user_address: Pubkey,
    pub subscription_end: i64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct User {
    pub user_address: Pubkey,
    pub creator: bool, 
    pub image: String, 
    pub bio: String, 
    pub month_price: u64,
    pub total: u64,
    pub contents: Vec<Content>,
    pub subscriptions: Vec<Subscription>,
    pub followers: Vec<Subscription>,
    pub name: String,
}

#[account]
pub struct BaseAccount {
    pub users: Vec<User>,
}