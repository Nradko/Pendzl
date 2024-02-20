import type Keyring from "@polkadot/keyring";
import { createTestKeyring } from "@polkadot/keyring/testing";
import type { KeyringPair } from "@polkadot/keyring/types";
import { mnemonicGenerate } from "@polkadot/util-crypto";

const testKeyring = createTestKeyring({ type: "sr25519" });
export const getSigners = () => {
  return testKeyring.pairs;
};
export const getSignersWithoutOwner = (
  signers: KeyringPair[],
  ownerIndex: number
) => [...signers.slice(0, ownerIndex), ...signers.slice(ownerIndex + 1)];
export function converSignerToAddress(signer?: KeyringPair | string): string {
  if (!signer) return "";
  return typeof signer !== "string" ? signer.address : signer;
}
export function getRandomSigner() {
  const mnemonic = mnemonicGenerate();
  const pair = testKeyring.addFromUri(mnemonic, {}, "sr25519");

  return { pair, mnemonic };
}
