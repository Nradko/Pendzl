import type { Abi, ContractPromise } from "@polkadot/api-contract";
import type { ApiPromise } from "@polkadot/api";
import type {
  EventDataTypeDescriptions,
  GasLimit,
  Result,
  ReturnNumber,
  SignAndSendSuccessResponse,
} from "wookashwackomytest-typechain-types";
import type { QueryReturnType } from "wookashwackomytest-typechain-types";
import type BN from "bn.js";
import type { KeyringPair } from "@polkadot/keyring/types";
import {
  AccountId,
  LangError,
  PSP22Error,
} from "wookashwackomytest-polkahat-chai-matchers";
import { Id, PSP34Error } from "./PSP34.type";

export interface PSP34InternalQuery {
  tMint(
    to: AccountId,
    id: Id,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP34Error>, LangError>>>;
  tBurn(
    from: AccountId,
    id: Id,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP34Error>, LangError>>>;
  tTransfer(
    from: AccountId,
    to: AccountId,
    id: Id,
    data: Array<number | string | BN>,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP34Error>, LangError>>>;
  tUpdate(
    from: AccountId | null,
    to: AccountId | null,
    id: Id,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP34Error>, LangError>>>;
}

export interface PSP34InternalTx {
  tMint(
    to: AccountId,
    id: Id,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  tBurn(
    from: AccountId,
    id: Id,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  tTransfer(
    from: AccountId,
    to: AccountId,
    id: Id,
    data: Array<number | string | BN>,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  tUpdate(
    from: AccountId | null,
    to: AccountId | null,
    id: Id,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
}

export interface PSP34Internal {
  readonly query: PSP34InternalQuery;
  readonly tx: PSP34InternalTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => PSP34Internal;
  withAddress: (address: string) => PSP34Internal;
  withAPI: (api: ApiPromise) => PSP34Internal;
}
