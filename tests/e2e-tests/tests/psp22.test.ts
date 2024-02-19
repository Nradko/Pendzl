import { ApiPromise } from '@polkadot/api';
import { BN } from 'bn.js';
import { getLocalApiProviderWrapper } from 'tests/setup/helpers';
import { getSigners, shouldBehaveLikeERC20 } from 'wookashwackomytest-pendzl-tests';
import TPsp22Deployer from 'typechain/deployers/t_psp22';
import TPsp22Contract from 'typechain/contracts/t_psp22';
import type { KeyringPair } from '@polkadot/keyring/types';
import { MAX_U128 } from 'wookashwackomytest-polkahat-chai-matchers';
import 'wookashwackomytest-polkahat-chai-matchers';
import { expect } from 'chai';

const [owner, ...others] = getSigners();
const initialSupply = new BN(1000);
const TOKEN_NAME = 'SomeToken';
const SYBOL = 'ST';
const DECIMALS = 18;
async function prepareEnvBase(api: ApiPromise) {
  const deployRet = await new TPsp22Deployer(api, owner).new(initialSupply, TOKEN_NAME, SYBOL, DECIMALS);

  return { tPSP22: deployRet.contract };
}
describe('PSP 22', () => {
  const ctx: { token: TPsp22Contract; recipient: KeyringPair } = {} as any;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);

  beforeEach(async () => {
    const api = await apiProviderWrapper.getAndWaitForReady();
    const contracts = await prepareEnvBase(api);
    ctx.token = contracts.tPSP22;
    ctx.recipient = others[0];
  });

  shouldBehaveLikeERC20(() => ({ initialSupply, holder: owner, recipient: others[0], other: others[1], token: ctx.token }));

  describe('metadata', () => {
    it('has a name', async function () {
      expect(await ctx.token.query.tokenName()).to.haveOkResult(TOKEN_NAME);
    });

    it('has a symbol', async function () {
      expect(await ctx.token.query.tokenSymbol()).to.equal(SYBOL);
    });

    it('has 18 decimals', async function () {
      expect(await ctx.token.query.tokenDecimals()).to.haveOkResult(18);
    });
  });
  // describe('_mint', function () {
  //   const value = 50;
  //   // it('rejects a null account', async function () {
  //   //   await expect(ctx.token.tMint(ethers.ZeroAddress, value))
  //   //     .to.be.revertedWithCustomError(ctx.token, 'ERC20InvalidReceiver')
  //   //     .withArgs(ethers.ZeroAddress);
  //   // });

  //   it('rejects overflow', async function () {
  //     await expect(ctx.token.query.tMint(ctx.recipient.address, MAX_U128)).to.be.revertedWithPanic(PANIC_CODES.ARITHMETIC_UNDER_OR_OVERFLOW);
  //   });

  //   describe('for a non zero account', function () {
  //     beforeEach('minting', async function () {
  //       ctx.tx = await ctx.token.tMint(ctx.recipient, value);
  //     });

  //     it('increments totalSupply', async function () {
  //       await expect(await ctx.token.totalSupply()).to.equal(initialSupply + value);
  //     });

  //     it('increments recipient balance', async function () {
  //       await expect(ctx.tx).to.changeTokenBalance(ctx.token, ctx.recipient, value);
  //     });

  //     it('emits Transfer event', async function () {
  //       await expect(ctx.tx).to.emit(ctx.token, 'Transfer').withArgs(ethers.ZeroAddress, ctx.recipient, value);
  //     });
  //   });
  // });

  // describe('_burn', function () {
  //   it('rejects a null account', async function () {
  //     await expect(ctx.token.$_burn(ethers.ZeroAddress, 1n))
  //       .to.be.revertedWithCustomError(ctx.token, 'ERC20InvalidSender')
  //       .withArgs(ethers.ZeroAddress);
  //   });

  //   describe('for a non zero account', function () {
  //     it('rejects burning more than balance', async function () {
  //       await expect(ctx.token.$_burn(ctx.holder, initialSupply + 1n))
  //         .to.be.revertedWithCustomError(ctx.token, 'ERC20InsufficientBalance')
  //         .withArgs(ctx.holder, initialSupply, initialSupply + 1n);
  //     });

  //     const describeBurn = function (description, value) {
  //       describe(description, function () {
  //         beforeEach('burning', async function () {
  //           ctx.tx = await ctx.token.$_burn(ctx.holder, value);
  //         });

  //         it('decrements totalSupply', async function () {
  //           expect(await ctx.token.totalSupply()).to.equal(initialSupply - value);
  //         });

  //         it('decrements holder balance', async function () {
  //           await expect(ctx.tx).to.changeTokenBalance(ctx.token, ctx.holder, -value);
  //         });

  //         it('emits Transfer event', async function () {
  //           await expect(ctx.tx).to.emit(ctx.token, 'Transfer').withArgs(ctx.holder, ethers.ZeroAddress, value);
  //         });
  //       });
  //     };

  //     describeBurn('for entire balance', initialSupply);
  //     describeBurn('for less value than balance', initialSupply - 1n);
  //   });
  // });

  // describe('_update', function () {
  //   const value = 1n;

  //   beforeEach(async function () {
  //     ctx.totalSupply = await ctx.token.totalSupply();
  //   });

  //   it('from is the zero address', async function () {
  //     const tx = await ctx.token.$_update(ethers.ZeroAddress, ctx.holder, value);
  //     await expect(tx).to.emit(ctx.token, 'Transfer').withArgs(ethers.ZeroAddress, ctx.holder, value);

  //     expect(await ctx.token.totalSupply()).to.equal(ctx.totalSupply + value);
  //     await expect(tx).to.changeTokenBalance(ctx.token, ctx.holder, value);
  //   });

  //   it('to is the zero address', async function () {
  //     const tx = await ctx.token.$_update(ctx.holder, ethers.ZeroAddress, value);
  //     await expect(tx).to.emit(ctx.token, 'Transfer').withArgs(ctx.holder, ethers.ZeroAddress, value);

  //     expect(await ctx.token.totalSupply()).to.equal(ctx.totalSupply - value);
  //     await expect(tx).to.changeTokenBalance(ctx.token, ctx.holder, -value);
  //   });

  //   describe('from and to are the same address', function () {
  //     it('zero address', async function () {
  //       const tx = await ctx.token.$_update(ethers.ZeroAddress, ethers.ZeroAddress, value);
  //       await expect(tx).to.emit(ctx.token, 'Transfer').withArgs(ethers.ZeroAddress, ethers.ZeroAddress, value);

  //       expect(await ctx.token.totalSupply()).to.equal(ctx.totalSupply);
  //       await expect(tx).to.changeTokenBalance(ctx.token, ethers.ZeroAddress, 0n);
  //     });

  //     describe('non zero address', function () {
  //       it('reverts without balance', async function () {
  //         await expect(ctx.token.$_update(ctx.recipient, ctx.recipient, value))
  //           .to.be.revertedWithCustomError(ctx.token, 'ERC20InsufficientBalance')
  //           .withArgs(ctx.recipient, 0n, value);
  //       });

  //       it('executes with balance', async function () {
  //         const tx = await ctx.token.$_update(ctx.holder, ctx.holder, value);
  //         await expect(tx).to.changeTokenBalance(ctx.token, ctx.holder, 0n);
  //         await expect(tx).to.emit(ctx.token, 'Transfer').withArgs(ctx.holder, ctx.holder, value);
  //       });
  //     });
  //   });
  // });

  // describe('_transfer', function () {
  //   beforeEach(function () {
  //     ctx.transfer = ctx.token.$_transfer;
  //   });

  //   shouldBehaveLikeERC20Transfer(initialSupply);

  //   // it('reverts when the sender is the zero address', async function () {
  //   //   await expect(ctx.token.$_transfer(ethers.ZeroAddress, ctx.recipient, initialSupply))
  //   //     .to.be.revertedWithCustomError(ctx.token, 'ERC20InvalidSender')
  //   //     .withArgs(ethers.ZeroAddress);
  //   // });
  // });

  // describe('_approve', function () {
  //   beforeEach(function () {
  //     ctx.approve = ctx.token.$_approve;
  //   });

  //   shouldBehaveLikeERC20Approve(initialSupply);

  //   it('reverts when the owner is the zero address', async function () {
  //     await expect(ctx.token.$_approve(ethers.ZeroAddress, ctx.recipient, initialSupply))
  //       .to.be.revertedWithCustomError(ctx.token, 'ERC20InvalidApprover')
  //       .withArgs(ethers.ZeroAddress);
  //   });
  // });
});
