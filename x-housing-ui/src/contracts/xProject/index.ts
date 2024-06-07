import BigNumber from 'bignumber.js';
import { ContractWithAbi } from '..';
import abi from './x-project.abi.json';
import {
  AbiRegistry,
  BinaryCodec,
  TokenTransfer,
  TypedValue
} from '@multiversx/sdk-core/out';

const xProjectRegistry = AbiRegistry.create(abi);

export class XPTokenAttributes {
  constructor(
    readonly reward_per_share: BigNumber,
    readonly token_weight: BigNumber,
    readonly original_owner: string
  ) {}

  static fromBase64(base64String: string) {
    const type = xProjectRegistry.getStruct('XPTokenAttributes');

    const decoder = new BinaryCodec();
    const value = decoder.decodeTopLevel(
      Buffer.from(base64String, 'base64'),
      type
    );

    return XPTokenAttributes.fromTypedValue(value);
  }

  static fromTypedValue(obj: TypedValue) {
    const { reward_per_share, token_weight, original_owner } = obj.valueOf();

    return new XPTokenAttributes(
      reward_per_share,
      token_weight,
      original_owner.bech32()
    );
  }
}

export class XProjectSC extends ContractWithAbi {
  protected getAbiJson() {
    return abi;
  }

  async getXPTokenMaxSupply(): Promise<BigNumber> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getXPTokenMaxSupply'
    });

    return firstValue?.valueOf();
  }

  async getXPTokenId(): Promise<string> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getXPTokenId'
    });

    return firstValue?.valueOf();
  }

  async getRentClaimAble({
    xptAttr
  }: {
    xptAttr: XPTokenAttributes;
  }): Promise<BigNumber> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getRentClaimAble',
      args: [xptAttr]
    });

    return firstValue?.valueOf();
  }

  async getInfo(): Promise<{ tokenId: string; maxSupply: BigNumber }> {
    const [tokenId, maxSupply] = await Promise.all([
      this.getXPTokenId(),
      this.getXPTokenMaxSupply()
    ]);

    return { tokenId, maxSupply };
  }

  makeReceiveRentTx({ payment }: { payment: TokenTransfer }) {
    return this.buildGenericTXEndPoint({
      endpoint: 'receiveRent',
      customiser(interaction) {
        return interaction
          .withGasLimit(50_000_000)
          .withSingleESDTTransfer(payment);
      }
    });
  }

  makeClaimRentRewardTx({
    payment,
    address
  }: {
    payment: TokenTransfer;
    address: string;
  }) {
    return this.buildGenericTXEndPoint({
      endpoint: 'claimRentReward',
      customiser(interaction) {
        return interaction
          .withSender({ bech32: () => address })
          .withGasLimit(50_000_000)
          .withSingleESDTNFTTransfer(payment);
      }
    });
  }
}
