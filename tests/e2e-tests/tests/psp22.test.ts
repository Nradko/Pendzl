import { ApiPromise } from '@polkadot/api';
import { BN } from 'bn.js';
import { expect } from 'chai';
import { getContractObjectWrapper, increaseBlockTimestamp, transferNoop } from 'tests/misc';
import { getLocalApiProviderWrapper, getSigners } from 'tests/setup/helpers';
import PSP22Constructor from 'typechain/constructors/my_psp22';
import PSP22 from 'typechain/contracts/my_psp22';
import 'wookashwackomytest-polkahat-chai-matchers';
import { shouldBehaveLikeERC20 } from 'wookashwackomytest-pendzl-tests';

const [owner, ...others] = getSigners();
const initialSupply = new BN(1000);
async function prepareEnvBase(api: ApiPromise) {
  await transferNoop(api);
  // to force using fake_time
  await increaseBlockTimestamp(api, 0);

  const deployRet = await new PSP22Constructor(api, owner).new(initialSupply);
  const myPSP22 = await getContractObjectWrapper(api, PSP22, deployRet.address, owner);

  return { myPSP22 };
}
describe.only('PSP 22', () => {
  let myPSP22: PSP22;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);
  beforeEach(async () => {
    const api = await apiProviderWrapper.getAndWaitForReady();
    const contracts = await prepareEnvBase(api);
    myPSP22 = contracts.myPSP22;
  });

  shouldBehaveLikeERC20(() => ({ initialSupply, holder: owner, recipient: others[0], other: others[1], token: myPSP22 as any }));
});
