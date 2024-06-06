import { INetworkProvider } from '@multiversx/sdk-network-providers/out/interface';
import {
  AbiRegistry,
  Address,
  BinaryCodec,
  ContractFunction,
  Interaction,
  ResultsParser,
  ReturnCode,
  SmartContract,
  TypedValue
} from '@multiversx/sdk-core/out';

import { XHousingnNetworkError } from '@/errors';

class Contract {
  protected contract: SmartContract;
  protected readonly resultParser = new ResultsParser();

  constructor(
    readonly contractAddress: string,
    readonly chainID: string,
    protected provider: INetworkProvider
  ) {
    this.contract = new SmartContract({
      address: new Address(contractAddress)
    });
  }

  async handleQueryResponse<
    Bundle extends { returnCode: ReturnCode; returnMessage: string }
  >(cb: () => Promise<Bundle>, skipErrMsgs: string[] = []) {
    const resp = await cb();
    const { returnCode, returnMessage } = resp;

    if (!returnCode.isSuccess()) {
      const found = skipErrMsgs.find(
        (errMsg) => errMsg.length > 4 && returnMessage.includes(errMsg.trim())
      );

      if (typeof found === 'undefined') {
        throw new XHousingnNetworkError({
          code: returnCode,
          message: returnMessage
        });
      }
    }

    return resp;
  }
}

export class ContractNoAbi extends Contract {
  protected decoder = new BinaryCodec();

  protected async queryEndpoint(
    endpoint: string,
    args?: TypedValue[],
    skipErrMsgs: string[] = []
  ) {
    try {
      return this.handleQueryResponse(async () => {
        const query = this.contract.createQuery({
          func: new ContractFunction(endpoint),
          args
        });

        const queryResp = await this.provider.queryContract(query);

        return this.resultParser.parseUntypedQueryResponse(queryResp);
      }, skipErrMsgs);
    } catch (error: any) {
      if (error instanceof TypeError) {
        error.message += `
  {
    endpoint: ${endpoint}
    args: ${JSON.stringify(args)}
  }
      `;
      }

      throw error;
    }
  }
}

export abstract class ContractWithAbi extends Contract {
  constructor(
    contractAddress: string,
    chainID: string,
    provider: INetworkProvider
  ) {
    super(contractAddress, chainID, provider);
    this.contract = new SmartContract({
      address: Address.fromBech32(contractAddress),
      abi: this.makeAbi()
    });
  }

  protected abstract getAbiJson(): any;

  private makeAbi = () => {
    const json = this.getAbiJson();

    return AbiRegistry.create(json);
  };

  protected async sendEndpointQuery({
    endpoint,
    args = [],
    skipErrMsgs = [],
    caller
  }: {
    caller?: string;
    endpoint: string;
    args?: any[];
    skipErrMsgs?: string[];
  }) {
    try {
      return this.handleQueryResponse(async () => {
        const interaction = this.contract.methods[endpoint](args);

        const query = interaction.check().buildQuery();
        caller && (query.caller = { bech32: () => caller });

        const queryResp = await this.provider.queryContract(query);
        const endpointDefinition = interaction.getEndpoint();

        return this.resultParser.parseQueryResponse(
          queryResp,
          endpointDefinition
        );
      }, skipErrMsgs);
    } catch (error: any) {
      if (error instanceof XHousingnNetworkError) {
        error.message += `
  {
    endpoint: ${endpoint}
    args: ${JSON.stringify(args)}
  }
      `;
      }

      throw error;
    }
  }

  protected buildGenericTXEndPoint({
    methodType = 'methods',
    endpoint,
    args,
    customiser
  }: {
    methodType?: 'methods' | 'methodsExplicit';
    endpoint: string;
    customiser: (interaction: Interaction) => Interaction;
    args?: any[];
  }) {
    const interaction = this.contract[methodType][endpoint](args).withChainID(
      this.chainID
    );

    return customiser(interaction).buildTransaction();
  }
}
