import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LotteryDapp } from "../target/types/lottery_dapp";

describe("lottery-dapp", () => {
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.LotteryDapp as Program<LotteryDapp>;
  const pg = anchor.web3.Keypair.generate();
  const payer = provider.wallet as anchor.Wallet;

  const [lotteryPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("LOTTER_STATE")],
    program.programId
  )

  it("Is initialized!", async () => {
    const price = new anchor.BN(1);
    const timestamp = new anchor.BN(Date.now());
    const tx = await program.methods
    .initialize(price , timestamp)
    .accounts({
      lottery : lotteryPDA,
      authority : payer.publicKey,
    })
    .rpc()
    console.log("Your transaction signature", tx);
  });

  it("buys ticket", async () =>{
    const tx = await program.methods
    .buyTicket()
    .accounts({
        buyer : payer.publicKey,
    })
    .signers([payer.payer])
    .rpc()
    console.log("your transaction signature" , tx )
  })

});
