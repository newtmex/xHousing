import { ContractWithAbi } from '..';
import abi from './x-housing.abi.json';
import {
  AbiRegistry,
  CompositeValue,
  VariadicValue
} from '@multiversx/sdk-core/out';
import { chainID, xHousingAddress } from '@/config';
import { apiProvider } from '@/providers/apiProvider';

const xHousingRegistry = AbiRegistry.create(abi);

class XHousingSC extends ContractWithAbi {
  protected getAbiJson() {
    return abi;
  }

  async getUserReferrals({ id }: { id: number }): Promise<[number, string][]> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getUserReferrals',
      args: [id]
    });

    return (<CompositeValue[]>(<VariadicValue>firstValue).getItems()).map(
      (item) => {
        const [id, address] = item.valueOf();

        return [+id, address.bech32()];
      }
    );
  }

  async getUserAddress({ id }: { id: number }): Promise<string> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getUserAddress',
      args: [id]
    });

    return firstValue?.valueOf();
  }

  async getAffiliateDetails({
    address
  }: {
    address: string;
  }): Promise<[number, [number, string] | null]> {
    const { firstValue } = await this.sendEndpointQuery({
      endpoint: 'getAffiliateDetails',
      args: [address]
    });

    const { field0, field1 } = firstValue?.valueOf();

    return [+field0, field1];
  }
}

export const xHousingSC = new XHousingSC(xHousingAddress, chainID, apiProvider);
