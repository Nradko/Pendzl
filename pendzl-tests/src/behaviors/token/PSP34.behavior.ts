import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import "wookashwackomytest-polkahat-chai-matchers";
import { SignAndSendSuccessResponse } from "wookashwackomytest-typechain-types";
import { getSigners } from "../../helpers/signers";
import { BN } from "bn.js";

const firstTokenId = 5042;
const secondTokenId = 79217;
const nonExistentTokenId = 13;

export type ShouldBehaveLikePSP34Params = {
  token: any;
  owner: KeyringPair;
};

function shouldBehaveLikeERC721(getParams: () => ShouldBehaveLikePSP34Params) {
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
        `to use shouldBehaveLikeERC721 test you should set ownership of ${firstTokenId} to owner`
      ).to.haveOkResult(ctx.owner.address);
      await expect(
        ctx.token.query.ownerOf(firstTokenId),
        `to use shouldBehaveLikeERC721 test you should set ownership of ${secondTokenId} to owner`
      ).to.haveOkResult(ctx.owner.address);

      await expect(
        ctx.token.query.ownerOf(nonExistentTokenId),
        `to use shouldBehaveLikeERC721 test  ${nonExistentTokenId} should not exist`
      ).to.haveOkResult(null);

      await expect(
        ctx.token.query.totalSupply(),
        `to use shouldBehaveLikeERC721 test only ${firstTokenId} and ${secondTokenId} should exist. The total_supply should be 2.`
      ).to.haveOkResult(2);
    });

    describe("balanceOf", function () {
      describe(`when the given address owns ${firstTokenId} and ${secondTokenId} tokens`, function () {
        it("returns the amount of tokens owned by the given address", async function () {
          expect(await ctx.token.query.balanceOf()).to.haveOkResult(2);
        });
      });

      describe("when the given address does not own any tokens", function () {
        it("returns 0", async function () {
          expect(await ctx.token.balanceOf(ctx.other)).to.haveOkResult(0);
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

        it("reverts", async function () {
          await expect(
            ctx.token.query.ownerOf(tokenId)
          ).to.be.revertedWithError("TokenNotExists");
        });
      });
    });

    describe("transfers", function () {
      const tokenId = firstTokenId;
      const data = "0x42";

      for (const { fnName, opts } of [
        { fnName: "transfer", opts: {} },
        { fnName: "t_transfer", opts: {} },
      ]) {
        describe(`via ${fnName}`, function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.owner)
              .approve(ctx.approved, tokenId, true);
            await ctx.token
              .withSigner(ctx.owner)
              .approve(ctx.operator, null, true);
            ctx.tx = () =>
              ctx.token
                .withSigner(ctx.owner)
                .tx.transfer(ctx.to.address, tokenId, data);
          });
          shouldTransferTokensByUsers(
            fnName,
            ctx.token,
            ctx.owner,
            ctx.approved!,
            ctx.operator!,
            ctx.other,
            ctx.to.address,
            firstTokenId,
            nonExistentTokenId,
            opts
          );
        });
      }

      describe("approve", function () {
        const tokenId = firstTokenId;

        describe("when clearing approval", function () {
          describe("when there was no prior approval", function () {
            beforeEach(async function () {
              ctx.tx = ctx.token
                .withSigner(ctx.owner)
                .tx.approve(ctx.approved, tokenId, false);
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
                .tx.approve(ctx.approved, tokenId, true);
              ctx.tx = ctx.token
                .withSigner(ctx.owner)
                .tx.approve(ctx.approved, tokenId, false);
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
                .tx.approve(ctx.approved, tokenId, true);
              ctx.tx = ctx.token
                .withSigner(ctx.owner)
                .tx.approve(ctx.approved, tokenId, true);
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
                .approve(ctx.approved, tokenId, true)
            ).to.be.revertedWithError("NotApprovedcaller");
          });
        });

        describe("when the sender is approved for the given token ID", function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.owner)
              .approve(ctx.approved, tokenId, true);
          });
          it("reverts", async function () {
            await expect(
              ctx.token.withSigner(ctx.approved).approve(ctx.other, tokenId)
            ).to.be.revertedWithError("NotApproved");
          });
        });

        describe("when the sender is an operator", function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.operator, null, true);
          });

          it("reverts", async function () {
            await expect(
              ctx.token
                .withSigner(ctx.operator)
                .query.approve(ctx.approved, tokenId)
            ).to.be.revertedWithError("NotApproved");
          });
        });
      });
    });
  });
}

export async function transferWasSuccessful(
  tx: () => Promise<SignAndSendSuccessResponse>,
  token: any,
  from: string,
  to: string,
  tokenId: number
) {
  it("transfers the ownership of the given token ID to the given address", async function () {
    await tx();
    expect(await token.query.ownerOf(tokenId)).to.equal(to);
  });

  it("emits a Transfer event", async function () {
    await expect(tx()).to.emitEvent(token, "Transfer", {
      from: from,
      to: to,
      id: tokenId,
    });
  });

  it("clears the approval for the token ID with no event", async function () {
    await expect(tx()).not.to.emitEvent(token, "Approval");

    expect(await token.query.allowance(from, to, tokenId)).to.equal(null);
  });

  it("adjusts owners and receiver balances", async function () {
    if (from === to) {
      expect(tx).to.changeTokenBalances(token, [from], [new BN(0)]);
    } else {
      await expect(tx).to.changeTokenBalances(
        token,
        [from, to],
        [new BN(-1), new BN(1)]
      );
    }
  });
}

export async function shouldTransferTokensByUsers(
  fnName: string,
  token: any,
  owner: KeyringPair,
  approved: KeyringPair,
  operator: KeyringPair,
  notApproved: KeyringPair,
  to: string,
  tokenId: number,
  nonExistentTokenId: number,
  opts: any
) {
  describe("when called by the owner", function () {
    const tx = () =>
      token.withSigner(owner)[fnName](to, tokenId, ...(opts.extra ?? []));
    transferWasSuccessful(tx, token, owner.address, to, tokenId);
  });

  describe("when called by the approved individual", function () {
    const tx = () =>
      token.withSigner(approved)[fnName](to, tokenId, ...(opts.extra ?? []));
    transferWasSuccessful(tx, token, owner.address, to, tokenId);
  });

  describe("when called by the operator", function () {
    const tx = () =>
      token.withSigner(operator)[fnName](to, tokenId, ...(opts.extra ?? []));
    transferWasSuccessful(tx, token, owner.address, to, tokenId);
  });

  describe("when sent to the owner", function () {
    let tx: () => Promise<SignAndSendSuccessResponse>;
    beforeEach(async function () {
      tx = () =>
        token
          .withSigner(owner)
          [fnName](owner.address, tokenId, ...(opts.extra ?? []));
    });

    it("keeps ownership of the token", async function () {
      await tx();
      expect(await token.ownerOf(tokenId)).to.equal(owner.address);
    });

    it("emits only a transfer event", async function () {
      await expect(tx()).to.emitEvent(token, "Transfer", {
        from: owner.address,
        to: owner.address,
        id: tokenId,
      });
    });

    it("keeps the owner balance", async function () {
      expect(tx).to.changeTokenBalances(token, [owner.address], [new BN(0)]);
    });
  });

  describe("when the sender is not authorized for the token id", function () {
    it("reverts", async function () {
      await expect(
        token
          .withSigner(notApproved)
          [fnName](to, tokenId, ...(opts.extra ?? []))
      ).to.be.revertedWithError("NotApproved");
    });
  });

  describe("when the given token ID does not exist", function () {
    it("reverts", async function () {
      await expect(
        token
          .withSigner(owner)
          [fnName](to, nonExistentTokenId, ...(opts.extra ?? []))
      ).to.be.revertedWithError("TokenNotExists");
    });
  });
}

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
