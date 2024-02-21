// import { ApiPromise } from '@polkadot/api';
// import { expect } from 'chai';
// import { getLocalApiProviderWrapper } from 'wookashwackomytest-polkahat-network-helpers';
// import TPSP34MetadataContract from 'typechain/contracts/t_psp34_metadata';
// import TPSP34MetadataDeployer from 'typechain/deployers/t_psp34_metadata';
// import { getSigners, shouldBehaveLikeOwnable } from 'wookashwackomytest-pendzl-tests';
// import { firstTokenId, secondTokenId, shouldBehaveLikePSP34 } from 'wookashwackomytest-pendzl-tests/src/behaviors/token/PSP34.behavior';
// import 'wookashwackomytest-polkahat-chai-matchers';

// const [deployer, owner, ...others] = getSigners();
// describe.only('PSP34', () => {
//   let tPSP34: TPSP34MetadataContract;
//   let api: ApiPromise;
//   const apiProviderWrapper = getLocalApiProviderWrapper(9944);
//   beforeEach(async () => {
//     api = await apiProviderWrapper.getAndWaitForReady();
//     tPSP34 = (await new TPSP34MetadataDeployer(api, deployer).new('', '')).contract;
//     await tPSP34.withSigner(owner).tx.tMint(owner.address, firstTokenId);
//     await tPSP34.withSigner(owner).tx.tMint(owner.address, secondTokenId);
//   });

//   shouldBehaveLikePSP34(() => ({
//     token: tPSP34 as any,
//     owner,
//   }));

//   // describe('OwnableInteral', function () {
//   //   beforeEach(async function () {
//   //     tOwnable = (await new TOwnableDeployer(api, owner).new(owner.address)).contract;
//   //   });

//   //   describe('_update_owner', function () {
//   //     it('emits event and updates owner', async () => {
//   //       await tOwnable.withSigner(owner).tx.renounceOwnership();
//   //       expect((await tOwnable.query.owner()).value.ok).to.equal(null);

//   //       await expect(tOwnable.tx.tUpdateOwner(owner.address)).to.emitEvent(tOwnable, 'OwnershipTransferred', {
//   //         new: owner.address,
//   //       });
//   //       expect((await tOwnable.query.owner()).value.ok).to.equal(owner.address);
//   //     });
//   //   });

//   //   describe('_only_owner', function () {
//   //     it('reverts if not owner', async () => {
//   //       await expect(tOwnable.withSigner(others[0]).query.tOnlyOwner()).to.be.revertedWithError('CallerIsNotOwner');
//   //     });

//   //     it('does pass if owner', async () => {
//   //       await expect(tOwnable.withSigner(owner).tx.tOnlyOwner()).to.be.eventually.fulfilled;
//   //     });
//   //   });
//   // });
// });
