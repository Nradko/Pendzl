import { ApiPromise } from '@polkadot/api';
import BN from 'bn.js';
import { expect } from 'chai';
import { getLocalApiProviderWrapper } from 'tests/setup/helpers';
import TPsp22Contract from 'typechain/contracts/t_psp22';
import TVesterContract from 'typechain/contracts/t_vester';
import TPsp22Deployer from 'typechain/deployers/t_psp22';
import TVesterDeployer from 'typechain/deployers/t_vester';
import { VestingSchedule } from 'typechain/types-arguments/t_vester';
import { getSigners } from 'wookashwackomytest-pendzl-tests';
import 'wookashwackomytest-polkahat-chai-matchers';

const [deployer, alice, bob, charlie] = getSigners();
describe.only('Vester', () => {
  const ctx: {
    mock: TVesterContract;
  } = {} as any;

  let api: ApiPromise;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);
  beforeEach(async () => {
    api = await apiProviderWrapper.getAndWaitForReady();
    const mock = await new TVesterDeployer(api, deployer).new();
    ctx.mock = mock.contract;
  });

  interface CreateVestingScheduleArgs {
    vestTo: string;
    asset: string | null;
    amount: number;
    schedule: VestingSchedule;
  }

  function createDurationAsAmountScheduleArgs(
    vestTo: string,
    asset: string | null,
    waiting_duration: number,
    duration: number,
  ): CreateVestingScheduleArgs {
    return {
      vestTo,
      asset,
      amount: duration,
      schedule: {
        constant: [waiting_duration, duration],
      },
    };
  }

  describe('vesting psp22', () => {
    const vesterSubmitter = alice;
    let asset: TPsp22Contract;
    let createVestArgs: CreateVestingScheduleArgs;
    const initialSupply = new BN(1000);
    const TOKEN_NAME = 'SomeToken';
    const SYBOL = 'ST';
    const DECIMALS = 18;
    beforeEach(async () => {
      asset = (await new TPsp22Deployer(api, deployer).new(initialSupply, TOKEN_NAME, SYBOL, DECIMALS)).contract;
      createVestArgs = createDurationAsAmountScheduleArgs(charlie.address, asset.address, 0, 100);
    });
    it('should fail to create vesting schedule due to insufficient allowance', async function () {
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []),
      ).to.be.revertedWithError({
        psp22Error: { insufficientAllowance: null },
      });
    });

    it('should fail to create vesting schedule due to insufficient balance after allowance', async function () {
      await asset.withSigner(vesterSubmitter).tx.increaseAllowance(ctx.mock.address, createVestArgs.amount);
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []),
      ).to.be.revertedWithError({
        psp22Error: { insufficientBalance: null },
      });
    });

    it('should successfully create a vesting schedule after minting', async function () {
      await asset.tx.tMint(vesterSubmitter.address, createVestArgs.amount);
      await asset.withSigner(vesterSubmitter).tx.increaseAllowance(ctx.mock.address, createVestArgs.amount);
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []),
      ).to.emitEvent(ctx.mock, 'VestingScheduled', {
        creator: vesterSubmitter.address,
        asset: createVestArgs.asset,
        receiver: createVestArgs.vestTo,
        amount: createVestArgs.amount,
        schedule: createVestArgs.schedule as any,
      });
    });
  });

  describe('vesting native token', () => {
    // TODO
  });
});
