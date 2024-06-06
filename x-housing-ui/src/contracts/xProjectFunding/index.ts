import BigNumber from 'bignumber.js';
import { ContractWithAbi } from '..';
import abi from './x-project-funding.abi.json';
import { chainID, xProjectFundingAddress } from '@/config';
import { apiProvider } from '@/providers/apiProvider';
import {
  AbiRegistry,
  BinaryCodec,
  List,
  TokenTransfer,
  TypedValue
} from '@multiversx/sdk-core/out';
import { NonFungibleTokenOfAccountOnNetwork } from '@multiversx/sdk-network-providers/out';

const xProjectRegistry = AbiRegistry.create(abi);

export class XProjectData {
  constructor(
    readonly id: number,
    readonly address: string,
    readonly funding_goal: BigNumber,
    readonly funding_deadline: number,
    readonly funding_token_id: string,
    readonly collected_funds: BigNumber
  ) {}

  get isTokensClaimable(): boolean {
    return this.collected_funds.gte(this.funding_goal);
  }

  static fromBase64(base64String: string) {
    let type = xProjectRegistry.getStruct('XProjectData');

    const decoder = new BinaryCodec();
    const value = decoder.decodeTopLevel(
      Buffer.from(base64String, 'base64'),
      type
    );

    return XProjectData.fromTypedValue(value);
  }

  static fromTypedValue(obj: TypedValue) {
    const {
      id,
      address,
      funding_goal,
      funding_deadline,
      funding_token_id,
      collected_funds
    } = obj.valueOf();

    return new XProjectData(
      +id,
      address.bech32(),
      funding_goal,
      +funding_deadline,
      funding_token_id,
      collected_funds
    );
  }
}

class XProjectFundingSC extends ContractWithAbi {
  protected getAbiJson() {
    return abi;
  }

  async getAllXProjectData(): Promise<XProjectData[]> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getAllXProjectData'
    });

    return (<List>firstValue)
      .valueOf()
      .map((d) => XProjectData.fromTypedValue(d));
  }

  async getLkXhtID(): Promise<string> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getLkXhtID'
    });

    return firstValue?.valueOf();
  }

  makeFundProjectTx({
    projectID,
    amount,
    referrerID
  }: {
    projectID: number;
    referrerID?: number;
    amount: BigNumber;
  }) {
    return this.buildGenericTXEndPoint({
      endpoint: 'fundProject',
      args: [projectID, referrerID],
      customiser(interaction) {
        return interaction.withValue(amount).withGasLimit(50_000_000);
      }
    });
  }
  
  makeUnlockXhtTx({
    token,
    address
  }: {
    token: NonFungibleTokenOfAccountOnNetwork;
    address: string;
  }) {
    return this.buildGenericTXEndPoint({
      endpoint: 'unlockXht',
      customiser(interaction) {
        return interaction
          .withSender({ bech32: () => address })
          .withSingleESDTNFTTransfer(
            TokenTransfer.metaEsdtFromAmount(
              token.collection,
              token.nonce,
              token.balance,
              0
            )
          )
          .withGasLimit(50_000_000);
      }
    });
  }

  makeClaimXProjectTokenTx({ projectID }: { projectID: number }) {
    return this.buildGenericTXEndPoint({
      endpoint: 'claimXProjectToken',
      args: [projectID],
      customiser(interaction) {
        return interaction.withGasLimit(50_000_000);
      }
    });
  }
}

export const xProjectFundingSC = new XProjectFundingSC(
  xProjectFundingAddress,
  chainID,
  apiProvider
);
