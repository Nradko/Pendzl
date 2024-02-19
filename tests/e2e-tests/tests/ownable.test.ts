import { ApiPromise } from '@polkadot/api';
import { BN } from 'bn.js';
import { getLocalApiProviderWrapper } from 'tests/setup/helpers';
import 'wookashwackomytest-polkahat-chai-matchers';
import { getSigners, increaseBlockTimestamp, shouldBehaveLikeOwnable, transferNoop } from 'wookashwackomytest-pendzl-tests';
import TOwnableDeployer from 'typechain/deployers/t_ownable';
import TOwnableContract from 'typechain/contracts/t_ownable';
import { expect } from 'chai';

const [deployer, owner, ...others] = getSigners();
describe.only('Ownable', () => {
  let tOwnable: TOwnableContract;
  let api;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);
  beforeEach(async () => {
    api = await apiProviderWrapper.getAndWaitForReady();
  });

  shouldBehaveLikeOwnable(() => ({ ownableDeployerCall: () => new TOwnableDeployer(api, deployer).new(owner.address), owner, other: others[0] }));

  describe('OwnableInteral', function () {
    beforeEach(async function () {
      tOwnable = (await new TOwnableDeployer(api, owner).new(owner.address)).contract;
    });

    describe('_update_owner', function () {
      it('emits event and updates owner', async () => {
        await tOwnable.withSigner(owner).tx.renounceOwnership();
        expect((await tOwnable.query.owner()).value.ok).to.equal(null);

        await expect(tOwnable.tx.tUpdateOwner(owner.address)).to.emitEvent(tOwnable, 'OwnershipTransferred', {
          new: owner.address,
        });
        expect((await tOwnable.query.owner()).value.ok).to.equal(owner.address);
      });
    });

    describe('_only_owner', function () {
      it('reverts if not owner', async () => {
        await expect(tOwnable.withSigner(others[0]).query.tOnlyOwner()).to.be.revertedWithError('CallerIsNotOwner');
      });

      it('does pass if owner', async () => {
        await expect(tOwnable.withSigner(owner).tx.tOnlyOwner()).to.be.eventually.fulfilled;
      });
    });
  });
});
