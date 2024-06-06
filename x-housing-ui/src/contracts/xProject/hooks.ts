import useSwr from 'swr';

import { chainID } from '@/config';
import { XProjectSC } from '@/contracts/xProject';
import { XProjectData, xProjectFundingSC } from '@/contracts/xProjectFunding';
import { apiProvider } from '@/providers/apiProvider';
import BigNumber from 'bignumber.js';

export interface XProjectsValue {
  image: string;
  projectData: {
    data: XProjectData;
    maxSupply: BigNumber;
    tokenId: string;
    contract: XProjectSC;
  };
  features: string[];
  description: string;
  rentPrice: number;
  unitPrice: BigNumber;
}

export const useXProjects = (): XProjectsValue[] => {
  const { data, error } = useSwr('useXProjects-data', () =>
    xProjectFundingSC.getAllXProjectData().then((projectsData) =>
      // Merge with maxSupply
      Promise.all(
        projectsData.map(async (data) => {
          const contract = new XProjectSC(data.address, chainID, apiProvider);

          const { maxSupply, tokenId } = await contract.getInfo();
          return {
            data,
            maxSupply,
            tokenId,
            contract
          };
        })
      )
    )
  );

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
