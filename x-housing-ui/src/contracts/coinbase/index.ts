import { ContractWithAbi } from '..';
import abi from './coinbase.abi.json';
import { chainID, coinbaseAddress } from '@/config';
import { apiProvider } from '@/providers/apiProvider';

// const coinbaseRegistry = AbiRegistry.create(abi);

class CoinbaseSC extends ContractWithAbi {
  protected getAbiJson() {
    return abi;
  }

  async getXhtID(): Promise<string> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getXhtID'
    });

    return firstValue?.valueOf();
  }
}

export const coinbaseSC = new CoinbaseSC(coinbaseAddress, chainID, apiProvider);
