import { ApiPromise } from "@polkadot/api";
import { getSigners } from "./signers";
import { transferNoop } from "./transferNoop/transferNoop";
import { ApiProviderWrapper } from "wookashwackomytest-polkahat-chai-matchers";
import "@polkadot/api-augment"; //https://github.com/polkadot-js/api/issues/4450
import { SignAndSendSuccessResponse } from "wookashwackomytest-typechain-types";
import durationImport from "./duration";
import { mapValues } from "./iterate";

export async function getApiAt(api: ApiPromise, blockNumber: number) {
  const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
  const apiAt = await api.at(blockHash);
  return apiAt;
}

export async function setBlockTimestamp(api: ApiPromise, timestamp: number) {
  const signer = getSigners()[0];
  if (process.env.DEBUG) console.log(`setting timestamp to: ${timestamp}`);
  await transferNoop(api);
  await api.tx.timestamp.setTime(timestamp).signAndSend(signer, {});
  await transferNoop(api);
  const timestampNowPostChange = parseInt(
    (await api.query.timestamp.now()).toString()
  );
  if (timestampNowPostChange !== timestamp)
    throw new Error("Failed to set custom timestamp");
}
export async function increaseBlockTimestamp(
  api: ApiPromise,
  deltaTimestamp: number
): Promise<number> {
  const timestampNow = await api.query.timestamp.now();
  const timestampToSet = parseInt(timestampNow.toString()) + deltaTimestamp;
  if (process.env.DEBUG)
    console.log(`increasing timestamp by ${deltaTimestamp}`);
  await setBlockTimestamp(api, timestampToSet);
  const timestampNowPostChange = parseInt(
    (await api.query.timestamp.now()).toString()
  );
  if (timestampNowPostChange !== timestampToSet)
    throw new Error("Failed to set custom timestamp");
  return timestampToSet;
}

export const getApiProviderWrapper = (port: number) =>
  new ApiProviderWrapper(`ws://127.0.0.1:${port}`);
const apiProvider = getApiProviderWrapper(9944);

export const time = {
  latest: async () => {
    return (
      await (await apiProvider.getAndWaitForReady()).query.timestamp.now()
    ).toNumber();
  },
  latestBlock: async () => {
    const block = await (
      await apiProvider.getAndWaitForReady()
    ).rpc.chain.getBlock();
    return block.block.header.number.toNumber();
  },
  increase: async (delta: number) =>
    increaseBlockTimestamp(await apiProvider.getAndWaitForReady(), delta),
  setTo: async (timestamp: number) =>
    setBlockTimestamp(await apiProvider.getAndWaitForReady(), timestamp),
  duration: durationImport,
};

export const clock = {
  blocknumber: () => time.latestBlock(),
  timestamp: () => time.latest(),
  fromTx: {
    blocknumber: async (tx: SignAndSendSuccessResponse) => {
      const block = await (
        await apiProvider.getAndWaitForReady()
      ).rpc.chain.getBlock(tx.blockHash);
      return block.block.header.number.toNumber();
    },
    timestamp: async (tx: SignAndSendSuccessResponse) => {
      const api = await apiProvider.getAndWaitForReady();
      const block = await api.rpc.chain.getBlock(tx.blockHash);
      const postTxBlockNumber = block.block.header.number.toNumber();
      const preTxBlockNumber = postTxBlockNumber - 1;
      const apiAt = await getApiAt(api, preTxBlockNumber);
      const timestamp = await apiAt.query.timestamp.now();
      return timestamp.toNumber();
    },
  },
};

export const duration = mapValues(
  time.duration,
  (fn) => (n: number) => fn(n)
) as Record<keyof typeof durationImport, (n: number) => number>;
