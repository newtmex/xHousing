import BigNumber from 'bignumber.js';
import { ContractWithAbi } from '..';
import abi from './x-project.abi.json';
import { AbiRegistry } from '@multiversx/sdk-core/out';

const xProjectRegistry = AbiRegistry.create(abi);

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

  async getInfo(): Promise<{ tokenId: string; maxSupply: BigNumber }> {
    const [tokenId, maxSupply] = await Promise.all([
      this.getXPTokenId(),
      this.getXPTokenMaxSupply()
    ]);

    return { tokenId, maxSupply };
  }
}
