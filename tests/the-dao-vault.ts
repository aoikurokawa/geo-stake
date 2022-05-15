import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TheDaoVault } from "../target/types/the_dao_vault";

describe("the-dao-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TheDaoVault as Program<TheDaoVault>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
