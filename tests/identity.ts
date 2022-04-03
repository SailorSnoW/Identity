import * as anchor from "@project-serum/anchor";
import { Program, web3, BN, EventParser } from "@project-serum/anchor";
import { ProgramIdentity } from "../target/types/program_identity";
import { expect } from "chai"
import { base64 } from "@project-serum/anchor/dist/cjs/utils/bytes";


describe("identity", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Identity as Program<ProgramIdentity>;

  // accounts
  // Static keypair only for testing purpose !! DO NOT use these private keys
  const alice: web3.Keypair = web3.Keypair.fromSeed(
    Uint8Array.from([200 ,131 ,215 ,228 ,31 ,46 ,26 ,84 ,255 ,248 ,18 ,26 ,154 ,144 ,90 ,87 ,62 ,63 ,76 ,65 ,68 ,61 ,7 ,193 ,212 ,249 ,117 ,225 ,154 ,64 ,152 ,212])
  )

  // accounts identity datas
  const alice_first_name = "Alice"
  const alice_last_name = "Testing"
  const alice_username = "XxAlicexX"
  const alice_mail = "alice@gmail.com"
  const alice_birth = new BN(954781773)

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
      const [alice_pda, alice_pda_bump] = await web3.PublicKey.findProgramAddress(
        [Buffer.from("Identity"), alice.publicKey.toBuffer()],
        program.programId
      )
      
      let signature = await program.methods.createIdentity(
        alice_first_name,
        alice_last_name,
        alice_username,
        alice_birth,
        alice_mail
      )
      .accounts({
        user: alice.publicKey,
        identity: alice_pda,
        systemProgram: web3.SystemProgram.programId
      })
      .signers([alice])
      .rpc();
    
      const alice_identity = await program.account.identity.fetch(alice_pda)
      expect(alice_identity.firstName).to.equal(alice_first_name);
      expect(alice_identity.lastName).to.equal(alice_last_name);
      expect(alice_identity.birth).to.eql(alice_birth);
      expect(alice_identity.username).to.equal(alice_username);
      expect(alice_identity.mail).to.equal(alice_mail);
      expect(alice_identity.bump).to.equal(alice_pda_bump);
    })
  })
});
