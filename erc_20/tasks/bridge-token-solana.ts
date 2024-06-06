import { task } from "hardhat/config";
import { PublicKey } from "@solana/web3.js";
task("bridge-token-solana", "")
  .addParam("amount", "Amount of tokens in ETH")
  .addOptionalParam("wallet", "Custom wallet")
  .addOptionalParam("signer", "Custom signer (private key)")
  .addOptionalParam("provider", "Custom provider RPC url")
  .setAction(async (args, hre: any) => {
    const ethers = hre.ethers;
    const network = hre.network.name;

    const [deployer] = await ethers.getSigners();

    let signer = deployer;
    let wallet = deployer.address;
    if (args.signer)
      signer = new ethers.Wallet(
        args.signer,
        new ethers.providers.JsonRpcProvider(args.provider)
      );
    if (args.wallet) wallet = args.wallet;

    const helloERC20 = await ethers.getContract("HelloERC20");
    const coder = new ethers.AbiCoder();
    const encodedWallet = coder.encode(
      ["bytes32"],
      [new PublicKey(args.wallet).toBuffer()]
    );

    await (
      await helloERC20
        .connect(signer)
        .approve(signer.address, ethers.parseEther("1000"))
    ).wait();

    try {
      const tx = await (
        await helloERC20
          .connect(signer)
          .bridge(
            BigInt(19999999991),
            encodedWallet,
            Math.pow(10, 6) * args.amount
          )
      ).wait();
      console.log(tx);
    } catch (error) {
      console.log(error);
    }

    console.log(
      "sent",
      args.amount,
      " of tokens to",
      wallet,
      "on chain id",
      args.dest
    );
  });
