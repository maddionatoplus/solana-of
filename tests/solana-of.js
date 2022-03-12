const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaOf;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your transaction signature", tx);
  
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Users', account.users.length)

  console.log("\nADD USER: toplus\n")
  
  await program.rpc.addUser({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  const to = "A6ZMMWnMkENd91DoBwP2JoFcNUKCHt632NeRa3Rxbsjc";
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Users', account.users.length)
  console.log('\t- ', account.users[0])

  console.log("\nBecome Creator\n")
  
  await program.rpc.becomeCreator("toplus", "image", "cover", "Top content Co-Founder", 15, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  }); 
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Users', account.users.length)
  console.log('\t- ', account.users[0])

  console.log("\nUPDATE USER INFO\n")
  
  await program.rpc.updateUserInfo("toplus90", "image2", "cover2", "Top content N00B", 105, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Users', account.users.length)
  console.log('\t- ', account.users[0])
  
  console.log("\nADD toplus first content\n")
  
  await program.rpc.addContent("first content", "first title", "first description", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Users', account.users.length)
  console.log('\t- ', account.users[0])
  
  console.log("\tDELETE toplus first content\n")
  
  await program.rpc.deleteContent(0, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Users', account.users.length)
  console.log('\t- ', account.users[0])
  
  console.log("\nADD again toplus first content\n")
  
  await program.rpc.addContent("first content", "first title", "first description", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  console.log("\nvote toplus first content\n")
  
  await program.rpc.upVote("first content",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      voter: provider.wallet.publicKey,
    },
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('\t Contents', account.users[0].contents.length)
  console.log('', account.users[0].contents[0])
  
  console.log("\nADD toplus subscription to: " + to + "\n")
  
  await program.rpc.addSubscription({
    accounts: {
      baseAccount: baseAccount.publicKey,
      subscriber: provider.wallet.publicKey,
      creator: to
    },
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('\t Subscriptions', account.users[0].subscriptions.length)
  console.log('', account.users[0].subscriptions[0])

  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Users', account.users.length)
  console.log('\t- ', account.users[0])  
  
  console.log("\REMOVE toplus subscription to: " + to + "\n")
  
  await program.rpc.removeSubscription({
    accounts: {
      baseAccount: baseAccount.publicKey,
      subscriber: provider.wallet.publicKey,
      creator: to
    },
  });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('\t Subscriptions', account.users[0].subscriptions.length) 
  console.log('\t- ', account.users[0])  

}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();