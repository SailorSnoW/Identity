import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { ProgramIdentity } from "../target/types/program_identity";


describe("identity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Identity as Program<ProgramIdentity>;

  // accounts
  // Static keypair only for testing purpose !! DO NOT use these private keys
  const alice: web3.Keypair = web3.Keypair.fromSeed(
    Uint8Array.from([200 ,131 ,215 ,228 ,31 ,46 ,26 ,84 ,255 ,248 ,18 ,26 ,154 ,144 ,90 ,87 ,62 ,63 ,76 ,65 ,68 ,61 ,7 ,193 ,212 ,249 ,117 ,225 ,154 ,64 ,152 ,212])
  )

  before(async () => {
    // Send some lamports to `alice` account
    const transaction = new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: anchor.getProvider().wallet.publicKey,
        toPubkey: alice.publicKey,
        lamports: web3.LAMPORTS_PER_SOL // 1 SOL
      })
    );
    await anchor.getProvider().send(transaction);
  })

  context("Identity creation", () => {
    it("create the identity of alice", async () => {

    })
  })
});
