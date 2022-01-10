import * as anchor from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import { Token, TOKEN_PROGRAM_ID, MintLayout } from "@solana/spl-token";
import { BN, Program } from "@project-serum/anchor";
import { Cultures } from "../target/types/cultures";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAccountAddress,
} from "./helpers/tokenHelpers";

declare var TextEncoder: any;

describe("cultures", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const anyAnchor: any = anchor;
  const Cultures = anyAnchor.workspace.Cultures as Program<Cultures>;

  interface Pda {
    address: web3.PublicKey;
    bump: number;
  }
  let MembershipToken: Token;
  let testCulture: Pda;
  let testCollection = web3.Keypair.generate();
  let stakeAuthority: Pda;
  let testName = "test";
  let membershipMint = web3.Keypair.generate();
  let membership: Pda;
  let creatorTokenAccount: PublicKey;
  let payer = web3.Keypair.generate();
  let creatorStakePool: Pda;
  let creatorRedemptionMint: Pda;
  let audienceStakePool: Pda;
  let audienceRedemptionMint: Pda;
  let post = web3.Keypair.generate();

  let makeToken = true;
  let programInit = true;
  let cultureInit = true;
  let createMembershipAcct = true;
  let increaseCreatorStake = true;
  let decreaseCreatorStake = false;
  let increaseAudienceStake = false;
  let decreaseAudienceStake = false;
  let submitLike = true;

  it("setup", async () => {
    testCulture = await findCulture(testName);
    stakeAuthority = await findAuthority("stake");
    membership = await findMembership(
      testCulture.address,
      provider.wallet.publicKey
    );
    MembershipToken = new Token(
      provider.connection,
      membershipMint.publicKey,
      TOKEN_PROGRAM_ID,
      payer
    );
    creatorTokenAccount = await getAssociatedTokenAccountAddress(
      provider.wallet.publicKey,
      membershipMint.publicKey
    );
    creatorStakePool = await findCreatorStakePool(testCulture.address);
    creatorRedemptionMint = await findCreatorRedemptionMint(
      testCulture.address
    );
    audienceStakePool = await findAudienceStakePool(testCulture.address);
    audienceRedemptionMint = await findAudienceRedemptionMint(
      testCulture.address
    );
  });

  if (makeToken) {
    it("make a token", async () => {
      //create subscription mint account
      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(
          payer.publicKey,
          1 * web3.LAMPORTS_PER_SOL
        ),
        "confirmed"
      );
      let transaction = new web3.Transaction().add(
        SystemProgram.createAccount({
          fromPubkey: payer.publicKey,
          newAccountPubkey: membershipMint.publicKey,
          space: MintLayout.span,
          lamports: await provider.connection.getMinimumBalanceForRentExemption(
            MintLayout.span
          ),
          programId: TOKEN_PROGRAM_ID,
        }),
        //init subscription mint account
        Token.createInitMintInstruction(
          TOKEN_PROGRAM_ID,
          membershipMint.publicKey,
          0,
          payer.publicKey,
          null
        ),
        createAssociatedTokenAccountInstruction(
          membershipMint.publicKey,
          creatorTokenAccount,
          provider.wallet.publicKey,
          payer.publicKey
        )
      );
      await web3.sendAndConfirmTransaction(provider.connection, transaction, [
        payer,
        membershipMint,
      ]);

      MembershipToken.mintTo(creatorTokenAccount, payer, [], 100);
    });
  }

  if (programInit) {
    it("program init", async () => {
      const tx = await Cultures.rpc.initializeProgram(stakeAuthority.bump, {
        accounts: {
          initializer: provider.wallet.publicKey,
          stakeAuthority: stakeAuthority.address,
          systemProgram: SystemProgram.programId,
        },
      });
    });
  }

  if (cultureInit) {
    it("culture init", async () => {
      // Add your test here.

      const tx = await Cultures.rpc.createCulture(testCulture.bump, testName, {
        accounts: {
          payer: provider.wallet.publicKey,
          culture: testCulture.address,
          collection: testCollection.publicKey,
          creatorMint: membershipMint.publicKey,
          creatorStakePool: creatorStakePool.address,
          creatorRedemptionMint: creatorRedemptionMint.address,
          audienceMint: membershipMint.publicKey,
          audienceStakePool: audienceStakePool.address,
          audienceRedemptionMint: audienceRedemptionMint.address,
          stakeAuthority: stakeAuthority.address,
          rent: web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
        },
      });
      console.log("Your transaction signature", tx);
      let newCulture = await Cultures.account.culture.fetch(
        testCulture.address
      );
      //console.log(newCulture);
    });
  }

  if (createMembershipAcct) {
    it("create membership account", async () => {
      const tx = await Cultures.rpc.createMembership(membership.bump, {
        accounts: {
          newMember: provider.wallet.publicKey,
          culture: testCulture.address,
          membership: membership.address,
          systemProgram: SystemProgram.programId,
        },
      });
    });
  }

  if (increaseCreatorStake) {
    it("increase creator stake", async () => {
      const tx = await Cultures.rpc.changeCreatorStake(
        membership.bump,
        creatorStakePool.bump,
        new BN(50),
        {
          accounts: {
            member: provider.wallet.publicKey,
            culture: testCulture.address,
            membership: membership.address,
            creatorTokenAccount: creatorTokenAccount,
            creatorStakePool: creatorStakePool.address,
            stakeAuthority: stakeAuthority.address,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          },
        }
      );

      let creatorAcct = await provider.connection.getTokenAccountBalance(
        creatorTokenAccount
      );
      console.log(creatorAcct);
      let membershipp = await Cultures.account.membership.fetch(
        membership.address
      );
      printMembership(membershipp);
    });
  }

  if (decreaseCreatorStake) {
    it("decrease creator stake", async () => {
      const tx = await Cultures.rpc.changeCreatorStake(
        membership.bump,
        creatorStakePool.bump,
        new BN(-20),
        {
          accounts: {
            member: provider.wallet.publicKey,
            culture: testCulture.address,
            membership: membership.address,
            creatorTokenAccount: creatorTokenAccount,
            creatorStakePool: creatorStakePool.address,
            stakeAuthority: stakeAuthority.address,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          },
        }
      );

      let creatorAcct = await provider.connection.getTokenAccountBalance(
        creatorTokenAccount
      );
      console.log(creatorAcct);
      let membershipp = await Cultures.account.membership.fetch(
        membership.address
      );
      printMembership(membershipp);
    });
  }

  if (increaseAudienceStake) {
    it("increase audience stake", async () => {
      const tx = await Cultures.rpc.changeAudienceStake(
        membership.bump,
        audienceStakePool.bump,
        new BN(20),
        {
          accounts: {
            member: provider.wallet.publicKey,
            culture: testCulture.address,
            membership: membership.address,
            audienceTokenAccount: creatorTokenAccount,
            audienceStakePool: audienceStakePool.address,
            stakeAuthority: stakeAuthority.address,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          },
        }
      );

      let creatorAcct = await provider.connection.getTokenAccountBalance(
        creatorTokenAccount
      );
      console.log(creatorAcct);
      let membershipp = await Cultures.account.membership.fetch(
        membership.address
      );
      printMembership(membershipp);
    });
  }

  if (decreaseAudienceStake) {
    it("decrease audience stake", async () => {
      const tx = await Cultures.rpc.changeAudienceStake(
        membership.bump,
        audienceStakePool.bump,
        new BN(-10),
        {
          accounts: {
            member: provider.wallet.publicKey,
            culture: testCulture.address,
            membership: membership.address,
            audienceTokenAccount: creatorTokenAccount,
            audienceStakePool: audienceStakePool.address,
            stakeAuthority: stakeAuthority.address,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          },
        }
      );

      let creatorAcct = await provider.connection.getTokenAccountBalance(
        creatorTokenAccount
      );
      console.log(creatorAcct);
      let membershipp = await Cultures.account.membership.fetch(
        membership.address
      );
      printMembership(membershipp);
    });
  }

  it("submit post", async () => {
    let body = "baby's first post 😘";
    let tx = await Cultures.rpc.submitPost(calculatePostSize(body), body, {
      accounts: {
        culture: testCulture.address,
        member: provider.wallet.publicKey,
        membership: membership.address,
        post: post.publicKey,
        clock: web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: SystemProgram.programId,
      },
      signers: [post],
    });

    let postInfo = await Cultures.account.post.fetch(post.publicKey);
    console.log(postInfo);
    calculatePostSize(body);
  });

  if (submitLike) {
    it("submit like", async () => {
      let likeAttr = await findLikeAttribution(
        membership.address,
        post.publicKey
      );
      const tx = await Cultures.rpc.submitLike(likeAttr.bump, {
        accounts: {
          culture: testCulture.address,
          liker: provider.wallet.publicKey,
          likerMembership: membership.address,
          post: post.publicKey,
          posterMembership: membership.address,
          likeAttribution: likeAttr.address,
          systemProgram: SystemProgram.programId,
        },
      });

      let postInfo = await Cultures.account.post.fetch(post.publicKey);
      console.log("post score,   ", postInfo.score.toNumber());
    });
  }

  const calculatePostSize = (body: String) => {
    let defaultSize = Cultures.account.post.size + 3; //4 byte setup on the string
    let encodedLength = new TextEncoder().encode(body).length;
    return defaultSize + encodedLength;
  };

  const findLikeAttribution = async (
    membership: PublicKey,
    post: PublicKey
  ) => {
    return PublicKey.findProgramAddress(
      [membership.toBuffer(), post.toBuffer()],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findCulture = async (name: String) => {
    return PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("culture"),
        anchor.utils.bytes.utf8.encode(name.toLowerCase()),
      ],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findAuthority = async (seed: string) => {
    return PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode(seed)],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findMembership = async (culture: PublicKey, authority: PublicKey) => {
    return PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("membership"),
        culture.toBuffer(),
        authority.toBuffer(),
      ],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findCreatorStakePool = async (culture: PublicKey) => {
    return PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("c_stake"), culture.toBuffer()],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findCreatorRedemptionMint = async (culture: PublicKey) => {
    return PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("c_redemption"), culture.toBuffer()],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findAudienceStakePool = async (culture: PublicKey) => {
    return PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("a_stake"), culture.toBuffer()],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const findAudienceRedemptionMint = async (culture: PublicKey) => {
    return PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("a_redemption"), culture.toBuffer()],
      Cultures.programId
    ).then(([address, bump]) => {
      return {
        address: address,
        bump: bump,
      };
    });
  };
  const printMembership = (membership: any) => {
    let newMembership = {
      culture: membership.culture.toBase58(),
      member: membership.member.toBase58(),
      creatorStake: membership.creatorStake.toNumber(),
      audienceStake: membership.audienceStake.toNumber(),
      allTimeScore: membership.allTimeScore.toNumber(),
    };
    console.log(newMembership);
  };
});
