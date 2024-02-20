import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import "wookashwackomytest-polkahat-chai-matchers";
import { SignAndSendSuccessResponse } from "wookashwackomytest-typechain-types";
import { getSigners } from "../../helpers/signers";
import { BN } from "bn.js";
import { Id, PSP34 } from "../../types/PSP34.type";

export const firstTokenId: Id = { u128: new BN(79216) };
export const secondTokenId = { u128: new BN(79217) };
export const nonExistentTokenId = { u8: new BN(13) };

export type ShouldBehaveLikePSP34Params = {
  token: PSP34;
  owner: KeyringPair;
};

export function shouldBehaveLikePSP34(
  getParams: () => ShouldBehaveLikePSP34Params
) {
  const ctx: ShouldBehaveLikePSP34Params & {
    tx: any;
    newOwner: KeyringPair;
    approved: KeyringPair;
    operator: KeyringPair;
    to: KeyringPair;
    other: KeyringPair;
  } = {} as any;

  describe("with minted tokens", function () {
    beforeEach(async function () {
      Object.assign(ctx, getParams());
      [ctx.newOwner, ctx.approved, ctx.operator, ctx.to, ctx.other] =
        getSigners().filter((signer) => signer.address !== ctx.owner.address);
      await expect(
        ctx.token.query.ownerOf(firstTokenId),
        `to use shouldBehaveLikePSP34 test you should set ownership of ${firstTokenId} to owner`
      ).to.haveOkResult(ctx.owner.address);
      await expect(
        ctx.token.query.ownerOf(secondTokenId),
        `to use shouldBehaveLikePSP34 test you should set ownership of ${secondTokenId} to owner`
      ).to.haveOkResult(ctx.owner.address);

      await expect(
        ctx.token.query.ownerOf(nonExistentTokenId),
        `to use shouldBehaveLikePSP34 test  ${nonExistentTokenId} should not exist`
      ).to.haveOkResult(null);

      await expect(
        ctx.token.query.totalSupply(),
        `to use shouldBehaveLikePSP34 test only ${firstTokenId} and ${secondTokenId} should exist. The total_supply should be 2.`
      ).to.haveOkResult(2);
    });

    describe("balanceOf", function () {
      describe(`when the given address owns ${firstTokenId} and ${secondTokenId} tokens`, function () {
        it("returns the amount of tokens owned by the given address", async function () {
          expect(
            await ctx.token.query.balanceOf(ctx.owner.address)
          ).to.haveOkResult(2);
        });
      });

      describe("when the given address does not own any tokens", function () {
        it("returns 0", async function () {
          expect(
            await ctx.token.query.balanceOf(ctx.other.address)
          ).to.haveOkResult(0);
        });
      });
    });
    describe("ownerOf", function () {
      describe("when the given token ID was tracked by this token", function () {
        const tokenId = firstTokenId;

        it("returns the owner of the given token ID", async function () {
          expect(await ctx.token.query.ownerOf(tokenId)).to.haveOkResult(
            ctx.owner.address
          );
        });
      });

      describe("when the given token ID was not tracked by this token", function () {
        const tokenId = nonExistentTokenId;

        it("returns null", async function () {
          await expect(ctx.token.query.ownerOf(tokenId)).to.haveOkResult(null);
        });
      });
    });

    describe("transfer", function () {
      const tokenId = firstTokenId;
      let tx: Promise<SignAndSendSuccessResponse>;

      beforeEach(async function () {
        await ctx.token
          .withSigner(ctx.owner)
          .tx.approve(ctx.approved.address, tokenId, true);
        await ctx.token
          .withSigner(ctx.owner)
          .tx.approve(ctx.operator.address, null, true);
        ctx.tx = () =>
          ctx.token
            .withSigner(ctx.owner)
            .tx.transfer(ctx.to.address, tokenId, []);
      });
      describe("when called by the owner", function () {
        beforeEach(function () {
          tx = ctx.token
            .withSigner(ctx.owner)
            .tx.transfer(ctx.to.address, tokenId, []);
        });
        transferWasSuccessful(() => ({
          tx: tx,
          token: ctx.token,
          from: ctx.owner.address,
          to: ctx.to.address,
          tokenId: tokenId,
        }));
      });

      describe("when called by the approved individual", function () {
        beforeEach(function () {
          tx = ctx.token
            .withSigner(ctx.approved)
            .tx.transfer(ctx.to.address, tokenId, []);
        });
        transferWasSuccessful(() => ({
          tx: tx,
          from: ctx.owner.address,
          to: ctx.to.address,
          tokenId: tokenId,
          token: ctx.token,
        }));
      });

      describe("when called by the operator", function () {
        beforeEach(function () {
          tx = ctx.token
            .withSigner(ctx.operator)
            .tx.transfer(ctx.to.address, tokenId, []);
        });
        transferWasSuccessful(() => ({
          tx: tx,
          from: ctx.owner.address,
          to: ctx.to.address,
          tokenId: tokenId,
          token: ctx.token,
        }));
      });

      describe("when sent to the owner", function () {
        beforeEach(function () {
          tx = ctx.token
            .withSigner(ctx.owner)
            .tx.transfer(ctx.owner.address, tokenId, []);
        });

        it("keeps ownership of the token", async function () {
          await tx;
          expect(await ctx.token.query.ownerOf(tokenId)).to.equal(
            ctx.owner.address
          );
        });

        it("emits only a transfer event", async function () {
          await expect(tx).to.emitEvent(ctx.token, "Transfer", {
            from: ctx.owner.address,
            to: ctx.owner.address,
            id: tokenId as any,
          });
        });

        it("keeps the owner balance", async function () {
          expect(tx).to.changePSP22Balances(
            ctx.token,
            [ctx.owner.address],
            [new BN(0)]
          );
        });
      });

      describe("when the sender is not authorized for the token id", function () {
        beforeEach(function () {
          tx = ctx.token
            .withSigner(ctx.other)
            .tx.transfer(ctx.other.address, tokenId, []);
        });
        it("reverts", async function () {
          await expect(tx).to.be.revertedWithError({ notApproved: null });
        });
      });

      describe("when the given token ID does not exist", function () {
        beforeEach(function () {
          tx = ctx.token
            .withSigner(ctx.owner)
            .tx.transfer(ctx.owner.address, nonExistentTokenId, []);
        });

        it("reverts", async function () {
          await expect(tx).to.be.revertedWithError({ tokenNotExists: null });
        });
      });
    });

    describe("approve", function () {
      const tokenId = firstTokenId;

      describe("when clearing approval", function () {
        describe("when there was no prior approval", function () {
          beforeEach(async function () {
            ctx.tx = ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, false);
          });

          it("is stays unapproved", async function () {
            await ctx.tx;
            expect(
              await ctx.token.query.allowance(
                ctx.owner.address,
                ctx.approved!.address,
                tokenId
              )
            ).to.haveOkResult(false);
          });
          it("emits an Approval event", async function () {
            await expect(ctx.tx).to.emitEvent(ctx.token, "Approval", {
              owner: ctx.owner.address,
              operator: ctx.approved!.address,
              approved: false,
            });
          });
        });

        describe("when there was a prior approval", function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, true);
            ctx.tx = ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, false);
          });

          it("is clears approval", async function () {
            await ctx.tx;
            expect(
              await ctx.token.query.allowance(
                ctx.owner.address,
                ctx.approved.address,
                tokenId
              )
            ).to.haveOkResult(false);
          });
          it("emits an Approval event", async function () {
            await expect(ctx.tx).to.emitEvent(ctx.token, "Approval", {
              owner: ctx.owner.address,
              operator: ctx.approved!.address,
              approved: false,
            });
          });
        });
      });

      describe("when approving anaccount", function () {
        describe("when there was a prior approval to the same address", function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, true);
            ctx.tx = ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, true);
          });

          it("is stays approved", async function () {
            await ctx.tx;
            expect(
              await ctx.token.query.allowance(
                ctx.owner.address,
                ctx.approved.address,
                tokenId
              )
            ).to.haveOkResult(true);
          });
          it("emits an Approval event", async function () {
            await expect(ctx.tx).to.emitEvent(ctx.token, "Approval", {
              owner: ctx.owner.address,
              operator: ctx.approved!.address,
              approved: true,
            });
          });
        });
      });

      describe("when the sender does not own the given token ID", function () {
        it("reverts", async function () {
          await expect(
            ctx.token
              .withSigner(ctx.other)
              .query.approve(ctx.approved.address, tokenId, true)
          ).to.be.revertedWithError({ notApproved: null });
        });
      });

      describe("when the sender is approved for the given token ID", function () {
        beforeEach(async function () {
          await ctx.token
            .withSigner(ctx.owner)
            .query.approve(ctx.approved.address, tokenId, true);
        });
        it("reverts", async function () {
          await expect(
            ctx.token
              .withSigner(ctx.approved)
              .query.approve(ctx.other.address, tokenId, true)
          ).to.be.revertedWithError({ notApproved: null });
        });
      });

      describe("when the sender is an operator", function () {
        beforeEach(async function () {
          await ctx.token
            .withSigner(ctx.owner)
            .tx.approve(ctx.operator.address, null, true);
        });

        it("reverts", async function () {
          await expect(
            ctx.token
              .withSigner(ctx.operator)
              .query.approve(ctx.approved.address, tokenId, true)
          ).to.be.revertedWithError({ notApproved: null });
        });
      });
    });
  });
}

export type transferWasSuccesfulParams = {
  tx: Promise<SignAndSendSuccessResponse>;
  token: any;
  from: string;
  to: string;
  tokenId: Id;
};

function transferWasSuccessful(ctx: () => transferWasSuccesfulParams) {
  it("transfers the ownership of the given token ID to the given address", async function () {
    await ctx().tx;
    expect(await ctx().token.query.ownerOf(ctx().tokenId)).to.equal(ctx().to);
  });

  it("emits a Transfer event", async function () {
    await expect(ctx().tx).to.emitEvent(ctx().token, "Transfer", {
      from: ctx().from,
      to: ctx().to,
      id: ctx().tokenId as any,
    });
  });

  it("clears the approval for the token ID with no event", async function () {
    await expect(ctx().tx).not.to.emitEvent(ctx().token, "Approval");

    expect(
      await ctx().token.query.allowance(ctx().from, ctx().to, ctx().tokenId)
    ).to.equal(null);
  });

  it.only("adjusts owners and receiver balances", async function () {
    if (ctx().from === ctx().to) {
      await expect(ctx().tx).to.changePSP22Balances(
        ctx().token,
        [ctx().from],
        [new BN(0)]
      );
    } else {
      await expect(ctx().tx).to.changePSP22Balances(
        ctx().token,
        [ctx().from, ctx().to],
        [new BN(-1), new BN(1)]
      );
    }
  });
}

// export type shouldTransferTokensByUsersParams = {
//   token: any;
//   fnName: string;
//   owner: KeyringPair;
//   approved: KeyringPair;
//   operator: KeyringPair;
//   notApproved: KeyringPair;
//   to: string;
//   tokenId: Id;
//   nonExistentTokenId: Id;
//   opts: any;
// };
// export async function shouldTransferTokensByUsers(
//   getParams: () => shouldTransferTokensByUsersParams
// ) {}

// export type transferWasSuccessfulParams = {
//   tx: () => Promise<SignAndSendSuccessResponse>;
//   token: any;
//   from: string;
//   to: string;
//   tokenId: Id;
// };

// export async function transferWasSuccessful(
//   getParams: () => transferWasSuccessfulParams
// ) {
//   it("transfers the ownership of the given token ID to the given address", async function () {
//     const ctx = getParams();
//     const res = await ctx.tx();
//     expect(await ctx.token.query.ownerOf(ctx.tokenId)).to.equal(ctx.to);
//   });

//   it("emits a Transfer event", async function () {
//     const ctx = getParams();
//     const res = await ctx.tx();

//     await expect(ctx.tx()).to.emitEvent(ctx.token, "Transfer", {
//       from: ctx.from,
//       to: ctx.to,
//       id: ctx.tokenId as any,
//     });
//   });

//   it("clears the approval for the token ID with no event", async function () {
//     const ctx = getParams();
//     await expect(ctx.tx()).not.to.emitEvent(ctx.token, "Approval");

//     expect(
//       await ctx.token.query.allowance(ctx.from, ctx.to, ctx.tokenId)
//     ).to.equal(null);
//   });

//   it("adjusts owners and receiver balances", async function () {
//     const ctx = getParams();
//     if (ctx.from === ctx.to) {
//       expect(ctx.tx).to.changePSP22Balances(ctx.token, [ctx.from], [new BN(0)]);
//     } else {
//       await expect(ctx.tx).to.changePSP22Balances(
//         ctx.token,
//         [ctx.from, ctx.to],
//         [new BN(-1), new BN(1)]
//       );
//     }
//   });
// }

// describe("_mint(address, uint256)", function () {
//   it("reverts with a null destination address", async function () {
//     await expect(ctx.token.$_mint(ethers.ZeroAddress, firstTokenId))
//       .to.be.revertedWithCustomError(ctx.token, "ERC721InvalidReceiver")
//       .withArgs(ethers.ZeroAddress);
//   });

//   describe("with minted token", async function () {
//     beforeEach(async function () {
//       ctx.tx = await ctx.token.$_mint(ctx.owner, firstTokenId);
//     });

//     it("emits a Transfer event", async function () {
//       await expect(ctx.tx)
//         .to.emit(ctx.token, "Transfer")
//         .withArgs(ethers.ZeroAddress, ctx.owner, firstTokenId);
//     });

//     it("creates the token", async function () {
//       expect(await ctx.token.balanceOf(ctx.owner)).to.equal(1n);
//       expect(await ctx.token.ownerOf(firstTokenId)).to.equal(ctx.owner);
//     });

//     it("reverts when adding a token id that already exists", async function () {
//       await expect(ctx.token.$_mint(ctx.owner, firstTokenId))
//         .to.be.revertedWithCustomError(ctx.token, "ERC721InvalidSender")
//         .withArgs(ethers.ZeroAddress);
//     });
//   });
// });

// describe("_burn", function () {
//   it("reverts when burning a non-existent token id", async function () {
//     await expect(ctx.token.$_burn(nonExistentTokenId))
//       .to.be.revertedWithCustomError(ctx.token, "ERC721NonexistentToken")
//       .withArgs(nonExistentTokenId);
//   });

//   describe("with minted tokens", function () {
//     beforeEach(async function () {
//       await ctx.token.$_mint(ctx.owner, firstTokenId);
//       await ctx.token.$_mint(ctx.owner, secondTokenId);
//     });

//     describe("with burnt token", function () {
//       beforeEach(async function () {
//         ctx.tx = await ctx.token.$_burn(firstTokenId);
//       });

//       it("emits a Transfer event", async function () {
//         await expect(ctx.tx)
//           .to.emit(ctx.token, "Transfer")
//           .withArgs(ctx.owner, ethers.ZeroAddress, firstTokenId);
//       });

//       it("deletes the token", async function () {
//         expect(await ctx.token.balanceOf(ctx.owner)).to.equal(1n);
//         await expect(ctx.token.ownerOf(firstTokenId))
//           .to.be.revertedWithCustomError(
//             ctx.token,
//             "ERC721NonexistentToken"
//           )
//           .withArgs(firstTokenId);
//       });

//       it("reverts when burning a token id that has been deleted", async function () {
//         await expect(ctx.token.$_burn(firstTokenId))
//           .to.be.revertedWithCustomError(
//             ctx.token,
//             "ERC721NonexistentToken"
//           )
//           .withArgs(firstTokenId);
//       });
//     });
//   });
// });

// export type shouldBehaveLikeERC721MetadataParams = {
//   token: any;
//   owner: KeyringPair;
// };

// function shouldBehaveLikeERC721Metadata(
//   getParams: () => shouldBehaveLikeERC721MetadataParams
// ) {
//   const ctx: shouldBehaveLikeERC721MetadataParams & {
//     tx: any;
//     newOwner: KeyringPair;
//     approved: KeyringPair;
//     operator: KeyringPair;
//     to: KeyringPair;
//     other: KeyringPair;
//   } = {} as any;

//   describe("with shouldBehaveLikeERC721Metadata tokens", function () {
//     beforeEach(function () {
//       Object.assign(ctx, getParams());
//       [ctx.newOwner, ctx.approved, ctx.operator, ctx.to, ctx.other] =
//         getSigners().filter((signer) => signer.address !== ctx.owner.address);
//     });

//     describe("token atribute", function () {
//       it("return none by default", async function () {
//         expect(
//           await ctx.token.query.getAtribute(firstTokenId, [])
//         ).to.haveOkResult(null);
//       });

//       it("reverts when queried for non existent token id", async function () {
//         await expect(ctx.token.tokenURI(nonExistentTokenId))
//           .to.be.revertedWithCustomError(ctx.token, "ERC721NonexistentToken")
//           .withArgs(nonExistentTokenId);
//       });

//       describe("base URI", function () {
//         beforeEach(function () {
//           if (!ctx.token.interface.hasFunction("setBaseURI")) {
//             ctx.skip();
//           }
//         });

//         it("base URI can be set", async function () {
//           await ctx.token.setBaseURI(baseURI);
//           expect(await ctx.token.baseURI()).to.equal(baseURI);
//         });

//         it("base URI is added as a prefix to the token URI", async function () {
//           await ctx.token.setBaseURI(baseURI);
//           expect(await ctx.token.tokenURI(firstTokenId)).to.equal(
//             baseURI + firstTokenId.toString()
//           );
//         });

//         it("token URI can be changed by changing the base URI", async function () {
//           await ctx.token.setBaseURI(baseURI);
//           const newBaseURI = "https://api.example.com/v2/";
//           await ctx.token.setBaseURI(newBaseURI);
//           expect(await ctx.token.tokenURI(firstTokenId)).to.equal(
//             newBaseURI + firstTokenId.toString()
//           );
//         });
//       });
//     });
//   });
// }
