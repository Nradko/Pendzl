import { ApiPromise } from '@polkadot/api';
import { getLocalApiProviderWrapper } from 'tests/setup/helpers';
import TAccessControlContract from 'typechain/contracts/t_access_control';
import TAccessControlDeployer from 'typechain/deployers/t_access_control';
import { getSigners, shouldBehaveLikeAccessControl } from 'wookashwackomytest-pendzl-tests';
import 'wookashwackomytest-polkahat-chai-matchers';

const [defaultAdmin, ...others] = getSigners();

describe.only('AccessControl', () => {
  let api: ApiPromise;
  let accessControlMock: TAccessControlContract;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);
  beforeEach(async () => {
    api = await apiProviderWrapper.getAndWaitForReady();
    accessControlMock = (await new TAccessControlDeployer(api, defaultAdmin).new()).contract;
  });

  shouldBehaveLikeAccessControl(() => ({
    mock: accessControlMock,
    accounts: others,
    defaultAdmin,
  }));
});
