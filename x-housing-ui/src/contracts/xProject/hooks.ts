import useSwr from 'swr';

import { chainID } from '@/config';
import { XProjectSC } from '@/contracts/xProject';
import { XProjectData, xProjectFundingSC } from '@/contracts/xProjectFunding';
import { apiProvider } from '@/providers/apiProvider';
import BigNumber from 'bignumber.js';
import { DefinitionOfTokenCollectionOnNetwork } from '@multiversx/sdk-network-providers/out';

export interface XProjectsValue {
  image: string;
  projectData: {
    data: XProjectData;
    maxSupply: BigNumber;
    tokenId: string;
    contract: XProjectSC;
    tokenInfo: DefinitionOfTokenCollectionOnNetwork;
  };
  features: string[];
  description: string;
  rentPrice: number;
  unitPrice: BigNumber;
}

export const useXProjectsInfo = () =>
  useSwr('useXProjects-data', () =>
    xProjectFundingSC.getAllXProjectData().then((projectsData) =>
      // Merge with maxSupply
      Promise.all(
        projectsData.map(async (data) => {
          const contract = new XProjectSC(data.address, chainID, apiProvider);

          const { maxSupply, tokenId } = await contract.getInfo();
          const tokenInfo =
            await apiProvider.getDefinitionOfTokenCollection(tokenId);

          return {
            data,
            maxSupply,
            tokenId,
            contract,
            tokenInfo
          };
        })
      )
    )
  );

export const useXProjects = (): XProjectsValue[] => {
  const { data } = useXProjectsInfo();

  return (
    data?.map((projectData, index) => ({
      description: '',
      features: ['ketchin', 'en-suite', 'cctv'],
      image: `img/property${((index + 2) % 3) + 1}.jpg`,
      rentPrice: 0,
      projectData,
      unitPrice: projectData.data.funding_goal.dividedBy(projectData.maxSupply)
    })) || []
  );
};
