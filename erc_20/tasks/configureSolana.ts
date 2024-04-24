import { task } from "hardhat/config";
const chainsConfig = require("@cryptolink/contracts/config/chains");
const networks = require("../networks-testnet.json");
import bs58 from "bs58";

const fs = require("fs");

// bugfix for metis + ethers6
const GAS_LIMIT = 0x600000;

const SOLANA_CHAIN_ID = 19999999991;

task("configureSolana", "")
  .addParam("solanaContract")
  .addOptionalParam("signer", "Custom signer (private key)")
  .addOptionalParam("provider", "Custom provider RPC url")
  .setAction(async (args, hre: any) => {
    const ethers = hre.ethers;

    const chainids = [SOLANA_CHAIN_ID];
    const [addresses] = [bs58.decode(args.solanaContract)];
    const confirmations = [1];

    const coder = new ethers.AbiCoder();

    const encoded = coder.encode(["bytes[]"], [[addresses]]);

    console.log(
      "setting remote contract addresses .. CLT message address:",
      chainsConfig[hre.network.config.chainId].message
    );
    const helloERC20 = await ethers.getContract("HelloERC20");
    console.log(chainsConfig[hre.network.config.chainId].message, "CCC");
    await (
      await helloERC20.configureClientExtended(
        chainsConfig[hre.network.config.chainId].message,
        chainids,
        [encoded],
        confirmations
      )
    ).wait();

    console.log("Configured client on address " + helloERC20.target);
  });
