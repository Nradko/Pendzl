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

export enum OwnableError {
  CallerIsNotOwner = "CallerIsNotOwner",
  ActionRedundant = "ActionRedundant",
}

interface OwnableQuery {
  burn(
    account: AccountId,
    amount: string | number | BN,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  mint(
    account: AccountId,
    amount: string | number | BN,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  totalSupply(
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<ReturnNumber, LangError>>>;
  balanceOf(
    owner: AccountId,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<ReturnNumber, LangError>>>;
  allowance(
    owner: AccountId,
    spender: AccountId,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<ReturnNumber, LangError>>>;
  transfer(
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  transferFrom(
    from: AccountId,
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  approve(
    spender: AccountId,
    value: string | number | BN,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  increaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  decreaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  owner(
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<AccountId | null, LangError>>>;
  renounceOwnership(
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, OwnableError>, LangError>>>;
  transferOwnership(
    newOwner: AccountId,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<Result<null, OwnableError>, LangError>>>;
}

interface OwnableTx {
  burn(
    account: AccountId,
    amount: string | number | BN,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  mint(
    account: AccountId,
    amount: string | number | BN,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  totalSupply(__options?: GasLimit): Promise<SignAndSendSuccessResponse>;
  balanceOf(
    owner: AccountId,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  allowance(
    owner: AccountId,
    spender: AccountId,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  transfer(
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  transferFrom(
    from: AccountId,
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  approve(
    spender: AccountId,
    value: string | number | BN,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  increaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  decreaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  owner(__options?: GasLimit): Promise<SignAndSendSuccessResponse>;
  renounceOwnership(__options?: GasLimit): Promise<SignAndSendSuccessResponse>;
  transferOwnership(
    newOwner: AccountId,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
}

export interface Ownable {
  readonly query: OwnableQuery;
  readonly tx: OwnableTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => Ownable;
  withAddress: (address: string) => Ownable;
  withAPI: (api: ApiPromise) => Ownable;
}
