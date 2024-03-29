import type { Abi, ContractPromise } from "@polkadot/api-contract";
import type { ApiPromise } from "@polkadot/api";
import type {
  EventDataTypeDescriptions,
  Result,
  SignAndSendSuccessResponse,
} from "@c-forge/typechain-types";
import type { QueryReturnType } from "@c-forge/typechain-types";
import type BN from "bn.js";
import type { KeyringPair } from "@polkadot/keyring/types";
import {
  AccountId,
  LangError,
  PSP22Error,
} from "@c-forge/polkahat-chai-matchers";
import { ContractOptions } from "@polkadot/api-contract/types";

export interface Id {
  u8?: BN;
  u16?: BN;
  u32?: BN;
  u64?: BN;
  u128?: BN;
  bytes?: Array<BN>;
}

export interface PSP34Error {
  custom?: string;
  selfApprove?: null;
  notApproved?: null;
  tokenExists?: null;
  tokenNotExists?: null;
  safeTransferCheckFailed?: string;
}

export interface PSP34Query {
  collectionId(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Id, LangError>>>;
  balanceOf(
    owner: AccountId,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
  ownerOf(
    id: Id,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<AccountId | null, LangError>>>;
  allowance(
    owner: AccountId,
    operator: AccountId,
    id: Id | null,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<boolean, LangError>>>;
  approve(
    operator: AccountId,
    id: Id | null,
    approved: boolean,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP34Error>, LangError>>>;
  transfer(
    to: AccountId,
    id: Id,
    data: Array<number | string | BN>,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP34Error>, LangError>>>;
  totalSupply(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
}

export interface PSP34Tx {
  collectionId(
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  balanceOf(
    owner: AccountId,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  ownerOf(
    id: Id,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  allowance(
    owner: AccountId,
    operator: AccountId,
    id: Id | null,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  approve(
    operator: AccountId,
    id: Id | null,
    approved: boolean,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  transfer(
    to: AccountId,
    id: Id,
    data: Array<number | string | BN>,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  totalSupply(__options?: ContractOptions): Promise<SignAndSendSuccessResponse>;
}

export interface PSP34 {
  readonly query: PSP34Query;
  readonly tx: PSP34Tx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => PSP34;
  withAddress: (address: string) => PSP34;
  withAPI: (api: ApiPromise) => PSP34;
}
