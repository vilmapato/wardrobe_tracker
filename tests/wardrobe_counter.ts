import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WardrobeCounter } from "../target/types/wardrobe_counter";
import { assert } from "chai";
import { Keypair } from "@solana/web3.js";

describe("wardrobe_counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.wardrobeCounter as Program<WardrobeCounter>;
  const user = Keypair.generate();

  async function airdrop(
    connection: anchor.web3.Connection,
    address: anchor.web3.PublicKey,
    amount = 1_000_000_000
  ) {
    const sig = await connection.requestAirdrop(address, amount);
    await connection.confirmTransaction(sig);
  }

  async function getWardrobePDA(authority: anchor.web3.PublicKey) {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("wardrobe"), authority.toBuffer()],
      program.programId
    );
  }

  it("Initializes the wardrobe counter", async () => {
    await airdrop(provider.connection, user.publicKey);

    const [wardrobePda, bump] = await getWardrobePDA(user.publicKey);

    await program.methods
      .initialize(bump)
      .accounts({
        authority: user.publicKey,
        wardrobeCounter: wardrobePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const account = await program.account.wardrobeCounter.fetch(wardrobePda);

    assert.ok(account.authority.equals(user.publicKey));
    assert.equal(account.clothingUsage.toNumber(), 0);
    assert.equal(account.shoeUsage.toNumber(), 0);
    assert.equal(account.accessoryUsage.toNumber(), 0);
    assert.equal(account.bump, bump);
  });

  it("Increments clothing usage", async () => {
    const [wardrobePda] = await getWardrobePDA(user.publicKey);

    await program.methods
      .increment({ clothing: {} })
      .accounts({
        authority: user.publicKey,
        wardrobe_counter: wardrobePda,
      })
      .signers([user])
      .rpc();

    const account = await program.account.wardrobeCounter.fetch(wardrobePda);

    assert.equal(account.clothingUsage.toNumber(), 1);
    assert.equal(account.shoeUsage.toNumber(), 0);
    assert.equal(account.accessoryUsage.toNumber(), 0);
  });

  it("Increments shoe usage", async () => {
    const [wardrobePda] = await getWardrobePDA(user.publicKey);

    await program.methods
      .increment({ shoes: {} })
      .accounts({
        authority: user.publicKey,
        wardrobe_counter: wardrobePda,
      })
      .signers([user])
      .rpc();

    const account = await program.account.wardrobeCounter.fetch(wardrobePda);

    assert.equal(account.clothingUsage.toNumber(), 1);
    assert.equal(account.shoeUsage.toNumber(), 1);
    assert.equal(account.accessoryUsage.toNumber(), 0);
  });
});
