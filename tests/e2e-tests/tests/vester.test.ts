import { ApiPromise } from '@polkadot/api';
import BN from 'bn.js';
import { expect } from 'chai';
import { getLocalApiProviderWrapper } from 'tests/setup/helpers';
import TPsp22Contract from 'typechain/contracts/t_psp22';
import TVesterContract from 'typechain/contracts/t_vester';
import TPsp22Deployer from 'typechain/deployers/t_psp22';
import TVesterDeployer from 'typechain/deployers/t_vester';
import { VestingSchedule } from 'typechain/types-arguments/t_vester';
import { getSigners, increaseBlockTimestamp, time, transferNoop } from 'wookashwackomytest-pendzl-tests';
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
    await increaseBlockTimestamp(api, 0);
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
      const tx = ctx.mock
        .withSigner(vesterSubmitter)
        .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []);
      await expect(tx).to.changeTokenBalances(
        asset,
        [vesterSubmitter.address, ctx.mock.address],
        [new BN(createVestArgs.amount).neg(), new BN(createVestArgs.amount)],
      );
      await expect(tx).to.emitEvent(ctx.mock, 'VestingScheduled', {
        creator: vesterSubmitter.address,
        asset: createVestArgs.asset,
        receiver: createVestArgs.vestTo,
        amount: createVestArgs.amount,
        schedule: createVestArgs.schedule,
      });
      await expect(tx).to.emitEvent(asset, 'Transfer', {
        from: vesterSubmitter.address,
        to: ctx.mock.address,
        value: createVestArgs.amount,
      });
    });

    describe('release', () => {
      beforeEach(async () => {
        await asset.tx.tMint(vesterSubmitter.address, createVestArgs.amount);
        await asset.withSigner(vesterSubmitter).tx.increaseAllowance(ctx.mock.address, createVestArgs.amount);
        await ctx.mock
          .withSigner(vesterSubmitter)
          .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []);
        // await time.increase(
        //   parseInt(createVestArgs.schedule.constant![0].toString()) + parseInt(createVestArgs.schedule.constant![1].toString()) + 1,
        // );
      });

      it.only('anyone can release', async function () {
        await expect(ctx.mock.withSigner(charlie).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
        await expect(ctx.mock.withSigner(alice).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
        await expect(ctx.mock.withSigner(bob).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
      });
      it('should release tokens correctly', async function () {
        const tx = ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
        await expect(tx).to.emitEvent(asset, 'Transfer', {
          from: ctx.mock.address,
          to: createVestArgs.vestTo,
          value: createVestArgs.amount,
        });

        await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
          receiver: createVestArgs.vestTo,
          asset: createVestArgs.asset,
          amount: createVestArgs.amount,
        });
        await expect(tx).to.changeTokenBalances(
          asset,
          [ctx.mock.address, createVestArgs.vestTo],
          [new BN(createVestArgs.amount).neg(), new BN(createVestArgs.amount)],
        );
      });
    });
  });

  describe('vesting native token', () => {
    const vesterSubmitter = alice;
    let createVestArgs: CreateVestingScheduleArgs;
    const amount = 10_000_000; // Adjusted to match the Rust test's amount
    beforeEach(async () => {
      createVestArgs = createDurationAsAmountScheduleArgs(charlie.address, null, 0, amount); // asset is null for native token
    });

    it('should fail to create vesting schedule due to invalid amount paid (less than required)', async function () {
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
            value: createVestArgs.amount - 1,
          }),
      ).to.be.revertedWithError({ invalidAmountPaid: null });
    });

    it('should fail to create vesting schedule due to invalid amount paid (more than required)', async function () {
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
            value: createVestArgs.amount + 1,
          }),
      ).to.be.revertedWithError({ invalidAmountPaid: null });
    });

    it('should successfully create a vesting schedule with the exact amount paid', async function () {
      const tx = ctx.mock
        .withSigner(vesterSubmitter)
        .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
          value: createVestArgs.amount,
        });

      await expect(tx).to.emitNativeEvent('Transfer', {
        from: vesterSubmitter.address,
        to: ctx.mock.address,
        amount: createVestArgs.amount,
      });
      await expect(tx).to.emitEvent(ctx.mock, 'VestingScheduled', {
        creator: vesterSubmitter.address,
        asset: createVestArgs.asset,
        receiver: createVestArgs.vestTo,
        amount: createVestArgs.amount,
        schedule: createVestArgs.schedule,
      });
      await expect(tx).to.changeBalances([ctx.mock.address], [new BN(createVestArgs.amount)]);
    });
  });
});
