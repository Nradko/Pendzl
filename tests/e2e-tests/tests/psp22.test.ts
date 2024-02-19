import { ApiPromise } from '@polkadot/api';
import { BN } from 'bn.js';
import { getLocalApiProviderWrapper } from 'tests/setup/helpers';
import 'wookashwackomytest-polkahat-chai-matchers';
import { getSigners, shouldBehaveLikeERC20 } from 'wookashwackomytest-pendzl-tests';
import TPsp22Deployer from 'typechain/deployers/t_psp22';
import TPsp22Contract from 'typechain/contracts/t_psp22';

const [owner, ...others] = getSigners();
const initialSupply = new BN(1000);
async function prepareEnvBase(api: ApiPromise) {
  const deployRet = await new TPsp22Deployer(api, owner).new(initialSupply);

  return { tPSP22: deployRet.contract };
}
describe('PSP 22', () => {
  let tPSP22: TPsp22Contract;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);
  beforeEach(async () => {
    const api = await apiProviderWrapper.getAndWaitForReady();
    const contracts = await prepareEnvBase(api);
    tPSP22 = contracts.tPSP22;
  });

  shouldBehaveLikeERC20(() => ({ initialSupply, holder: owner, recipient: others[0], other: others[1], token: tPSP22 }));
});
