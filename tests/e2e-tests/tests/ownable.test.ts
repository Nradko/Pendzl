import { ApiPromise } from '@polkadot/api';
import { BN } from 'bn.js';
import { getLocalApiProviderWrapper } from 'tests/setup/helpers';
import 'wookashwackomytest-polkahat-chai-matchers';
import { getSigners, increaseBlockTimestamp, shouldBehaveLikeOwnable, transferNoop } from 'wookashwackomytest-pendzl-tests';
import TOWnableDeployer from 'typechain/deployers/t_ownable';
import TOwnableContract from 'typechain/contracts/t_ownable';

const [deployer, owner, ...others] = getSigners();
describe.only('Ownable', () => {
  let tOwnable: TOwnableContract;
  let api;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);
  beforeEach(async () => {
    api = await apiProviderWrapper.getAndWaitForReady();
  });

  shouldBehaveLikeOwnable(() => ({ ownableDeployerCall: () => new TOWnableDeployer(api, deployer).new(owner.address), owner, other: others[0] }));

  //   describe('transferOwnership', function () {
  //     beforeEach(async function () {
  //       tOwnable = (await new TOwnableDeployer(api, owner).new(owner)).contract;
  //     });
  //       it('should behave like ERC20', async () => {
  //       tOwnable.query.tUpdateOwner
  //       });
  //   });
});
