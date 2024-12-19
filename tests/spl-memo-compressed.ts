import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplMemoCompressed } from "../target/types/spl_memo_compressed";
import { PublicKey } from "@solana/web3.js";
import { createRpc, compressData, ZKCompression } from "@lightprotocol/stateless.js";

describe("spl-memo-compressed", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SplMemoCompressed as Program<SplMemoCompressed>;

  it("Creates a ZK-compressed memo", async () => {
    const connection = createRpc();
    const memo = "Hello, Solana!";
    
    // Initialize ZK Compression
    const zkCompression = new ZKCompression({
      connection,
      commitment: 'confirmed',
    });
    
    // Compress the memo
    const compressedMemo = await zkCompression.compress({
      data: Buffer.from(memo, 'utf-8'),
      type: 'memo',
    });
    
    try {
      const tx = await program.methods
        .createMemo(compressedMemo.data)
        .accounts({
          memoProgram: new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
          signer: provider.wallet.publicKey,
        })
        .rpc();

      console.log("Transaction signature:", tx);
      
      // Verify compression
      const decompressedMemo = await zkCompression.decompress({
        compressedData: compressedMemo.data,
        type: 'memo',
      });
      
      console.log("Original memo:", memo);
      console.log("Decompressed memo:", decompressedMemo.toString('utf-8'));
      
      // Assert the decompressed data matches the original
      assert.strictEqual(
        decompressedMemo.toString('utf-8'),
        memo,
        "Decompressed memo should match original"
      );
      
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});
